<template>
  <div class="nodes-view">
    <v-container fluid>
      <!-- Header with Add Node button -->
      <v-row class="mb-4">
        <v-col>
          <h2 class="text-h5 font-weight-bold">Server Nodes</h2>
          <p class="text-subtitle-2">Manage your local and remote server nodes</p>
        </v-col>
        <v-col cols="auto" class="d-flex align-center">
          <v-btn 
            color="primary" 
            prepend-icon="mdi-server-plus"
            @click="openAddNodeDialog"
          >
            Add VPS Node
          </v-btn>
        </v-col>
      </v-row>
      
      <!-- Nodes list -->
      <v-row>
        <v-col cols="12">
          <v-card class="mb-4" v-if="isLoading">
            <v-card-text class="d-flex justify-center align-center pa-5">
              <v-progress-circular indeterminate color="primary"></v-progress-circular>
              <span class="ml-3">Loading nodes...</span>
            </v-card-text>
          </v-card>
          
          <v-card v-else-if="nodes.length === 0" class="mb-4">
            <v-card-text class="text-center pa-5">
              <v-icon size="48" color="grey">mdi-server-off</v-icon>
              <p class="mt-3 text-body-1">No server nodes found</p>
              <p class="text-caption text-medium-emphasis">
                Add a remote VPS node or use your local machine to host Minecraft servers
              </p>
              <v-btn 
                color="primary" 
                class="mt-3"
                prepend-icon="mdi-server-plus"
                @click="openAddNodeDialog"
              >
                Add VPS Node
              </v-btn>
            </v-card-text>
          </v-card>
          
          <template v-else>
            <v-card v-for="node in nodes" :key="node.id" class="mb-4 node-card">
              <v-card-item>
                <template v-slot:prepend>
                  <v-avatar 
                    :color="getNodeStatusColor(node.status)" 
                    size="40"
                    class="mr-4"
                  >
                    <v-icon :color="getNodeIconColor(node.status)">
                      {{ node.node_type === 'Local' ? 'mdi-desktop-tower' : 'mdi-server' }}
                    </v-icon>
                  </v-avatar>
                </template>
                
                <v-card-title class="text-h6 font-weight-bold">
                  {{ node.name }}
                  <v-chip 
                    size="small" 
                    :color="getNodeStatusColor(node.status)" 
                    class="ml-2"
                    :text-color="getNodeIconColor(node.status)"
                  >
                    {{ formatNodeStatus(node.status) }}
                  </v-chip>
                </v-card-title>
                
                <v-card-subtitle>
                  <span v-if="node.node_type === 'Local'">Local Machine</span>
                  <span v-else>{{ node.config.hostname || 'Remote VPS' }}</span>
                  <span class="mx-1">•</span>
                  <span>{{ node.servers.length }} servers</span>
                  <span v-if="node.last_seen" class="mx-1">•</span>
                  <span v-if="node.last_seen">Last seen {{ formatLastSeen(node.last_seen) }}</span>
                </v-card-subtitle>
                
                <template v-slot:append>
                  <div class="d-flex align-center">
                    <v-btn 
                      icon="mdi-refresh" 
                      variant="text"
                      :disabled="node.status !== 'Online'"
                      @click="refreshNode(node)"
                      class="mr-2"
                    ></v-btn>
                    <v-menu v-if="node.id !== 'local'">
                      <template v-slot:activator="{ props }">
                        <v-btn 
                          icon="mdi-dots-vertical" 
                          variant="text"
                          v-bind="props"
                        ></v-btn>
                      </template>
                      <v-list>
                        <v-list-item @click="editNode(node)">
                          <template v-slot:prepend>
                            <v-icon>mdi-pencil</v-icon>
                          </template>
                          <v-list-item-title>Edit</v-list-item-title>
                        </v-list-item>
                        <v-list-item @click="confirmRemoveNode(node)">
                          <template v-slot:prepend>
                            <v-icon color="error">mdi-delete</v-icon>
                          </template>
                          <v-list-item-title class="text-error">Remove</v-list-item-title>
                        </v-list-item>
                      </v-list>
                    </v-menu>
                  </div>
                </template>
              </v-card-item>
              
              <v-divider></v-divider>
              
              <v-card-text>
                <v-row>
                  <v-col cols="12" md="6">
                    <h3 class="text-subtitle-1 font-weight-bold mb-3">Servers</h3>
                    <div v-if="node.servers.length === 0" class="text-center py-4">
                      <v-icon color="grey" size="32">mdi-minecraft</v-icon>
                      <p class="text-caption text-medium-emphasis mt-2">No servers on this node</p>
                      <v-btn 
                        size="small" 
                        color="primary" 
                        variant="text"
                        prepend-icon="mdi-plus"
                        @click="createServer(node)"
                        class="mt-2"
                      >
                        Create Server
                      </v-btn>
                    </div>
                    <v-list v-else density="compact" class="bg-transparent">
                      <v-list-item 
                        v-for="serverId in node.servers" 
                        :key="serverId"
                        :title="getServerName(serverId)"
                        :subtitle="getServerStatus(serverId)"
                        @click="openServer(node.id, serverId)"
                      >
                        <template v-slot:prepend>
                          <v-avatar size="32" :color="getServerStatusColor(getServerStatus(serverId))">
                            <v-icon color="white">mdi-minecraft</v-icon>
                          </v-avatar>
                        </template>
                        <template v-slot:append>
                          <v-btn 
                            icon="mdi-arrow-right" 
                            variant="text" 
                            size="small"
                            @click.stop="openServer(node.id, serverId)"
                          ></v-btn>
                        </template>
                      </v-list-item>
                    </v-list>
                  </v-col>
                  
                  <v-col cols="12" md="6" v-if="node.node_type === 'Remote' && node.status === 'Online'">
                    <h3 class="text-subtitle-1 font-weight-bold mb-3">System Info</h3>
                    <div class="d-flex align-center mb-2">
                      <span class="text-caption text-medium-emphasis mr-2" style="width: 80px">CPU:</span>
                      <v-progress-linear
                        :model-value="node.metrics?.cpu || 0"
                        color="primary"
                        height="8"
                        rounded
                        class="flex-grow-1"
                      ></v-progress-linear>
                      <span class="text-caption ml-2">{{ Math.round(node.metrics?.cpu || 0) }}%</span>
                    </div>
                    <div class="d-flex align-center mb-2">
                      <span class="text-caption text-medium-emphasis mr-2" style="width: 80px">Memory:</span>
                      <v-progress-linear
                        :model-value="node.metrics?.memory || 0"
                        color="success"
                        height="8"
                        rounded
                        class="flex-grow-1"
                      ></v-progress-linear>
                      <span class="text-caption ml-2">{{ Math.round(node.metrics?.memory || 0) }}%</span>
                    </div>
                    <div class="d-flex align-center">
                      <span class="text-caption text-medium-emphasis mr-2" style="width: 80px">Disk:</span>
                      <v-progress-linear
                        :model-value="node.metrics?.disk || 0"
                        color="info"
                        height="8"
                        rounded
                        class="flex-grow-1"
                      ></v-progress-linear>
                      <span class="text-caption ml-2">{{ Math.round(node.metrics?.disk || 0) }}%</span>
                    </div>
                  </v-col>
                </v-row>
              </v-card-text>
              
              <v-card-actions v-if="node.status === 'Online'">
                <v-spacer></v-spacer>
                <v-btn 
                  color="primary" 
                  variant="text"
                  prepend-icon="mdi-plus"
                  @click="createServer(node)"
                >
                  Create Server
                </v-btn>
              </v-card-actions>
            </v-card>
          </template>
        </v-col>
      </v-row>
    </v-container>
    
    <!-- Add Node Dialog -->
    <v-dialog v-model="showAddNodeDialog" max-width="600px">
      <v-card>
        <v-card-title class="text-h5 pb-2 pt-4 px-4">
          Add VPS Node
        </v-card-title>
        
        <v-card-text class="px-4 pt-2">
          <v-stepper v-model="addNodeStep">
            <v-stepper-header>
              <v-stepper-item value="1" title="Generate Token"></v-stepper-item>
              <v-divider></v-divider>
              <v-stepper-item value="2" title="Install Agent"></v-stepper-item>
              <v-divider></v-divider>
              <v-stepper-item value="3" title="Finish"></v-stepper-item>
            </v-stepper-header>
            
            <v-stepper-window>
              <!-- Step 1: Generate Token -->
              <v-stepper-window-item value="1">
                <div class="pa-4">
                  <p class="mb-4">Generate a pairing token to connect your VPS to Servermint.</p>
                  
                  <v-text-field
                    v-model="pairingToken"
                    label="Pairing Token"
                    readonly
                    variant="outlined"
                    :loading="generatingToken"
                    append-inner-icon="mdi-content-copy"
                    @click:append-inner="copyToken"
                  ></v-text-field>
                  
                  <div class="d-flex justify-center mt-4">
                    <v-btn 
                      color="primary" 
                      :loading="generatingToken"
                      @click="generateToken"
                    >
                      Generate Token
                    </v-btn>
                  </div>
                </div>
              </v-stepper-window-item>
              
              <!-- Step 2: Install Agent -->
              <v-stepper-window-item value="2">
                <div class="pa-4">
                  <p class="mb-4">Run the following command on your VPS to install the Servermint agent:</p>
                  
                  <v-card color="grey-darken-4" class="mb-4">
                    <v-card-text class="pa-3">
                      <code>curl -sSL https://install.servermint.io | sudo bash</code>
                      <v-btn 
                        icon="mdi-content-copy" 
                        size="small" 
                        variant="text" 
                        class="ml-2"
                        @click="copyInstallCommand"
                      ></v-btn>
                    </v-card-text>
                  </v-card>
                  
                  <p class="mb-4">When prompted, enter the pairing token from the previous step.</p>
                  
                  <div class="d-flex justify-center mt-4">
                    <v-btn 
                      color="primary" 
                      :loading="checkingConnection"
                      @click="checkConnection"
                    >
                      Check Connection
                    </v-btn>
                  </div>
                </div>
              </v-stepper-window-item>
              
              <!-- Step 3: Finish -->
              <v-stepper-window-item value="3">
                <div class="pa-4 text-center">
                  <v-icon color="success" size="64">mdi-check-circle</v-icon>
                  <h3 class="text-h6 mt-4">VPS Node Connected!</h3>
                  <p class="mt-2">Your VPS node has been successfully connected to Servermint.</p>
                  
                  <v-text-field
                    v-model="nodeName"
                    label="Node Name"
                    variant="outlined"
                    class="mt-4"
                  ></v-text-field>
                  
                  <div class="d-flex justify-center mt-4">
                    <v-btn 
                      color="primary" 
                      @click="finishAddNode"
                    >
                      Finish
                    </v-btn>
                  </div>
                </div>
              </v-stepper-window-item>
            </v-stepper-window>
          </v-stepper>
        </v-card-text>
        
        <v-card-actions class="pa-4">
          <v-btn 
            variant="text" 
            @click="cancelAddNode"
          >
            Cancel
          </v-btn>
          <v-spacer></v-spacer>
          <v-btn 
            v-if="addNodeStep !== '3'"
            color="primary" 
            variant="text"
            @click="nextStep"
            :disabled="!canProceed"
          >
            Next
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    
    <!-- Remove Node Confirmation Dialog -->
    <v-dialog v-model="showRemoveDialog" max-width="400px">
      <v-card>
        <v-card-title class="text-h5">Remove Node?</v-card-title>
        <v-card-text>
          Are you sure you want to remove <strong>{{ nodeToRemove?.name }}</strong>? 
          This will disconnect the node from Servermint, but will not affect any servers on the node.
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn variant="text" @click="showRemoveDialog = false">Cancel</v-btn>
          <v-btn color="error" @click="removeNode">Remove</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'NodesView',
  data() {
    return {
      isLoading: true,
      nodes: [],
      updateInterval: null,
      
      // Add Node Dialog
      showAddNodeDialog: false,
      addNodeStep: '1',
      pairingToken: '',
      generatingToken: false,
      checkingConnection: false,
      nodeName: '',
      
      // Remove Node Dialog
      showRemoveDialog: false,
      nodeToRemove: null,
      
      // Mock data for servers (will be replaced with actual data)
      servers: {
        'server-1': { name: 'Vanilla 1.21', status: 'online' },
        'server-2': { name: 'Paper 1.20', status: 'offline' },
        'server-3': { name: 'Fabric 1.21', status: 'starting' }
      }
    };
  },
  computed: {
    canProceed() {
      if (this.addNodeStep === '1') {
        return this.pairingToken !== '';
      } else if (this.addNodeStep === '2') {
        return true; // Will be based on connection check
      }
      return false;
    }
  },
  async mounted() {
    await this.loadNodes();
    // Start periodic updates
    this.updateInterval = setInterval(this.loadNodes, 30000);
  },
  beforeUnmount() {
    // Clear update interval
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
    }
  },
  methods: {
    async loadNodes() {
      this.isLoading = true;
      try {
        this.nodes = await invoke('list_nodes');
        console.log('Loaded nodes:', this.nodes);
      } catch (error) {
        console.error('Error loading nodes:', error);
        this.nodes = [];
      } finally {
        this.isLoading = false;
      }
    },
    
    getNodeStatusColor(status) {
      switch (status) {
        case 'Online': return 'success';
        case 'Offline': return 'grey-darken-1';
        case 'Error': return 'error';
        case 'Connecting': return 'warning';
        default: return 'grey';
      }
    },
    
    getNodeIconColor() {
      return 'white'; // Keep icon color consistent
    },
    
    formatNodeStatus(status) {
      return status;
    },
    
    formatLastSeen(timestamp) {
      // Format timestamp to relative time (e.g. "2 hours ago")
      const date = new Date(timestamp);
      const now = new Date();
      const diff = Math.floor((now - date) / 1000); // seconds
      
      if (diff < 60) return 'just now';
      if (diff < 3600) return `${Math.floor(diff / 60)} minutes ago`;
      if (diff < 86400) return `${Math.floor(diff / 3600)} hours ago`;
      return `${Math.floor(diff / 86400)} days ago`;
    },
    
    getServerName(serverId) {
      return this.servers[serverId]?.name || 'Unknown Server';
    },
    
    getServerStatus(serverId) {
      return this.servers[serverId]?.status || 'unknown';
    },
    
    getServerStatusColor(status) {
      switch (status) {
        case 'online': return 'success';
        case 'offline': return 'grey';
        case 'starting': return 'warning';
        case 'stopping': return 'error';
        default: return 'grey';
      }
    },
    
    async refreshNode(node) {
      // Refresh node info
      console.log('Refreshing node:', node.id);
      await this.loadNodes();
    },
    
    openServer(nodeId, serverId) {
      console.log('Opening server:', serverId, 'on node:', nodeId);
      // Navigate to server management page
      this.$router.push(`/server/${serverId}?node=${nodeId}`);
    },
    
    createServer(node) {
      console.log('Creating server on node:', node.id);
      // Navigate to server creation page with node pre-selected
      this.$router.push(`/create-server?node=${node.id}`);
    },
    
    editNode(node) {
      console.log('Editing node:', node.id);
      // Open edit node dialog
    },
    
    confirmRemoveNode(node) {
      this.nodeToRemove = node;
      this.showRemoveDialog = true;
    },
    
    async removeNode() {
      if (!this.nodeToRemove) return;
      
      try {
        await invoke('remove_node', { id: this.nodeToRemove.id });
        console.log('Node removed:', this.nodeToRemove.id);
        this.showRemoveDialog = false;
        this.nodeToRemove = null;
        await this.loadNodes();
      } catch (error) {
        console.error('Error removing node:', error);
      }
    },
    
    // Add Node Dialog Methods
    openAddNodeDialog() {
      this.showAddNodeDialog = true;
      this.addNodeStep = '1';
      this.pairingToken = '';
      this.nodeName = '';
    },
    
    async generateToken() {
      this.generatingToken = true;
      try {
        // Call relay server directly
        const response = await fetch('https://relay.servermint.app/api/token', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            userId: 'current-user' // TODO: Get from auth
          })
        });
        
        if (!response.ok) {
          throw new Error(`Failed to generate token: ${response.statusText}`);
        }
        
        const data = await response.json();
        this.pairingToken = data.token;
        console.log('Generated token:', this.pairingToken);
      } catch (error) {
        console.error('Error generating token:', error);
        window.showError('Token Generation Failed', error.toString());
      } finally {
        this.generatingToken = false;
      }
    },
    
    copyToken() {
      navigator.clipboard.writeText(this.pairingToken);
      // Show toast notification
      this.$emit('show-toast', 'Token copied to clipboard', 'success');
    },
    
    copyInstallCommand() {
      // Include the token in the install command
      const command = `curl -sSL https://install.servermint.app | sudo TOKEN="${this.pairingToken}" bash`;
      navigator.clipboard.writeText(command);
      // Show toast notification
      window.showSuccess('Install command copied to clipboard', 'Command copied. Run this on your VPS to install the agent.');
    },
    
    async checkConnection() {
      this.checkingConnection = true;
      try {
        console.log(`Checking connection for token: ${this.pairingToken}`);
        
        // Poll for node connection status
        let attempts = 0;
        const maxAttempts = 30; // 30 seconds timeout
        
        while (attempts < maxAttempts) {
          // Check connection via WebSocket
          const ws = new WebSocket('wss://relay.servermint.app/ws');
          
          const connected = await new Promise((resolve) => {
            ws.onopen = () => {
              // Send auth message
              ws.send(JSON.stringify({
                type: 'Authenticate',
                data: {
                  userId: 'current-user',
                  token: this.pairingToken
                }
              }));
            };
            
            ws.onmessage = (event) => {
              const data = JSON.parse(event.data);
              if (data.type === 'NodeList') {
                resolve(true);
                ws.close();
              } else if (data.type === 'Error') {
                resolve(false);
                ws.close();
              }
            };
            
            ws.onerror = () => {
              resolve(false);
              ws.close();
            };
            
            // Timeout after 1 second
            setTimeout(() => {
              resolve(false);
              ws.close();
            }, 1000);
          });
          
          if (connected) {
            this.addNodeStep = '3';
            return;
          }
          
          attempts++;
          await new Promise(resolve => setTimeout(resolve, 1000));
        }
        
        throw new Error('Connection timeout - please check if the agent is running and try again');
      } catch (error) {
        console.error('Error checking connection:', error);
        window.showError('Connection Error', error.toString());
      } finally {
        this.checkingConnection = false;
      }
    },
    
    nextStep() {
      if (this.addNodeStep === '1' && this.pairingToken) {
        this.addNodeStep = '2';
      } else if (this.addNodeStep === '2') {
        this.checkConnection();
      }
    },
    
    async finishAddNode() {
      if (!this.nodeName) {
        this.nodeName = `VPS Node ${Math.floor(Math.random() * 1000)}`;
      }
      
      try {
        console.log('Finalizing node addition with name:', this.nodeName);
        
        // Generate a new node ID that will match what the relay server will assign
        // when the agent connects
        const nodeId = `node-${Date.now()}`;
        console.log(`Generated node ID: ${nodeId}`);
        
        const newNode = {
          id: nodeId,
          name: this.nodeName,
          node_type: 'Remote',
          status: 'Connecting', // Set to connecting initially
          config: {
            name: this.nodeName,
            hostname: 'pending-connection', // Will be updated when agent reports
            token: this.pairingToken // Store the token with the node
          },
          last_seen: new Date().toISOString(),
          servers: []
        };
        
        console.log('Adding node with data:', JSON.stringify(newNode));
        await invoke('add_node', { node: newNode });
        
        this.showAddNodeDialog = false;
        await this.loadNodes();
        
        // Show success notification with instructions
        window.showSuccess('Node Registration Started', 
          `Node "${this.nodeName}" has been registered. Please complete the installation on your VPS by entering the token: ${this.pairingToken}`
        );
      } catch (error) {
        console.error('Error adding node:', error);
        window.showError('Error Adding Node', error.toString());
      }
    },
    
    cancelAddNode() {
      this.showAddNodeDialog = false;
    }
  }
};
</script>

<style scoped>
.node-card {
  transition: transform 0.2s ease;
}

.node-card:hover {
  transform: translateY(-2px);
}
</style> 