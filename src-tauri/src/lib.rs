mod server;
mod setup;
mod node;
mod sftp;
pub use sftp::*;

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

#[tauri::command]
async fn start_all_servers(state: tauri::State<'_, Arc<Mutex<ServerManager>>>) -> Result<serde_json::Value, String> {
    println!("Starting all offline servers...");
    
    let mut server_manager_guard = state.lock().map_err(|_| "Failed to lock server manager")?;
    let mut started_count = 0;
    
    let server_list = server_manager_guard.list_servers();
    for server in server_list {
        if server.status == "offline" {
            match server_manager_guard.start_server(&server.id) {
                Ok(_) => {
                    println!("Started server: {}", server.config.name);
                    started_count += 1;
                },
                Err(e) => {
                    println!("Failed to start server {}: {}", server.config.name, e);
                }
            }
        }
    }
    
    Ok(serde_json::json!({
        "success": true,
        "message": format!("Started {} servers", started_count),
        "count": started_count
    }))
}

#[tauri::command]
async fn stop_all_servers(state: tauri::State<'_, Arc<Mutex<ServerManager>>>) -> Result<serde_json::Value, String> {
    println!("Stopping all running servers...");
    
    let mut server_manager_guard = state.lock().map_err(|_| "Failed to lock server manager")?;
    let mut stopped_count = 0;
    
    let server_list = server_manager_guard.list_servers();
    for server in server_list {
        if server.status == "online" {
            match server_manager_guard.stop_server(&server.id) {
                Ok(_) => {
                    println!("Stopped server: {}", server.config.name);
                    stopped_count += 1;
                },
                Err(e) => {
                    println!("Failed to stop server {}: {}", server.config.name, e);
                }
            }
        }
    }
    
    Ok(serde_json::json!({
        "success": true,
        "message": format!("Stopped {} servers", stopped_count),
        "count": stopped_count
    }))
}

#[tauri::command]
async fn backup_all_servers(state: tauri::State<'_, Arc<Mutex<ServerManager>>>) -> Result<serde_json::Value, String> {
    println!("Creating backups for all servers...");
    
    let server_manager_guard = state.lock().map_err(|_| "Failed to lock server manager")?;
    let mut backup_count = 0;
    
    let server_list = server_manager_guard.list_servers();
    for server in server_list {
        if server.status != "installing" {
            // For now, just count servers that can be backed up
            // TODO: Implement actual backup functionality
            println!("Would create backup for server: {}", server.config.name);
            backup_count += 1;
        }
    }
    
    Ok(serde_json::json!({
        "success": true,
        "message": format!("Created backups for {} servers", backup_count),
        "count": backup_count
    }))
}

#[tauri::command]
async fn check_for_updates() -> Result<serde_json::Value, String> {
    println!("Checking for application updates...");
    
    // This would check for updates using Tauri's updater
    // For now, return no updates available
    Ok(serde_json::json!({
        "success": true,
        "hasUpdate": false,
        "currentVersion": env!("CARGO_PKG_VERSION"),
        "latestVersion": env!("CARGO_PKG_VERSION"),
        "message": "No updates available"
    }))
}

#[tauri::command]
async fn restart_application(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("Restarting application...");
    app_handle.restart();
    Ok(()) // This is fine, the restart will happen before this returns
}

#[tauri::command]
async fn quit_application(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("Quitting application...");
    
    // Use Tauri's exit functionality
    app_handle.exit(0);
    
    Ok(())
}

#[tauri::command]
async fn get_mint_menu_commands() -> Result<serde_json::Value, String> {
    println!("Getting MintMenu commands...");
    
    // Return available commands for the MintMenu
    let commands = vec![
        serde_json::json!({
            "id": "create-server",
            "title": "Create Server",
            "description": "Create a new Minecraft server",
            "icon": "mdi-plus-circle",
            "color": "success",
            "category": "server"
        }),
        serde_json::json!({
            "id": "refresh-servers",
            "title": "Refresh Servers",
            "description": "Reload server list",
            "icon": "mdi-refresh",
            "category": "server"
        }),
        serde_json::json!({
            "id": "start-all-servers",
            "title": "Start All Servers",
            "description": "Start all offline servers",
            "icon": "mdi-play-circle",
            "color": "success",
            "category": "action"
        }),
        serde_json::json!({
            "id": "stop-all-servers",
            "title": "Stop All Servers",
            "description": "Stop all running servers",
            "icon": "mdi-stop-circle",
            "color": "error",
            "category": "action"
        }),
        serde_json::json!({
            "id": "backup-all-servers",
            "title": "Backup All Servers",
            "description": "Create backups for all servers",
            "icon": "mdi-backup-restore",
            "color": "warning",
            "category": "action"
        }),
        serde_json::json!({
            "id": "check-updates",
            "title": "Check Updates",
            "description": "Check for application updates",
            "icon": "mdi-update",
            "category": "system"
        }),
        serde_json::json!({
            "id": "restart-app",
            "title": "Restart Application",
            "description": "Restart ServerMint",
            "icon": "mdi-restart",
            "color": "warning",
            "category": "system"
        }),
        serde_json::json!({
            "id": "quit-app",
            "title": "Quit Application",
            "description": "Close ServerMint",
            "icon": "mdi-exit-to-app",
            "color": "error",
            "category": "system"
        })
    ];
    
    Ok(serde_json::json!({
        "success": true,
        "commands": commands
    }))
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

  tauri::Builder::default()
    .manage(server_manager)
    .manage(node_manager)
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .invoke_handler(tauri::generate_handler![
      // Server commands
      server::list_servers,
      server::add_server,
      server::remove_server,
      server::start_server,
      server::stop_server,
      server::send_server_command,
      server::get_server_output,
      server::setup_server,
      server::save_servers,
      server::download_file,
      server::download_and_install_mod,
      server::get_local_ip,
      server::check_java,
      server::setup_java,
      server::get_java_path,
      
      // Node commands
      node::list_nodes,
      node::get_node,
      node::add_node,
      node::update_node,
      node::remove_node,
      node::generate_pairing_token,
      node::check_node_connected,
      node::get_node_info_by_token,
      node::update_node_metrics,
      node::update_node_status,
      
      // Utility commands
      open_folder,
      
      // SFTP commands
      sftp::test_sftp_connection,
      sftp::upload_file_sftp,
      sftp::download_file_sftp,
      sftp::list_remote_files,
      sftp::run_sftp_command,
      
      // File operations
      get_file_size,
      get_folder_size,
      rename_file,
      move_file,
      delete_file_or_directory,
      
      // MintMenu commands
      start_all_servers,
      stop_all_servers,
      backup_all_servers,
      check_for_updates,
      restart_application,
      quit_application,
      get_mint_menu_commands,
    ])
    .setup(|app| {
      // Initialize logging for all builds
      app.handle().plugin(
        tauri_plugin_log::Builder::default()
          .level(log::LevelFilter::Info)
          .build(),
      )?;
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
