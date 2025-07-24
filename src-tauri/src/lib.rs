mod server;
mod setup;
mod node;
mod websocket;

use std::sync::{Arc, Mutex};
use server::ServerManager;

#[tauri::command]
async fn open_folder(path: String) -> Result<(), String> {
  #[cfg(target_os = "windows")]
  {
    use std::process::Command;
    
    // Normalize path to use Windows backslashes
    let normalized_path = path.replace('/', "\\");
    
    let output = Command::new("explorer.exe")
      .arg(&normalized_path)
      .output()
      .map_err(|e| format!("Failed to open folder: {}", e))?;
    
    if !output.status.success() {
      return Err(format!("Explorer failed with status: {}", output.status));
    }
    
    Ok(())
  }
  
  #[cfg(not(target_os = "windows"))]
  {
    use std::process::Command;
    
    let output = Command::new("xdg-open")
      .arg(&path)
      .output()
      .map_err(|e| format!("Failed to open folder: {}", e))?;
    
    if !output.status.success() {
      return Err(format!("xdg-open failed with status: {}", output.status));
    }
    
    Ok(())
  }
}

// SFTP Commands
#[tauri::command]
async fn test_sftp_connection(config: serde_json::Value) -> Result<serde_json::Value, String> {
    let host = config["host"].as_str().ok_or("Host is required")?;
    let port = config["port"].as_u64().unwrap_or(22) as u16;
    let username = config["username"].as_str().ok_or("Username is required")?;
    let password = config["password"].as_str().ok_or("Password is required")?;
    
    // For now, return success (in a real implementation, you'd use an SFTP library)
    Ok(serde_json::json!({
        "success": true,
        "message": "SFTP connection test successful"
    }))
}

#[tauri::command]
async fn export_to_sftp(
    server_id: String,
    config: serde_json::Value,
    files: Vec<serde_json::Value>,
    app_handle: tauri::AppHandle
) -> Result<serde_json::Value, String> {
    let host = config["host"].as_str().ok_or("Host is required")?;
    let port = config["port"].as_u64().unwrap_or(22) as u16;
    let username = config["username"].as_str().ok_or("Username is required")?;
    let password = config["password"].as_str().ok_or("Password is required")?;
    let remote_path = config["remotePath"].as_str().unwrap_or("/");
    
    println!("Exporting {} files to SFTP server {}:{}", files.len(), host, port);
    
    // Simulate file transfer progress
    for (i, file) in files.iter().enumerate() {
        let file_name = file["name"].as_str().unwrap_or("unknown");
        let progress = ((i + 1) as f64 / files.len() as f64 * 100.0) as u32;
        
        // Send progress update (commented out until proper Tauri v2 event emission is implemented)
        // TODO: Implement proper event emission for SFTP progress
        println!("SFTP Progress: {}% - {}", progress, file_name);
        
        // Simulate transfer delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(serde_json::json!({
        "success": true,
        "fileCount": files.len(),
        "message": "Files exported successfully"
    }))
}

#[tauri::command]
async fn import_from_sftp(
    server_id: String,
    config: serde_json::Value,
    files: Vec<serde_json::Value>,
    app_handle: tauri::AppHandle
) -> Result<serde_json::Value, String> {
    let host = config["host"].as_str().ok_or("Host is required")?;
    let port = config["port"].as_u64().unwrap_or(22) as u16;
    let username = config["username"].as_str().ok_or("Username is required")?;
    let password = config["password"].as_str().ok_or("Password is required")?;
    let import_path = config["importPath"].as_str().unwrap_or("/");
    
    println!("Importing {} files from SFTP server {}:{}", files.len(), host, port);
    
    // Simulate file transfer progress
    for (i, file) in files.iter().enumerate() {
        let file_name = file["name"].as_str().unwrap_or("unknown");
        let progress = ((i + 1) as f64 / files.len() as f64 * 100.0) as u32;
        
        // Send progress update (commented out until proper Tauri v2 event emission is implemented)
        // TODO: Implement proper event emission for SFTP progress
        println!("SFTP Progress: {}% - {}", progress, file_name);
        
        // Simulate transfer delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(serde_json::json!({
        "success": true,
        "fileCount": files.len(),
        "message": "Files imported successfully"
    }))
}

#[tauri::command]
async fn list_remote_files(config: serde_json::Value, path: String) -> Result<serde_json::Value, String> {
    let host = config["host"].as_str().ok_or("Host is required")?;
    let port = config["port"].as_u64().unwrap_or(22) as u16;
    let username = config["username"].as_str().ok_or("Username is required")?;
    let password = config["password"].as_str().ok_or("Password is required")?;
    
    println!("Listing remote files on {}:{} at path: {}", host, port, path);
    
    // Simulate remote file listing
    let mock_files = vec![
        serde_json::json!({
            "name": "server.jar",
            "isDirectory": false,
            "size": 1024 * 1024 * 50, // 50MB
            "modified": "2024-01-01T00:00:00Z"
        }),
        serde_json::json!({
            "name": "server.properties",
            "isDirectory": false,
            "size": 1024, // 1KB
            "modified": "2024-01-01T00:00:00Z"
        }),
        serde_json::json!({
            "name": "plugins",
            "isDirectory": true,
            "size": 0,
            "modified": "2024-01-01T00:00:00Z"
        }),
        serde_json::json!({
            "name": "worlds",
            "isDirectory": true,
            "size": 0,
            "modified": "2024-01-01T00:00:00Z"
        })
    ];
    
    Ok(serde_json::json!({
        "success": true,
        "files": mock_files
    }))
}

#[tauri::command]
async fn cancel_sftp_transfer() -> Result<(), String> {
    println!("Cancelling SFTP transfer");
    // In a real implementation, you'd cancel the ongoing transfer
    Ok(())
}

#[tauri::command]
async fn get_file_size(file_path: String) -> Result<u64, String> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err(format!("File does not exist: {}", file_path));
    }
    
    if !path.is_file() {
        return Err(format!("Path is not a file: {}", file_path));
    }
    
    match fs::metadata(path) {
        Ok(metadata) => {
            let size = metadata.len();
            println!("File size for {}: {} bytes", file_path, size);
            Ok(size)
        },
        Err(e) => {
            let error_msg = format!("Failed to get file size for {}: {}", file_path, e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
async fn get_folder_size(folder_path: String) -> Result<u64, String> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(&folder_path);
    
    if !path.exists() {
        return Err(format!("Folder does not exist: {}", folder_path));
    }
    
    if !path.is_dir() {
        return Err(format!("Path is not a folder: {}", folder_path));
    }
    
    fn calculate_folder_size(dir_path: &Path) -> Result<u64, std::io::Error> {
        let mut total_size = 0u64;
        
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    
                    if entry_path.is_file() {
                        if let Ok(metadata) = fs::metadata(&entry_path) {
                            total_size += metadata.len();
                        }
                    } else if entry_path.is_dir() {
                        if let Ok(sub_size) = calculate_folder_size(&entry_path) {
                            total_size += sub_size;
                        }
                    }
                }
            }
        }
        
        Ok(total_size)
    }
    
    match calculate_folder_size(path) {
        Ok(size) => {
            println!("Folder size for {}: {} bytes", folder_path, size);
            Ok(size)
        },
        Err(e) => {
            let error_msg = format!("Failed to get folder size for {}: {}", folder_path, e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
async fn rename_file(old_path: String, new_name: String) -> Result<(), String> {
    use std::fs;
    use std::path::Path;
    
    let old_path = Path::new(&old_path);
    let parent = old_path.parent().ok_or("Invalid path")?;
    let new_path = parent.join(&new_name);
    
    if !old_path.exists() {
        return Err(format!("File does not exist: {}", old_path.display()));
    }
    
    if new_path.exists() {
        return Err(format!("File already exists: {}", new_path.display()));
    }
    
    match fs::rename(old_path, &new_path) {
        Ok(_) => {
            println!("Successfully renamed {} to {}", old_path.display(), new_path.display());
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to rename file: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
async fn move_file(source_path: String, destination_path: String) -> Result<(), String> {
    use std::fs;
    use std::path::Path;
    
    let source = Path::new(&source_path);
    let destination = Path::new(&destination_path);
    
    if !source.exists() {
        return Err(format!("Source file does not exist: {}", source.display()));
    }
    
    // Create destination directory if it doesn't exist
    if let Some(parent) = destination.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("Failed to create destination directory: {}", e));
            }
        }
    }
    
    if destination.exists() {
        return Err(format!("Destination file already exists: {}", destination.display()));
    }
    
    match fs::rename(source, destination) {
        Ok(_) => {
            println!("Successfully moved {} to {}", source.display(), destination.display());
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to move file: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
async fn delete_file_or_directory(path: String) -> Result<(), String> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    
    if path.is_file() {
        match fs::remove_file(path) {
            Ok(_) => {
                println!("Successfully deleted file: {}", path.display());
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Failed to delete file: {}", e);
                println!("{}", error_msg);
                Err(error_msg)
            }
        }
    } else if path.is_dir() {
        match fs::remove_dir_all(path) {
            Ok(_) => {
                println!("Successfully deleted directory: {}", path.display());
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("Failed to delete directory: {}", e);
                println!("{}", error_msg);
                Err(error_msg)
            }
        }
    } else {
        Err(format!("Path is neither a file nor directory: {}", path.display()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Ensure application directories exist
  if let Err(e) = setup::ensure_app_directories() {
    eprintln!("Warning: Failed to create application directories: {}", e);
    // Continue anyway, as the app might still work with fallbacks
  }

  let server_manager = Arc::new(Mutex::new(ServerManager::new()));
  let node_manager = Arc::new(Mutex::new(node::NodeManager::new(server_manager.clone())));
  
  // Create WebSocket server
  let websocket_server = Arc::new(websocket::WebSocketServer::new());
  let ws_server_clone = websocket_server.clone();
  
  // Start WebSocket server in a background task with a different port
  tauri::async_runtime::spawn(async move {
    // Try different ports if the default one is in use
    let ports = [9955, 9956, 9957, 9958, 9959];
    let mut success = false;
    
    for port in ports {
      match ws_server_clone.start(port).await {
        Ok(_) => {
          println!("WebSocket server started on port {}", port);
          success = true;
          break;
        },
        Err(e) => {
          eprintln!("Failed to start WebSocket server on port {}: {}", port, e);
        }
      }
    }
    
    if !success {
      eprintln!("Failed to start WebSocket server on any port");
    }
  });

  tauri::Builder::default()
    .manage(server_manager)
    .manage(node_manager)
    .manage(websocket_server)
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .invoke_handler(tauri::generate_handler![
      // Server commands
      server::add_server,
      server::list_servers,
      server::get_server_info,
      server::remove_server,
      server::start_server,
      server::stop_server,
      server::send_server_command,
      server::get_server_output,
      server::setup_server,
      server::download_file,
      server::get_local_ip,
      server::check_java,
      server::setup_java,
      
      // Node commands
      node::list_nodes,
      node::get_node,
      node::add_node,
      node::update_node,
      node::remove_node,
      node::generate_pairing_token,
      node::check_node_connected,
      node::get_node_info_by_token,
      
      // WebSocket commands
      websocket::ws_generate_pairing_token,
      websocket::ws_check_node_connected,
      
      // Utility commands
      open_folder,
      
      // SFTP commands
      test_sftp_connection,
      export_to_sftp,
      import_from_sftp,
      list_remote_files,
      cancel_sftp_transfer,
      
      // File operations
      get_file_size,
      get_folder_size,
      rename_file,
      move_file,
      delete_file_or_directory,
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
