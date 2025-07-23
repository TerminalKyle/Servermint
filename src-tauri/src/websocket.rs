use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::{accept_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;
use log::{info, error};

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

// Connection types
#[derive(Debug, Clone)]
enum ConnectionType {
    Agent { token: String, node_id: String },
    Desktop { user_id: String },
}

// Connection state
#[derive(Debug)]
struct Connection {
    conn_type: ConnectionType,
    sender: mpsc::UnboundedSender<Message>,
}

// WebSocket server
#[derive(Debug)]
pub struct WebSocketServer {
    connections: Arc<Mutex<HashMap<String, Connection>>>,
    // Map of token to node_id
    tokens: Arc<Mutex<HashMap<String, String>>>,
    // Map of node_id to user_id
    node_owners: Arc<Mutex<HashMap<String, String>>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            tokens: Arc::new(Mutex::new(HashMap::new())),
            node_owners: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            connections: Arc::clone(&self.connections),
            tokens: Arc::clone(&self.tokens),
            node_owners: Arc::clone(&self.node_owners),
        }
    }
    
    // Generate a pairing token for a new node
    pub fn generate_token(&self, user_id: &str) -> String {
        let token = format!("sm-{}", Uuid::new_v4().to_string());
        let node_id = format!("node-{}", Uuid::new_v4().to_string());
        
        info!("Generating token {} for user {} with node ID {}", token, user_id, node_id);
        
        // Store token -> node_id mapping
        let mut tokens = self.tokens.lock().unwrap();
        tokens.insert(token.clone(), node_id.clone());
        
        // Store node_id -> user_id mapping
        let mut node_owners = self.node_owners.lock().unwrap();
        node_owners.insert(node_id, user_id.to_string());
        
        token
    }
    
    // Start the WebSocket server
    pub async fn start(&self, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        info!("WebSocket server listening on: {}", addr);
        
        let connections = Arc::clone(&self.connections);
        let tokens = Arc::clone(&self.tokens);
        let node_owners = Arc::clone(&self.node_owners);
        
        while let Ok((stream, addr)) = listener.accept().await {
            info!("New WebSocket connection from: {}", addr);
            
            let connections = Arc::clone(&connections);
            let tokens = Arc::clone(&tokens);
            let node_owners = Arc::clone(&node_owners);
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, connections, tokens, node_owners).await {
                    error!("Error handling WebSocket connection: {}", e);
                }
            });
        }
        
        Ok(())
    }
    
    // Handle a new WebSocket connection
    async fn handle_connection(
        stream: TcpStream,
        connections: Arc<Mutex<HashMap<String, Connection>>>,
        tokens: Arc<Mutex<HashMap<String, String>>>,
        node_owners: Arc<Mutex<HashMap<String, String>>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        // Create a channel for sending messages to the WebSocket
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // Generate a temporary connection ID
        let temp_id = Uuid::new_v4().to_string();
        
        // Add the connection to the map with a temporary ID
        {
            let mut connections = connections.lock().unwrap();
            connections.insert(temp_id.clone(), Connection {
                conn_type: ConnectionType::Desktop { user_id: "anonymous".to_string() },
                sender: tx.clone(),
            });
        }
        
        // Handle incoming messages
        let mut authenticated = false;
        let mut conn_id = temp_id.clone();
        
        // Spawn a task to forward messages from the channel to the WebSocket
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = ws_sender.send(msg).await {
                    error!("Error sending WebSocket message: {}", e);
                    break;
                }
            }
        });
        
        // Handle incoming messages
        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        // Try to parse as agent message first
                        if let Ok(agent_msg) = serde_json::from_str::<AgentMessage>(&text) {
                            match agent_msg {
                                AgentMessage::Authenticate { token } => {
                                    // Check if token is valid
                                    let node_id = {
                                        let tokens = tokens.lock().unwrap();
                                        tokens.get(&token).cloned()
                                    };
                                    
                                    if let Some(node_id) = node_id {
                                        // Clone token for logging
                                        let token_clone = token.clone();
                                        
                                        // Update connection type
                                        {
                                            let mut connections = connections.lock().unwrap();
                                            if let Some(conn) = connections.remove(&temp_id) {
                                                connections.insert(node_id.clone(), Connection {
                                                    conn_type: ConnectionType::Agent { token, node_id: node_id.clone() },
                                                    sender: conn.sender,
                                                });
                                            }
                                        }
                                        
                                        conn_id = node_id.clone();
                                        authenticated = true;
                                        info!("Agent authenticated with token: {}", token_clone);
                                        
                                        // Find the owner of this node
                                        let user_id = {
                                            let node_owners = node_owners.lock().unwrap();
                                            node_owners.get(&node_id).cloned()
                                        };
                                        
                                        // Forward authentication success to desktop client
                                        if let Some(user_id) = user_id {
                                            Self::forward_to_user(&connections, &user_id, Message::Text(text));
                                        }
                                    } else {
                                        error!("Invalid token: {}", token);
                                        // Send error message
                                        let error_msg = serde_json::to_string(&AgentMessage::Error {
                                            message: "Invalid token".to_string(),
                                        })?;
                                        
                                        if let Some(conn) = connections.lock().unwrap().get(&temp_id) {
                                            let _ = conn.sender.send(Message::Text(error_msg));
                                        }
                                    }
                                },
                                // Forward all other agent messages to the desktop client
                                _ => {
                                    if authenticated {
                                        // Get the user ID for this node
                                        let user_id = {
                                            let node_owners = node_owners.lock().unwrap();
                                            node_owners.get(&conn_id).cloned()
                                        };
                                        
                                        // Forward the message to the desktop client
                                        if let Some(user_id) = user_id {
                                            Self::forward_to_user(&connections, &user_id, Message::Text(text));
                                        }
                                    } else {
                                        error!("Received message from unauthenticated agent");
                                        // Send error message
                                        let error_msg = serde_json::to_string(&AgentMessage::Error {
                                            message: "Not authenticated".to_string(),
                                        })?;
                                        
                                        if let Some(conn) = connections.lock().unwrap().get(&temp_id) {
                                            let _ = conn.sender.send(Message::Text(error_msg));
                                        }
                                    }
                                }
                            }
                        }
                        // Try to parse as client message
                        else if let Ok(_client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                            // Forward client messages to the appropriate agent
                            // In a real implementation, you would need to determine which agent to forward to
                            // For now, we'll just forward to all agents
                            
                            // Get all agent connections
                            let agent_conns = {
                                let connections = connections.lock().unwrap();
                                connections.iter()
                                    .filter_map(|(id, conn)| {
                                        if let ConnectionType::Agent { .. } = conn.conn_type {
                                            Some((id.clone(), conn.sender.clone()))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            };
                            
                            // Forward the message to all agents
                            for (_, sender) in agent_conns {
                                let _ = sender.send(Message::Text(text.clone()));
                            }
                        }
                        // Unknown message format
                        else {
                            error!("Unknown message format: {}", text);
                        }
                    }
                },
                Err(e) => {
                    error!("Error receiving WebSocket message: {}", e);
                    break;
                }
            }
        }
        
        // Remove the connection from the map
        {
            let mut connections = connections.lock().unwrap();
            connections.remove(&conn_id);
        }
        
        Ok(())
    }
    
    // Forward a message to a specific user
    fn forward_to_user(
        connections: &Arc<Mutex<HashMap<String, Connection>>>,
        user_id: &str,
        msg: Message,
    ) {
        let desktop_conns = {
            let connections = connections.lock().unwrap();
            connections.iter()
                .filter_map(|(_, conn)| {
                    if let ConnectionType::Desktop { user_id: conn_user_id } = &conn.conn_type {
                        if conn_user_id == user_id {
                            Some(conn.sender.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };
        
        // Forward the message to all desktop clients for this user
        for sender in desktop_conns {
            let _ = sender.send(msg.clone());
        }
    }

    // Check if a node with the given token is connected
    pub fn is_token_connected(&self, token: &str) -> bool {
        // Check if the token is valid and has a node_id
        let node_id = {
            let tokens = self.tokens.lock().unwrap();
            let node_id = tokens.get(token).cloned();
            
            if node_id.is_none() {
                info!("Token validation failed: Token '{}' not found in tokens map", token);
            } else {
                info!("Token validation successful: Token '{}' maps to node_id '{}'", token, node_id.as_ref().unwrap());
            }
            
            node_id
        };
        
        if let Some(node_id) = node_id {
            // Check if there's a connection with this node_id
            let connections = self.connections.lock().unwrap();
            
            info!("Checking {} connections for node_id: {}", connections.len(), node_id);
            
            let mut found = false;
            for (_conn_id, conn) in connections.iter() {
                if let ConnectionType::Agent { node_id: conn_node_id, .. } = &conn.conn_type {
                    info!("Found agent connection with node_id: {}", conn_node_id);
                    if conn_node_id == &node_id {
                        info!("Connection match found for node_id: {}", node_id);
                        found = true;
                        break;
                    }
                }
            }
            
            if !found {
                info!("No connection found for node_id: {}", node_id);
            }
            
            found
        } else {
            false
        }
    }
} 

// Command handlers for Tauri
#[tauri::command]
pub fn ws_generate_pairing_token(
    websocket_server: tauri::State<'_, Arc<WebSocketServer>>
) -> Result<String, String> {
    // Generate a token for the current user
    // In a real app, you would get the user ID from authentication
    let user_id = "current-user";
    Ok(websocket_server.generate_token(user_id))
}

#[tauri::command]
pub fn ws_check_node_connected(
    websocket_server: tauri::State<'_, Arc<WebSocketServer>>,
    token: String
) -> Result<bool, String> {
    // Just check if the token exists in the tokens map - this matches the relay server behavior
    let tokens = websocket_server.tokens.lock().unwrap();
    let is_valid = tokens.contains_key(&token);
    
    info!("WebSocket token check: token {} is {}", token, if is_valid { "valid" } else { "invalid" });
    
    Ok(is_valid)
} 