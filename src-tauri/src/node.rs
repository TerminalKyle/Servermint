use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use log::info;

use crate::server::ServerManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Local,
    Remote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Connecting,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub name: String,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub ssh_key_path: Option<String>,
    pub username: Option<String>,
    pub api_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub status: NodeStatus,
    pub config: NodeConfig,
    pub last_seen: Option<DateTime<Utc>>,
    pub servers: Vec<String>, // Server IDs
}

#[derive(Debug)]
pub struct NodeManager {
    nodes: HashMap<String, Node>,
    server_manager: Arc<Mutex<ServerManager>>,
    active_tokens: HashMap<String, (String, DateTime<Utc>)>, // token -> (node_id, expiry)
}

impl NodeManager {
    pub fn new(server_manager: Arc<Mutex<ServerManager>>) -> Self {
        // Create default local node
        let mut nodes = HashMap::new();
        let local_node = Node {
            id: "local".to_string(),
            name: "My PC".to_string(),
            node_type: NodeType::Local,
            status: NodeStatus::Online,
            config: NodeConfig {
                name: "Local".to_string(),
                hostname: None,
                port: None,
                ssh_key_path: None,
                username: None,
                api_token: None,
            },
            last_seen: Some(Utc::now()),
            servers: Vec::new(),
        };
        nodes.insert(local_node.id.clone(), local_node);
        
        Self {
            nodes,
            server_manager,
            active_tokens: HashMap::new(),
        }
    }
    
    pub fn list_nodes(&self) -> Vec<Node> {
        // Update local node servers
        let mut nodes = self.nodes.values().cloned().collect::<Vec<_>>();
        
        // Update local node with current servers
        if let Some(local_node) = nodes.iter_mut().find(|n| n.id == "local") {
            if let Ok(server_manager) = self.server_manager.lock() {
                let server_infos = server_manager.list_servers();
                local_node.servers = server_infos.iter().map(|s| s.id.clone()).collect();
            }
        }
        
        nodes
    }
    
    pub fn get_node(&self, id: &str) -> Option<Node> {
        self.nodes.get(id).cloned()
    }
    
    pub fn add_node(&mut self, node: Node) -> Result<(), String> {
        if self.nodes.contains_key(&node.id) {
            return Err(format!("Node with ID {} already exists", node.id));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }
    
    pub fn update_node(&mut self, id: &str, node: Node) -> Result<(), String> {
        if !self.nodes.contains_key(id) {
            return Err(format!("Node with ID {} not found", id));
        }
        self.nodes.insert(id.to_string(), node);
        Ok(())
    }
    
    pub fn remove_node(&mut self, id: &str) -> Result<(), String> {
        if id == "local" {
            return Err("Cannot remove local node".to_string());
        }
        if !self.nodes.contains_key(id) {
            return Err(format!("Node with ID {} not found", id));
        }
        self.nodes.remove(id);
        Ok(())
    }
    
    pub fn generate_pairing_token(&mut self) -> String {
        let token = Uuid::new_v4().to_string();
        // Token valid for 10 minutes
        let expiry = Utc::now() + chrono::Duration::minutes(10);
        // We'll associate the token with a node ID when the node connects
        self.active_tokens.insert(token.clone(), ("pending".to_string(), expiry));
        token
    }
    
    pub fn validate_token(&mut self, token: &str) -> Option<String> {
        if let Some((node_id, expiry)) = self.active_tokens.get(token) {
            if expiry > &Utc::now() {
                if node_id == "pending" {
                    // Generate a new node ID
                    let node_id = format!("node-{}", Uuid::new_v4().to_string());
                    // Update token with node ID
                    self.active_tokens.insert(token.to_string(), (node_id.clone(), *expiry));
                    return Some(node_id);
                } else {
                    return Some(node_id.clone());
                }
            }
        }
        None
    }
    
    pub fn cleanup_expired_tokens(&mut self) {
        let now = Utc::now();
        self.active_tokens.retain(|_, (_, expiry)| *expiry > now);
    }
}

// Tauri command handlers
type NodeManagerState<'a> = State<'a, Arc<Mutex<NodeManager>>>;

#[tauri::command]
pub fn list_nodes(state: NodeManagerState) -> Result<Vec<Node>, String> {
    let node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    Ok(node_manager.list_nodes())
}

#[tauri::command]
pub fn get_node(state: NodeManagerState, id: String) -> Result<Node, String> {
    let node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    node_manager.get_node(&id).ok_or_else(|| format!("Node with ID {} not found", id))
}

#[tauri::command]
pub fn add_node(state: NodeManagerState, node: Node) -> Result<(), String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    node_manager.add_node(node)
}

#[tauri::command]
pub fn update_node(state: NodeManagerState, id: String, node: Node) -> Result<(), String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    node_manager.update_node(&id, node)
}

#[tauri::command]
pub fn remove_node(state: NodeManagerState, id: String) -> Result<(), String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    node_manager.remove_node(&id)
}

#[tauri::command]
pub fn generate_pairing_token(state: NodeManagerState) -> Result<String, String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    Ok(node_manager.generate_pairing_token())
} 

#[tauri::command]
pub fn check_node_connected(state: NodeManagerState, token: String) -> Result<bool, String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    
    // Check if token is valid in the node manager
    if let Some(node_id) = node_manager.validate_token(&token) {
        info!("Token {} is valid in node manager, associated with node ID {}", token, node_id);
        return Ok(true);
    } else {
        info!("Token {} not found in node manager", token);
    }
    
    // Token not found or node not connected
    Ok(false)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoResponse {
    pub id: String,
    pub hostname: Option<String>,
}

#[tauri::command]
pub fn get_node_info_by_token(state: NodeManagerState, token: String) -> Result<NodeInfoResponse, String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    
    // Check if token is valid and has been used by a node to connect
    if let Some(node_id) = node_manager.validate_token(&token) {
        // Check if node exists with this ID
        if let Some(node) = node_manager.get_node(&node_id) {
            // Return node information
            Ok(NodeInfoResponse {
                id: node_id,
                hostname: node.config.hostname.clone(),
            })
        } else {
            // Node ID exists but node doesn't exist yet
            Err(format!("Node info not available for token: {}", token))
        }
    } else {
        // Token is invalid or not used yet
        Err(format!("Invalid or unused token: {}", token))
    }
} 