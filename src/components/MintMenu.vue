<template>
  <div v-if="show" class="mint-menu-overlay" @click="hideMenu">
    <div class="mint-menu-container" @click.stop>
      <div class="mint-menu-header">
        <div class="mint-menu-icon">
          <v-icon color="primary" size="24">mdi-leaf</v-icon>
        </div>
        <div class="mint-menu-title">MintMenu</div>
        <div class="mint-menu-shortcut">
          <kbd>⌘</kbd><kbd>K</kbd>
        </div>
      </div>
      
      <div class="mint-menu-search">
        <v-text-field
          v-model="searchQuery"
          placeholder="Search commands..."
          variant="outlined"
          density="compact"
          hide-details
          class="search-input"
          @keydown="handleKeyDown"
          ref="searchInput"
        >
          <template v-slot:prepend-inner>
            <v-icon>mdi-magnify</v-icon>
          </template>
        </v-text-field>
      </div>
      
      <div class="mint-menu-content">
        <div v-if="filteredCommands.length === 0" class="no-results">
          <v-icon size="48" color="grey" class="mb-2">mdi-magnify</v-icon>
          <div class="text-subtitle-1">No commands found</div>
          <p class="text-caption">Try a different search term</p>
        </div>
        
        <div v-else class="command-list">
          <div
            v-for="(command, index) in filteredCommands"
            :key="command.id"
            class="command-item"
            :class="{ active: index === selectedIndex }"
            @click="executeCommand(command)"
            @mouseenter="selectedIndex = index"
          >
            <div class="command-icon">
              <v-icon :color="command.color || 'primary'" size="20">
                {{ command.icon }}
              </v-icon>
            </div>
            <div class="command-content">
              <div class="command-title">{{ command.title }}</div>
              <div class="command-description">{{ command.description }}</div>
            </div>
            <div class="command-shortcut" v-if="command.shortcut">
              <kbd v-for="key in command.shortcut.split('+')" :key="key">{{ key }}</kbd>
            </div>
          </div>
        </div>
      </div>
      
      <div class="mint-menu-footer">
        <div class="footer-hint">
          <span class="text-caption">Use ↑↓ to navigate, Enter to select, Esc to close</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { store } from '../store.js'

export default {
  name: 'MintMenu',
  data() {
    return {
      show: false,
      searchQuery: '',
      selectedIndex: 0,
      commands: [
        {
          id: 'create-server',
          title: 'Create Server',
          description: 'Create a new Minecraft server',
          icon: 'mdi-plus-circle',
          color: 'success',
          action: () => this.$router.push({ name: 'CreateServer' })
        },
        {
          id: 'refresh-servers',
          title: 'Refresh Servers',
          description: 'Reload server list',
          icon: 'mdi-refresh',
          action: () => store.refreshServers()
        },
        {
          id: 'open-servers',
          title: 'Open Servers',
          description: 'Go to servers list',
          icon: 'mdi-view-grid',
          action: () => this.$router.push({ name: 'Servers' })
        },
        
        {
          id: 'start-all-servers',
          title: 'Start All Servers',
          description: 'Start all offline servers',
          icon: 'mdi-play-circle',
          color: 'success',
          action: async () => await this.startAllServers()
        },
        {
          id: 'stop-all-servers',
          title: 'Stop All Servers',
          description: 'Stop all running servers',
          icon: 'mdi-stop-circle',
          color: 'error',
          action: async () => await this.stopAllServers()
        },
        {
          id: 'backup-all-servers',
          title: 'Backup All Servers',
          description: 'Create backups for all servers',
          icon: 'mdi-backup-restore',
          color: 'warning',
          action: async () => await this.backupAllServers()
        },
        
        {
          id: 'open-settings',
          title: 'Open Settings',
          description: 'Open application settings',
          icon: 'mdi-cog',
          action: () => this.$router.push({ name: 'Settings' })
        },
        {
          id: 'check-updates',
          title: 'Check Updates',
          description: 'Check for application updates',
          icon: 'mdi-update',
          action: async () => await this.checkUpdates()
        },
        
        {
          id: 'open-file-manager',
          title: 'Open File Manager',
          description: 'Open server files manager',
          icon: 'mdi-folder-open',
          action: () => this.openFileManager()
        },
        {
          id: 'create-backup',
          title: 'Create Backup',
          description: 'Create a new backup',
          icon: 'mdi-backup-restore',
          color: 'warning',
          action: () => this.createBackup()
        },
        
        {
          id: 'restart-app',
          title: 'Restart Application',
          description: 'Restart ServerMint',
          icon: 'mdi-restart',
          color: 'warning',
          action: async () => await this.restartApp()
        },
        {
          id: 'quit-app',
          title: 'Quit Application',
          description: 'Close ServerMint',
          icon: 'mdi-exit-to-app',
          color: 'error',
          action: async () => await this.quitApp()
        },
        
        {
          id: 'toggle-ips',
          title: store.settings.general.showServerIPs ? 'Hide Server IPs' : 'Show Server IPs',
          description: store.settings.general.showServerIPs ? 'Hide server IP addresses' : 'Show server IP addresses',
          icon: store.settings.general.showServerIPs ? 'mdi-eye-off' : 'mdi-eye',
          color: 'info',
          action: () => this.toggleServerIPs()
        }
      ]
    }
  },
  computed: {
    filteredCommands() {
      if (!this.searchQuery.trim()) {
        return this.commands;
      }
      
      const query = this.searchQuery.toLowerCase();
      return this.commands.filter(command => 
        command.title.toLowerCase().includes(query) ||
        command.description.toLowerCase().includes(query) ||
        command.id.toLowerCase().includes(query)
      );
    }
  },
  mounted() {
    document.addEventListener('keydown', this.handleGlobalKeyDown);
    
    window.addEventListener('show-mint-menu', this.showMenu);
    window.addEventListener('hide-mint-menu', this.hideMenu);
  },
  beforeUnmount() {
    document.removeEventListener('keydown', this.handleGlobalKeyDown);
    window.removeEventListener('show-mint-menu', this.showMenu);
    window.removeEventListener('hide-mint-menu', this.hideMenu);
  },
  methods: {
    handleGlobalKeyDown(event) {
      if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
        event.preventDefault();
        this.toggleMenu();
      }
      
      if (event.key === 'Escape' && this.show) {
        event.preventDefault();
        this.hideMenu();
      }
    },
    
    handleKeyDown(event) {
      if (event.key === 'ArrowDown') {
        event.preventDefault();
        this.selectedIndex = Math.min(this.selectedIndex + 1, this.filteredCommands.length - 1);
      } else if (event.key === 'ArrowUp') {
        event.preventDefault();
        this.selectedIndex = Math.max(this.selectedIndex - 1, 0);
      } else if (event.key === 'Enter') {
        event.preventDefault();
        if (this.filteredCommands[this.selectedIndex]) {
          this.executeCommand(this.filteredCommands[this.selectedIndex]);
        }
      }
    },
    
    toggleMenu() {
      if (this.show) {
        this.hideMenu();
      } else {
        this.showMenu();
      }
    },
    
    showMenu() {
      this.show = true;
      this.searchQuery = '';
      this.selectedIndex = 0;
      
      this.$nextTick(() => {
        if (this.$refs.searchInput) {
          this.$refs.searchInput.focus();
        }
      });
    },
    
    hideMenu() {
      this.show = false;
      this.searchQuery = '';
      this.selectedIndex = 0;
    },
    
    async executeCommand(command) {
      try {
        console.log(`[MintMenu] Executing command: ${command.title}`);
        await command.action();
        this.hideMenu();
        
        if (['start-all-servers', 'stop-all-servers', 'backup-all-servers', 'refresh-servers'].includes(command.id)) {
          if (this.$root && this.$root.store && this.$root.store.loadServers) {
            await this.$root.store.loadServers();
          }
        }
        
        if (this.$root && this.$root.store && this.$root.store.showToast) {
          this.$root.store.showToast(`"${command.title}" completed successfully.`, 'success');
        }
      } catch (error) {
        console.error(`[MintMenu] Error executing command: ${error}`);
        
        if (this.$root && this.$root.store && this.$root.store.showToast) {
          this.$root.store.showToast(`Failed to execute "${command.title}": ${error.message}`, 'error');
        }
      }
    },
    
    async startAllServers() {
      const result = await store.startAllServers();
      if (result.count === 0) {
        throw new Error('No offline servers to start');
      }
      return result;
    },
    
    async stopAllServers() {
      const result = await store.stopAllServers();
      if (result.count === 0) {
        throw new Error('No running servers to stop');
      }
      return result;
    },
    
    async backupAllServers() {
      const result = await store.backupAllServers();
      if (result.count === 0) {
        throw new Error('No servers to backup');
      }
      return result;
    },
    
    async checkUpdates() {
      const result = await store.checkForUpdates();
      return result;
    },
    
    openFileManager() {
      const firstServer = store.servers[0];
      if (firstServer) {
        this.$router.push({
          name: 'ServerManagement',
          params: { serverId: firstServer.id },
          query: { tab: 'files' }
        });
      } else {
        throw new Error('No servers available');
      }
    },
    
    createBackup() {
      this.$router.push({ name: 'Backups' });
    },
    
    async restartApp() {
      await store.restartApplication();
    },
    
    async quitApp() {
      await store.quitApplication();
    },

    toggleServerIPs() {
      store.toggleServerIPs();      
      const toggleCommand = this.commands.find(cmd => cmd.id === 'toggle-ips');
      if (toggleCommand) {
        toggleCommand.title = store.settings.general.showServerIPs ? 'Hide Server IPs' : 'Show Server IPs';
        toggleCommand.description = store.settings.general.showServerIPs ? 'Hide server IP addresses' : 'Show server IP addresses';
        toggleCommand.icon = store.settings.general.showServerIPs ? 'mdi-eye-off' : 'mdi-eye';
      }
    }
  }
}
</script>

<style scoped>
.mint-menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(8px);
  z-index: 9999;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 10vh;
}

.mint-menu-container {
  background-color: #1a1a1a;
  border: 1px solid rgba(74, 222, 128, 0.2);
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 70vh;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
  animation: slideIn 0.2s ease-out;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.mint-menu-header {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(74, 222, 128, 0.1);
  background-color: #1e1e1e;
}

.mint-menu-icon {
  margin-right: 12px;
}

.mint-menu-title {
  font-size: 18px;
  font-weight: 600;
  color: #e0e0e0;
  flex-grow: 1;
}

.mint-menu-shortcut {
  display: flex;
  gap: 4px;
}

.mint-menu-shortcut kbd {
  background-color: #2a2a2a;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  padding: 2px 6px;
  font-size: 12px;
  color: #e0e0e0;
  font-family: 'JetBrains Mono', monospace;
}

.mint-menu-search {
  padding: 16px 20px;
  border-bottom: 1px solid rgba(74, 222, 128, 0.1);
}

.search-input {
  background-color: #2a2a2a;
}

.search-input :deep(.v-field__input) {
  color: #e0e0e0 !important;
}

.search-input :deep(.v-field__outline) {
  border-color: rgba(74, 222, 128, 0.3) !important;
}

.search-input :deep(.v-field--focused .v-field__outline) {
  border-color: #4ade80 !important;
}

.mint-menu-content {
  max-height: 400px;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: rgba(74, 222, 128, 0.3) transparent;
}

.mint-menu-content::-webkit-scrollbar {
  width: 8px;
}

.mint-menu-content::-webkit-scrollbar-track {
  background: transparent;
}

.mint-menu-content::-webkit-scrollbar-thumb {
  background-color: rgba(74, 222, 128, 0.3);
  border-radius: 4px;
}

.mint-menu-content::-webkit-scrollbar-thumb:hover {
  background-color: rgba(74, 222, 128, 0.5);
}

.no-results {
  text-align: center;
  padding: 40px 20px;
  color: #808080;
}

.command-list {
  padding: 8px 0;
}

.command-item {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  cursor: pointer;
  transition: background-color 0.2s;
  border-left: 3px solid transparent;
}

.command-item:hover,
.command-item.active {
  background-color: rgba(74, 222, 128, 0.1);
  border-left-color: #4ade80;
}

.command-icon {
  margin-right: 12px;
  width: 24px;
  display: flex;
  justify-content: center;
}

.command-content {
  flex-grow: 1;
  min-width: 0;
}

.command-title {
  font-weight: 500;
  color: #e0e0e0;
  margin-bottom: 2px;
}

.command-description {
  font-size: 12px;
  color: #808080;
  line-height: 1.4;
}

.command-shortcut {
  display: flex;
  gap: 2px;
  margin-left: 12px;
}

.command-shortcut kbd {
  background-color: #2a2a2a;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  padding: 1px 4px;
  font-size: 10px;
  color: #808080;
  font-family: 'JetBrains Mono', monospace;
}

.mint-menu-footer {
  padding: 12px 20px;
  border-top: 1px solid rgba(74, 222, 128, 0.1);
  background-color: #1e1e1e;
}

.footer-hint {
  text-align: center;
  color: #808080;
}
</style> 