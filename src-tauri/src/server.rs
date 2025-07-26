use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tauri::State;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, Read};
use std::thread;
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub path: String,
    pub version: String,
    pub server_type: String,
    pub java_path: Option<String>,
    pub min_memory: u32,
    pub max_memory: u32,
    pub jvm_args: Option<String>,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub id: String,
    pub config: ServerConfig,
    pub status: String,
    pub players: u32,
    pub max_players: u32,
}

#[derive(Debug)]
pub struct ServerManager {
    servers: HashMap<String, ServerProcess>,
    persistence_file: String,
}

#[derive(Debug)]
struct ServerProcess {
    config: ServerConfig,
    process: Option<Child>,
    status: String,
    output: Arc<Mutex<Vec<String>>>,
    stdin: Option<std::process::ChildStdin>,
}

impl ServerManager {
    pub fn new() -> Self {
        let app_data_dir = std::env::var("APPDATA")
            .unwrap_or_else(|_| std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        let persistence_file = format!("{}/ServerMint/servers.json", app_data_dir);
        
        let mut manager = ServerManager {
            servers: HashMap::new(),
            persistence_file,
        };
        
        // Load existing servers from disk
        if let Err(e) = manager.load_servers() {
            println!("Warning: Failed to load servers from disk: {}", e);
        }
        
        manager
    }

    pub fn add_server(&mut self, id: String, config: ServerConfig) -> Result<(), String> {
        if self.servers.contains_key(&id) {
            return Err("Server with this ID already exists".to_string());
        }

        self.servers.insert(id.clone(), ServerProcess {
            config,
            process: None,
            status: "offline".to_string(),
            output: Arc::new(Mutex::new(Vec::new())),
            stdin: None,
        });

        // Save to disk after adding
        if let Err(e) = self.save_servers() {
            println!("Warning: Failed to save servers after adding {}: {}", id, e);
        }

        Ok(())
    }

    pub fn start_server(&mut self, id: &str) -> Result<(), String> {
        let server = self.servers.get_mut(id).ok_or_else(|| format!("Server {} not found", id))?;
        
        if server.process.is_some() {
            return Err("Server is already running".to_string());
        }

        // Check if server files exist
        let is_bedrock = server.config.server_type.to_lowercase() == "pocketmine";
        
        if is_bedrock {
            let phar_path = format!("{}/PocketMine-MP.phar", server.config.path);
            if !std::path::Path::new(&phar_path).exists() {
                return Err(format!("PocketMine-MP.phar not found at {}", phar_path));
            }
        } else {
            let jar_path = format!("{}/server.jar", server.config.path);
            if !std::path::Path::new(&jar_path).exists() {
                return Err(format!("Server JAR not found at {}", jar_path));
            }
        }

        // Try to start the server
        self.try_start_server(id)
    }
    
    fn try_start_server(&mut self, id: &str) -> Result<(), String> {
        let server = self.servers.get_mut(id).ok_or_else(|| format!("Server {} not found", id))?;

        // Check if this is a Bedrock server
        let is_bedrock = server.config.server_type.to_lowercase() == "pocketmine";

        if is_bedrock {
            // For Bedrock servers, we use PHP
            // Try to find PHP in the server directory first, then system PATH
            let php_path = if std::path::Path::new(&format!("{}/bin/php/php.exe", server.config.path)).exists() {
                format!("{}/bin/php/php.exe", server.config.path)
            } else if std::path::Path::new(&format!("{}/php.exe", server.config.path)).exists() {
                format!("{}/php.exe", server.config.path)
            } else {
                "php".to_string() // Fall back to system PHP
            };
            
            println!("Starting Bedrock server {} with PHP at: {}", id, php_path);
            
            let mut command = Command::new(&php_path);
            command.arg("PocketMine-MP.phar");
            
            // Set working directory
            command.current_dir(&server.config.path);
            
            // Capture stdout, stderr, and stdin
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
            command.stdin(Stdio::piped());
            
            // Hide the window on Windows
            #[cfg(target_os = "windows")]
            command.creation_flags(0x08000000); // CREATE_NO_WINDOW flag
            
            println!("Executing Bedrock command: {:?}", command);
            
            // Start the process
            match command.spawn() {
                Ok(mut child) => {
                    println!("Bedrock server {} started successfully with PID: {}", id, child.id());
                    
                    // Get stdout, stderr, and stdin handles
                    let stdout = child.stdout.take().expect("Failed to capture stdout");
                    let stderr = child.stderr.take().expect("Failed to capture stderr");
                    let stdin = child.stdin.take().expect("Failed to capture stdin");
                    
                    // Clone the server output Arc for the threads
                    let output_clone = server.output.clone();
                    
                    // Spawn a thread to read stdout
                    thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                if let Ok(mut output) = output_clone.lock() {
                                    output.push(line);
                                }
                            }
                        }
                    });
                    
                    // Spawn a thread to read stderr
                    let output_clone = server.output.clone();
                    thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                if let Ok(mut output) = output_clone.lock() {
                                    output.push(line);
                                }
                            }
                        }
                    });
                    
                    server.process = Some(child);
                    server.stdin = Some(stdin);
                    server.status = "online".to_string();
                    
                    Ok(())
                },
                Err(e) => {
                    println!("Failed to start Bedrock server {}: {}", id, e);
                    Err(format!("Failed to start Bedrock server: {}", e))
                },
            }
        } else {
            // For Java servers, use the existing logic
            // Build the Java command
            let java_path = if let Some(custom_path) = &server.config.java_path {
                custom_path.clone()
            } else {
                // Try to get the appropriate Java version for this server
                let version = &server.config.version;
                let required_java = if version.starts_with("1.21") || version.starts_with("1.22") {
                    "21"
                } else if version.starts_with("1.20") || version.starts_with("1.19") {
                    "17"
                } else {
                    "17"
                };
                
                // Try to get local Java first, fall back to system Java
                match get_java_path_internal(Some(required_java.to_string())) {
                    Ok(path) => path,
                    Err(_) => "java".to_string()
                }
            };
            
            println!("Starting Java server {} with Java path: {}", id, java_path);
            
            let mut command = Command::new(&java_path);
            
            // Add memory settings
            command.arg(format!("-Xms{}M", server.config.min_memory));
            command.arg(format!("-Xmx{}M", server.config.max_memory));
            
            // Add custom JVM args if any
            if let Some(jvm_args) = &server.config.jvm_args {
                for arg in jvm_args.split_whitespace() {
                    command.arg(arg);
                }
            }
            
            // Add jar file
            command.arg("-jar");
            command.arg("server.jar");
            command.arg("nogui");
            
            // Set working directory
            command.current_dir(&server.config.path);
            
            // Capture stdout, stderr, and stdin
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
            command.stdin(Stdio::piped());
            
            // Hide the window on Windows - use CREATE_NO_WINDOW to prevent command prompt from showing
            #[cfg(target_os = "windows")]
            command.creation_flags(0x08000000); // CREATE_NO_WINDOW flag
            
            println!("Executing Java command: {:?}", command);
            
            // Start the process
            match command.spawn() {
                Ok(mut child) => {
                    println!("Java server {} started successfully with PID: {}", id, child.id());
                    
                    // Get stdout, stderr, and stdin handles
                    let stdout = child.stdout.take().expect("Failed to capture stdout");
                    let stderr = child.stderr.take().expect("Failed to capture stderr");
                    let stdin = child.stdin.take().expect("Failed to capture stdin");
                    
                    // Clone the server output Arc for the threads
                    let output_clone = server.output.clone();
                    
                    // Spawn a thread to read stdout
                    thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                if let Ok(mut output) = output_clone.lock() {
                                    output.push(line);
                                }
                            }
                        }
                    });
                    
                    // Spawn a thread to read stderr
                    let output_clone = server.output.clone();
                    thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                if let Ok(mut output) = output_clone.lock() {
                                    output.push(line);
                                }
                            }
                        }
                    });
                    
                    server.process = Some(child);
                    server.stdin = Some(stdin);
                    server.status = "online".to_string();
                    
                    Ok(())
                },
                Err(e) => {
                    println!("Failed to start Java server {}: {}", id, e);
                    Err(format!("Failed to start Java server: {}", e))
                },
            }
        }
    }

    pub fn stop_server(&mut self, id: &str) -> Result<(), String> {
        let server = self.servers.get_mut(id).ok_or_else(|| format!("Server {} not found", id))?;
        
        if let Some(mut process) = server.process.take() {
            // Try to terminate gracefully
            match process.kill() {
                Ok(_) => {
                    server.status = "offline".to_string();
                    Ok(())
                },
                Err(e) => Err(format!("Failed to stop server: {}", e)),
            }
        } else {
            Err("Server is not running".to_string())
        }
    }

    pub fn get_server_info(&self, id: &str) -> Result<ServerInfo, String> {
        let server = self.servers.get(id).ok_or_else(|| format!("Server {} not found", id))?;
        
        Ok(ServerInfo {
            id: id.to_string(),
            config: server.config.clone(),
            status: server.status.clone(),
            players: 0, // In a real implementation, this would query the server
            max_players: 20, // In a real implementation, this would be from server.properties
        })
    }

    pub fn list_servers(&self) -> Vec<ServerInfo> {
        self.servers.iter().map(|(id, server)| {
            ServerInfo {
                id: id.clone(),
                config: server.config.clone(),
                status: server.status.clone(),
                players: 0,
                max_players: 20,
            }
        }).collect()
    }

    pub fn remove_server(&mut self, id: &str) -> Result<(), String> {
        if self.servers.contains_key(id) {
            // Stop the server if it's running
            if let Err(e) = self.stop_server(id) {
                // Only log as error if it's not just "not running"
                if !e.contains("not running") {
                eprintln!("Error stopping server during removal: {}", e);
                } else {
                    println!("Server {} was already stopped", id);
                }
            }
            
            self.servers.remove(id);
            
            // Save to disk after removing
            if let Err(e) = self.save_servers() {
                println!("Warning: Failed to save servers after removing {}: {}", id, e);
            }
            
            Ok(())
        } else {
            Err(format!("Server {} not found", id))
        }
    }
    
    pub fn clear_all_servers(&mut self) {
        // Stop all running servers
        for (id, server) in self.servers.iter_mut() {
            if let Some(mut process) = server.process.take() {
                let _ = process.kill();
                println!("Stopped server: {}", id);
            }
        }
        
        // Clear all servers
        self.servers.clear();
        println!("Cleared all servers from manager");
        
        // Save to disk after clearing
        if let Err(e) = self.save_servers() {
            println!("Warning: Failed to save servers after clearing: {}", e);
        }
    }
    
    pub fn get_server_output(&self, id: &str) -> Result<Vec<String>, String> {
        let server = self.servers.get(id).ok_or_else(|| format!("Server {} not found", id))?;
        
        match server.output.lock() {
            Ok(output) => Ok(output.clone()),
            Err(_) => Err("Failed to lock output".to_string()),
        }
    }
    
    pub fn send_server_command(&mut self, id: &str, command: &str) -> Result<(), String> {
        let server = self.servers.get_mut(id).ok_or_else(|| format!("Server {} not found", id))?;
        
        if let Some(stdin) = &mut server.stdin {
            let command_with_newline = format!("{}\n", command);
            if let Err(e) = stdin.write_all(command_with_newline.as_bytes()) {
                return Err(format!("Failed to send command: {}", e));
            }
            if let Err(e) = stdin.flush() {
                return Err(format!("Failed to flush command: {}", e));
            }
            Ok(())
        } else {
            Err("Server stdin not available".to_string())
        }
    }
    
    // Persistence methods
    pub fn save_servers(&self) -> Result<(), String> {
        // Create the directory if it doesn't exist
        if let Some(parent) = Path::new(&self.persistence_file).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
        }
        
        // Prepare server data for serialization (without process handles)
        let mut server_data = Vec::new();
        for (id, server) in &self.servers {
            server_data.push(ServerInfo {
                id: id.clone(),
                config: server.config.clone(),
                status: server.status.clone(),
                players: 0, // We don't persist player count
                max_players: 20, // Default max players
            });
        }
        
        // Serialize to JSON
        let json = serde_json::to_string_pretty(&server_data)
            .map_err(|e| format!("Failed to serialize servers: {}", e))?;
        
        // Write to file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.persistence_file)
            .map_err(|e| format!("Failed to open persistence file: {}", e))?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write to persistence file: {}", e))?;
        
        println!("Saved {} servers to {}", server_data.len(), self.persistence_file);
        Ok(())
    }
    
    fn load_servers(&mut self) -> Result<(), String> {
        if !Path::new(&self.persistence_file).exists() {
            println!("No persistence file found at {}, starting with empty server list", self.persistence_file);
            return Ok(());
        }
        
        // Read the file
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.persistence_file)
            .map_err(|e| format!("Failed to open persistence file: {}", e))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read persistence file: {}", e))?;
        
        if contents.trim().is_empty() {
            println!("Persistence file is empty, starting with empty server list");
            return Ok(());
        }
        
        // Deserialize from JSON
        let server_data: Vec<ServerInfo> = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to deserialize servers: {}", e))?;
        
        let server_count = server_data.len();
        
        // Add servers to the manager
        for server_info in server_data {
            self.servers.insert(server_info.id.clone(), ServerProcess {
                config: server_info.config,
                process: None,
                status: "offline".to_string(), // Always start as offline
                output: Arc::new(Mutex::new(Vec::new())),
                stdin: None,
            });
        }
        
        println!("Loaded {} servers from {}", server_count, self.persistence_file);
        Ok(())
    }
}

// Tauri command wrappers
type ServerManagerState<'a> = State<'a, Arc<Mutex<ServerManager>>>;

#[tauri::command]
pub fn add_server(
    state: ServerManagerState,
    id: String,
    config: ServerConfig,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.add_server(id, config)
}

#[tauri::command]
pub fn start_server(
    state: ServerManagerState,
    id: String,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.start_server(&id)
}

#[tauri::command]
pub fn stop_server(
    state: ServerManagerState,
    id: String,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.stop_server(&id)
}

#[tauri::command]
pub fn get_server_info(
    state: ServerManagerState,
    id: String,
) -> Result<ServerInfo, String> {
    let manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.get_server_info(&id)
}

#[tauri::command]
pub fn list_servers(
    state: ServerManagerState,
) -> Vec<ServerInfo> {
    let manager = match state.lock() {
        Ok(guard) => guard,
        Err(e) => {
            eprintln!("Failed to lock server manager: {}", e);
            return Vec::new();
        }
    };
    manager.list_servers()
}

#[tauri::command]
pub fn remove_server(
    state: ServerManagerState,
    id: String,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.remove_server(&id)
}

#[tauri::command]
pub fn get_server_output(
    state: ServerManagerState,
    id: String,
) -> Result<Vec<String>, String> {
    let manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.get_server_output(&id)
}

#[tauri::command]
pub fn send_server_command(
    state: ServerManagerState,
    id: String,
    command: String,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.send_server_command(&id, &command)
}

#[tauri::command]
pub fn clear_all_servers(
    state: ServerManagerState,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.clear_all_servers();
    Ok(())
}

#[tauri::command]
pub fn save_servers(
    state: ServerManagerState,
) -> Result<(), String> {
    let manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    manager.save_servers()
} 

#[tauri::command]
pub async fn download_and_install_mod(
    url: String,
    server_id: String,
    server_path: String,
    folder: String,
    filename: String
) -> Result<String, String> {
    println!("=== Starting mod installation ===");
    println!("URL: {}", url);
    println!("Server ID: {}", server_id);
    println!("Server Path: {}", server_path);
    println!("Folder: {}", folder);
    println!("Filename: {}", filename);
    
    // Create the full mod folder path
    let mod_folder = format!("{}/{}", server_path, folder).replace("//", "/");
    println!("Full mod folder path: {}", mod_folder);
    
    // Create the mod folder if it doesn't exist
    match std::fs::create_dir_all(&mod_folder) {
        Ok(_) => println!("Successfully created/verified mod folder: {}", mod_folder),
        Err(e) => {
            let error_msg = format!("Failed to create mod folder: {}", e);
            println!("Error: {}", error_msg);
            return Err(error_msg);
        }
    }
    
    // Create the full destination path
    let destination = format!("{}/{}", mod_folder, filename).replace("//", "/");
    println!("Full destination path: {}", destination);
    
    // Download the mod file
    println!("Starting download to {}", destination);
    match download_file(url, destination.clone()).await {
        Ok(_) => println!("Successfully downloaded mod file"),
        Err(e) => {
            let error_msg = format!("Failed to download mod: {}", e);
            println!("Error: {}", error_msg);
            return Err(error_msg);
        }
    }
    
    // Verify the file exists
    match std::fs::metadata(&destination) {
        Ok(metadata) => println!("Verified file exists with size: {} bytes", metadata.len()),
        Err(e) => {
            let error_msg = format!("Failed to verify downloaded file: {}", e);
            println!("Error: {}", error_msg);
            return Err(error_msg);
        }
    }
    
    println!("=== Mod installation completed successfully ===");
    Ok(destination)
}

// Add a download function to handle file downloads
#[tauri::command]
pub async fn download_file(url: String, destination: String) -> Result<String, String> {
    println!("=== Starting file download ===");
    println!("URL: {}", url);
    println!("Destination: {}", destination);
    
    // Create a reqwest client with a longer timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))  // 5 minute timeout
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Send the request
    println!("Sending HTTP request...");
    let response = client.get(&url)
        .header("User-Agent", "ServerMint/1.0")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Check if request was successful
    if !response.status().is_success() {
        let error_msg = format!("Failed to download: HTTP status {}", response.status());
        println!("Error: {}", error_msg);
        return Err(error_msg);
    }
    println!("Received successful HTTP response");

    // Get the total size if available
    let total_size = response.content_length().unwrap_or(0);
    println!("Total file size: {} bytes", total_size);

    // Create the destination directory if it doesn't exist
    println!("Creating parent directories if needed...");
    if let Some(parent) = std::path::Path::new(&destination).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
            println!("Successfully created parent directories");
        }
    }

    // Open file for writing
    println!("Opening destination file...");
    let mut file = std::fs::File::create(&destination)
        .map_err(|e| format!("Failed to create file: {}", e))?;

    // Download the file in chunks
    println!("Starting download...");
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Error while downloading file: {}", e))?;
        std::io::Write::write_all(&mut file, &chunk)
            .map_err(|e| format!("Error while writing to file: {}", e))?;
        
        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64 * 100.0) as u32;
            println!("Download progress: {}% ({}/{} bytes)", progress, downloaded, total_size);
        }
    }

    // Verify the downloaded file
    println!("Verifying downloaded file...");
    match std::fs::metadata(&destination) {
        Ok(metadata) => {
            let file_size = metadata.len();
            if total_size > 0 && file_size != total_size {
                let error_msg = format!("File size mismatch. Expected: {}, Got: {}", total_size, file_size);
                println!("Error: {}", error_msg);
                // Try to clean up the incomplete file
                let _ = std::fs::remove_file(&destination);
                return Err(error_msg);
            }
            println!("File size verified: {} bytes", file_size);
        },
        Err(e) => {
            let error_msg = format!("Failed to verify downloaded file: {}", e);
            println!("Error: {}", error_msg);
            return Err(error_msg);
        }
    }

    println!("=== File download completed successfully ===");
    Ok(destination)
}

// Add a function to extract ZIP files
async fn extract_zip(zip_path: &str, extract_to: &str) -> Result<(), String> {
    println!("Extracting {} to {}", zip_path, extract_to);
    
    // Use the zip crate to extract the file
    let file = File::open(zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to access file in zip: {}", e))?;
        
        let outpath = PathBuf::from(extract_to).join(file.name());
        
        if file.name().ends_with('/') {
            // Create directory
            std::fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            // Create parent directory if it doesn't exist
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
            }
            
            // Extract file
            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }
    
    println!("Extraction completed");
    Ok(())
}

// Add a function to setup a new server
#[tauri::command]
pub async fn setup_server(
    state: ServerManagerState<'_>,
    server_id: String,
    server_path: String,
    server_type: String,
    version: String,
    download_url: Option<String>,
    server_name: String,
) -> Result<String, String> {
    println!("Setting up server {} at {} with type {} version {}", server_id, server_path, server_type, version);
    
    // Create the server directory
    std::fs::create_dir_all(&server_path)
        .map_err(|e| format!("Failed to create server directory: {}", e))?;
    
    // Determine the download URL based on server type and version
    let final_url = match download_url {
        Some(url) => url,
        None => {
            match server_type.to_lowercase().as_str() {
                "vanilla" => format!("https://download.servermint.app/vanilla/vanilla_{}.jar", version),
                "paper" => format!("https://download.servermint.app/paper/paper-{}.jar", version),
                "fabric" => format!("https://download.servermint.app/fabric/fabric-{}.jar", version),
                "forge" => format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{}/forge-{}-installer.jar", version, version),
                "neoforge" => format!("https://maven.neoforged.net/releases/net/neoforged/forge/{}/forge-{}-installer.jar", version, version),
                "pocketmine" => format!("https://github.com/pmmp/PocketMine-MP/releases/download/{}/PocketMine-MP.phar", version),
                _ => return Err(format!("Unsupported server type: {}", server_type)),
            }
        }
    };
    
    // For Bedrock servers, we need to handle them differently
    let is_bedrock = server_type.to_lowercase() == "pocketmine";
    
    if is_bedrock {
        // For PocketMine-MP, download the .phar file directly
        let phar_path = format!("{}/PocketMine-MP.phar", server_path);
        download_file(final_url, phar_path.clone()).await?;
        
        // Download and extract PocketMine-MP PHP binary automatically
        println!("Downloading PocketMine-MP PHP binary...");
        let php_zip_path = format!("{}/php.zip", server_path);
        
        // Try different PHP binary URLs with correct format
        let php_urls = vec![
            "https://github.com/pmmp/PHP-Binaries/releases/download/pm5-php-8.3-latest/PHP-8.3-Windows-x64-PM5.zip",
            "https://github.com/pmmp/PHP-Binaries/releases/download/pm5-php-8.2-latest/PHP-8.2-Windows-x64-PM5.zip",
            "https://github.com/pmmp/PHP-Binaries/releases/download/pm5-php-8.1-latest/PHP-8.1-Windows-x64-PM5.zip",
            "https://github.com/pmmp/PHP-Binaries/releases/latest/download/PHP-8.3-Windows-x64-PM5.zip",
        ];
        
        let mut download_success = false;
        for (i, url) in php_urls.iter().enumerate() {
            println!("Trying PHP download URL {}: {}", i + 1, url);
            match download_file(url.to_string(), php_zip_path.clone()).await {
                Ok(_) => {
                    println!("PHP binary downloaded successfully from URL {}", i + 1);
                    download_success = true;
                    break;
                },
                Err(e) => {
                    println!("Failed to download from URL {}: {}", i + 1, e);
                    if i == php_urls.len() - 1 {
                        println!("All PHP download URLs failed, will use system PHP");
                    }
                }
            }
        }
        
        if download_success {
            println!("PHP binary downloaded successfully, extracting...");
            // Extract the ZIP file
            if let Err(e) = extract_zip(&php_zip_path, &server_path).await {
                println!("Warning: Failed to extract PHP binary: {}", e);
            } else {
                println!("PHP binary extracted successfully");
                // Clean up the zip file
                let _ = std::fs::remove_file(&php_zip_path);
            }
        } else {
            println!("Warning: Failed to download PHP binary from all URLs, will use system PHP");
        }
        
        // Create a startup script that uses the local PHP
        let start_script = format!(
            "@echo off
echo ========================================
echo PocketMine-MP Bedrock Server
echo ========================================
echo.
echo Starting PocketMine-MP Bedrock Server...
if exist bin\\php\\php.exe (
    echo Using local PocketMine-MP PHP binary...
    bin\\php\\php.exe PocketMine-MP.phar
) else if exist php.exe (
    echo Using local PHP binary...
    php.exe PocketMine-MP.phar
) else (
    echo Local PHP binary not found. Trying system PHP...
    php PocketMine-MP.phar
)"
        );
        
        let start_script_path = format!("{}/start.bat", server_path);
        std::fs::write(&start_script_path, start_script)
            .map_err(|e| format!("Failed to write start.bat: {}", e))?;
            
        // Also create a shell script for cross-platform compatibility
        let start_sh = format!(
            "#!/bin/sh
echo Starting PocketMine-MP Bedrock Server...
php PocketMine-MP.phar"
        );
        
        let start_sh_path = format!("{}/start.sh", server_path);
        std::fs::write(&start_sh_path, start_sh)
            .map_err(|e| format!("Failed to write start.sh: {}", e))?;
    } else {
        // For Java servers, download the server jar
        let jar_path = format!("{}/server.jar", server_path);
        download_file(final_url, jar_path.clone()).await?;
        
        // For Java servers, create the standard startup scripts
        let start_script = format!(
            "@echo off
start /min java -Xmx{}M -Xms1G -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -jar server.jar nogui",
            4096 // Default 4GB for Java servers
        );
        
        let start_script_path = format!("{}/start.bat", server_path);
        std::fs::write(&start_script_path, start_script)
            .map_err(|e| format!("Failed to write start.bat: {}", e))?;
            
        // Also create a shell script for cross-platform compatibility
        let start_sh = format!(
            "#!/bin/sh
java -Xmx{}M -Xms1G -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -jar server.jar nogui",
            4096 // Default 4GB for Java servers
        );
        
        let start_sh_path = format!("{}/start.sh", server_path);
        std::fs::write(&start_sh_path, start_sh)
            .map_err(|e| format!("Failed to write start.sh: {}", e))?;
    }
    
    // Create server.properties file
    let server_properties = if is_bedrock {
        // PocketMine-MP configuration
        format!(
            "# PocketMine-MP server properties
server-port=19132
server-ip=0.0.0.0
motd1=A PocketMine-MP Server
motd2=Powered by ServerMint
gamemode=0
difficulty=1
spawn-protection=16
max-players=20
allow-flight=false
white-list=false
online-mode=true
enable-query=true
query-port=19132
enable-rcon=false
rcon-port=19133
rcon-password=
auto-save=true
view-distance=10
spawn-animals=true
spawn-npcs=true
spawn-monsters=true
generate-structures=true
allow-nether=true
force-gamemode=false
hardcore=false
broadcast-console-to-ops=true
broadcast-rcon-to-ops=true
enable-command-block=false
max-tick-time=60000
use-native-transport=true
enable-jmx-monitoring=false
enable-status=true
entity-broadcast-range-percentage=100
function-permission-level=2
network-compression-threshold=256
max-world-size=29999984
resource-pack=
resource-pack-sha1=
require-resource-pack=false
player-idle-timeout=0
force-gamemode=false
rate-limit=0
server-authoritative-movement=true
player-movement-score-threshold=20
player-movement-distance-threshold=0.3
player-movement-duration-threshold-in-ms=500
correct-player-movement=false
server-authoritative-block-breaking=false"
        )
    } else {
        // Java server properties
        format!(
            "# Minecraft server properties
spawn-protection=16
max-tick-time=60000
query.port=25565
generator-settings=
sync-chunk-writes=true
force-gamemode=false
allow-nether=true
enforce-whitelist=false
gamemode=survival
broadcast-console-to-ops=true
enable-query=false
player-idle-timeout=0
difficulty=easy
spawn-monsters=true
broadcast-rcon-to-ops=true
op-permission-level=4
pvp=true
entity-broadcast-range-percentage=100
snooper-enabled=true
level-type=default
hardcore=false
enable-status=true
enable-command-block=false
max-players=20
network-compression-threshold=256
resource-pack-sha1=
max-world-size=29999984
function-permission-level=2
rcon.port=25575
server-port=25565
server-ip=
spawn-npcs=true
allow-flight=false
level-name=world
view-distance=10
resource-pack=
spawn-animals=true
white-list=false
rcon.password=
generate-structures=true
max-build-height=256
online-mode=true
level-seed=
use-native-transport=true
prevent-proxy-connections=false
enable-jmx-monitoring=false
enable-rcon=false
rate-limit=0
motd=A Minecraft Server"
        )
    };
    
    let properties_path = format!("{}/server.properties", server_path);
    std::fs::write(&properties_path, server_properties)
        .map_err(|e| format!("Failed to write server.properties: {}", e))?;
    
    // Create eula.txt file
    let eula_content = format!(
        "#By changing the setting below to TRUE you are indicating your agreement to our EULA (https://account.mojang.com/documents/minecraft_eula).
#{}
eula=true",
        chrono::Utc::now().to_rfc3339()
    );
    
    let eula_path = format!("{}/eula.txt", server_path);
    std::fs::write(&eula_path, eula_content)
        .map_err(|e| format!("Failed to write eula.txt: {}", e))?;
    
    // Create additional directories
    let dirs = ["world", "logs", "plugins", "mods"];
    for dir in &dirs {
        let dir_path = format!("{}/{}", server_path, dir);
        std::fs::create_dir_all(&dir_path)
            .map_err(|e| format!("Failed to create {} directory: {}", dir, e))?;
    }
    
    // Add the server to the manager
    let mut manager = state.lock().map_err(|_| "Failed to lock server manager")?;
    
    let config = ServerConfig {
        name: server_name,
        path: server_path.clone(),
        version,
        server_type,
        java_path: None, // Will use system default
        min_memory: if is_bedrock { 512 } else { 1024 }, // Bedrock servers use less memory
        max_memory: if is_bedrock { 2048 } else { 4096 }, // Bedrock servers use less memory
        jvm_args: None,
        port: if is_bedrock { 19132 } else { 25565 }, // Bedrock uses port 19132, Java uses 25565
    };
    
    println!("Adding server {} to manager with config: {:?}", server_id, config);
    manager.add_server(server_id.clone(), config)?;
    
    // Verify the server was added
    if let Some(server) = manager.servers.get(&server_id) {
        println!("Server {} successfully added to manager, status: {}", server_id, server.status);
    } else {
        println!("Warning: Server {} was not found in manager after adding", server_id);
    }
    
    println!("Successfully set up server at {}", server_path);
    Ok(server_path)
}

// Add a command to check if Java is available
#[tauri::command]
pub fn check_java() -> Result<String, String> {
    println!("Checking if Java is available...");
    
    match Command::new("java").arg("-version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stderr);
                println!("Java found: {}", version.lines().next().unwrap_or("Unknown version"));
                Ok(format!("Java is available: {}", version.lines().next().unwrap_or("Unknown version")))
            } else {
                println!("Java command failed with status: {}", output.status);
                Err("Java command failed".to_string())
            }
        },
        Err(e) => {
            println!("Java not found: {}", e);
            Err(format!("Java not found: {}", e))
        }
    }
}

// Add a command to download and setup Java 21
#[tauri::command]
pub async fn setup_java() -> Result<String, String> {
    println!("Setting up Java 21...");
    
    // Create Java directory
    let java_dir = "C:/servermint/java";
    std::fs::create_dir_all(java_dir)
        .map_err(|e| format!("Failed to create Java directory: {}", e))?;
    
    // Download Java 21 for Windows (using OpenJDK 21 from Adoptium)
    let java_url = "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.2%2B13/OpenJDK21U-jdk_x64_windows_hotspot_21.0.2_13.zip";
    let java_zip_path = format!("{}/OpenJDK21U-jdk_x64_windows_hotspot_21.0.2_13.zip", java_dir);
    
    println!("Downloading Java 21 from {}", java_url);
    download_file(java_url.to_string(), java_zip_path.clone()).await?;
    
    // Extract the zip file
    println!("Extracting Java...");
    let mut extract_command = Command::new("powershell");
    extract_command.args(&[
        "-Command",
        &format!("Expand-Archive -Path '{}' -DestinationPath '{}' -Force", java_zip_path, java_dir)
    ]);
    
    // Hide the window on Windows
    #[cfg(target_os = "windows")]
    extract_command.creation_flags(0x08000000); // CREATE_NO_WINDOW flag
    
    let extract_result = extract_command.output();
    
    match extract_result {
        Ok(output) => {
            if output.status.success() {
                println!("Java extracted successfully");
                
                // Find the extracted directory
                let entries = std::fs::read_dir(java_dir)
                    .map_err(|e| format!("Failed to read Java directory: {}", e))?;
                
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() && path.file_name().unwrap().to_string_lossy().starts_with("jdk") {
                            let java_bin = path.join("bin").join("java.exe");
                            if java_bin.exists() {
                                let java_path = java_bin.to_string_lossy().to_string();
                                println!("Java 17 found at: {}", java_path);
                                
                                // Test the Java installation
                                match Command::new(&java_path).arg("-version").output() {
                                    Ok(test_output) => {
                                        if test_output.status.success() {
                                            let version = String::from_utf8_lossy(&test_output.stderr);
                                            println!("Java test successful: {}", version.lines().next().unwrap_or("Unknown version"));
                                            return Ok(format!("Java 21 setup complete: {}", java_path));
                                        } else {
                                            return Err("Java test failed after installation".to_string());
                                        }
                                    },
                                    Err(e) => return Err(format!("Failed to test Java: {}", e)),
                                }
                            }
                        }
                    }
                }
                
                Err("Could not find Java executable after extraction".to_string())
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Err(format!("Failed to extract Java: {}", error))
            }
        },
        Err(e) => Err(format!("Failed to run extraction command: {}", e)),
    }
}

// Internal function to get Java path (used by other functions in this module)
fn get_java_path_internal(version: Option<String>) -> Result<String, String> {
    let _required_version = version.unwrap_or_else(|| "17".to_string());
    
    // First check if we have a local Java installation
    let java_dir = "C:/servermint/java";
    println!("Checking for local Java in: {}", java_dir);
    
    if std::path::Path::new(java_dir).exists() {
        println!("Java directory exists, checking contents...");
        let entries = std::fs::read_dir(java_dir)
            .map_err(|e| format!("Failed to read Java directory: {}", e))?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                println!("Found entry: {:?}", path);
                
                if path.is_dir() {
                    let dir_name = path.file_name().unwrap().to_string_lossy();
                    println!("Checking directory: {}", dir_name);
                    
                    if dir_name.starts_with("jdk") {
                        let java_bin = path.join("bin").join("java.exe");
                        println!("Checking for Java executable at: {:?}", java_bin);
                        
                        if java_bin.exists() {
                            let java_path = java_bin.to_string_lossy().to_string();
                            println!("Found local Java at: {}", java_path);
                            
                            // Test the Java installation
                            match Command::new(&java_path).arg("-version").output() {
                                Ok(output) => {
                                    if output.status.success() {
                                        let version_output = String::from_utf8_lossy(&output.stderr);
                                        println!("Java test successful: {}", version_output.lines().next().unwrap_or("Unknown version"));
                                        return Ok(java_path);
                                    } else {
                                        println!("Java test failed with status: {}", output.status);
                                    }
                                },
                                Err(e) => {
                                    println!("Java test failed: {}", e);
                                }
                            }
                        } else {
                            println!("Java executable not found at: {:?}", java_bin);
                        }
                    }
                }
            }
        }
    } else {
        println!("Java directory does not exist: {}", java_dir);
    }
    
    // Fall back to system Java
    println!("No local Java found, using system Java");
    Ok("java".to_string())
}

// Add a command to get the Java path for a specific version
#[tauri::command]
pub fn get_java_path(version: Option<String>) -> Result<String, String> {
    get_java_path_internal(version)
} 

#[tauri::command]
pub fn get_local_ip() -> Result<String, String> {
    use std::net::{IpAddr, Ipv4Addr};

    let interfaces = get_if_addrs::get_if_addrs().map_err(|e| e.to_string())?;

    for iface in interfaces {
        if let IpAddr::V4(ip) = iface.ip() {
            if !ip.is_loopback() {
                return Ok(ip.to_string());
            }
        }
    }

    Ok(Ipv4Addr::LOCALHOST.to_string())
}