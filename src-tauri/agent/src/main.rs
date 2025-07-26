use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info, warn};
use url::Url;

// Message types that match the server's expectations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum AgentMessage {
    Authenticate { token: String },
    NodeInfo { hostname: String, cpu: f32, memory: f32, disk: f32 },
    ServerList { servers: Vec<AgentServer> },
    ServerStatus { server_id: String, status: String },
    ServerLogs { server_id: String, logs: Vec<String> },
    CommandResult { command_id: String, success: bool, message: String },
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentServer {
    id: String,
    name: String,
    server_type: String,
    version: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ClientMessage {
    CreateServer { server_name: String, server_type: String, version: String },
    StartServer { server_id: String },
    StopServer { server_id: String },
    SendCommand { server_id: String, command: String, command_id: String },
    RequestLogs { server_id: String, lines: usize },
    RequestServerList,
    RequestNodeInfo,
}

// Agent configuration
#[derive(Debug, Deserialize)]
struct Config {
    agent: AgentConfig,
    servers: ServersConfig,
}

#[derive(Debug, Deserialize)]
struct AgentConfig {
    name: String,
    token: String,
    ws_url: String,
}

#[derive(Debug, Deserialize)]
struct ServersConfig {
    directory: String,
}

// Server process manager
struct ServerManager {
    servers: HashMap<String, ServerProcess>,
    config: ServersConfig,
}

struct ServerProcess {
    id: String,
    name: String,
    server_type: String,
    version: String,
    status: String,
    // Add process handle, working directory, etc.
}

impl ServerManager {
    fn new(config: ServersConfig) -> Self {
        Self {
            servers: HashMap::new(),
            config,
        }
    }

    fn list_servers(&self) -> Vec<AgentServer> {
        self.servers
            .values()
            .map(|s| AgentServer {
                id: s.id.clone(),
                name: s.name.clone(),
                server_type: s.server_type.clone(),
                version: s.version.clone(),
                status: s.status.clone(),
            })
            .collect()
    }
}

// System metrics collector
struct SystemMonitor {
    sys: sysinfo::System,
}

impl SystemMonitor {
    fn new() -> Self {
        let mut sys = sysinfo::System::new();
        sys.refresh_all();
        Self { sys }
    }

    fn get_metrics(&mut self) -> (f32, f32, f32) {
        self.sys.refresh_all();
        
        let cpu = self.sys.global_cpu_info().cpu_usage();
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let memory = if total_mem > 0 {
            (used_mem as f32 / total_mem as f32) * 100.0
        } else {
            0.0
        };
        
        // Get root disk usage
        let disks = self.sys.disks();
        let mut disk_usage = 0.0;
        if let Some(disk) = disks.iter().find(|d| d.mount_point() == std::path::Path::new("/")) {
            let total = disk.total_space();
            let free = disk.available_space();
            if total > 0 {
                disk_usage = ((total - free) as f32 / total as f32) * 100.0;
            }
        }

        (cpu, memory, disk_usage)
    }
}

// Main agent struct
struct Agent {
    config: Config,
    server_manager: Arc<Mutex<ServerManager>>,
    system_monitor: Arc<Mutex<SystemMonitor>>,
}

impl Agent {
    async fn new(config_path: PathBuf) -> Result<Self> {
        // Load configuration
        let config_str = tokio::fs::read_to_string(&config_path)
            .await
            .context("Failed to read config file")?;
        let config: Config = toml::from_str(&config_str).context("Failed to parse config")?;

        // Initialize components
        let server_manager = Arc::new(Mutex::new(ServerManager::new(config.servers.clone())));
        let system_monitor = Arc::new(Mutex::new(SystemMonitor::new()));

        Ok(Self {
            config,
            server_manager,
            system_monitor,
        })
    }

    async fn run(&self) -> Result<()> {
        let ws_url = Url::parse(&self.config.agent.ws_url)?;
        
        loop {
            match self.connect_and_handle(ws_url.clone()).await {
                Ok(_) => {
                    info!("WebSocket connection closed normally");
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                }
            }

            // Wait before reconnecting
            warn!("Reconnecting in 5 seconds...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    async fn connect_and_handle(&self, ws_url: Url) -> Result<()> {
        info!("Connecting to {}", ws_url);
        let (ws_stream, _) = connect_async(ws_url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Send authentication message
        let auth_msg = AgentMessage::Authenticate {
            token: self.config.agent.token.clone(),
        };
        write.send(Message::Text(serde_json::to_string(&auth_msg)?)).await?;

        // Start system metrics reporting task
        let system_monitor = Arc::clone(&self.system_monitor);
        let mut write_clone = write.clone();
        tokio::spawn(async move {
            loop {
                let (cpu, memory, disk) = {
                    let mut monitor = system_monitor.lock().await;
                    monitor.get_metrics()
                };

                let metrics_msg = AgentMessage::NodeInfo {
                    hostname: hostname::get()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    cpu,
                    memory,
                    disk,
                };

                if let Err(e) = write_clone
                    .send(Message::Text(serde_json::to_string(&metrics_msg).unwrap()))
                    .await
                {
                    error!("Failed to send metrics: {}", e);
                    break;
                }

                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });

        // Handle incoming messages
        while let Some(msg) = read.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                let client_msg: ClientMessage = serde_json::from_str(&text)?;
                self.handle_client_message(client_msg, &mut write).await?;
            }
        }

        Ok(())
    }

    async fn handle_client_message(
        &self,
        msg: ClientMessage,
        write: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            Message,
        >,
    ) -> Result<()> {
        match msg {
            ClientMessage::RequestNodeInfo => {
                let (cpu, memory, disk) = {
                    let mut monitor = self.system_monitor.lock().await;
                    monitor.get_metrics()
                };

                let response = AgentMessage::NodeInfo {
                    hostname: hostname::get()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    cpu,
                    memory,
                    disk,
                };
                write.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ClientMessage::RequestServerList => {
                let servers = {
                    let server_manager = self.server_manager.lock().await;
                    server_manager.list_servers()
                };
                let response = AgentMessage::ServerList { servers };
                write.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            // Implement other message handlers
            _ => {
                warn!("Unhandled message type: {:?}", msg);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get config path from environment or use default
    let config_path = std::env::var("SERVERMINT_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/etc/servermint/config.toml"));

    // Create and run agent
    let agent = Agent::new(config_path).await?;
    agent.run().await?;

    Ok(())
} 