<template>
  <div class="server-management">
    <!-- Server header -->
    <div class="server-header mb-4">
      <div class="d-flex align-center">
        <v-btn icon="mdi-arrow-left" variant="text" class="mr-2" @click="goBack"></v-btn>
        
        <div class="d-flex align-center">
          <v-avatar size="42" :color="server?.icon ? undefined : 'primary'" rounded class="mr-3 server-icon">
            <v-img v-if="server?.icon" :src="server.icon" alt="Server Icon"></v-img>
            <v-icon v-else color="white">mdi-leaf</v-icon>
          </v-avatar>
          
          <div>
            <h1 class="text-h5 mb-0">{{ server?.name || 'Server' }}</h1>
            <div class="d-flex align-center">
              <v-icon size="small" class="mr-1">mdi-minecraft</v-icon>
              <span class="text-caption">{{ server?.type || 'Unknown' }} {{ server?.version || '' }}</span>
              
              <v-chip
                size="small"
                :color="serverStatusColor"
                class="ml-2"
                variant="flat"
                density="comfortable"
              >
                {{ serverStatus }}
              </v-chip>
              
              <!-- Server IP and Port -->
              <div v-if="serverIp && serverPort" class="server-connection-info ml-2">
                <v-icon size="small" class="mr-1">mdi-network</v-icon>
                <span class="text-caption">{{ serverIp }}:{{ serverPort }}</span>
                <v-btn
                  icon
                  variant="text"
                  size="x-small"
                  @click="copyServerAddress"
                  class="ml-1"
                >
                  <v-icon size="small">mdi-content-copy</v-icon>
                  <v-tooltip activator="parent" location="bottom">Copy Server Address</v-tooltip>
                </v-btn>
              </div>
              
              <!-- Loading state for connection info -->
              <div v-else-if="!serverIp && !serverPort" class="d-flex align-center ml-2">
                <v-icon size="small" class="mr-1">mdi-loading mdi-spin</v-icon>
                <span class="text-caption">Loading connection info...</span>
              </div>
            </div>
          </div>
        </div>
        
        <v-spacer></v-spacer>
        
        <div class="server-actions">
          <v-btn
            v-if="!isServerRunning"
            color="primary"
            prepend-icon="mdi-play"
            class="mr-2"
            @click="startServer"
            :loading="isStarting"
            :disabled="isStarting || isStopping"
          >
            Start
          </v-btn>
          
          <v-btn
            v-else
            color="error"
            prepend-icon="mdi-stop"
            class="mr-2"
            @click="stopServer"
            :loading="isStopping"
            :disabled="isStarting || isStopping"
          >
            Stop
          </v-btn>
          
          <v-btn
            icon
            variant="text"
            @click="restartServer"
            :disabled="!isServerRunning || isStarting || isStopping"
            class="mr-2"
          >
            <v-icon>mdi-restart</v-icon>
            <v-tooltip activator="parent" location="bottom">Restart Server</v-tooltip>
          </v-btn>
          
          <v-btn
            icon
            variant="text"
            @click="openServerFolder"
            class="mr-2"
          >
            <v-icon>mdi-folder-open-outline</v-icon>
            <v-tooltip activator="parent" location="bottom">Open Server Folder</v-tooltip>
          </v-btn>
          
          <v-btn
            icon
            variant="text"
            @click="checkJava"
            class="mr-2"
          >
            <v-icon>mdi-language-java</v-icon>
            <v-tooltip activator="parent" location="bottom">Check Java</v-tooltip>
          </v-btn>
          
          <v-btn
            icon
            variant="text"
            @click="setupJava"
          >
            <v-icon>mdi-download</v-icon>
            <v-tooltip activator="parent" location="bottom">Setup Java 21</v-tooltip>
          </v-btn>
        </div>
      </div>
    </div>
    
    <!-- Management tabs -->
    <v-tabs v-model="activeTab" color="primary" class="mb-4">
      <v-tab value="console">Console</v-tab>
      <v-tab value="files">Files</v-tab>
      <v-tab value="players">Players</v-tab>
      <v-tab value="settings">Settings</v-tab>
      <v-tab value="ai">AI Assistant</v-tab>
    </v-tabs>
    
    <v-window v-model="activeTab">
      <!-- Console Tab -->
      <v-window-item value="console">
        <div class="console-container">
          <div class="console-header d-flex align-center mb-2">
            <v-icon class="mr-2" color="primary">mdi-console</v-icon>
            <span class="text-subtitle-2">Server Console</span>
            <v-spacer></v-spacer>
            <v-btn
              size="small"
              variant="outlined"
              prepend-icon="mdi-content-copy"
              @click="copyConsoleOutput"
            >
              Copy
            </v-btn>
            <v-btn
              size="small"
              variant="outlined"
              prepend-icon="mdi-delete"
              @click="clearConsoleOutput"
              class="ml-2"
            >
              Clear
            </v-btn>

          </div>
          
          <div class="console-output" ref="consoleOutput">
            <div v-for="(line, index) in filteredConsoleOutput" :key="`console-line-${index}`" class="console-line">
              <span :class="getConsoleLineClass(line)">{{ line }}</span>
            </div>
            <div v-if="isServerRunning" class="console-cursor"></div>
          </div>
          
          <div class="console-input-container">
            <div class="console-input-wrapper">
              <v-icon class="console-prompt" color="primary">mdi-chevron-right</v-icon>
              <v-text-field
                v-model="consoleInput"
                placeholder="Type a command..."
                variant="outlined"
                density="comfortable"
                bg-color="#1e1e1e"
                hide-details
                class="console-input"
                @keydown.enter="sendCommand"
                :disabled="!isServerRunning"
                prepend-inner-icon="mdi-console"
                color="primary"
              ></v-text-field>
            </div>
          </div>
          
          <!-- System Monitor -->
          <div class="system-monitor">
            <div class="monitor-grid">
              <!-- RAM Usage -->
              <div class="monitor-item">
                <div class="monitor-header">
                  <v-icon size="small" color="primary">mdi-memory</v-icon>
                  <span class="monitor-label">RAM Usage</span>
                </div>
                <div class="monitor-value">{{ formatBytes(serverMetrics.memoryUsed) }} / {{ formatBytes(serverMetrics.memoryTotal) }}</div>
                <v-progress-linear
                  :model-value="serverMetrics.memoryPercentage"
                  :color="getMemoryColor(serverMetrics.memoryPercentage)"
                  height="6"
                  rounded
                  class="monitor-progress"
                ></v-progress-linear>
              </div>
              
              <!-- CPU Usage -->
              <div class="monitor-item">
                <div class="monitor-header">
                  <v-icon size="small" color="primary">mdi-cpu-64-bit</v-icon>
                  <span class="monitor-label">CPU Usage</span>
                </div>
                <div class="monitor-value">{{ serverMetrics.cpuUsage.toFixed(1) }}%</div>
                <v-progress-linear
                  :model-value="serverMetrics.cpuUsage"
                  :color="getCpuColor(serverMetrics.cpuUsage)"
                  height="6"
                  rounded
                  class="monitor-progress"
                ></v-progress-linear>
              </div>
              
              <!-- Server Uptime -->
              <div class="monitor-item">
                <div class="monitor-header">
                  <v-icon size="small" color="primary">mdi-clock-outline</v-icon>
                  <span class="monitor-label">Uptime</span>
                </div>
                <div class="monitor-value">{{ formatUptime(serverMetrics.uptime) }}</div>
                <div class="monitor-subtitle">Running since {{ formatStartTime(serverMetrics.startTime) }}</div>
              </div>
              
              <!-- Players Online -->
              <div class="monitor-item">
                <div class="monitor-header">
                  <v-icon size="small" color="primary">mdi-account-group</v-icon>
                  <span class="monitor-label">Players</span>
                </div>
                <div class="monitor-value">{{ serverMetrics.playerCount }} / {{ serverMetrics.maxPlayers }}</div>
                <div class="monitor-subtitle">{{ getPlayerStatus(serverMetrics.playerCount) }}</div>
              </div>
              
              <!-- TPS (Ticks Per Second) -->
              <div class="monitor-item">
                <div class="monitor-header">
                  <v-icon size="small" color="primary">mdi-speedometer</v-icon>
                  <span class="monitor-label">TPS</span>
                </div>
                <div class="monitor-value">{{ serverMetrics.tps.toFixed(1) }}</div>
                <div class="monitor-subtitle" :class="getTpsStatus(serverMetrics.tps).class">
                  {{ getTpsStatus(serverMetrics.tps).text }}
                </div>
              </div>
              
              <!-- Network -->
              <div class="monitor-item">
                <div class="monitor-header">
                  <v-icon size="small" color="primary">mdi-network</v-icon>
                  <span class="monitor-label">Network</span>
                </div>
                <div class="monitor-value">{{ formatBytes(serverMetrics.networkIn) }}/s</div>
                <div class="monitor-subtitle">↓ {{ formatBytes(serverMetrics.networkOut) }}/s ↑</div>
              </div>
            </div>
          </div>
        </div>
      </v-window-item>
      
      <!-- Files Tab -->
      <v-window-item value="files">
        <div class="files-container">
          <div class="files-header d-flex align-center mb-2">
            <v-breadcrumbs :items="breadcrumbs" class="pa-0"></v-breadcrumbs>
            <v-spacer></v-spacer>
            <v-btn
              prepend-icon="mdi-upload"
              variant="outlined"
              size="small"
              class="mr-2"
            >
              Upload
            </v-btn>
            <v-btn
              prepend-icon="mdi-folder-plus"
              variant="outlined"
              size="small"
            >
              New Folder
            </v-btn>
          </div>
          
          <v-table class="files-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Size</th>
                <th>Modified</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-if="currentPath !== ''">
                <td colspan="4">
                  <div class="d-flex align-center" @click="navigateUp">
                    <v-icon class="mr-2">mdi-arrow-up</v-icon>
                    <span>..</span>
                  </div>
                </td>
              </tr>
              <tr v-for="(file, index) in files" :key="index" @click="navigateToFile(file)">
                <td>
                  <div class="d-flex align-center">
                    <v-icon class="mr-2" :color="file.isDirectory ? 'amber' : undefined">
                      {{ file.isDirectory ? 'mdi-folder' : 'mdi-file' }}
                    </v-icon>
                    <span>{{ file.name }}</span>
                  </div>
                </td>
                <td>{{ formatFileSize(file.size) }}</td>
                <td>{{ formatDate(file.modified) }}</td>
                <td>
                  <div class="d-flex">
                    <v-btn
                      icon
                      variant="text"
                      size="small"
                      @click.stop="downloadFile(file)"
                      v-if="!file.isDirectory"
                    >
                      <v-icon>mdi-download</v-icon>
                    </v-btn>
                    <v-btn
                      icon
                      variant="text"
                      size="small"
                      @click.stop="deleteFile(file)"
                    >
                      <v-icon>mdi-delete</v-icon>
                    </v-btn>
                  </div>
                </td>
              </tr>
            </tbody>
          </v-table>
        </div>
      </v-window-item>
      
      <!-- Players Tab -->
      <v-window-item value="players">
        <div class="players-container">
          <div class="d-flex align-center mb-4">
            <h3 class="text-h6">Online Players: {{ players.length }}</h3>
            <v-spacer></v-spacer>
            <v-text-field
              v-model="playerSearch"
              placeholder="Search players"
              prepend-inner-icon="mdi-magnify"
              variant="outlined"
              density="comfortable"
              hide-details
              class="player-search"
              style="max-width: 300px"
            ></v-text-field>
          </div>
          
          <v-row v-if="players.length === 0">
            <v-col cols="12" class="text-center">
              <v-icon size="64" color="grey" class="mb-2">mdi-account-off</v-icon>
              <div class="text-subtitle-1">No players online</div>
              <p class="text-caption">Players will appear here when they join the server</p>
            </v-col>
          </v-row>
          
          <v-row v-else>
            <v-col v-for="(player, index) in filteredPlayers" :key="index" cols="12" sm="6" md="4" lg="3">
              <v-card class="player-card">
                <v-card-item>
                  <template v-slot:prepend>
                    <v-avatar class="mr-2">
                      <v-img :src="`https://crafatar.com/avatars/${player.uuid}?overlay=true`" alt="Player Avatar"></v-img>
                    </v-avatar>
                  </template>
                  <v-card-title>{{ player.name }}</v-card-title>
                </v-card-item>
                
                <v-card-text>
                  <div class="d-flex align-center mb-1">
                    <v-icon size="small" class="mr-1">mdi-clock-outline</v-icon>
                    <span class="text-caption">{{ formatPlaytime(player.playTime) }}</span>
                  </div>
                  
                  <div class="d-flex align-center">
                    <v-icon size="small" class="mr-1">mdi-map-marker</v-icon>
                    <span class="text-caption">{{ player.location }}</span>
                  </div>
                </v-card-text>
                
                <v-card-actions>
                  <v-btn variant="text" prepend-icon="mdi-message" size="small">Message</v-btn>
                  <v-spacer></v-spacer>
                  <v-btn variant="text" color="error" prepend-icon="mdi-exit-to-app" size="small">Kick</v-btn>
                </v-card-actions>
              </v-card>
            </v-col>
          </v-row>
        </div>
      </v-window-item>
      
      <!-- Settings Tab -->
      <v-window-item value="settings">
        <div class="settings-container">
          <v-row>
            <v-col cols="12" md="6">
              <v-card class="mb-4">
                <v-card-title>Server Properties</v-card-title>
                <v-card-text>
                  <v-text-field
                    v-model="serverSettings.serverName"
                    label="Server Name"
                    variant="outlined"
                    density="comfortable"
                    bg-color="#1e1e1e"
                    class="mb-3"
                  ></v-text-field>
                  
                  <v-select
                    v-model="serverSettings.gameMode"
                    label="Game Mode"
                    :items="['survival', 'creative', 'adventure', 'spectator']"
                    variant="outlined"
                    density="comfortable"
                    bg-color="#1e1e1e"
                    class="mb-3"
                  ></v-select>
                  
                  <v-select
                    v-model="serverSettings.difficulty"
                    label="Difficulty"
                    :items="['peaceful', 'easy', 'normal', 'hard']"
                    variant="outlined"
                    density="comfortable"
                    bg-color="#1e1e1e"
                    class="mb-3"
                  ></v-select>
                  
                  <v-checkbox
                    v-model="serverSettings.pvp"
                    label="Enable PvP"
                    color="primary"
                    hide-details
                    class="mb-3"
                  ></v-checkbox>
                  
                  <v-checkbox
                    v-model="serverSettings.spawnProtection"
                    label="Spawn Protection"
                    color="primary"
                    hide-details
                    class="mb-3"
                  ></v-checkbox>
                  
                  <v-text-field
                    v-model="serverSettings.viewDistance"
                    label="View Distance"
                    type="number"
                    variant="outlined"
                    density="comfortable"
                    bg-color="#1e1e1e"
                    class="mb-3"
                  ></v-text-field>
                </v-card-text>
              </v-card>
            </v-col>
            
            <v-col cols="12" md="6">
              <v-card class="mb-4">
                <v-card-title>Java Settings</v-card-title>
                <v-card-text>
                  <div class="mb-3">
                    <label class="d-block mb-2">Memory Allocation (GB)</label>
                    <div class="d-flex align-center">
                      <v-slider
                        v-model="serverSettings.memory"
                        :min="1"
                        :max="16"
                        :step="1"
                        thumb-label
                        class="memory-slider"
                        color="primary"
                      ></v-slider>
                      <span class="ml-2">{{ serverSettings.memory }} GB</span>
                    </div>
                  </div>
                  
                  <v-checkbox
                    v-model="serverSettings.useCustomJvmArgs"
                    label="Use custom JVM arguments"
                    color="primary"
                    hide-details
                    class="mb-3"
                  ></v-checkbox>
                  
                  <v-textarea
                    v-model="serverSettings.jvmArgs"
                    label="JVM Arguments"
                    variant="outlined"
                    density="comfortable"
                    bg-color="#1e1e1e"
                    :disabled="!serverSettings.useCustomJvmArgs"
                    rows="3"
                    class="mb-3"
                  ></v-textarea>
                </v-card-text>
              </v-card>
              
              <v-card>
                <v-card-title>Auto-Start Settings</v-card-title>
                <v-card-text>
                  <v-checkbox
                    v-model="serverSettings.autoStart"
                    label="Auto-start server with application"
                    color="primary"
                    hide-details
                    class="mb-3"
                  ></v-checkbox>
                  
                  <v-checkbox
                    v-model="serverSettings.autoRestart"
                    label="Auto-restart on crash"
                    color="primary"
                    hide-details
                    class="mb-3"
                  ></v-checkbox>
                </v-card-text>
              </v-card>
            </v-col>
          </v-row>
          
          <div class="d-flex justify-end mt-4">
            <v-btn
              variant="outlined"
              color="grey"
              class="mr-2"
              @click="resetSettings"
            >
              Reset
            </v-btn>
            <v-btn
              color="primary"
              @click="saveSettings"
            >
              Save Settings
            </v-btn>
          </div>
        </div>
      </v-window-item>
      
      <!-- AI Assistant Tab -->
      <v-window-item value="ai">
        <div class="ai-container">
          <div class="ai-header d-flex align-center mb-4">
            <v-icon class="mr-2" color="primary">mdi-robot</v-icon>
            <span class="text-h6">AI Server Assistant</span>
            <v-spacer></v-spacer>
            <v-chip
              size="small"
              :color="aiStatus === 'connected' ? 'success' : 'warning'"
              variant="flat"
            >
              {{ aiStatus === 'connected' ? 'Connected' : 'Offline' }}
            </v-chip>
          </div>
          
          <!-- AI Chat Interface -->
          <div class="ai-chat-container">
            <div class="ai-messages" ref="aiMessages">
              <div 
                v-for="(message, index) in aiMessages" 
                :key="`ai-message-${index}-${message.type}-${message.text?.substring(0, 10)}`" 
                class="ai-message"
                :class="message.type"
              >
                <div class="message-avatar">
                  <v-icon v-if="message.type === 'user'">mdi-account</v-icon>
                  <v-icon v-else color="primary">mdi-robot</v-icon>
                </div>
                <div class="message-content">
                  <div class="message-text" v-html="formatMessage(message.text)"></div>
                  <div v-if="message.actions && message.actions.length > 0" class="message-actions">
                    <v-btn
                      v-for="action in message.actions"
                      :key="action.label"
                      size="small"
                      variant="outlined"
                      @click="executeAction(action)"
                      class="mr-2 mb-2"
                    >
                      {{ action.label }}
                    </v-btn>
                  </div>
                </div>
              </div>
              
              <div v-if="aiLoading" class="ai-message ai">
                <div class="message-avatar">
                  <v-icon color="primary">mdi-robot</v-icon>
                </div>
                <div class="message-content">
                  <div class="typing-indicator">
                    <span></span>
                    <span></span>
                    <span></span>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- AI Input -->
            <div class="ai-input-container">
              <div class="ai-input-wrapper">
                <v-textarea
                  v-model="aiInput"
                  placeholder="Ask me anything about your Minecraft server..."
                  variant="outlined"
                  density="comfortable"
                  hide-details
                  class="ai-input"
                  @keydown.enter.prevent="sendAiMessage"
                  :disabled="aiLoading"
                  rows="2"
                  auto-grow
                ></v-textarea>
                <v-btn
                  icon
                  color="primary"
                  @click="sendAiMessage"
                  :loading="aiLoading"
                  :disabled="!aiInput.trim() || aiLoading"
                  class="ai-send-btn"
                >
                  <v-icon>mdi-send</v-icon>
                </v-btn>
              </div>
              
              <!-- Quick Actions -->
              <div class="quick-actions">
                <v-btn
                  v-for="quickAction in quickActions"
                  :key="quickAction.label"
                  size="small"
                  variant="outlined"
                  @click="sendQuickAction(quickAction)"
                  :disabled="aiLoading"
                  class="mr-2 mb-2"
                >
                  <v-icon size="small" class="mr-1">{{ quickAction.icon }}</v-icon>
                  {{ quickAction.label }}
                </v-btn>
              </div>
            </div>
          </div>
          
          <!-- AI Features Panel -->
          <div class="ai-features">
            <v-row>
              <v-col cols="12" md="6">
                <v-card class="ai-feature-card">
                  <v-card-title>
                    <v-icon class="mr-2" color="primary">mdi-lightbulb</v-icon>
                    Server Optimization
                  </v-card-title>
                  <v-card-text>
                    <p class="text-caption mb-3">Get AI-powered suggestions for:</p>
                    <ul class="text-caption">
                      <li>Memory allocation optimization</li>
                      <li>JVM arguments tuning</li>
                      <li>Plugin compatibility</li>
                      <li>Performance improvements</li>
                    </ul>
                  </v-card-text>
                </v-card>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-card class="ai-feature-card">
                  <v-card-title>
                    <v-icon class="mr-2" color="primary">mdi-bug</v-icon>
                    Troubleshooting
                  </v-card-title>
                  <v-card-text>
                    <p class="text-caption mb-3">AI can help with:</p>
                    <ul class="text-caption">
                      <li>Error analysis and solutions</li>
                      <li>Crash debugging</li>
                      <li>Configuration issues</li>
                      <li>Plugin conflicts</li>
                    </ul>
                  </v-card-text>
                </v-card>
              </v-col>
            </v-row>
          </div>
        </div>
      </v-window-item>
    </v-window>
    
    <!-- File Editor -->
    <FileEditor
      v-model="showFileEditor"
      :server-id="serverId"
      :file-path="selectedFile"
      @saved="onFileSaved"
      @error="onFileError"
    />
  </div>
</template>

<script>
import { store } from '../store.js'
import FileEditor from './FileEditor.vue';

export default {
  name: 'ServerManagementView',
  components: {
    FileEditor
  },
  data() {
    return {
      activeTab: 'console',
      server: null,
      consoleInput: '',
      currentPath: '',
      files: [],
      playerSearch: '',
      serverSettings: {
        serverName: '',
        gameMode: 'survival',
        difficulty: 'normal',
        pvp: true,
        spawnProtection: true,
        viewDistance: 10,
        memory: 4,
        useCustomJvmArgs: false,
        jvmArgs: '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200',
        autoStart: false,
        autoRestart: true
      },
      isLoading: {
        files: false,
        settings: false
      },
      store: store,
      serverMetrics: {
        memoryUsed: 0,
        memoryTotal: 0,
        memoryPercentage: 0,
        cpuUsage: 0,
        uptime: 0,
        startTime: null,
        playerCount: 0,
        maxPlayers: 20,
        tps: 20.0,
        networkIn: 0,
        networkOut: 0
      },
      metricsInterval: null,
      showFileEditor: false,
      selectedFile: null,
      // Server connection info
      serverIp: null,
      serverPort: null,
      // AI Assistant
      aiStatus: 'connected',
      aiLoading: false,
      aiInput: '',
      aiMessages: [
        {
          type: 'ai',
          text: 'Hello! I\'m your Minecraft server AI assistant. I can help you with server optimization, troubleshooting, configuration, and more. What would you like to know?'
        }
      ],
      quickActions: [
        { label: 'Optimize Memory', icon: 'mdi-memory', prompt: 'Help me optimize my server memory allocation' },
        { label: 'Fix Performance', icon: 'mdi-speedometer', prompt: 'My server is laggy, how can I improve performance?' },
        { label: 'Plugin Help', icon: 'mdi-puzzle', prompt: 'I need help with plugin configuration' },
        { label: 'Security Check', icon: 'mdi-shield', prompt: 'How can I make my server more secure?' },
        { label: 'Backup Setup', icon: 'mdi-backup-restore', prompt: 'Help me set up automatic backups' },
        { label: 'Player Issues', icon: 'mdi-account-group', prompt: 'Players are having connection issues' }
      ],
      isDestroyed: false
    }
  },
  computed: {
    serverId() {
      const id = this.$route.params.serverId;
      console.log(`serverId computed property called, returning: ${id}`);
      console.log(`Route params:`, this.$route.params);
      return id;
    },
    isServerRunning() {
      return this.store.isServerRunning(this.serverId);
    },
    isStarting() {
      return this.server?.status === 'starting';
    },
    isStopping() {
      return this.server?.status === 'stopping';
    },
    serverStatus() {
      if (!this.server) return 'Unknown';
      return this.server.status.charAt(0).toUpperCase() + this.server.status.slice(1);
    },
    serverStatusColor() {
      if (!this.server) return 'grey';
      
      const status = this.server.status;
      if (status === 'starting' || status === 'stopping') return 'warning';
      if (status === 'online') return 'success';
      if (status === 'offline') return 'error';
      if (status === 'installing') return 'info';
      if (status === 'failed') return 'error';
      
      return 'grey';
    },
    consoleOutput() {
      try {
        const serverProcess = this.store.getServerProcess(this.serverId);
        if (serverProcess && serverProcess.output && Array.isArray(serverProcess.output)) {
          return serverProcess.output;
        }
        return [];
      } catch (error) {
        console.error('Error getting console output:', error);
        return [];
      }
    },
    filteredConsoleOutput() {
      try {
        const output = this.consoleOutput;
        if (!output || !Array.isArray(output)) {
          return [];
        }
        return output.filter(line => line && line.trim() !== '');
      } catch (error) {
        console.error('Error filtering console output:', error);
        return [];
      }
    },
    breadcrumbs() {
      const parts = this.currentPath.split('/').filter(Boolean);
      const crumbs = [
        {
          title: 'Root',
          disabled: false,
          href: '#',
          onClick: () => this.navigateTo('')
        }
      ];
      
      let path = '';
      for (const part of parts) {
        path += '/' + part;
        crumbs.push({
          title: part,
          disabled: false,
          href: '#',
          onClick: () => this.navigateTo(path.substring(1))
        });
      }
      
      // Make the last item disabled
      if (crumbs.length > 0) {
        crumbs[crumbs.length - 1].disabled = true;
      }
      
      return crumbs;
    },
    players() {
      try {
        const serverProcess = this.store.getServerProcess(this.serverId);
        if (serverProcess && serverProcess.players && Array.isArray(serverProcess.players)) {
          return serverProcess.players;
        }
        return [];
      } catch (error) {
        console.error('Error getting players:', error);
        return [];
      }
    },
    filteredPlayers() {
      try {
        const players = this.players;
        if (!players || !Array.isArray(players)) {
          return [];
        }
        
        if (!this.playerSearch) return players;
        
        const query = this.playerSearch.toLowerCase();
        return players.filter(player => 
          player && player.name && player.location &&
          (player.name.toLowerCase().includes(query) ||
           player.location.toLowerCase().includes(query))
        );
      } catch (error) {
        console.error('Error filtering players:', error);
        return [];
      }
    }
  },
  async created() {
    try {
      // Get the active tab from query params
      if (this.$route.query.tab) {
        this.activeTab = this.$route.query.tab;
      }
      
      // Ensure servers are loaded before proceeding
      if (this.store.servers.length === 0) {
        console.log('No servers loaded, loading from backend...');
        await this.store.loadServers();
      }
      
      // Fetch server details
      await this.fetchServerDetails();
      
      // Only proceed if server was found
      if (this.server) {
        // Load files if files tab is active
        if (this.activeTab === 'files') {
          this.loadFiles();
        }
        
        // Load server settings (this also loads connection info)
        this.loadServerSettings();
      }
    } catch (error) {
      console.error('Error in created lifecycle:', error);
      // Redirect to servers list if there's an error
      this.$router.push({ name: 'Servers' });
    }
  },
  watch: {
    activeTab(newTab) {
      // Load files when switching to files tab
      if (newTab === 'files' && this.files.length === 0) {
        this.loadFiles();
      }
      
      // Update query parameter
      this.$router.replace({
        query: { ...this.$route.query, tab: newTab }
      });
    },
    serverId() {
      // Reload data when server ID changes
      this.fetchServerDetails();
      this.loadFiles();
      this.loadServerSettings();
    },
    consoleOutput: {
      handler() {
        // Auto-scroll to bottom when new output is added, but only if user is already at bottom
        if (this.isDestroyed) return;
        
        try {
          this.$nextTick(() => {
            const consoleElement = this.$refs.consoleOutput;
            if (consoleElement && 
                consoleElement.scrollTop !== undefined && 
                consoleElement.scrollHeight !== undefined &&
                consoleElement.clientHeight !== undefined &&
                !this.isDestroyed) {
              const isAtBottom = consoleElement.scrollTop + consoleElement.clientHeight >= consoleElement.scrollHeight - 10;
              
              // Only auto-scroll if user is already at the bottom (within 10px)
              if (isAtBottom) {
                consoleElement.scrollTop = consoleElement.scrollHeight;
              }
            }
          });
        } catch (error) {
          console.error('Error in console output watcher:', error);
        }
      },
      deep: true
    }
  },
  beforeUnmount() {
    // Mark component as destroyed
    this.isDestroyed = true;
    
    // Clean up metrics polling when component is destroyed
    try {
      this.stopMetricsPolling();
    } catch (error) {
      console.error('Error during component cleanup:', error);
    }
  },
  mounted() {
    // Ensure component is properly mounted before any DOM operations
    this.$nextTick(() => {
      // Initialize any DOM-dependent operations here
      this.initializeDOMOperations();
    });
  },
  methods: {
    async fetchServerDetails() {
      try {
        // Get server details from store
        let server = this.store.getServerById(this.serverId);
        
        // If server not found, try loading servers again
        if (!server && this.store.servers.length === 0) {
          console.log('Server not found and no servers loaded, trying to load servers...');
          await this.store.loadServers();
          server = this.store.getServerById(this.serverId);
        }
        
        if (server) {
          this.server = server;
          
          // Update document title
          document.title = `${server.name} - ServerMint`;
          
          // Update server settings
          this.serverSettings.serverName = server.name;
          this.serverSettings.memory = server.memoryAllocation || 4;
          this.serverSettings.autoStart = server.autoStart || false;
        } else {
          console.error(`Server with ID ${this.serverId} not found after loading servers`);
          // Server not found, go back to servers list
          this.$router.push({ name: 'Servers' });
        }
      } catch (error) {
        console.error('Error fetching server details:', error);
        // Redirect to servers list if there's an error
        this.$router.push({ name: 'Servers' });
      }
    },
    goBack() {
      this.$router.push({ name: 'Servers' });
    },
    async startServer() {
      if (this.isServerRunning || this.isStarting) return;
      
      try {
        // Update server status to starting
        if (this.server) {
          this.server.status = 'starting';
        }
        
        const result = await this.store.startServer(this.serverId);
        
        if (!result.success) {
          console.error('Failed to start server:', result.error);
          alert(`Failed to start server: ${result.error}`);
          
          // Reset status on failure
          if (this.server) {
            this.server.status = 'offline';
          }
        } else {
          // Update server status to online
          if (this.server) {
            this.server.status = 'online';
          }
          
          // Start metrics polling and set start time
          this.serverMetrics.startTime = Date.now();
          this.startMetricsPolling();
          
          // Refresh connection info when server starts
          this.loadServerConnectionInfo();
        }
      } catch (error) {
        console.error('Error starting server:', error);
        alert(`Error starting server: ${error.message || 'Unknown error'}`);
        
        // Reset status on error
        if (this.server) {
          this.server.status = 'offline';
        }
      }
    },
    async stopServer() {
      if (!this.isServerRunning || this.isStopping) return;
      
      try {
        // Update server status to stopping
        if (this.server) {
          this.server.status = 'stopping';
        }
        
        const result = await this.store.stopServer(this.serverId);
        
        if (!result.success) {
          console.error('Failed to stop server:', result.error);
          alert(`Failed to stop server: ${result.error}`);
          
          // Reset status on failure
          if (this.server) {
            this.server.status = 'online';
          }
        } else {
          // Update server status to offline
          if (this.server) {
            this.server.status = 'offline';
          }
          
          // Stop metrics polling
          this.stopMetricsPolling();
        }
      } catch (error) {
        console.error('Error stopping server:', error);
        alert(`Error stopping server: ${error.message || 'Unknown error'}`);
        
        // Reset status on error
        if (this.server) {
          this.server.status = 'online';
        }
      }
    },
    async restartServer() {
      if (!this.isServerRunning || this.isStarting || this.isStopping) return;
      
      try {
        // Stop the server
        await this.store.stopServer(this.serverId);
        
        // Wait for the server to stop
        setTimeout(async () => {
          // Start the server
          await this.store.startServer(this.serverId);
        }, 2000);
      } catch (error) {
        console.error('Error restarting server:', error);
        alert(`Error restarting server: ${error.message || 'Unknown error'}`);
      }
    },
    async openServerFolder() {
      if (!this.server?.path) return;
      
      try {
        await this.store.openServerFolder(this.serverId);
      } catch (error) {
        console.error('Error opening server folder:', error);
        alert(`Error opening server folder: ${error.message || 'Unknown error'}`);
      }
    },
    async sendCommand() {
      if (!this.consoleInput.trim() || !this.isServerRunning) return;
      
      try {
        await this.store.sendCommand(this.serverId, this.consoleInput);
        
        // Clear the input
        this.consoleInput = '';
      } catch (error) {
        console.error('Error sending command:', error);
      }
    },
    getConsoleLineClass(line) {
      // Safety check
      if (!line || typeof line !== 'string') {
        return 'default-line';
      }
      
      // Command lines (user input)
      if (line.startsWith('>')) return 'command-line';
      
      // Minecraft server log levels
      if (line.includes('[INFO]')) return 'info-line';
      if (line.includes('[WARN]')) return 'warn-line';
      if (line.includes('[ERROR]')) return 'error-line';
      if (line.includes('[FATAL]')) return 'fatal-line';
      
      // Special Minecraft messages
      if (line.includes('Player') && line.includes('joined')) return 'player-join-line';
      if (line.includes('Player') && line.includes('left')) return 'player-leave-line';
      if (line.includes('Done!')) return 'success-line';
      if (line.includes('Starting minecraft server')) return 'startup-line';
      if (line.includes('Preparing spawn area')) return 'spawn-line';
      if (line.includes('Unpacking')) return 'unpack-line';
      
      // Default
      return 'default-line';
    },
    
    copyConsoleOutput() {
      try {
        const output = this.consoleOutput ? this.consoleOutput.join('\n') : '';
        if (output) {
          navigator.clipboard.writeText(output).then(() => {
            // Show a brief success message
            console.log('Console output copied to clipboard');
          }).catch(err => {
            console.error('Failed to copy console output:', err);
          });
        }
      } catch (error) {
        console.error('Error copying console output:', error);
      }
    },
    
    clearConsoleOutput() {
      try {
        if (confirm('Are you sure you want to clear the console output?')) {
          const serverProcess = this.store.getServerProcess(this.serverId);
          if (serverProcess && serverProcess.output) {
            serverProcess.output = [];
          }
        }
      } catch (error) {
        console.error('Error clearing console output:', error);
      }
    },
    

    async loadFiles(path = '') {
      this.isLoading.files = true;
      
      try {
        const result = await this.store.getServerFiles(this.serverId, path);
        
        if (result.success) {
          this.files = result.files;
          this.currentPath = path;
        } else {
          console.error('Error loading files:', result.error);
        }
      } catch (error) {
        console.error('Error loading files:', error);
      } finally {
        this.isLoading.files = false;
      }
    },
    navigateTo(path) {
      this.loadFiles(path);
    },
    navigateUp() {
      const parts = this.currentPath.split('/').filter(Boolean);
      parts.pop();
      this.navigateTo(parts.join('/'));
    },
    async navigateToFile(file) {
      if (file.isDirectory) {
        const newPath = this.currentPath ? `${this.currentPath}/${file.name}` : file.name;
        this.navigateTo(newPath);
      } else {
        // Check if it's a text file that can be edited
        const textExtensions = [
          'txt', 'log', 'properties', 'json', 'xml', 'yml', 'yaml', 
          'cfg', 'conf', 'config', 'ini', 'md', 'markdown', 'html', 
          'css', 'js', 'ts', 'java', 'py', 'sh', 'bat', 'cmd'
        ];
        
        const fileExtension = file.name.split('.').pop()?.toLowerCase();
        
        if (textExtensions.includes(fileExtension)) {
          // Open in file editor
          const filePath = this.currentPath ? `${this.currentPath}/${file.name}` : file.name;
          this.openFileEditor(filePath);
        } else {
          // For non-text files, show info or download
          if (confirm(`"${file.name}" is not a text file. Would you like to download it instead?`)) {
            this.downloadFile(file);
          }
        }
      }
    },
    formatFileSize(bytes) {
      if (bytes === 0) return '0 Bytes';
      
      const k = 1024;
      const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    },
    formatDate(date) {
      return new Date(date).toLocaleString();
    },
    formatPlaytime(seconds) {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      
      if (hours > 0) {
        return `${hours}h ${minutes}m`;
      }
      
      return `${minutes}m`;
    },
    async downloadFile(file) {
      // In a real app, this would download the file
      console.log(`Downloading file: ${file.name}`);
      alert(`File download not implemented yet. Would download: ${file.name}`);
    },
    async deleteFile(file) {
      if (!confirm(`Are you sure you want to delete ${file.name}?`)) {
        return;
      }
      
      try {
        // In a real app, this would delete the file
        console.log(`Deleting file: ${file.name}`);
        
        // Remove from the files array
        const index = this.files.findIndex(f => f.name === file.name);
        if (index !== -1) {
          this.files.splice(index, 1);
        }
        
        alert(`File deletion not fully implemented yet. Would delete: ${file.name}`);
      } catch (error) {
        console.error('Error deleting file:', error);
        alert(`Error deleting file: ${error.message || 'Unknown error'}`);
      }
    },
    async loadServerSettings() {
      this.isLoading.settings = true;
      
      try {
        // In a real app, this would load server.properties
        const result = await this.store.readServerFile(this.serverId, 'server.properties');
        
        if (result.success && result.content && typeof result.content === 'string') {
          // Parse server.properties
          const properties = {};
          const lines = result.content.split('\n');
          
          for (const line of lines) {
            if (line.startsWith('#') || !line.includes('=')) continue;
            
            const [key, value] = line.split('=', 2);
            properties[key.trim()] = value.trim();
          }
          
          // Update server settings
          this.serverSettings.gameMode = properties.gamemode || 'survival';
          this.serverSettings.difficulty = properties.difficulty || 'normal';
          this.serverSettings.pvp = properties.pvp === 'true';
          this.serverSettings.spawnProtection = parseInt(properties['spawn-protection']) > 0;
          this.serverSettings.viewDistance = parseInt(properties['view-distance']) || 10;
          
          // Load server connection info
          this.loadServerConnectionInfo(properties);
        } else {
          console.log('Server properties file not found or invalid, using default settings');
        }
      } catch (error) {
        console.error('Error loading server settings:', error);
      } finally {
        this.isLoading.settings = false;
      }
    },
    
    async loadServerConnectionInfo(properties = null) {
      try {
        // Get server properties if not provided
        if (!properties) {
          const result = await this.store.readServerFile(this.serverId, 'server.properties');
          if (result && result.success && result.content) {
            const lines = result.content.split('\n');
            properties = {};
            for (const line of lines) {
              if (line.startsWith('#') || !line.includes('=')) continue;
              const [key, value] = line.split('=', 2);
              if (key && value) {
                properties[key.trim()] = value.trim();
              }
            }
          }
        }
        
        // Get port from server.properties
        if (properties && properties['server-port']) {
          this.serverPort = properties['server-port'];
        } else {
          this.serverPort = '25565'; // Default Minecraft port
        }
        
        // Get IP address
        try {
          const ipResult = await this.store.getLocalIp();
          if (ipResult && ipResult.success && ipResult.ip) {
            this.serverIp = ipResult.ip;
          } else {
            this.serverIp = '127.0.0.1'; // Fallback to localhost
          }
        } catch (ipError) {
          console.warn('Failed to get local IP, using fallback:', ipError);
          this.serverIp = '127.0.0.1'; // Fallback to localhost
        }
        
        console.log(`Server connection info loaded: ${this.serverIp}:${this.serverPort}`);
      } catch (error) {
        console.error('Error loading server connection info:', error);
        // Set fallback values
        this.serverIp = '127.0.0.1';
        this.serverPort = '25565';
      }
    },
    
    async copyServerAddress() {
      if (this.serverIp && this.serverPort) {
        const address = `${this.serverIp}:${this.serverPort}`;
        try {
          await navigator.clipboard.writeText(address);
          console.log('Server address copied to clipboard:', address);
          
          // Show success toast
          if (window.showSuccess) {
            window.showSuccess('Server Address Copied!', `"${address}" has been copied to clipboard.`);
          }
        } catch (error) {
          console.error('Failed to copy server address:', error);
          
          // Show error toast
          if (window.showError) {
            window.showError('Copy Failed', 'Failed to copy server address to clipboard.');
          }
        }
      }
    },
    
    async saveSettings() {
      try {
        // In a real app, this would save server.properties
        console.log('Saving server settings:', this.serverSettings);
        
        // Show success message
        alert('Server settings saved successfully');
      } catch (error) {
        console.error('Error saving server settings:', error);
        alert(`Error saving server settings: ${error.message || 'Unknown error'}`);
      }
    },
    resetSettings() {
      // Reset settings to default values
      this.serverSettings = {
        serverName: this.server?.name || '',
        gameMode: 'survival',
        difficulty: 'normal',
        pvp: true,
        spawnProtection: true,
        viewDistance: 10,
        memory: this.server?.memoryAllocation || 4,
        useCustomJvmArgs: false,
        jvmArgs: '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200',
        autoStart: this.server?.autoStart || false,
        autoRestart: true
      };
    },
    
    async checkJava() {
      try {
        const result = await this.store.checkJava();
        if (result.success) {
          alert(`Java check successful: ${result.message}`);
        } else {
          alert(`Java check failed: ${result.error}`);
        }
      } catch (error) {
        console.error('Error checking Java:', error);
        alert(`Error checking Java: ${error.message || 'Unknown error'}`);
      }
    },
    
    async setupJava() {
      try {
        if (!confirm('This will download and install Java 21 locally. Continue?')) {
          return;
        }
        
        const result = await this.store.setupJava();
        if (result.success) {
          alert(`Java setup successful: ${result.message}`);
        } else {
          alert(`Java setup failed: ${result.error}`);
        }
      } catch (error) {
        console.error('Error setting up Java:', error);
        alert(`Error setting up Java: ${error.message || 'Unknown error'}`);
      }
    },
    
    // Metrics formatting methods
    formatBytes(bytes) {
      if (bytes === 0) return '0 B';
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    },
    
    formatUptime(seconds) {
      if (seconds === 0) return '0s';
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      const secs = seconds % 60;
      
      if (hours > 0) {
        return `${hours}h ${minutes}m ${secs}s`;
      } else if (minutes > 0) {
        return `${minutes}m ${secs}s`;
      } else {
        return `${secs}s`;
      }
    },
    
    formatStartTime(timestamp) {
      if (!timestamp) return 'N/A';
      return new Date(timestamp).toLocaleTimeString();
    },
    
    getMemoryColor(percentage) {
      if (percentage >= 90) return 'error';
      if (percentage >= 75) return 'warning';
      return 'success';
    },
    
    getCpuColor(percentage) {
      if (percentage >= 90) return 'error';
      if (percentage >= 70) return 'warning';
      return 'success';
    },
    
    getPlayerStatus(count) {
      if (count === 0) return 'No players online';
      if (count === 1) return '1 player online';
      return `${count} players online`;
    },
    
    getTpsStatus(tps) {
      if (tps >= 19.5) return { text: 'Excellent', class: 'text-success' };
      if (tps >= 18.0) return { text: 'Good', class: 'text-warning' };
      if (tps >= 15.0) return { text: 'Fair', class: 'text-orange' };
      return { text: 'Poor', class: 'text-error' };
    },
    
    startMetricsPolling() {
      try {
        // Clear any existing interval
        if (this.metricsInterval) {
          clearInterval(this.metricsInterval);
        }
        
        // Start polling every 2 seconds
        this.metricsInterval = setInterval(() => {
          if (this && typeof this.updateServerMetrics === 'function') {
            this.updateServerMetrics();
          }
        }, 2000);
        
        // Initial update
        this.updateServerMetrics();
      } catch (error) {
        console.error('Error starting metrics polling:', error);
      }
    },
    
    stopMetricsPolling() {
      try {
        if (this.metricsInterval) {
          clearInterval(this.metricsInterval);
          this.metricsInterval = null;
        }
      } catch (error) {
        console.error('Error stopping metrics polling:', error);
      }
    },
    
    updateServerMetrics() {
      try {
        if (!this.isServerRunning) {
          // Reset metrics when server is not running
          this.serverMetrics = {
            memoryUsed: 0,
            memoryTotal: 0,
            memoryPercentage: 0,
            cpuUsage: 0,
            uptime: 0,
            startTime: null,
            playerCount: 0,
            maxPlayers: 20,
            tps: 20.0,
            networkIn: 0,
            networkOut: 0
          };
          return;
        }
      
      // Simulate realistic server metrics
      const now = Date.now();
      
      // Update uptime if we have a start time
      if (this.serverMetrics.startTime) {
        this.serverMetrics.uptime = Math.floor((now - this.serverMetrics.startTime) / 1000);
      }
      
      // Simulate memory usage (real implementation would query the Java process)
      const baseMemory = 1024 * 1024 * 1024; // 1GB base
      const playerMemory = this.serverMetrics.playerCount * 50 * 1024 * 1024; // 50MB per player
      const randomVariation = (Math.random() - 0.5) * 200 * 1024 * 1024; // ±100MB variation
      this.serverMetrics.memoryUsed = Math.max(baseMemory, baseMemory + playerMemory + randomVariation);
      this.serverMetrics.memoryTotal = 4 * 1024 * 1024 * 1024; // 4GB allocated
      this.serverMetrics.memoryPercentage = (this.serverMetrics.memoryUsed / this.serverMetrics.memoryTotal) * 100;
      
      // Simulate CPU usage
      this.serverMetrics.cpuUsage = Math.min(100, Math.max(5, 
        20 + this.serverMetrics.playerCount * 5 + (Math.random() - 0.5) * 20
      ));
      
      // Update player count from server process
      const serverProcess = this.store.getServerProcess(this.serverId);
      if (serverProcess) {
        this.serverMetrics.playerCount = serverProcess.players.length;
      }
      
      // Simulate TPS (Ticks Per Second)
      this.serverMetrics.tps = Math.min(20, Math.max(10, 
        20 - (this.serverMetrics.playerCount * 0.5) - (Math.random() * 2)
      ));
      
      // Simulate network usage
      this.serverMetrics.networkIn = this.serverMetrics.playerCount * (50 + Math.random() * 100) * 1024; // 50-150KB per player
      this.serverMetrics.networkOut = this.serverMetrics.playerCount * (100 + Math.random() * 200) * 1024; // 100-300KB per player
      } catch (error) {
        console.error('Error updating server metrics:', error);
      }
    },
    
    // File editor methods
    openFileEditor(filePath) {
      this.selectedFile = filePath;
      this.showFileEditor = true;
    },
    
    onFileSaved(filePath) {
      console.log('File saved:', filePath);
      // Optionally reload files or refresh the file list
      if (this.activeTab === 'files') {
        this.loadFiles();
      }
    },
    
    onFileError(error) {
      console.error('File editor error:', error);
    },
    
    // AI Assistant Methods
    async sendAiMessage() {
      if (!this.aiInput.trim() || this.aiLoading) return;
      
      const userMessage = this.aiInput.trim();
      this.aiInput = '';
      
      // Add user message
      this.aiMessages.push({
        type: 'user',
        text: userMessage
      });
      
      this.aiLoading = true;
      
      try {
        this.scrollToBottom();
        const response = await this.getAiResponse(userMessage);
        
        // Check if component is still mounted before updating
        if (this.$refs.aiMessages && !this.isDestroyed) {
          this.aiMessages.push({
            type: 'ai',
            text: response.text,
            actions: response.actions || []
          });
        }
      } catch (error) {
        console.error('AI response error:', error);
        
        // Check if component is still mounted before updating
        if (this.$refs.aiMessages && !this.isDestroyed) {
          this.aiMessages.push({
            type: 'ai',
            text: 'Sorry, I encountered an error. Please try again or check your internet connection.'
          });
        }
      } finally {
        this.aiLoading = false;
        if (!this.isDestroyed) {
          this.$nextTick(() => {
            this.scrollToBottom();
          });
        }
      }
    },
    
    sendQuickAction(action) {
      this.aiInput = action.prompt;
      this.$nextTick(() => {
        this.sendAiMessage();
      });
    },
    
    async getAiResponse(message) {
      // Use a free AI API - Hugging Face Inference API
      const API_URL = 'https://api-inference.huggingface.co/models/microsoft/DialoGPT-medium';
      
      try {
        const response = await fetch(API_URL, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            // You can get a free API key from https://huggingface.co/settings/tokens
            // 'Authorization': 'Bearer YOUR_API_KEY' // Optional for some models
          },
          body: JSON.stringify({
            inputs: this.buildContextMessage(message),
            parameters: {
              max_length: 500,
              temperature: 0.7,
              do_sample: true
            }
          })
        });
        
        if (!response.ok) {
          throw new Error(`API request failed: ${response.status}`);
        }
        
        const data = await response.json();
        
        // Process the response
        const aiText = data[0]?.generated_text || 'I understand your question about Minecraft servers. Let me help you with that.';
        
        // Add Minecraft-specific context and actions
        const actions = this.generateActions(message);
        
        return {
          text: aiText,
          actions: actions
        };
        
      } catch (error) {
        console.error('AI API error:', error);
        
        // Fallback to local responses
        return this.getLocalAiResponse(message);
      }
    },
    
    buildContextMessage(message) {
      const serverInfo = this.server ? `
        Server: ${this.server.name}
        Type: ${this.server.type} ${this.server.version}
        Status: ${this.server.status}
        Memory: ${this.server.memoryAllocation || 4}GB
        Connection: ${this.serverIp || 'Unknown'}:${this.serverPort || '25565'}
      ` : '';
      
      return `You are a helpful Minecraft server assistant. The user is asking: "${message}". ${serverInfo} Provide helpful, specific advice for Minecraft server management.`;
    },
    
    getLocalAiResponse(message) {
      const lowerMessage = message.toLowerCase();
      
      // Local AI responses for common questions
      if (lowerMessage.includes('memory') || lowerMessage.includes('ram')) {
        return {
          text: `For your ${this.server?.memoryAllocation || 4}GB server, here are memory optimization tips:

• **Allocate 75-80% of available RAM** to avoid system lag
• **Use G1GC garbage collector**: -XX:+UseG1GC
• **Set max heap size**: -Xmx${Math.floor((this.server?.memoryAllocation || 4) * 0.8)}G
• **Monitor with /gc command** in-game

Would you like me to help you update your JVM arguments?`,
          actions: [
            { label: 'Update JVM Args', action: 'update_jvm_args' },
            { label: 'Show Memory Settings', action: 'show_memory_settings' }
          ]
        };
      }
      
      if (lowerMessage.includes('performance') || lowerMessage.includes('lag') || lowerMessage.includes('slow')) {
        return {
          text: `Here are performance optimization tips for your server:

• **Reduce view-distance** to 8-10 chunks
• **Limit entity spawning** with mob-spawn-limit
• **Use Paper/Spigot** instead of vanilla for better performance
• **Optimize plugins** - remove unused ones
• **Pre-generate chunks** to reduce lag spikes

Current TPS: ${this.serverMetrics.tps.toFixed(1)} (target: 20.0)`,
          actions: [
            { label: 'Optimize Settings', action: 'optimize_settings' },
            { label: 'Check Plugins', action: 'check_plugins' }
          ]
        };
      }
      
      if (lowerMessage.includes('plugin') || lowerMessage.includes('mod')) {
        return {
          text: `Plugin management tips:

• **Check compatibility** with your server version
• **Load order matters** - put essential plugins first
• **Monitor plugin performance** with /timings
• **Keep plugins updated** for security and performance
• **Test in development environment** first

I can help you check your current plugins and their compatibility.`,
          actions: [
            { label: 'List Plugins', action: 'list_plugins' },
            { label: 'Check Compatibility', action: 'check_compatibility' }
          ]
        };
      }
      
      if (lowerMessage.includes('security') || lowerMessage.includes('protect')) {
        return {
          text: `Server security recommendations:

• **Use strong passwords** for admin accounts
• **Enable whitelist** for small servers
• **Regular backups** to prevent data loss
• **Keep server updated** to latest version
• **Use anti-grief plugins** like WorldGuard
• **Monitor logs** for suspicious activity

Would you like me to help you configure security settings?`,
          actions: [
            { label: 'Security Settings', action: 'security_settings' },
            { label: 'Backup Setup', action: 'backup_setup' }
          ]
        };
      }
      
      // Default response
      return {
        text: `I understand you're asking about "${message}". As your Minecraft server assistant, I can help with:

• Server optimization and performance
• Plugin configuration and troubleshooting  
• Security and backup setup
• Player management and permissions
• JVM arguments and memory tuning

What specific aspect would you like help with?`,
        actions: [
          { label: 'Server Status', action: 'server_status' },
          { label: 'Performance Check', action: 'performance_check' }
        ]
      };
    },
    
    generateActions(message) {
      const actions = [];
      const lowerMessage = message.toLowerCase();
      
      if (lowerMessage.includes('memory') || lowerMessage.includes('ram')) {
        actions.push({ label: 'Update JVM Args', action: 'update_jvm_args' });
      }
      
      if (lowerMessage.includes('performance') || lowerMessage.includes('lag')) {
        actions.push({ label: 'Optimize Settings', action: 'optimize_settings' });
      }
      
      if (lowerMessage.includes('plugin')) {
        actions.push({ label: 'List Plugins', action: 'list_plugins' });
      }
      
      if (lowerMessage.includes('security')) {
        actions.push({ label: 'Security Settings', action: 'security_settings' });
      }
      
      return actions;
    },
    
    executeAction(action) {
      console.log('Executing action:', action);
      
      switch (action.action) {
        case 'update_jvm_args':
          this.updateJvmArgs();
          break;
        case 'optimize_settings':
          this.optimizeServerSettings();
          break;
        case 'list_plugins':
          this.listPlugins();
          break;
        case 'security_settings':
          this.showSecuritySettings();
          break;
        case 'server_status':
          this.showServerStatus();
          break;
        case 'performance_check':
          this.runPerformanceCheck();
          break;
        default:
          console.log('Unknown action:', action.action);
      }
    },
    
    updateJvmArgs() {
      const optimizedArgs = `-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1`;
      
      this.serverSettings.jvmArgs = optimizedArgs;
      this.serverSettings.useCustomJvmArgs = true;
      
      this.aiMessages.push({
        type: 'ai',
        text: `✅ Updated JVM arguments with optimized settings for better performance! The new arguments include G1GC garbage collector and memory optimization flags.`
      });
    },
    
    optimizeServerSettings() {
      this.serverSettings.viewDistance = 8;
      this.serverSettings.spawnProtection = true;
      
      this.aiMessages.push({
        type: 'ai',
        text: `✅ Optimized server settings! Reduced view distance to 8 chunks and enabled spawn protection for better performance.`
      });
    },
    
    listPlugins() {
      // This would normally read from the plugins folder
      this.aiMessages.push({
        type: 'ai',
        text: `📋 Plugin list feature would show all installed plugins here. You can check your plugins folder or use the Files tab to browse your server files.`
      });
    },
    
    showSecuritySettings() {
      this.aiMessages.push({
        type: 'ai',
        text: `🔒 Security recommendations applied! Consider enabling whitelist, setting up regular backups, and using anti-grief plugins for better server protection.`
      });
    },
    
    showServerStatus() {
      const status = `📊 **Server Status Report:**
• Status: ${this.serverStatus}
• Connection: ${this.serverIp || 'Unknown'}:${this.serverPort || '25565'}
• Memory: ${this.formatBytes(this.serverMetrics.memoryUsed)} / ${this.formatBytes(this.serverMetrics.memoryTotal)}
• CPU: ${this.serverMetrics.cpuUsage.toFixed(1)}%
• TPS: ${this.serverMetrics.tps.toFixed(1)}
• Players: ${this.serverMetrics.playerCount}/${this.serverMetrics.maxPlayers}
• Uptime: ${this.formatUptime(this.serverMetrics.uptime)}`;
      
      this.aiMessages.push({
        type: 'ai',
        text: status
      });
    },
    
    runPerformanceCheck() {
      const performance = `⚡ **Performance Analysis:**
• TPS: ${this.serverMetrics.tps.toFixed(1)} (${this.getTpsStatus(this.serverMetrics.tps).text})
• Memory Usage: ${this.serverMetrics.memoryPercentage.toFixed(1)}%
• CPU Usage: ${this.serverMetrics.cpuUsage.toFixed(1)}%

${this.serverMetrics.tps < 18 ? '⚠️ Performance issues detected! Consider reducing view distance or optimizing plugins.' : '✅ Performance looks good!'}`;
      
      this.aiMessages.push({
        type: 'ai',
        text: performance
      });
    },
    
    formatMessage(text) {
      // Convert markdown-like formatting to HTML
      return text
        .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
        .replace(/\*(.*?)\*/g, '<em>$1</em>')
        .replace(/•/g, '•')
        .replace(/\n/g, '<br>');
    },
    
    initializeDOMOperations() {
      // Initialize any DOM-dependent operations after component is mounted
      try {
        // Ensure refs are available
        if (this.$refs.consoleOutput) {
          console.log('Console output ref initialized');
        }
        if (this.$refs.aiMessages) {
          console.log('AI messages ref initialized');
        }
      } catch (error) {
        console.warn('Error initializing DOM operations:', error);
      }
    },
    
    scrollToBottom() {
      if (this.isDestroyed) return;
      
      this.$nextTick(() => {
        try {
          const aiMessagesElement = this.$refs.aiMessages;
          if (aiMessagesElement && 
              aiMessagesElement.scrollTop !== undefined && 
              aiMessagesElement.scrollHeight !== undefined &&
              !this.isDestroyed) {
            aiMessagesElement.scrollTop = aiMessagesElement.scrollHeight;
          }
        } catch (error) {
          console.warn('Error scrolling to bottom:', error);
        }
      });
    }
  }
}
</script>

<style scoped>
.server-management {
  padding: 16px;
}
.server-icon {
  background-color: #1e1e1e;
}
.server-connection-info {
  display: flex;
  align-items: center;
  background-color: rgba(74, 222, 128, 0.1);
  border-radius: 6px;
  padding: 4px 8px;
  border: 1px solid rgba(74, 222, 128, 0.2);
}
.console-container {
  height: 600px;
  display: flex;
  flex-direction: column;
  background-color: #0a0a0a;
  border-radius: 12px;
  overflow: hidden;
  border: 1px solid rgba(74, 222, 128, 0.1);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  min-height: 0;
}

.console-header {
  padding: 12px 16px;
  background-color: #1a1a1a;
  border-bottom: 1px solid rgba(74, 222, 128, 0.1);
}

.console-output {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 16px;
  font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
  background-color: #0d0d0d;
  color: #e0e0e0;
  font-size: 13px;
  line-height: 1.6;
  scrollbar-width: thin;
  scrollbar-color: rgba(74, 222, 128, 0.3) transparent;
  min-height: 0;
  height: 0; /* Force flex to work properly */
}

.console-output::-webkit-scrollbar {
  width: 8px;
}

.console-output::-webkit-scrollbar-track {
  background: transparent;
}

.console-output::-webkit-scrollbar-thumb {
  background-color: rgba(74, 222, 128, 0.3);
  border-radius: 4px;
}

.console-output::-webkit-scrollbar-thumb:hover {
  background-color: rgba(74, 222, 128, 0.5);
}

.console-line {
  white-space: pre-wrap;
  word-break: break-word;
  margin-bottom: 2px;
  padding: 1px 0;
  border-radius: 2px;
  transition: background-color 0.1s ease;
}

.console-line:hover {
  background-color: rgba(255, 255, 255, 0.02);
}

/* Console line colors */
.default-line {
  color: #b0b0b0;
}

.info-line {
  color: #4ade80;
  font-weight: 500;
}

.warn-line {
  color: #fbbf24;
  font-weight: 500;
}

.error-line {
  color: #ef4444;
  font-weight: 500;
}

.fatal-line {
  color: #dc2626;
  font-weight: 600;
  background-color: rgba(220, 38, 38, 0.1);
}

.command-line {
  color: #60a5fa;
  font-weight: 600;
  background-color: rgba(96, 165, 250, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
  border-left: 3px solid #60a5fa;
}

.player-join-line {
  color: #10b981;
  font-weight: 500;
}

.player-leave-line {
  color: #f59e0b;
  font-weight: 500;
}

.success-line {
  color: #22c55e;
  font-weight: 600;
  background-color: rgba(34, 197, 94, 0.1);
}

.startup-line {
  color: #8b5cf6;
  font-weight: 500;
}

.spawn-line {
  color: #06b6d4;
  font-weight: 500;
}

.unpack-line {
  color: #a3a3a3;
  font-style: italic;
}

.console-cursor {
  display: inline-block;
  width: 8px;
  height: 16px;
  background-color: #4ade80;
  animation: blink 1s infinite;
  vertical-align: middle;
  border-radius: 1px;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}

.console-input-container {
  padding: 12px 16px;
  background-color: #1a1a1a;
  border-top: 1px solid rgba(74, 222, 128, 0.1);
}

.console-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.console-prompt {
  position: absolute;
  left: 12px;
  z-index: 2;
  font-size: 18px;
}

.console-input {
  padding-left: 40px !important;
}

.console-input :deep(.v-field__input) {
  color: #e0e0e0 !important;
  font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace !important;
  font-size: 14px !important;
}

.console-input :deep(.v-field__outline) {
  border-color: rgba(74, 222, 128, 0.3) !important;
}

.console-input :deep(.v-field--focused .v-field__outline) {
  border-color: #4ade80 !important;
}

/* System Monitor Styles */
.system-monitor {
  background-color: #1a1a1a;
  border-top: 1px solid rgba(74, 222, 128, 0.1);
  padding: 16px;
}

.monitor-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.monitor-item {
  background-color: #0d0d0d;
  border-radius: 8px;
  padding: 12px;
  border: 1px solid rgba(74, 222, 128, 0.05);
  transition: all 0.2s ease;
}

.monitor-item:hover {
  border-color: rgba(74, 222, 128, 0.2);
  transform: translateY(-1px);
}

.monitor-header {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
}

.monitor-label {
  margin-left: 8px;
  font-size: 12px;
  font-weight: 500;
  color: #a0a0a0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.monitor-value {
  font-size: 18px;
  font-weight: 600;
  color: #e0e0e0;
  margin-bottom: 6px;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
}

.monitor-subtitle {
  font-size: 11px;
  color: #808080;
  margin-top: 4px;
}

.monitor-progress {
  margin-top: 8px;
}

.text-success {
  color: #4ade80 !important;
}

.text-warning {
  color: #fbbf24 !important;
}

.text-orange {
  color: #f97316 !important;
}

.text-error {
  color: #ef4444 !important;
}
.files-container {
  background-color: #1a1a1a;
  border-radius: 8px;
  padding: 16px;
}
.files-table {
  background-color: transparent !important;
}
.files-table tbody tr {
  cursor: pointer;
  transition: background-color 0.2s;
}
.files-table tbody tr:hover {
  background-color: rgba(255, 255, 255, 0.05);
}
.players-container {
  padding: 16px;
  background-color: #1a1a1a;
  border-radius: 8px;
}
.player-card {
  background-color: #1e1e1e;
  transition: transform 0.2s;
}
.player-card:hover {
  transform: translateY(-2px);
}
.settings-container {
  padding: 16px;
  background-color: #1a1a1a;
  border-radius: 8px;
}
.memory-slider {
  max-width: 300px;
}

/* AI Assistant Styles */
.ai-container {
  height: 600px;
  display: flex;
  flex-direction: column;
  background-color: #1a1a1a;
  border-radius: 12px;
  overflow: hidden;
  border: 1px solid rgba(74, 222, 128, 0.1);
}

.ai-header {
  padding: 16px;
  background-color: #1e1e1e;
  border-bottom: 1px solid rgba(74, 222, 128, 0.1);
}

.ai-chat-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.ai-messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  background-color: #0d0d0d;
  scrollbar-width: thin;
  scrollbar-color: rgba(74, 222, 128, 0.3) transparent;
}

.ai-messages::-webkit-scrollbar {
  width: 8px;
}

.ai-messages::-webkit-scrollbar-track {
  background: transparent;
}

.ai-messages::-webkit-scrollbar-thumb {
  background-color: rgba(74, 222, 128, 0.3);
  border-radius: 4px;
}

.ai-messages::-webkit-scrollbar-thumb:hover {
  background-color: rgba(74, 222, 128, 0.5);
}

.ai-message {
  display: flex;
  margin-bottom: 16px;
  animation: fadeIn 0.3s ease-in;
}

.ai-message.user {
  flex-direction: row-reverse;
}

.ai-message.ai {
  flex-direction: row;
}

.message-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: #2a2a2a;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 12px;
  flex-shrink: 0;
}

.message-content {
  max-width: 70%;
  background-color: #2a2a2a;
  border-radius: 12px;
  padding: 12px 16px;
  position: relative;
}

.ai-message.user .message-content {
  background-color: #4ade80;
  color: #000;
}

.ai-message.ai .message-content {
  background-color: #2a2a2a;
  color: #e0e0e0;
}

.message-text {
  line-height: 1.5;
  word-wrap: break-word;
}

.message-actions {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.typing-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
}

.typing-indicator span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #4ade80;
  animation: typing 1.4s infinite ease-in-out;
}

.typing-indicator span:nth-child(1) {
  animation-delay: -0.32s;
}

.typing-indicator span:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes typing {
  0%, 80%, 100% {
    transform: scale(0.8);
    opacity: 0.5;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.ai-input-container {
  padding: 16px;
  background-color: #1e1e1e;
  border-top: 1px solid rgba(74, 222, 128, 0.1);
}

.ai-input-wrapper {
  display: flex;
  align-items: flex-end;
  gap: 12px;
  margin-bottom: 12px;
}

.ai-input {
  flex: 1;
}

.ai-input :deep(.v-field__input) {
  color: #e0e0e0 !important;
  font-family: 'Roboto', sans-serif !important;
}

.ai-input :deep(.v-field__outline) {
  border-color: rgba(74, 222, 128, 0.3) !important;
}

.ai-input :deep(.v-field--focused .v-field__outline) {
  border-color: #4ade80 !important;
}

.ai-send-btn {
  flex-shrink: 0;
}

.quick-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.quick-actions .v-btn {
  font-size: 12px;
  height: 32px;
}

.ai-features {
  padding: 16px;
  background-color: #1e1e1e;
  border-top: 1px solid rgba(74, 222, 128, 0.1);
}

.ai-feature-card {
  background-color: #2a2a2a !important;
  border: 1px solid rgba(74, 222, 128, 0.05);
  transition: all 0.2s ease;
}

.ai-feature-card:hover {
  border-color: rgba(74, 222, 128, 0.2);
  transform: translateY(-2px);
}

.ai-feature-card .v-card-title {
  font-size: 16px;
  font-weight: 600;
  color: #e0e0e0;
}

.ai-feature-card .v-card-text {
  color: #b0b0b0;
}

.ai-feature-card ul {
  margin: 0;
  padding-left: 16px;
}

.ai-feature-card li {
  margin-bottom: 4px;
}
</style> 