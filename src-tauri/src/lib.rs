mod server;
mod setup;
mod node;
mod websocket;

use std::sync::{Arc, Mutex};
use server::ServerManager;

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
