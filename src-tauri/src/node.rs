use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use log::{info, error};

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
    pub servers: Vec<String>,   
    pub metrics: Option<NodeMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub cpu: f32,
    pub memory: f32,
    pub disk: f32,
}

#[derive(Debug)]
pub struct NodeManager {
    nodes: HashMap<String, Node>,
    server_manager: Arc<Mutex<ServerManager>>,
    pub active_tokens: HashMap<String, (String, DateTime<Utc>)>,
}

impl NodeManager {
    pub fn new(server_manager: Arc<Mutex<ServerManager>>) -> Self {
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
            metrics: None,
        };
        nodes.insert(local_node.id.clone(), local_node);
        
        Self {
            nodes,
            server_manager,
            active_tokens: HashMap::new(),
        }
    }
    
    pub fn list_nodes(&self) -> Vec<Node> {
        let mut nodes = self.nodes.values().cloned().collect::<Vec<_>>();
        
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
        let expiry = Utc::now() + chrono::Duration::minutes(10);
        self.active_tokens.insert(token.clone(), ("pending".to_string(), expiry));
        token
    }
    
    pub fn update_node_metrics(&mut self, node_id: &str, metrics: NodeMetrics) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.metrics = Some(metrics);
            node.last_seen = Some(Utc::now());
            node.status = NodeStatus::Online;
            Ok(())
        } else {
            Err(format!("Node with ID {} not found", node_id))
        }
    }

    pub fn update_node_status(&mut self, node_id: &str, status: NodeStatus) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.status = status;
            Ok(())
        } else {
            Err(format!("Node with ID {} not found", node_id))
        }
    }

    pub fn validate_token(&mut self, token: &str) -> Option<String> {
        if let Some((node_id, expiry)) = self.active_tokens.get(token) {
            if expiry > &Utc::now() {
                if node_id == "pending" {
                    let node_id = format!("node-{}", Uuid::new_v4().to_string());
                    self.active_tokens.insert(token.to_string(), (node_id.clone(), *expiry));
                    
                    let node = Node {
                        id: node_id.clone(),
                        name: format!("VPS Node {}", &node_id[5..11]),
                        node_type: NodeType::Remote,
                        status: NodeStatus::Connecting,
                        config: NodeConfig {
                            name: format!("VPS Node {}", &node_id[5..11]),
                            hostname: None,
                            port: None,
                            ssh_key_path: None,
                            username: None,
                            api_token: Some(token.to_string()),
                        },
                        last_seen: Some(Utc::now()),
                        servers: Vec::new(),
                        metrics: None,
                    };
                    
                    if let Err(e) = self.add_node(node) {
                        error!("Failed to create node for token {}: {}", token, e);
                        return None;
                    }
                    
                    return Some(node_id);
                } else {
                    return Some(node_id.clone());
                }
            }
        }
        
        if token.starts_with("sm-") {
            let node_id = format!("node-{}", Uuid::new_v4().to_string());
            let expiry = Utc::now() + chrono::Duration::minutes(10);
            self.active_tokens.insert(token.to_string(), (node_id.clone(), expiry));
            
            let node = Node {
                id: node_id.clone(),
                name: format!("VPS Node {}", &node_id[5..11]),
                node_type: NodeType::Remote,
                status: NodeStatus::Connecting,
                config: NodeConfig {
                    name: format!("VPS Node {}", &node_id[5..11]),
                    hostname: None,
                    port: None,
                    ssh_key_path: None,
                    username: None,
                    api_token: Some(token.to_string()),
                },
                last_seen: Some(Utc::now()),
                servers: Vec::new(),
                metrics: None,
            };
            
            if let Err(e) = self.add_node(node) {
                error!("Failed to create node for token {}: {}", token, e);
                return None;
            }
            
            return Some(node_id);
        }
        
        None
    }
    
    pub fn cleanup_expired_tokens(&mut self) {
        let now = Utc::now();
        self.active_tokens.retain(|_, (_, expiry)| *expiry > now);
    }
}

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
    
    if let Some(node_id) = node_manager.validate_token(&token) {
        info!("Token {} is valid in node manager, associated with node ID {}", token, node_id);
        return Ok(true);
    } else {
        info!("Token {} not found in node manager", token);
    }
    
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
    
    if let Some(node_id) = node_manager.validate_token(&token) {
        if let Some(node) = node_manager.get_node(&node_id) {
            Ok(NodeInfoResponse {
                id: node_id,
                hostname: node.config.hostname.clone(),
            })
        } else {
            Err(format!("Node info not available for token: {}", token))
        }
    } else {
        Err(format!("Invalid or unused token: {}", token))
    }
} 

#[tauri::command]
pub fn update_node_metrics(
    state: NodeManagerState,
    node_id: String,
    metrics: NodeMetrics,
) -> Result<(), String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    node_manager.update_node_metrics(&node_id, metrics)
}

#[tauri::command]
pub fn update_node_status(
    state: NodeManagerState,
    node_id: String,
    status: NodeStatus,
) -> Result<(), String> {
    let mut node_manager = state.lock().map_err(|e| format!("Failed to lock node manager: {}", e))?;
    node_manager.update_node_status(&node_id, status)
} 