const WebSocket = require('ws');
const http = require('http');
const https = require('https');
const fs = require('fs');
const { v4: uuidv4 } = require('uuid');
const express = require('express');
const app = express();

// Configuration
const config = {
  port: process.env.PORT || 8080,
  useHttps: process.env.USE_HTTPS === 'true',
  sslCert: process.env.SSL_CERT || './ssl/cert.pem',
  sslKey: process.env.SSL_KEY || './ssl/key.pem',
  logLevel: process.env.LOG_LEVEL || 'info' // debug, info, warn, error
};

// Logging
const logLevels = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3
};

const logLevel = logLevels[config.logLevel];

function log(level, message, data = null) {
  if (logLevels[level] >= logLevel) {
    const timestamp = new Date().toISOString();
    const logMessage = `[${timestamp}][${level.toUpperCase()}] ${message}`;
    
    if (data) {
      console.log(logMessage, data);
    } else {
      console.log(logMessage);
    }
  }
}

// Create HTTP or HTTPS server
let server;
if (config.useHttps) {
  try {
    const sslOptions = {
      cert: fs.readFileSync(config.sslCert),
      key: fs.readFileSync(config.sslKey)
    };
    server = https.createServer(sslOptions, app);
    log('info', 'Created HTTPS server');
  } catch (err) {
    log('error', 'Failed to load SSL certificates:', err);
    process.exit(1);
  }
} else {
  server = http.createServer(app);
  log('info', 'Created HTTP server');
}

// Create WebSocket server
const wss = new WebSocket.Server({ server });

// Data structures
const connections = new Map(); // connectionId -> { type, userId, nodeId, socket }
const tokens = new Map();      // token -> { nodeId, userId }
const nodeOwners = new Map();  // nodeId -> userId
const userNodes = new Map();   // userId -> Set(nodeId)

// Express routes
app.get('/', (req, res) => {
  res.send('Servermint Relay Server');
});

app.get('/health', (req, res) => {
  res.json({
    status: 'ok',
    connections: connections.size,
    tokens: tokens.size,
    nodes: nodeOwners.size
  });
});

// Generate a token for node pairing
app.post('/api/token', express.json(), (req, res) => {
  const userId = req.body.userId || 'anonymous';
  const token = `sm-${uuidv4()}`;
  const nodeId = `node-${uuidv4()}`;
  
  // Store token -> node mapping
  tokens.set(token, { nodeId, userId });
  
  // Store node -> user mapping
  nodeOwners.set(nodeId, userId);
  
  // Add node to user's nodes
  if (!userNodes.has(userId)) {
    userNodes.set(userId, new Set());
  }
  userNodes.get(userId).add(nodeId);
  
  log('info', `Generated token for user ${userId}, node ${nodeId}`);
  
  // Token expires after 1 hour
  setTimeout(() => {
    if (tokens.has(token)) {
      const { nodeId } = tokens.get(token);
      tokens.delete(token);
      log('info', `Token expired for node ${nodeId}`);
    }
  }, 60 * 60 * 1000);
  
  res.json({ token });
});

// WebSocket connection handler
wss.on('connection', (ws, req) => {
  const connectionId = uuidv4();
  
  // Add connection to map with temporary ID
  connections.set(connectionId, {
    type: 'unknown',
    socket: ws,
    userId: null,
    nodeId: null
  });
  
  log('info', `New connection: ${connectionId}`);
  
  // Handle messages
  ws.on('message', (message) => {
    try {
      const data = JSON.parse(message);
      log('debug', `Received message from ${connectionId}:`, data);
      
      // Handle authentication
      if (data.type === 'Authenticate') {
        const token = data.data.token;
        
        // Check if token is valid
        if (tokens.has(token)) {
          const { nodeId, userId } = tokens.get(token);
          
          // Update connection type
          const connection = connections.get(connectionId);
          connection.type = 'agent';
          connection.nodeId = nodeId;
          connection.userId = userId;
          
          log('info', `Agent authenticated: ${nodeId} (user: ${userId})`);
          
          // Forward authentication success to desktop clients
          forwardToUser(userId, {
            type: 'AgentConnected',
            data: { nodeId }
          });
        } else if (data.data.userId) {
          // Desktop client authentication
          const userId = data.data.userId;
          
          // Update connection type
          const connection = connections.get(connectionId);
          connection.type = 'desktop';
          connection.userId = userId;
          
          log('info', `Desktop client authenticated: ${userId}`);
          
          // Send list of nodes to desktop client
          const nodes = Array.from(nodeOwners.entries())
            .filter(([_, owner]) => owner === userId)
            .map(([nodeId]) => nodeId);
          
          ws.send(JSON.stringify({
            type: 'NodeList',
            data: { nodes }
          }));
        } else {
          log('warn', `Invalid authentication attempt from ${connectionId}`);
          ws.send(JSON.stringify({
            type: 'Error',
            data: { message: 'Invalid authentication' }
          }));
        }
      }
      // Handle agent messages (forward to desktop)
      else if (isAgentMessage(data.type)) {
        const connection = connections.get(connectionId);
        
        if (connection && connection.type === 'agent' && connection.nodeId) {
          // Forward to desktop clients of the same user
          forwardToUser(connection.userId, data);
        } else {
          log('warn', `Unauthorized agent message from ${connectionId}`);
          ws.send(JSON.stringify({
            type: 'Error',
            data: { message: 'Not authenticated as agent' }
          }));
        }
      }
      // Handle desktop client messages (forward to agent)
      else if (isClientMessage(data.type)) {
        const connection = connections.get(connectionId);
        
        if (connection && connection.type === 'desktop' && connection.userId) {
          // Add target node ID to message
          if (data.targetNodeId) {
            // Forward to specific node
            forwardToNode(data.targetNodeId, data);
          } else {
            log('warn', `Client message missing target node ID: ${connectionId}`);
            ws.send(JSON.stringify({
              type: 'Error',
              data: { message: 'Missing target node ID' }
            }));
          }
        } else {
          log('warn', `Unauthorized client message from ${connectionId}`);
          ws.send(JSON.stringify({
            type: 'Error',
            data: { message: 'Not authenticated as desktop client' }
          }));
        }
      }
      // Unknown message type
      else {
        log('warn', `Unknown message type from ${connectionId}: ${data.type}`);
        ws.send(JSON.stringify({
          type: 'Error',
          data: { message: 'Unknown message type' }
        }));
      }
    } catch (err) {
      log('error', `Error processing message from ${connectionId}:`, err);
      ws.send(JSON.stringify({
        type: 'Error',
        data: { message: 'Invalid message format' }
      }));
    }
  });
  
  // Handle disconnection
  ws.on('close', () => {
    const connection = connections.get(connectionId);
    
    if (connection) {
      if (connection.type === 'agent' && connection.nodeId) {
        log('info', `Agent disconnected: ${connection.nodeId}`);
        
        // Notify desktop clients
        forwardToUser(connection.userId, {
          type: 'AgentDisconnected',
          data: { nodeId: connection.nodeId }
        });
      } else if (connection.type === 'desktop') {
        log('info', `Desktop client disconnected: ${connection.userId}`);
      } else {
        log('info', `Unknown connection closed: ${connectionId}`);
      }
      
      // Remove connection
      connections.delete(connectionId);
    }
  });
  
  // Handle errors
  ws.on('error', (err) => {
    log('error', `WebSocket error for ${connectionId}:`, err);
    connections.delete(connectionId);
  });
});

// Helper functions
function forwardToUser(userId, message) {
  let count = 0;
  
  for (const [_, connection] of connections.entries()) {
    if (connection.type === 'desktop' && connection.userId === userId) {
      connection.socket.send(JSON.stringify(message));
      count++;
    }
  }
  
  log('debug', `Forwarded message to ${count} desktop clients for user ${userId}`);
}

function forwardToNode(nodeId, message) {
  for (const [_, connection] of connections.entries()) {
    if (connection.type === 'agent' && connection.nodeId === nodeId) {
      connection.socket.send(JSON.stringify(message));
      log('debug', `Forwarded message to node ${nodeId}`);
      return true;
    }
  }
  
  log('warn', `Failed to forward message: Node ${nodeId} not connected`);
  return false;
}

function isAgentMessage(type) {
  return [
    'Authenticate',
    'NodeInfo',
    'ServerList',
    'ServerStatus',
    'ServerLogs',
    'CommandResult',
    'Error'
  ].includes(type);
}

function isClientMessage(type) {
  return [
    'CreateServer',
    'StartServer',
    'StopServer',
    'SendCommand',
    'RequestLogs',
    'RequestServerList',
    'RequestNodeInfo'
  ].includes(type);
}

// Start server
server.listen(config.port, () => {
  log('info', `Server listening on port ${config.port}`);
  log('info', `WebSocket endpoint: ${config.useHttps ? 'wss' : 'ws'}://localhost:${config.port}`);
}); 