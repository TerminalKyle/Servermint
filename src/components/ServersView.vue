<template>
  <div class="server-view">
    <!-- Filter tabs -->
    <div class="filter-section mb-4">
      <div class="d-flex align-center">
        <v-btn-toggle v-model="activeTab" color="primary" density="comfortable" mandatory rounded="lg" class="filter-tabs">
          <v-btn value="all" variant="text" class="px-4 py-2"><v-icon color="white" class="mr-2">mdi-view-grid</v-icon>All instances</v-btn>
          <v-btn value="downloaded" variant="text" class="px-4 py-2"><v-icon color="white" class="mr-2">mdi-download</v-icon>Downloaded</v-btn>
          <v-btn value="custom" variant="text" class="px-4 py-2"><v-icon color="white" class="mr-2">mdi-cog</v-icon>Custom</v-btn>
        </v-btn-toggle>
      </div>
    </div>

    <!-- Search and filter bar -->
    <div class="d-flex align-center mb-6 search-filter-container">
      <v-text-field
        v-model="searchQuery"
        prepend-inner-icon="mdi-magnify"
        placeholder="Search"
        variant="outlined"
        density="comfortable"
        bg-color="rgba(255, 255, 255, 0.1)"
        hide-details
        class="search-field mr-3"
        rounded="lg"
        clearable
      ></v-text-field>
      
      <v-menu>
        <template v-slot:activator="{ props }">
          <v-btn variant="outlined" color="secondary" v-bind="props" class="filter-btn mr-3" rounded="lg">
            <span class="mr-1 text-white">Sort by: Name</span>
            <v-icon size="small" color="white">mdi-chevron-down</v-icon>
          </v-btn>
        </template>
        <v-list density="compact" bg-color="surface" rounded="lg">
          <v-list-item value="name">
            <v-list-item-title>Name</v-list-item-title>
          </v-list-item>
          <v-list-item value="version">
            <v-list-item-title>Version</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
      
      <v-menu>
        <template v-slot:activator="{ props }">
          <v-btn variant="outlined" color="secondary" v-bind="props" class="filter-btn" rounded="lg">
            <span class="mr-1 text-white">Group by: Group</span>
            <v-icon size="small" color="white">mdi-chevron-down</v-icon>
          </v-btn>
        </template>
        <v-list density="compact" bg-color="surface" rounded="lg">
          <v-list-item value="none">
            <v-list-item-title>None</v-list-item-title>
          </v-list-item>
          <v-list-item value="group">
            <v-list-item-title>Group</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
      
      <v-spacer></v-spacer>
      
      <v-btn
        icon
        variant="text"
        @click="refreshServers"
        class="mr-2"
        :loading="isRefreshing"
      >
        <v-icon>mdi-refresh</v-icon>
        <v-tooltip activator="parent" location="bottom">Refresh Servers</v-tooltip>
      </v-btn>
      
      <CreateServerDialog class="d-none d-sm-block" />
    </div>



    <!-- Floating add server button (visible on all screens) -->
    <div class="floating-add-btn">
      <CreateServerDialog />
    </div>

    <!-- Server list -->
    <div class="server-list">
      <v-row v-if="filteredServers.length === 0">
        <v-col cols="12" class="text-center">
          <v-card class="pa-6" rounded="lg" elevation="3">
            <v-card-text>
              <v-icon size="64" color="white" class="mb-4">mdi-server-off</v-icon>
              <div class="text-h6 mb-2">No servers found</div>
              <p>You haven't added any Minecraft servers yet. Click the "Add Server" button to get started.</p>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
      
      <v-row v-else>
        <v-col v-for="(server, index) in filteredServers" :key="index" cols="12">
          <div 
            class="server-card" 
            @click="openServer(server)"
            @contextmenu.prevent="showContextMenu($event, server)"
          >
            <div class="d-flex align-center">
              <div class="server-icon-container mr-3">
                <v-avatar size="42" :color="server.icon ? undefined : 'primary'" rounded class="server-icon">
                  <v-img v-if="server.icon" :src="server.icon" alt="Server Icon"></v-img>
                  <v-icon v-else color="white">mdi-leaf</v-icon>
                </v-avatar>
                <div class="play-button" @click.stop="playServer(server)" v-if="!isServerDownloading(server)">
                  <v-icon color="white">mdi-play</v-icon>
                </div>
              </div>
              <div class="flex-grow-1">
                <div class="text-subtitle-1 font-weight-bold">{{ server.name }}</div>
                <div class="d-flex align-center">
                  <v-icon size="small" color="white" class="mr-1">{{ getServerTypeIcon(server.type) }}</v-icon>
                  <span class="text-caption">{{ server.type }} {{ server.version }}</span>
                  
                  <v-chip
                    size="x-small"
                    :color="getServerStatusColor(server)"
                    class="ml-2"
                    variant="flat"
                    density="comfortable"
                  >
                    {{ getServerStatusText(server) }}
                  </v-chip>
                </div>
                
                <!-- Download progress indicator -->
                <div v-if="isServerDownloading(server)" class="mt-2">
                  <v-progress-linear
                    :model-value="getServerDownloadProgress(server)"
                    :color="getDownloadProgressColor(server)"
                    height="8"
                    rounded
                    striped
                  ></v-progress-linear>
                  <div class="d-flex align-center justify-space-between mt-1">
                    <span class="text-caption">{{ getDownloadStatusText(server) }}</span>
                    <span class="text-caption">{{ Math.round(getServerDownloadProgress(server)) }}%</span>
                  </div>
                </div>
                
                <!-- Error message if download failed -->
                <div v-if="server.status === 'failed'" class="mt-2 error-message">
                  <v-icon size="small" color="error" class="mr-1">mdi-alert-circle</v-icon>
                  <span class="text-caption">{{ getServerErrorMessage(server) }}</span>
                </div>
              </div>
            </div>
          </div>
        </v-col>
      </v-row>
    </div>
    
    <!-- Custom context menu -->
    <div 
      v-if="contextMenu.show" 
      class="context-menu" 
      :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
    >
      <div class="menu-item" @click="playContextMenu">
        <v-icon class="menu-icon" color="white">mdi-play</v-icon>
        <span>Play</span>
      </div>
      
      <div class="menu-item" @click="addContentContextMenu">
        <v-icon class="menu-icon" color="white">mdi-plus</v-icon>
        <span>Add content</span>
      </div>
      
      <div class="menu-divider"></div>
      
      <div class="menu-item" @click="viewInstanceContextMenu">
        <v-icon class="menu-icon" color="white">mdi-eye-outline</v-icon>
        <span>View instance</span>
      </div>
      
      <div class="menu-item" @click="duplicateInstanceContextMenu">
        <v-icon class="menu-icon" color="white">mdi-content-duplicate</v-icon>
        <span>Duplicate instance</span>
      </div>
      
      <div class="menu-item" @click="openFolderContextMenu">
        <v-icon class="menu-icon" color="white">mdi-folder-open-outline</v-icon>
        <span>Open folder</span>
      </div>
      
      <div class="menu-item" @click="copyPathContextMenu">
        <v-icon class="menu-icon" color="white">mdi-content-copy</v-icon>
        <span>Copy path</span>
      </div>
      
      <div class="menu-divider"></div>
      
      <div class="menu-item delete-item" @click="deleteServerContextMenu">
        <v-icon class="menu-icon" color="white">mdi-delete-outline</v-icon>
        <span>Delete</span>
      </div>
    </div>
    
    <!-- Delete Confirmation Dialog -->
    <ConfirmDialog
      v-model="showDeleteConfirmation"
      title="Delete Server"
      subtitle="This action cannot be undone"
      :message="`Are you sure you want to delete the server '${serverToDelete?.name}'?`"
      :warning-details="[
        'Remove the server from ServerMint',
        'Delete all server files and folders',
        'Stop the server if it\'s running',
        'Remove all server data permanently'
      ]"
      confirm-text="Delete Server"
      cancel-text="Cancel"
      confirm-color="error"
      confirm-variant="elevated"
      confirm-icon="mdi-delete"
      :require-confirmation="true"
      confirmation-label="Type to confirm deletion"
      confirmation-placeholder="DELETE"
      :loading="deleteLoading"
      @confirm="confirmDeleteServer"
      @cancel="cancelDeleteServer"
    />
  </div>
</template>

<script>
import CreateServerDialog from './CreateServerDialog.vue';
import { store } from '../store.js'
import ConfirmDialog from './ConfirmDialog.vue';

export default {
  name: 'ServersView',
  components: {
    CreateServerDialog,
    ConfirmDialog
  },
  data() {
    return {
      activeTab: 'all',
      searchQuery: '',
      contextMenu: {
              show: false,
      x: 0,
      y: 0,
      server: null
    },
    showDeleteConfirmation: false,
    serverToDelete: null,
    deleteLoading: false,
      store: store,
      isRefreshing: false
    }
  },
  mounted() {
    document.addEventListener('click', this.hideContextMenu);
    document.addEventListener('keydown', this.handleKeyDown);
    
    // Load servers from backend
    this.loadServers();
    
    // Listen for server creation events using global event bus
    window.addEventListener('server-created', this.handleServerCreated);
  },
  beforeUnmount() {
    document.removeEventListener('click', this.hideContextMenu);
    document.removeEventListener('keydown', this.handleKeyDown);
    
    // Remove event listener
    window.removeEventListener('server-created', this.handleServerCreated);
  },
  computed: {
    servers() {
      return this.store.servers;
    },
    filteredServers() {
      let filtered = [...this.servers];
      
      // Filter by tab
      if (this.activeTab === 'downloaded') {
        filtered = filtered.filter(server => server.status !== 'installing');
      } else if (this.activeTab === 'custom') {
        filtered = filtered.filter(server => server.type !== 'Vanilla');
      }
      
      // Filter by search
      if (this.searchQuery) {
        const query = this.searchQuery.toLowerCase();
        filtered = filtered.filter(server => 
          server.name.toLowerCase().includes(query) ||
          server.type.toLowerCase().includes(query) ||
          server.version.toLowerCase().includes(query)
        );
      }
      
      return filtered;
    }
  },
  methods: {
    async loadServers() {
      try {
        await this.store.loadServers();
      } catch (error) {
        console.error('Error loading servers:', error);
      }
    },
    
    handleServerCreated() {
      // Refresh the server list when a new server is created
      this.loadServers();
    },
    
    async refreshServers() {
      this.isRefreshing = true;
      try {
        await this.loadServers();
      } catch (error) {
        console.error('Error refreshing servers:', error);
      } finally {
        this.isRefreshing = false;
      }
    },
    
    getServerTypeIcon(type) {
      const icons = {
        'Paper': 'mdi-book-open-page-variant',
        'Vanilla': 'mdi-minecraft',
        'Spigot': 'mdi-minecraft',
        'Forge': 'mdi-anvil',
        'Fabric': 'mdi-fabric',
        'Neoforge': 'mdi-anvil'
      };
      return icons[type] || 'mdi-minecraft';
    },
    openServer(server) {
      if (server.status === 'installing') return; // Don't open if still installing
      console.log('Opening server:', server.name);
      this.navigateToServerManagement(server);
    },
    playServer(server) {
      if (server.status === 'installing') return; // Don't play if still installing
      
      if (server.status === 'offline') {
        // Start the server first
        this.store.startServer(server.id).then(() => {
          // Navigate to console tab after starting
          this.navigateToServerManagement(server, 'console');
        });
      } else {
        // Server is already running, just navigate to console
        this.navigateToServerManagement(server, 'console');
      }
    },
    navigateToServerManagement(server, activeTab = 'files') {
      // Navigate to the server management view with the selected server
      this.$router.push({
        name: 'ServerManagement',
        params: { serverId: server.id },
        query: { tab: activeTab }
      });
    },
    showContextMenu(event, server) {
      if (server.status === 'installing') return; // Don't show context menu if still installing
      
      // Position the menu at the mouse position
      this.contextMenu.x = event.clientX;
      this.contextMenu.y = event.clientY;
      this.contextMenu.server = server;
      this.contextMenu.show = true;
      
      // Prevent default browser context menu
      event.preventDefault();
    },
    hideContextMenu() {
      this.contextMenu.show = false;
    },
    handleKeyDown(event) {
      if (event.key === 'Escape') {
        this.hideContextMenu();
      }
    },
    playContextMenu() {
      if (this.contextMenu.server) {
        this.playServer(this.contextMenu.server);
      }
      this.hideContextMenu();
    },
    addContentContextMenu() {
      if (this.contextMenu.server) {
        console.log('Adding content to server:', this.contextMenu.server.name);
        alert('Add content functionality not implemented yet');
      }
      this.hideContextMenu();
    },
    viewInstanceContextMenu() {
      if (this.contextMenu.server) {
        this.navigateToServerManagement(this.contextMenu.server);
      }
      this.hideContextMenu();
    },
    duplicateInstanceContextMenu() {
      if (this.contextMenu.server) {
        console.log('Duplicating instance:', this.contextMenu.server.name);
        alert('Duplicate instance functionality not implemented yet');
      }
      this.hideContextMenu();
    },
    openFolderContextMenu() {
      if (this.contextMenu.server) {
        this.store.openServerFolder(this.contextMenu.server.id);
      }
      this.hideContextMenu();
    },
    copyPathContextMenu() {
      if (this.contextMenu.server) {
        console.log('Copying path:', this.contextMenu.server.path);
        navigator.clipboard.writeText(this.contextMenu.server.path);
      }
      this.hideContextMenu();
    },
    async deleteServerContextMenu() {
      if (this.contextMenu.server) {
        this.showDeleteConfirmation = true;
        this.serverToDelete = this.contextMenu.server;
      }
      this.hideContextMenu();
    },
    
    async confirmDeleteServer() {
      if (!this.serverToDelete) return;
      
      this.deleteLoading = true;
      
      try {
        console.log('Deleting server:', this.serverToDelete.name);
        
        const result = await this.store.deleteServer(this.serverToDelete.id);
        
        if (result.success) {
          // Show success toast
          if (window.showSuccess) {
            window.showSuccess('Server Deleted', `"${this.serverToDelete.name}" has been removed.`);
          }
        } else {
          // Show error toast
          if (window.showError) {
            window.showError('Delete Failed', `Failed to delete server: ${result.error}`);
          }
        }
      } catch (error) {
        console.error('Error deleting server:', error);
        if (window.showError) {
          window.showError('Delete Failed', `An error occurred while deleting the server.`);
        }
      } finally {
        this.deleteLoading = false;
        this.showDeleteConfirmation = false;
        this.serverToDelete = null;
      }
    },
    
    cancelDeleteServer() {
      this.showDeleteConfirmation = false;
      this.serverToDelete = null;
      this.deleteLoading = false;
    },
    isServerDownloading(server) {
      return server.status === 'installing';
    },
    getServerDownloadProgress(server) {
      if (server.status !== 'installing') return 100;
      
      const progress = this.store.getDownloadProgress(server.id);
      return progress.progress || 0;
    },
    
    getServerStatusText(server) {
      if (server.status === 'installing') {
        const progress = this.store.getDownloadProgress(server.id);
        if (progress && progress.status) {
          if (progress.status === 'preparing') {
            return 'Preparing';
          } else if (progress.status === 'downloading') {
            return 'Downloading';
          } else if (progress.status === 'downloading jar') {
            return 'Downloading JAR';
          } else if (progress.status === 'configuring') {
            return 'Configuring';
          } else if (progress.status === 'completed') {
            return 'Completed';
          } else if (progress.status === 'failed') {
            return 'Failed';
          }
        }
        return 'Installing';
      } else if (server.status === 'starting') {
        return 'Starting';
      } else if (server.status === 'stopping') {
        return 'Stopping';
      } else if (server.status === 'online') {
        return 'Online';
      } else if (server.status === 'offline') {
        return 'Offline';
      } else if (server.status === 'failed') {
        return 'Failed';
      }
      return server.status.charAt(0).toUpperCase() + server.status.slice(1);
    },
    
    getServerStatusColor(server) {
      if (server.status === 'online') return 'success';
      if (server.status === 'offline') return 'error';
      if (server.status === 'installing') return 'info';
      if (server.status === 'starting' || server.status === 'stopping') return 'warning';
      if (server.status === 'failed') return 'error';
      return 'grey';
    },

    getDownloadProgressColor(server) {
      if (server.status === 'failed') return 'error';
      return 'primary';
    },

    getDownloadStatusText(server) {
      const progress = this.store.getDownloadProgress(server.id);
      if (progress && progress.status) {
        if (progress.status === 'preparing') {
          return 'Preparing files...';
        } else if (progress.status === 'downloading') {
          return 'Downloading files...';
        } else if (progress.status === 'downloading jar') {
          return 'Downloading JAR...';
        } else if (progress.status === 'configuring') {
          return 'Configuring server...';
        } else if (progress.status === 'completed') {
          return 'Files ready!';
        } else if (progress.status === 'failed') {
          return 'Download failed!';
        }
      }
      return 'Downloading...';
    },

    getServerErrorMessage(server) {
      const progress = this.store.getDownloadProgress(server.id);
      if (progress && progress.error) {
        return progress.error;
      }
      return 'Unknown error';
    }
  }
}
</script>

<style scoped>
.server-view {
  padding: 16px;
  position: relative;
}
.filter-tabs {
  background-color: rgba(30, 30, 30, 0.8);
  border-radius: 50px !important;
  overflow: hidden;
}
.filter-tabs .v-btn {
  text-transform: none;
  font-weight: 500;
}
.filter-tabs .v-btn--active {
  background-color: #4ade80 !important;
  color: #121212 !important;
}
.search-filter-container {
  height: 48px;
  width: 100%;
}
.search-field {
  background-color: rgba(30, 30, 30, 0.8);
  border-radius: 8px;
  height: 100%;
  flex-grow: 1;
}
.search-field :deep(.v-field__field) {
  height: 100%;
}
.filter-btn {
  background-color: rgba(30, 30, 30, 0.8);
  text-transform: none;
  border: 1px solid rgba(255, 255, 255, 0.1);
  height: 100%;
  min-width: 150px;
  padding: 0 16px;
}
.server-card {
  background-color: #1e1e1e;
  border-radius: 8px;
  padding: 12px 16px;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(74, 222, 128, 0.05);
}
.server-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(74, 222, 128, 0) 0%,
    rgba(74, 222, 128, 0) 100%);
  opacity: 0;
  transition: opacity 0.5s ease, background 0.5s ease;
  z-index: 0;
  pointer-events: none !important;
}
.server-card:hover {
  background-color: #1e1e1e;
  cursor: pointer;
  transform: translateY(-2px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  border-color: rgba(74, 222, 128, 0.2);
}
.server-card:hover::before {
  opacity: 1;
  background: linear-gradient(135deg, 
    rgba(74, 222, 128, 0.15) 0%, 
    rgba(74, 222, 128, 0.05) 30%, 
    rgba(74, 222, 128, 0) 60%);
}
.server-card > div {
  position: relative;
  z-index: 10;
  pointer-events: auto;
}
.server-icon-container {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}
.server-icon {
  transition: opacity 0.2s ease;
}
.play-button {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(74, 222, 128, 0.9);
  border-radius: 8px;
  opacity: 0;
  transition: opacity 0.2s ease;
  cursor: pointer;
}
.server-icon-container:hover .server-icon {
  opacity: 0.3;
}
.server-icon-container:hover .play-button {
  opacity: 1;
}
.gap-3 {
  gap: 12px;
}
.floating-add-btn {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 100;
}
.floating-add-btn :deep(.add-server-btn) {
  background-color: #4ade80;
  color: #121212;
  font-weight: 600;
  box-shadow: 0 4px 12px rgba(74, 222, 128, 0.3);
}
@media (min-width: 600px) {
  .floating-add-btn {
    display: none;
  }
}

/* Custom context menu styling */
.context-menu {
  position: fixed;
  z-index: 1000;
  background-color: #121212;
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 4px;
  width: 220px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  overflow: hidden;
}
.menu-item {
  padding: 8px 16px;
  display: flex;
  align-items: center;
  cursor: pointer;
  transition: background-color 0.2s;
}
.menu-item:hover {
  background-color: rgba(255, 255, 255, 0.05);
}
.menu-icon {
  margin-right: 12px;
  font-size: 20px;
}
.menu-divider {
  height: 1px;
  background-color: rgba(255, 255, 255, 0.1);
  margin: 4px 0;
}
.delete-item {
  color: #ef4444;
}

/* Error message styling */
.error-message {
  color: #ef4444;
  font-size: 12px;
  padding: 4px 0;
  display: flex;
  align-items: center;
}
</style> 