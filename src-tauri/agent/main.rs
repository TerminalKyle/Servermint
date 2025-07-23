use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio::sync::mpsc;
use tokio::task;
use url::Url;
use log::{info, error, warn, debug};
use sysinfo::{System, SystemExt, ProcessorExt, DiskExt};
use std::sync::{Arc, Mutex};

// Message types for WebSocket communication
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentServer {
    id: String,
    name: String,
    server_type: String,
    version: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentConfig {
    name: String,
    token: String,
    ws_url: String,
}

#[derive(Debug, Clone)]
struct ServerProcess {
    id: String,
    name: String,
    server_type: String,
    version: String,
    status: String,
    path: String,
    process: Option<u32>, // PID
}

struct Agent {
    config: AgentConfig,
    servers_dir: String,
    servers: Arc<Mutex<Vec<ServerProcess>>>,
}

impl Agent {
    fn new(config: AgentConfig) -> Self {
        let servers_dir = env::var("SERVERMINT_SERVERS_DIR")
            .unwrap_or_else(|_| "/var/lib/servermint/servers".to_string());
        
        Self {
            config,
            servers_dir,
            servers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Load existing servers
        self.load_servers()?;
        
        // Connect to WebSocket server
        let url = Url::parse(&self.config.ws_url)?;
        let (ws_stream, _) = connect_async(url).await?;
        info!("Connected to WebSocket server");
        
        // Split the WebSocket stream
        let (mut write, mut read) = ws_stream.split();
        
        // Authenticate
        let auth_message = AgentMessage::Authenticate { token: self.config.token.clone() };
        let auth_json = serde_json::to_string(&auth_message)?;
        write.send(Message::Text(auth_json)).await?;
        info!("Sent authentication token");
        
        // Create channels for communication between tasks
        let (tx, mut rx) = mpsc::channel(100);
        
        // Clone references for the tasks
        let servers = self.servers.clone();
        let config = self.config.clone();
        
        // Start system monitoring task
        let tx_clone = tx.clone();
        task::spawn(async move {
            loop {
                // Collect system info
                let mut system = System::new_all();
                system.refresh_all();
                
                // Calculate CPU usage
                let cpu_usage = system.processors().iter()
                    .map(|p| p.cpu_usage())
                    .sum::<f32>() / system.processors().len() as f32;
                
                // Calculate memory usage
                let total_memory = system.total_memory() as f32;
                let used_memory = (system.total_memory() - system.available_memory()) as f32;
                let memory_usage = (used_memory / total_memory) * 100.0;
                
                // Calculate disk usage
                let mut total_space = 0;
                let mut used_space = 0;
                for disk in system.disks() {
                    total_space += disk.total_space();
                    used_space += disk.total_space() - disk.available_space();
                }
                let disk_usage = if total_space > 0 {
                    (used_space as f32 / total_space as f32) * 100.0
                } else {
                    0.0
                };
                
                // Send node info
                let hostname = system.host_name().unwrap_or_else(|| config.name.clone());
                let node_info = AgentMessage::NodeInfo {
                    hostname,
                    cpu: cpu_usage,
                    memory: memory_usage,
                    disk: disk_usage,
                };
                
                if let Err(e) = tx_clone.send(node_info).await {
                    error!("Failed to send node info: {}", e);
                }
                
                // Wait 30 seconds before next update
                sleep(Duration::from_secs(30)).await;
            }
        });
        
        // Start server status monitoring task
        let tx_clone = tx.clone();
        let servers_clone = servers.clone();
        task::spawn(async move {
            loop {
                // Get server list
                let server_list = {
                    let servers = servers_clone.lock().unwrap();
                    servers.iter().map(|s| AgentServer {
                        id: s.id.clone(),
                        name: s.name.clone(),
                        server_type: s.server_type.clone(),
                        version: s.version.clone(),
                        status: s.status.clone(),
                    }).collect::<Vec<_>>()
                };
                
                // Send server list
                let server_list_msg = AgentMessage::ServerList { servers: server_list };
                if let Err(e) = tx_clone.send(server_list_msg).await {
                    error!("Failed to send server list: {}", e);
                }
                
                // Wait 60 seconds before next update
                sleep(Duration::from_secs(60)).await;
            }
        });
        
        // Main event loop
        loop {
            tokio::select! {
                // Handle incoming WebSocket messages
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            // Parse the message
                            match serde_json::from_str::<ClientMessage>(&text) {
                                Ok(client_msg) => {
                                    match client_msg {
                                        ClientMessage::CreateServer { server_name, server_type, version } => {
                                            info!("Received create server request: {}", server_name);
                                            let result = self.create_server(server_name, server_type, version).await;
                                            match result {
                                                Ok(server_id) => {
                                                    let result_msg = AgentMessage::CommandResult {
                                                        command_id: "create_server".to_string(),
                                                        success: true,
                                                        message: format!("Server created with ID: {}", server_id),
                                                    };
                                                    tx.send(result_msg).await?;
                                                },
                                                Err(e) => {
                                                    let error_msg = AgentMessage::Error {
                                                        message: format!("Failed to create server: {}", e),
                                                    };
                                                    tx.send(error_msg).await?;
                                                }
                                            }
                                        },
                                        ClientMessage::StartServer { server_id } => {
                                            info!("Received start server request: {}", server_id);
                                            let result = self.start_server(&server_id).await;
                                            match result {
                                                Ok(_) => {
                                                    let result_msg = AgentMessage::CommandResult {
                                                        command_id: "start_server".to_string(),
                                                        success: true,
                                                        message: format!("Server started: {}", server_id),
                                                    };
                                                    tx.send(result_msg).await?;
                                                },
                                                Err(e) => {
                                                    let error_msg = AgentMessage::Error {
                                                        message: format!("Failed to start server: {}", e),
                                                    };
                                                    tx.send(error_msg).await?;
                                                }
                                            }
                                        },
                                        ClientMessage::StopServer { server_id } => {
                                            info!("Received stop server request: {}", server_id);
                                            let result = self.stop_server(&server_id).await;
                                            match result {
                                                Ok(_) => {
                                                    let result_msg = AgentMessage::CommandResult {
                                                        command_id: "stop_server".to_string(),
                                                        success: true,
                                                        message: format!("Server stopped: {}", server_id),
                                                    };
                                                    tx.send(result_msg).await?;
                                                },
                                                Err(e) => {
                                                    let error_msg = AgentMessage::Error {
                                                        message: format!("Failed to stop server: {}", e),
                                                    };
                                                    tx.send(error_msg).await?;
                                                }
                                            }
                                        },
                                        ClientMessage::SendCommand { server_id, command, command_id } => {
                                            info!("Received command for server {}: {}", server_id, command);
                                            let result = self.send_command(&server_id, &command).await;
                                            match result {
                                                Ok(_) => {
                                                    let result_msg = AgentMessage::CommandResult {
                                                        command_id,
                                                        success: true,
                                                        message: format!("Command executed: {}", command),
                                                    };
                                                    tx.send(result_msg).await?;
                                                },
                                                Err(e) => {
                                                    let error_msg = AgentMessage::Error {
                                                        message: format!("Failed to execute command: {}", e),
                                                    };
                                                    tx.send(error_msg).await?;
                                                }
                                            }
                                        },
                                        ClientMessage::RequestLogs { server_id, lines } => {
                                            info!("Received request for logs of server {}", server_id);
                                            let result = self.get_server_logs(&server_id, lines).await;
                                            match result {
                                                Ok(logs) => {
                                                    let logs_msg = AgentMessage::ServerLogs {
                                                        server_id,
                                                        logs,
                                                    };
                                                    tx.send(logs_msg).await?;
                                                },
                                                Err(e) => {
                                                    let error_msg = AgentMessage::Error {
                                                        message: format!("Failed to get server logs: {}", e),
                                                    };
                                                    tx.send(error_msg).await?;
                                                }
                                            }
                                        },
                                        ClientMessage::RequestServerList => {
                                            info!("Received request for server list");
                                            let server_list = {
                                                let servers = servers.lock().unwrap();
                                                servers.iter().map(|s| AgentServer {
                                                    id: s.id.clone(),
                                                    name: s.name.clone(),
                                                    server_type: s.server_type.clone(),
                                                    version: s.version.clone(),
                                                    status: s.status.clone(),
                                                }).collect::<Vec<_>>()
                                            };
                                            
                                            let server_list_msg = AgentMessage::ServerList { servers: server_list };
                                            tx.send(server_list_msg).await?;
                                        },
                                        ClientMessage::RequestNodeInfo => {
                                            info!("Received request for node info");
                                            // Collect system info
                                            let mut system = System::new_all();
                                            system.refresh_all();
                                            
                                            // Calculate CPU usage
                                            let cpu_usage = system.processors().iter()
                                                .map(|p| p.cpu_usage())
                                                .sum::<f32>() / system.processors().len() as f32;
                                            
                                            // Calculate memory usage
                                            let total_memory = system.total_memory() as f32;
                                            let used_memory = (system.total_memory() - system.available_memory()) as f32;
                                            let memory_usage = (used_memory / total_memory) * 100.0;
                                            
                                            // Calculate disk usage
                                            let mut total_space = 0;
                                            let mut used_space = 0;
                                            for disk in system.disks() {
                                                total_space += disk.total_space();
                                                used_space += disk.total_space() - disk.available_space();
                                            }
                                            let disk_usage = if total_space > 0 {
                                                (used_space as f32 / total_space as f32) * 100.0
                                            } else {
                                                0.0
                                            };
                                            
                                            // Send node info
                                            let hostname = system.host_name().unwrap_or_else(|| config.name.clone());
                                            let node_info = AgentMessage::NodeInfo {
                                                hostname,
                                                cpu: cpu_usage,
                                                memory: memory_usage,
                                                disk: disk_usage,
                                            };
                                            
                                            tx.send(node_info).await?;
                                        },
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to parse client message: {}", e);
                                    let error_msg = AgentMessage::Error {
                                        message: format!("Failed to parse message: {}", e),
                                    };
                                    tx.send(error_msg).await?;
                                }
                            }
                        },
                        Some(Ok(_)) => {}, // Ignore other message types
                        Some(Err(e)) => {
                            error!("WebSocket error: {}", e);
                            break;
                        },
                        None => {
                            info!("WebSocket connection closed");
                            break;
                        }
                    }
                },
                // Handle outgoing messages
                Some(msg) = rx.recv() => {
                    let json = serde_json::to_string(&msg)?;
                    write.send(Message::Text(json)).await?;
                }
            }
        }
        
        Ok(())
    }
    
    fn load_servers(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create servers directory if it doesn't exist
        if !Path::new(&self.servers_dir).exists() {
            fs::create_dir_all(&self.servers_dir)?;
        }
        
        // Scan servers directory for server folders
        let entries = fs::read_dir(&self.servers_dir)?;
        let mut servers = Vec::new();
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let server_id = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Read server.properties if it exists
                let properties_path = path.join("server.properties");
                let mut name = server_id.clone();
                let mut server_type = "vanilla".to_string();
                let mut version = "unknown".to_string();
                
                if properties_path.exists() {
                    let properties = fs::read_to_string(properties_path)?;
                    
                    // Parse server.properties
                    for line in properties.lines() {
                        if line.starts_with("motd=") {
                            name = line.trim_start_matches("motd=").to_string();
                        }
                    }
                }
                
                // Check if this is a Paper server
                if path.join("cache/mojang_1.20.1.jar").exists() {
                    server_type = "paper".to_string();
                    version = "1.20.1".to_string();
                }
                
                // Check server status
                let status = if path.join("logs/latest.log").exists() {
                    // TODO: Check if server is actually running
                    "offline".to_string()
                } else {
                    "offline".to_string()
                };
                
                servers.push(ServerProcess {
                    id: server_id,
                    name,
                    server_type,
                    version,
                    status,
                    path: path.to_string_lossy().to_string(),
                    process: None,
                });
            }
        }
        
        info!("Loaded {} servers", servers.len());
        let mut servers_lock = self.servers.lock().unwrap();
        *servers_lock = servers;
        
        Ok(())
    }
    
    async fn create_server(&self, name: String, server_type: String, version: String) -> Result<String, Box<dyn std::error::Error>> {
        // Generate server ID
        let server_id = format!("server-{}", uuid::Uuid::new_v4().to_string());
        let server_path = format!("{}/{}", self.servers_dir, server_id);
        
        // Create server directory
        fs::create_dir_all(&server_path)?;
        
        // Download server jar
        let jar_url = match (server_type.as_str(), version.as_str()) {
            ("paper", "1.20.1") => "https://api.papermc.io/v2/projects/paper/versions/1.20.1/builds/196/downloads/paper-1.20.1-196.jar",
            ("vanilla", "1.20.1") => "https://piston-data.mojang.com/v1/objects/84194a2f069e8c8a6823784a861e2b65ba9a1edd/server.jar",
            _ => return Err("Unsupported server type or version".into()),
        };
        
        // Download server jar
        let jar_path = format!("{}/server.jar", server_path);
        let output = Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&jar_path)
            .arg(jar_url)
            .output()?;
        
        if !output.status.success() {
            return Err(format!("Failed to download server jar: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        // Create server.properties
        let properties = format!(
            "motd={}\nserver-port=25565\nquery.port=25565\nenable-query=false\nspawn-protection=16\nmax-tick-time=60000\ndifficulty=easy\ngamemode=survival\nmax-players=20\nspawn-npcs=true\nspawn-animals=true\ngenerate-structures=true\nonline-mode=true\n",
            name
        );
        fs::write(format!("{}/server.properties", server_path), properties)?;
        
        // Create eula.txt
        fs::write(format!("{}/eula.txt", server_path), "eula=true\n")?;
        
        // Create start script
        let start_script = format!(
            "#!/bin/sh\njava -Xmx2G -Xms1G -jar server.jar nogui\n"
        );
        let start_script_path = format!("{}/start.sh", server_path);
        fs::write(&start_script_path, start_script)?;
        
        // Make start script executable
        Command::new("chmod")
            .arg("+x")
            .arg(&start_script_path)
            .output()?;
        
        // Add server to list
        let server = ServerProcess {
            id: server_id.clone(),
            name,
            server_type,
            version,
            status: "offline".to_string(),
            path: server_path,
            process: None,
        };
        
        let mut servers = self.servers.lock().unwrap();
        servers.push(server);
        
        Ok(server_id)
    }
    
    async fn start_server(&self, server_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Find server
        let mut servers = self.servers.lock().unwrap();
        let server = servers.iter_mut().find(|s| s.id == *server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        // Check if server is already running
        if server.process.is_some() {
            return Err("Server is already running".into());
        }
        
        // Start server process
        let mut command = Command::new("java");
        command
            .arg("-Xmx2G")
            .arg("-Xms1G")
            .arg("-jar")
            .arg("server.jar")
            .arg("nogui")
            .current_dir(&server.path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        let child = command.spawn()?;
        let pid = child.id();
        
        // Update server status
        server.status = "starting".to_string();
        server.process = Some(pid);
        
        // Spawn a task to monitor the process
        let server_id = server_id.to_string();
        let servers = self.servers.clone();
        task::spawn(async move {
            // Wait for process to exit
            let status = Command::new("sh")
                .arg("-c")
                .arg(format!("wait {}", pid))
                .status();
            
            // Update server status
            let mut servers = servers.lock().unwrap();
            if let Some(server) = servers.iter_mut().find(|s| s.id == server_id) {
                server.status = "offline".to_string();
                server.process = None;
            }
        });
        
        Ok(())
    }
    
    async fn stop_server(&self, server_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Find server
        let mut servers = self.servers.lock().unwrap();
        let server = servers.iter_mut().find(|s| s.id == *server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        // Check if server is running
        let pid = server.process.ok_or("Server is not running")?;
        
        // Stop server process
        let output = Command::new("kill")
            .arg(pid.to_string())
            .output()?;
        
        if !output.status.success() {
            return Err(format!("Failed to stop server: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        // Update server status
        server.status = "stopping".to_string();
        
        Ok(())
    }
    
    async fn send_command(&self, server_id: &str, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Find server
        let servers = self.servers.lock().unwrap();
        let server = servers.iter().find(|s| s.id == *server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        // Check if server is running
        let _pid = server.process.ok_or("Server is not running")?;
        
        // TODO: Send command to server process
        // For now, just log the command
        info!("Would send command to server {}: {}", server_id, command);
        
        Ok(())
    }
    
    async fn get_server_logs(&self, server_id: &str, lines: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Find server
        let servers = self.servers.lock().unwrap();
        let server = servers.iter().find(|s| s.id == *server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        // Read log file
        let log_path = format!("{}/logs/latest.log", server.path);
        
        if !Path::new(&log_path).exists() {
            return Ok(vec!["No logs available".to_string()]);
        }
        
        // Use tail to get the last N lines
        let output = Command::new("tail")
            .arg("-n")
            .arg(lines.to_string())
            .arg(&log_path)
            .output()?;
        
        if !output.status.success() {
            return Err(format!("Failed to read log file: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        let logs = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.to_string())
            .collect();
        
        Ok(logs)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Load configuration
    let config_path = env::var("SERVERMINT_CONFIG")
        .unwrap_or_else(|_| "/etc/servermint/config.toml".to_string());
    
    info!("Loading configuration from {}", config_path);
    
    let config_content = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read config file: {}", e);
            return Err(e.into());
        }
    };
    
    let config: toml::Value = match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to parse config file: {}", e);
            return Err(e.into());
        }
    };
    
    // Extract agent configuration
    let agent_config = AgentConfig {
        name: config["agent"]["name"].as_str().unwrap_or("unknown").to_string(),
        token: config["agent"]["token"].as_str().unwrap_or("").to_string(),
        ws_url: config["agent"]["ws_url"].as_str().unwrap_or("wss://api.servermint.gg/ws").to_string(),
    };
    
    info!("Starting Servermint agent with name: {}", agent_config.name);
    
    // Create and run agent
    let agent = Agent::new(agent_config);
    
    // Run agent with reconnect logic
    loop {
        info!("Connecting to WebSocket server...");
        match agent.run().await {
            Ok(_) => {
                info!("Agent exited gracefully");
                break;
            },
            Err(e) => {
                error!("Agent error: {}", e);
                info!("Reconnecting in 5 seconds...");
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
    
    Ok(())
} 