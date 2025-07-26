use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info, warn};
use url::Url;
use uuid::Uuid;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

// Message types that match the server's expectations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum AgentMessage {
    Authenticate { 
        token: String, 
        #[serde(rename = "userId")]
        user_id: Option<String> 
    },
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
enum ServerMessage {
    CreateServer { server_name: String, server_type: String, version: String },
    StartServer { server_id: String },
    StopServer { server_id: String },
    SendCommand { server_id: String, command: String, command_id: String },
    RequestLogs { server_id: String, lines: usize },
    RequestServerList,
    RequestNodeInfo,
    Error { message: String },
}

// Agent configuration
#[derive(Debug, Deserialize)]
struct Config {
    agent: AgentConfig,
    servers: ServersConfig,
}

#[derive(Debug, Deserialize)]
struct AgentConfig {
    #[serde(default = "default_name")]
    name: String,
    port: u16,  // Port to listen on
    #[serde(default = "default_directory")]
    directory: String,  // Directory for server files
}

fn default_name() -> String {
    "VPS Node".to_string()
}

fn default_directory() -> String {
    "/var/lib/servermint".to_string()
}

#[derive(Debug, Deserialize, Clone)]
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
    process: Option<tokio::process::Child>,
    working_dir: PathBuf,
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

    async fn create_server(&mut self, name: String, server_type: String, version: String) -> Result<String> {
        let id = format!("server-{}", Uuid::new_v4());
        let working_dir = PathBuf::from(&self.config.directory).join(&id);

        // Create server directory
        tokio::fs::create_dir_all(&working_dir).await?;

        let server = ServerProcess {
            id: id.clone(),
            name,
            server_type,
            version,
            status: "stopped".to_string(),
            process: None,
            working_dir,
        };

        self.servers.insert(id.clone(), server);
        Ok(id)
    }

    async fn start_server(&mut self, id: &str) -> Result<()> {
        let server = self.servers.get_mut(id).ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        if server.status == "running" {
            return Ok(());
        }

        // Build the command based on server type
        let mut command = tokio::process::Command::new("java");
        command.current_dir(&server.working_dir);
        command.arg("-Xmx2G")
               .arg("-Xms1G")
               .arg("-jar")
               .arg(format!("{}.jar", server.server_type));

        // Start the process
        let process = command.spawn()?;
        server.process = Some(process);
        server.status = "running".to_string();

        Ok(())
    }

    async fn stop_server(&mut self, id: &str) -> Result<()> {
        let server = self.servers.get_mut(id).ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        if let Some(mut process) = server.process.take() {
            // Try graceful shutdown first
            process.kill().await?;
            server.status = "stopped".to_string();
        }

        Ok(())
    }

    async fn send_command(&mut self, id: &str, command: &str) -> Result<()> {
        let server = self.servers.get_mut(id).ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        if let Some(process) = &mut server.process {
            // TODO: Write command to process stdin
            Ok(())
        } else {
            Err(anyhow::anyhow!("Server not running"))
        }
    }

    async fn get_logs(&self, id: &str, lines: usize) -> Result<Vec<String>> {
        let server = self.servers.get(id).ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        // Read last N lines from logs
        let log_file = server.working_dir.join("logs/latest.log");
        if !log_file.exists() {
            return Ok(Vec::new());
        }

        let content = tokio::fs::read_to_string(log_file).await?;
        Ok(content.lines().rev().take(lines).map(String::from).collect())
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
        
        // Get disk usage
        let mut disk_usage = 0.0;
        
        // On Windows, we'll check C: drive
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
            use std::os::windows::ffi::OsStrExt;
            use std::ffi::OsStr;
            
            // Convert path to wide string
            let path = OsStr::new("C:\\");
            let mut wide_path: Vec<u16> = path.encode_wide().chain(std::iter::once(0)).collect();
            
            let mut total_bytes = 0;
            let mut free_bytes = 0;
            
            unsafe {
                if GetDiskFreeSpaceExW(
                    wide_path.as_mut_ptr(),
                    std::ptr::null_mut(),
                    &mut total_bytes,
                    &mut free_bytes,
                ) != 0
                {
                    if total_bytes > 0 {
                        disk_usage = ((total_bytes - free_bytes) as f32 / total_bytes as f32) * 100.0;
                    }
                }
            }
        }
        
        // On Unix systems, check root directory
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(stats) = nix::sys::statvfs::statvfs("/") {
                let total = stats.blocks() * stats.fragment_size() as u64;
                let free = stats.blocks_free() * stats.fragment_size() as u64;
                if total > 0 {
                    disk_usage = ((total - free) as f32 / total as f32) * 100.0;
                }
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
        let addr = format!("0.0.0.0:{}", self.config.agent.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("WebSocket server listening on {}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            info!("New connection from {}", addr);
            
            let ws_stream = accept_async(stream).await?;
            let (write, read) = ws_stream.split();
            
            // Create a channel for sending messages
            let (tx, mut rx) = mpsc::channel(32);
            
            // Spawn task to forward messages to WebSocket
            let mut write = write;
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if let Err(e) = write.send(msg).await {
                        error!("Failed to send WebSocket message: {}", e);
                        break;
                    }
                }
            });
            
            // Handle incoming messages
            let mut read = read;
            while let Some(msg) = read.next().await {
                let msg = msg?;
                if let Message::Text(text) = msg {
                    info!("Received message: {}", text);
                    
                    // Parse and handle message
                    if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                        if let Err(e) = self.handle_server_message(server_msg, &tx).await {
                            error!("Error handling server message: {}", e);
                            let error_msg = AgentMessage::Error {
                                message: e.to_string(),
                            };
                            tx.send(Message::Text(serde_json::to_string(&error_msg)?)).await?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    async fn handle_server_message(
        &self,
        msg: ServerMessage,
        tx: &mpsc::Sender<Message>,
    ) -> Result<()> {
        match msg {
            ServerMessage::RequestNodeInfo => {
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
                tx.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ServerMessage::RequestServerList => {
                let servers = {
                    let server_manager = self.server_manager.lock().await;
                    server_manager.list_servers()
                };
                let response = AgentMessage::ServerList { servers };
                tx.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ServerMessage::CreateServer { server_name, server_type, version } => {
                let id = {
                    let mut server_manager = self.server_manager.lock().await;
                    server_manager.create_server(server_name, server_type, version).await?
                };
                let response = AgentMessage::CommandResult {
                    command_id: "create".to_string(),
                    success: true,
                    message: format!("Server created with ID: {}", id),
                };
                tx.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ServerMessage::StartServer { server_id } => {
                {
                    let mut server_manager = self.server_manager.lock().await;
                    server_manager.start_server(&server_id).await?;
                }
                let response = AgentMessage::ServerStatus {
                    server_id,
                    status: "running".to_string(),
                };
                tx.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ServerMessage::StopServer { server_id } => {
                {
                    let mut server_manager = self.server_manager.lock().await;
                    server_manager.stop_server(&server_id).await?;
                }
                let response = AgentMessage::ServerStatus {
                    server_id,
                    status: "stopped".to_string(),
                };
                tx.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ServerMessage::SendCommand { server_id, command, command_id } => {
                let result = {
                    let mut server_manager = self.server_manager.lock().await;
                    server_manager.send_command(&server_id, &command).await
                };
                let response = AgentMessage::CommandResult {
                    command_id,
                    success: result.is_ok(),
                    message: result.err().map(|e| e.to_string()).unwrap_or_else(|| "Command sent".to_string()),
                };
                tx.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ServerMessage::RequestLogs { server_id, lines } => {
                let logs = {
                    let server_manager = self.server_manager.lock().await;
                    server_manager.get_logs(&server_id, lines).await?
                };
                let response = AgentMessage::ServerLogs {
                    server_id,
                    logs,
                };
                tx.send(Message::Text(serde_json::to_string(&response)?)).await?;
            }
            ServerMessage::Error { message } => {
                error!("Received error from server: {}", message);
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