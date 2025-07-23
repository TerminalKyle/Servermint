<template>
  <div class="mods-view">
    <!-- Top Navigation Tabs -->
    <div class="nav-tabs mb-4">
      <v-btn-toggle v-model="activeTab" color="primary" density="comfortable" mandatory rounded="lg" class="nav-tabs-toggle">
        <v-btn value="modpacks" variant="text" class="px-4 py-2">Modpacks</v-btn>
        <v-btn value="mods" variant="text" class="px-4 py-2">Mods</v-btn>
        <v-btn value="resource-packs" variant="text" class="px-4 py-2">Resource Packs</v-btn>
        <v-btn value="data-packs" variant="text" class="px-4 py-2">Data Packs</v-btn>
        <v-btn value="shaders" variant="text" class="px-4 py-2">Shaders</v-btn>
      </v-btn-toggle>
    </div>

    <!-- Search and Filter Bar -->
    <div class="search-filter-bar mb-6">
      <div class="d-flex align-center justify-space-between">
        <div class="d-flex align-center flex-grow-1">
          <v-text-field
            v-model="searchQuery"
            prepend-inner-icon="mdi-magnify"
            placeholder="Search modpacks..."
            variant="outlined"
            density="comfortable"
            bg-color="rgba(255, 255, 255, 0.1)"
            hide-details
            class="search-field mr-4"
            rounded="lg"
            clearable
            @update:model-value="onSearchInput"
          ></v-text-field>
          
          <v-menu>
            <template v-slot:activator="{ props }">
              <v-btn variant="outlined" color="secondary" v-bind="props" class="filter-btn mr-3" rounded="lg">
                <span class="mr-1 text-white">Sort by: {{ sortBy }}</span>
                <v-icon size="small" color="white">mdi-chevron-down</v-icon>
              </v-btn>
            </template>
            <v-list density="compact" bg-color="surface" rounded="lg">
              <v-list-item @click="sortBy = 'Relevance'">
                <v-list-item-title>Relevance</v-list-item-title>
              </v-list-item>
              <v-list-item @click="sortBy = 'Downloads'">
                <v-list-item-title>Downloads</v-list-item-title>
              </v-list-item>
              <v-list-item @click="sortBy = 'Followers'">
                <v-list-item-title>Followers</v-list-item-title>
              </v-list-item>
              <v-list-item @click="sortBy = 'Updated'">
                <v-list-item-title>Updated</v-list-item-title>
              </v-list-item>
              <v-list-item @click="sortBy = 'Created'">
                <v-list-item-title>Created</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
          
          <v-menu>
            <template v-slot:activator="{ props }">
              <v-btn variant="outlined" color="secondary" v-bind="props" class="filter-btn" rounded="lg">
                <span class="mr-1 text-white">View: {{ viewCount }}</span>
                <v-icon size="small" color="white">mdi-chevron-down</v-icon>
              </v-btn>
            </template>
            <v-list density="compact" bg-color="surface" rounded="lg">
              <v-list-item @click="viewCount = 20">
                <v-list-item-title>20</v-list-item-title>
              </v-list-item>
              <v-list-item @click="viewCount = 50">
                <v-list-item-title>50</v-list-item-title>
              </v-list-item>
              <v-list-item @click="viewCount = 100">
                <v-list-item-title>100</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </div>
        
        <!-- Pagination -->
        <div class="pagination-controls">
          <v-btn-group>
            <v-btn size="small" variant="text" :disabled="currentPage === 1">‹</v-btn>
            <v-btn size="small" variant="text" :color="currentPage === 1 ? 'primary' : 'default'">1</v-btn>
            <v-btn size="small" variant="text" :color="currentPage === 2 ? 'primary' : 'default'">2</v-btn>
            <v-btn size="small" variant="text">...</v-btn>
            <v-btn size="small" variant="text">544</v-btn>
            <v-btn size="small" variant="text">›</v-btn>
          </v-btn-group>
        </div>
      </div>
    </div>

    <!-- Content Tabs -->
    <div class="content-tabs mb-4">
      <v-btn-toggle v-model="contentTab" color="primary" density="comfortable" mandatory rounded="lg" class="content-tabs-toggle">
        <v-btn value="available" variant="text" class="px-4 py-2">
          <v-icon class="mr-2">mdi-download</v-icon>Available
        </v-btn>
        <v-btn value="installed" variant="text" class="px-4 py-2">
          <v-icon class="mr-2">mdi-check-circle</v-icon>Installed
        </v-btn>
      </v-btn-toggle>
    </div>

    <!-- Modpack/Mod List -->
    <v-window v-model="contentTab">
      <v-window-item value="available">
        <div v-if="filteredAvailableMods.length === 0" class="text-center pa-8">
          <v-icon size="64" color="white" class="mb-4">mdi-package-variant-closed</v-icon>
          <div class="text-h6 mb-2">No modpacks found</div>
          <p>No modpacks available matching your criteria. Try changing your search or filters.</p>
          <v-btn color="primary" class="mt-4" @click="loadAvailableMods" :loading="isLoading">
            Browse Repository
          </v-btn>
        </div>
        
        <div v-else class="modpack-list">
          <div v-for="(mod, index) in filteredAvailableMods" :key="index" class="modpack-item">
            <div class="modpack-card" @click="showModDetails(mod)" @contextmenu.prevent="showContextMenu($event, mod)">
              <!-- Left Side - Icon -->
              <div class="modpack-icon">
                <v-avatar size="64" :color="getModpackColor(mod.name)" rounded class="modpack-avatar">
                  <v-img v-if="mod.image" :src="mod.image" alt="Modpack Icon"></v-img>
                  <span v-else class="modpack-initials">{{ getModpackInitials(mod.name) }}</span>
                </v-avatar>
                <div class="install-overlay" @click.stop="installMod(mod)">
                  <v-icon color="white">mdi-download</v-icon>
                </div>
              </div>
              
              <!-- Middle Section - Details -->
              <div class="modpack-details">
                <div class="modpack-title">{{ mod.name }}</div>
                <div class="modpack-author">by {{ mod.author || 'Unknown' }}</div>
                <div class="modpack-description">{{ mod.description }}</div>
                <div class="modpack-tags">
                  <v-chip v-for="tag in getModpackTags(mod)" :key="tag" size="x-small" variant="flat" color="grey-darken-3" class="mr-1 mb-1">
                    {{ tag }}
                  </v-chip>
                </div>
              </div>
              
              <!-- Right Side - Stats and Action -->
              <div class="modpack-stats">
                <div class="stat-item">
                  <v-icon size="small" color="white" class="mr-1">mdi-download</v-icon>
                  <span class="stat-text">{{ formatDownloads(mod.downloads) }} downloads</span>
                </div>
                <div class="stat-item">
                  <v-icon size="small" color="white" class="mr-1">mdi-heart</v-icon>
                  <span class="stat-text">{{ formatFollowers(mod.followers || 0) }} followers</span>
                </div>
                <v-btn 
                  color="primary" 
                  variant="flat" 
                  size="small" 
                  prepend-icon="mdi-download"
                  @click.stop="installMod(mod)"
                  class="install-btn"
                >
                  Install
                </v-btn>
              </div>
            </div>
          </div>
        </div>
      </v-window-item>
      
      <v-window-item value="installed">
        <div v-if="filteredInstalledMods.length === 0" class="text-center pa-8">
          <v-icon size="64" color="white" class="mb-4">mdi-package-variant</v-icon>
          <div class="text-h6 mb-2">No modpacks installed</div>
          <p>You haven't installed any modpacks yet. Browse the repository to get started.</p>
        </div>
        
        <div v-else class="modpack-list">
          <div v-for="(mod, index) in filteredInstalledMods" :key="index" class="modpack-item">
            <div class="modpack-card">
              <!-- Left Side - Icon -->
              <div class="modpack-icon">
                <v-avatar size="64" :color="getModpackColor(mod.name)" rounded class="modpack-avatar">
                  <v-img v-if="mod.image" :src="mod.image" alt="Modpack Icon"></v-img>
                  <span v-else class="modpack-initials">{{ getModpackInitials(mod.name) }}</span>
                </v-avatar>
              </div>
              
              <!-- Middle Section - Details -->
              <div class="modpack-details">
                <div class="modpack-title">{{ mod.name }}</div>
                <div class="modpack-author">by {{ mod.author || 'Unknown' }}</div>
                <div class="modpack-description">{{ mod.description }}</div>
                <div class="modpack-tags">
                  <v-chip size="x-small" variant="flat" color="success" class="mr-1 mb-1">Installed</v-chip>
                  <v-chip size="x-small" variant="flat" color="grey-darken-3" class="mr-1 mb-1">{{ mod.source }}</v-chip>
                </div>
              </div>
              
              <!-- Right Side - Stats and Action -->
              <div class="modpack-stats">
                <div class="stat-item">
                  <v-icon size="small" color="white" class="mr-1">mdi-server</v-icon>
                  <span class="stat-text">{{ getServerName(mod.serverId) }}</span>
                </div>
                <div class="stat-item">
                  <v-icon size="small" color="white" class="mr-1">mdi-folder</v-icon>
                  <span class="stat-text">{{ mod.folder || 'mods' }}</span>
                </div>
                <v-btn 
                  color="error" 
                  variant="outlined" 
                  size="small" 
                  @click="uninstallMod(mod)"
                  class="uninstall-btn"
                >
                  Uninstall
                </v-btn>
              </div>
            </div>
          </div>
        </div>
      </v-window-item>
    </v-window>

    <!-- Server selection dialog for installation -->
    <v-dialog v-model="serverSelectionDialog" max-width="500px">
      <v-card color="#1E1E1E">
        <v-card-title class="text-h6">Select Server</v-card-title>
        <v-card-text>
          <p class="mb-4">Choose which server to install "{{ pendingModInstall?.name }}" to:</p>
          <v-select
            label="Server"
            :items="servers"
            item-title="name"
            item-value="id"
            v-model="targetServerId"
            variant="outlined"
            density="comfortable"
            bg-color="#2A2A2A"
          ></v-select>
          
          <v-select
            v-if="pendingModInstall"
            label="Installation Folder"
            :items="getServerFolderOptions()"
            v-model="targetFolder"
            variant="outlined"
            density="comfortable"
            bg-color="#2A2A2A"
            class="mt-4"
          ></v-select>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="error" text @click="serverSelectionDialog = false">Cancel</v-btn>
          <v-btn color="primary" @click="confirmModInstall" :loading="installLoading">Install</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Mod details dialog -->
    <v-dialog v-model="showModDetailsDialog" max-width="700px">
      <v-card color="#1E1E1E">
        <v-card-title class="text-h5 font-weight-bold">{{ selectedMod?.name }}</v-card-title>
        <v-card-text>
          <div class="d-flex mb-4">
            <v-avatar size="80" :color="getModpackColor(selectedMod?.name)" rounded class="mr-4">
              <v-img v-if="selectedMod?.image" :src="selectedMod.image" alt="Modpack Icon"></v-img>
              <span v-else class="modpack-initials-large">{{ getModpackInitials(selectedMod?.name) }}</span>
            </v-avatar>
            <div>
              <div class="d-flex align-center mb-2">
                <v-chip :color="getSourceColor(selectedMod?.source)" variant="flat" size="small" class="mr-2">
                  {{ selectedMod?.source }}
                </v-chip>
                <span class="text-caption">{{ formatDownloads(selectedMod?.downloads) }} downloads</span>
              </div>
              <p class="text-body-2 mb-2">{{ selectedMod?.type }} {{ selectedMod?.version }}</p>
              <p class="text-body-2">{{ selectedMod?.description }}</p>
            </div>
          </div>
          
          <v-divider class="mb-4"></v-divider>
          
          <v-btn 
            color="primary" 
            block 
            class="mb-4" 
            prepend-icon="mdi-download" 
            @click="installMod(selectedMod)"
          >
            Install to Server
          </v-btn>
        </v-card-text>
      </v-card>
    </v-dialog>
    
    <!-- Custom context menu -->
    <div 
      v-if="contextMenu.show" 
      class="context-menu" 
      :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
    >
      <div class="menu-item" @click="installModContextMenu">
        <v-icon class="menu-icon" color="white">mdi-download</v-icon>
        <span>Install</span>
      </div>
      
      <div class="menu-item" @click="viewDetailsContextMenu">
        <v-icon class="menu-icon" color="white">mdi-information-outline</v-icon>
        <span>View details</span>
      </div>
      
      <div class="menu-item" @click="viewModWebsiteContextMenu" v-if="contextMenu.mod?.source === 'Modrinth'">
        <v-icon class="menu-icon" color="white">mdi-web</v-icon>
        <span>View on {{ contextMenu.mod?.source }}</span>
      </div>
      
      <div class="menu-divider"></div>
      
      <div class="menu-item" @click="hideContextMenu">
        <v-icon class="menu-icon" color="white">mdi-close</v-icon>
        <span>Close</span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ModsView',
  data() {
    return {
      activeTab: 'modpacks', // Default to Modpacks tab
      contentTab: 'available', // Default to Available mods/modpacks
      sortBy: 'Relevance',
      viewCount: 20,
      currentPage: 1,
      selectedServer: 1,
      selectedType: 'All',
      selectedSource: 'All',
      searchQuery: '',
      searchTimeout: null, // For debouncing search
      servers: [
        { id: 1, name: 'Survival Server' },
        { id: 2, name: 'Creative Server' }
      ],
      // Original data store for installed mods
      installedMods: [
        {
          name: 'OptiFine',
          version: '1.20.1_HD_U_I6',
          type: 'Mod',
          description: 'Improves Minecraft performance and adds features like dynamic lighting and shader support.',
          image: '',
          source: 'Local',
          serverId: 1
        },
        {
          name: 'JourneyMap',
          version: '5.9.7',
          type: 'Mod',
          description: 'Real-time mapping in-game or your browser as you explore.',
          image: '',
          source: 'CurseForge',
          serverId: 1
        },
        {
          name: 'WorldEdit',
          version: '7.2.15',
          type: 'Plugin',
          description: 'In-game map editor for single-player and multiplayer.',
          image: '',
          source: 'CurseForge',
          serverId: 2
        }
      ],
      // Original data store for available mods
      availableMods: [], // Start with empty array, will be populated by API
      
      // Server selection dialog
      serverSelectionDialog: false,
      pendingModInstall: null,
      targetServerId: null,
      targetFolder: null, // New for server selection dialog
      installLoading: false, // New for install button loading
      
      // Local mod import dialog
      importLocalDialog: false,
      localModFile: null,
      localModTargetServer: null,
      localModTargetFolder: null, // New for local mod import dialog
      
      // Mod details dialog
      showModDetailsDialog: false,
      selectedMod: null,
      
      // Custom context menu
      contextMenu: {
        show: false,
        x: 0,
        y: 0,
        mod: null
      },
      
      // Sample data for repository
      modCategories: [
        'All',
        'Optimization',
        'Utility',
        'World Generation',
        'Technology',
        'Magic',
        'Storage',
        'Adventure'
      ],
      gameVersions: [
        '1.20.1',
        '1.19.4',
        '1.19.2',
        '1.18.2',
        '1.17.1',
        '1.16.5'
      ],
      
      // Loading state
      isLoading: false
    }
  },
  computed: {
    filteredInstalledMods() {
      return this.installedMods.filter(mod => {
        // Server filter
        if (mod.serverId !== this.selectedServer) return false;
        
        // Type filter
        if (this.selectedType !== 'All' && mod.type !== this.selectedType) return false;
        
        // Source filter
        if (this.selectedSource !== 'All' && mod.source !== this.selectedSource) return false;
        
        // Search query
        if (this.searchQuery && !this.matchesSearch(mod, this.searchQuery)) return false;
        
        return true;
      });
    },
    
    filteredAvailableMods() {
      return this.availableMods.filter(mod => {
        // Type filter
        if (this.selectedType !== 'All' && mod.type !== this.selectedType) return false;
        
        // Source filter
        if (this.selectedSource !== 'All' && mod.source !== this.selectedSource) return false;
        
        // Search query
        if (this.searchQuery && !this.matchesSearch(mod, this.searchQuery)) return false;
        
        // Don't show mods that are already installed on the selected server
        const alreadyInstalled = this.installedMods.some(installedMod => 
          installedMod.name === mod.name && 
          installedMod.serverId === this.selectedServer
        );
        
        return !alreadyInstalled;
      });
    }
  },
  created() {
    // Initialize filtered lists
    // this.filterMods(); // This is now handled by computed properties
    
    // Initialize repository mods (in a real app this would fetch from API)
    this.loadAvailableMods(); // Auto-load available mods when component mounts
  },
  watch: {
    // Watch for filter changes
    selectedServer() {
      // this.filterMods(); // This is now handled by computed properties
    },
    selectedType() {
      // this.filterMods(); // This is now handled by computed properties
    },
    selectedSource() {
      // this.filterMods(); // This is now handled by computed properties
    },
    searchQuery() {
      // this.filterMods(); // This is now handled by computed properties
    },
    
    // Repository filter watchers
    // repositorySource() { // This property is removed
    //   this.searchRepository();
    // },
    // repositoryCategory() { // This property is removed
    //   this.searchRepository();
    // },
    // repositoryGameVersion() { // This property is removed
    //   this.searchRepository();
    // }
  },
  methods: {
    // filterMods() { // This method is removed
    //   // Filter installed mods
    //   this.filteredInstalledMods = this.installedMods.filter(mod => {
    //     // Server filter
    //     if (mod.serverId !== this.selectedServer) return false;
        
    //     // Type filter
    //     if (this.selectedType !== 'All' && mod.type !== this.selectedType) return false;
        
    //     // Source filter
    //     if (this.selectedSource !== 'All' && mod.source !== this.selectedSource) return false;
        
    //     // Search query
    //     if (this.searchQuery && !this.matchesSearch(mod, this.searchQuery)) return false;
        
    //     return true;
    //   });
      
    //   // Filter available mods
    //   this.filteredAvailableMods = this.availableMods.filter(mod => {
    //     // Type filter
    //     if (this.selectedType !== 'All' && mod.type !== this.selectedType) return false;
        
    //     // Source filter
    //     if (this.selectedSource !== 'All' && mod.source !== this.selectedSource) return false;
        
    //     // Search query
    //     if (this.searchQuery && !this.matchesSearch(mod, this.searchQuery)) return false;
        
    //     // Don't show mods that are already installed on the selected server
    //     const alreadyInstalled = this.installedMods.some(installedMod => 
    //       installedMod.name === mod.name && 
    //       installedMod.serverId === this.selectedServer
    //     );
        
    //     return !alreadyInstalled;
    //   });
    // },
    
    matchesSearch(mod, query) {
      // Convert to lowercase for case-insensitive search
      query = query.toLowerCase();
      
      // Search in name, description, type, version, source
      return mod.name.toLowerCase().includes(query) ||
        mod.description.toLowerCase().includes(query) ||
        mod.type.toLowerCase().includes(query) ||
        mod.version.toLowerCase().includes(query) ||
        mod.source.toLowerCase().includes(query);
    },
    
    installMod(mod) {
      // Set pending mod and show server selection dialog
      this.pendingModInstall = mod;
      this.targetServerId = this.selectedServer; // Default to currently selected server
      this.targetFolder = 'mods'; // Default to mods folder
      this.serverSelectionDialog = true;
    },
    
    confirmModInstall() {
      if (!this.pendingModInstall || !this.targetServerId) return;
      
      this.installLoading = true; // Start loading
      
      // In a real app, this would communicate with Tauri to install the mod
      console.log(`Installing ${this.pendingModInstall.name} to server ${this.targetServerId}`);
      
      // If the mod has a download_url, download the file and install it to the appropriate folder
      if (this.pendingModInstall.download_url) {
        this.downloadAndInstallMod(this.pendingModInstall);
      } else {
        // If no download_url, just add to installed mods
        const modToInstall = {
          ...this.pendingModInstall,
          serverId: this.targetServerId,
          folder: this.targetFolder // Add folder to the mod object
        };
        this.installedMods.push(modToInstall);
        this.availableMods = this.availableMods.filter(m => m.name !== this.pendingModInstall.name);
        // this.filterMods(); // This is now handled by computed properties
        this.serverSelectionDialog = false;
        this.pendingModInstall = null;
        this.installLoading = false; // End loading
        alert(`Mod ${modToInstall.name} installed successfully to ${this.servers.find(s => s.id === modToInstall.serverId).name}`);
      }
    },
    
    async downloadAndInstallMod(mod) {
      // In a real app with Tauri, this would:
      // 1. Download the file from mod.download_url
      // 2. Determine the correct folder based on mod.type (mods or plugins)
      // 3. Save the file to the appropriate server folder
      
      try {
        this.installLoading = true;
        
        // First, get the download URL if not provided
        let downloadUrl = mod.download_url;
        let version = mod.version;
        
        if (!downloadUrl && mod.id && mod.source === 'Modrinth') {
          console.log(`Fetching download URL for mod ${mod.name} (${mod.id})`);
          const downloadInfo = await this.getModDownloadUrl(mod.id);
          
          if (downloadInfo) {
            downloadUrl = downloadInfo.url;
            version = downloadInfo.version;
          } else {
            throw new Error('Could not find a download URL for this mod');
          }
        }
        
        if (!downloadUrl) {
          throw new Error('No download URL available for this mod');
        }
        
        // Log what we're about to do
        console.log(`Downloading ${mod.name} from ${downloadUrl}`);
        console.log(`Installing to server ${this.targetServerId}, folder: ${this.targetFolder}`);
        
        // In a real app, we would use Tauri commands to download and install
        /*
        await window.__TAURI__.invoke('download_and_install_mod', {
          url: downloadUrl,
          serverId: this.targetServerId,
          folder: this.targetFolder,
          filename: `${mod.name.replace(/\s/g, '_')}-${version}.jar`
        });
        */
        
        // Simulate a successful download
        await new Promise(resolve => setTimeout(resolve, 1500));
        
        // Add to installed mods
        const modToInstall = {
          ...mod,
          serverId: this.targetServerId,
          folder: this.targetFolder,
          version: version,
          download_url: downloadUrl
        };
        
        this.installedMods.push(modToInstall);
        
        // Remove from available mods
        this.availableMods = this.availableMods.filter(m => m.id !== mod.id);
        
        // Close dialog and update filtered lists
        this.serverSelectionDialog = false;
        this.pendingModInstall = null;
        // this.filterMods(); // This is now handled by computed properties
        
        // Show notification
        alert(`Mod ${mod.name} installed successfully to ${this.servers.find(s => s.id === this.targetServerId).name}`);
        
      } catch (error) {
        console.error('Failed to download and install mod:', error);
        alert(`Failed to install ${mod.name}: ${error.message}`);
      } finally {
        this.installLoading = false;
      }
    },
    
    uninstallMod(mod) {
      // In a real app, this would communicate with Tauri to uninstall the mod
      console.log('Uninstall mod', mod.name, 'from server', mod.serverId);
      
      // Remove from installed mods
      const index = this.installedMods.findIndex(m => 
        m.name === mod.name && 
        m.version === mod.version && 
        m.serverId === mod.serverId &&
        m.folder === mod.folder // Also check folder
      );
      
      if (index !== -1) {
        this.installedMods.splice(index, 1);
        
        // Add back to available mods
        const availableMod = {
          name: mod.name,
          version: mod.version,
          type: mod.type,
          description: mod.description,
          image: mod.image,
          source: mod.source
        };
        
        // Only add if it doesn't already exist
        if (!this.availableMods.some(m => m.name === availableMod.name && m.version === availableMod.version)) {
          this.availableMods.push(availableMod);
        }
        
        // this.filterMods(); // This is now handled by computed properties
        
        // Show notification (would be implemented in a real app)
        alert(`Mod ${mod.name} uninstalled successfully from ${this.servers.find(s => s.id === mod.serverId).name}`);
      }
    },
    
    checkForUpdates(mod) {
      // In a real app, this would check for updates from the mod source
      console.log('Check for updates', mod.name);
      
      // Simulate update check
      alert(`No updates available for ${mod.name}`);
    },
    
    importLocalMod() {
      this.localModFile = null;
      this.localModTargetServer = this.selectedServer; // Default to currently selected server
      this.localModTargetFolder = 'mods'; // Default to mods folder
      this.importLocalDialog = true;
    },
    
    confirmLocalModImport() {
      if (!this.localModFile || !this.localModTargetServer) return;
      
      // In a real app, this would process the file and extract mod metadata
      const fileName = this.localModFile.name;
      
      // Create a new mod entry from the file (in a real app, metadata would be extracted)
      const newMod = {
        name: fileName.replace('.jar', ''),
        version: '1.0.0', // In a real app, this would be extracted from the file
        type: 'Mod', // In a real app, this would be determined from the file
        description: 'Imported local mod',
        image: '',
        source: 'Local',
        serverId: this.localModTargetServer,
        folder: this.localModTargetFolder // Add folder to the mod object
      };
      
      // Add to installed mods
      this.installedMods.push(newMod);
      
      // Close dialog and update filtered lists
      this.importLocalDialog = false;
      
      // Show notification (would be implemented in a real app)
      alert(`Local mod ${newMod.name} imported successfully to ${this.servers.find(s => s.id === newMod.serverId).name}`);
    },
    
    searchRepository() {
      // In a real app, this would send a request to the API with filters
      console.log('Searching repository with:', {
        query: this.repositorySearch,
        source: this.repositorySource,
        category: this.repositoryCategory,
        gameVersion: this.repositoryGameVersion
      });
      
      // Simulate filtered results based on our sample data
      // In a real app, this would be handled by the API
      const query = this.repositorySearch.toLowerCase();
      this.repositoryMods = this.repositoryMods.filter(mod => {
        // Filter by source
        if (this.repositorySource !== 'All Sources' && mod.source !== this.repositorySource) {
          return false;
        }
        
        // Filter by category
        if (this.repositoryCategory !== 'All' && mod.category !== this.repositoryCategory) {
          return false;
        }
        
        // Filter by game version
        if (!mod.gameVersions.includes(this.repositoryGameVersion)) {
          return false;
        }
        
        // Filter by search query
        if (query && !mod.name.toLowerCase().includes(query) && !mod.description.toLowerCase().includes(query)) {
          return false;
        }
        
        return true;
      });
    },
    
    installRepoMod(mod) {
      this.pendingModInstall = mod;
      this.targetServerId = this.selectedServer;
      this.targetFolder = 'mods';
      this.serverSelectionDialog = true;
    },
    
    getSourceColor(source) {
      const colors = {
        'Modrinth': 'success',
        'CurseForge': 'warning',
        'Local': 'info'
      }
      return colors[source] || 'primary'
    },

    async fetchModrinthMods() {
      const query = encodeURIComponent(this.searchQuery || 'mod');
      // Add proper facets for Minecraft mods and version filtering
      const gameVersion = encodeURIComponent(this.repositoryGameVersion || '1.20.1');
      const limit = 100; // Increase the limit to fetch more mods
      const url = `https://api.modrinth.com/v2/search?query=${query}&limit=${limit}&facets=[["project_type:mod"],["versions:${gameVersion}"]]`;
      
      console.log("Fetching from Modrinth URL:", url);
      
      try {
        const response = await fetch(url);
        const data = await response.json();
        
        console.log("Modrinth API response:", data);
        
        if (!data.hits || data.hits.length === 0) {
          console.log("No hits found in Modrinth response");
          return [];
        }
        
        const mods = data.hits.map(mod => ({
          id: mod.project_id,
          name: mod.title,
          version: mod.latest_version || 'Unknown',
          type: 'Mod',
          description: mod.description || 'No description available',
          image: mod.icon_url || '',
          source: 'Modrinth',
          download_url: '', // Will be fetched when installing
          category: mod.categories?.[0] || 'Utility',
          downloads: mod.downloads || 0,
          gameVersions: mod.game_versions || ['1.20.1'],
          author: mod.author || 'Unknown',
          followers: mod.followers || 0
        }));
        
        console.log("Processed Modrinth mods:", mods);
        return mods;
      } catch (error) {
        console.error("Error fetching from Modrinth:", error);
        return [];
      }
    },
    
    async fetchRandomMods() {
      const count = 70; // Get 70 random mods
      const url = `https://api.modrinth.com/v2/projects_random?count=${count}`;
      
      console.log("Fetching random mods from Modrinth");
      
      try {
        const response = await fetch(url);
        const data = await response.json();
        
        console.log("Random mods response:", data);
        
        if (!data || !Array.isArray(data) || data.length === 0) {
          console.log("No random mods found");
          return [];
        }
        
        const mods = data.map(mod => ({
          id: mod.id,
          name: mod.title,
          version: mod.versions?.[0] || 'Unknown',
          type: 'Mod',
          description: mod.description || 'No description available',
          image: mod.icon_url || '',
          source: 'Modrinth',
          download_url: '', // Will be fetched when installing
          category: mod.categories?.[0] || 'Utility',
          downloads: mod.downloads || 0,
          gameVersions: mod.game_versions || ['1.20.1'],
          author: mod.author || 'Unknown',
          followers: mod.followers || 0
        }));
        
        console.log("Processed random mods:", mods);
        return mods;
      } catch (error) {
        console.error("Error fetching random mods:", error);
        return [];
      }
    },
    
    async getModDownloadUrl(modId) {
      // Get the latest version of the mod that matches our game version
      const url = `https://api.modrinth.com/v2/project/${modId}/version`;
      
      try {
        const response = await fetch(url);
        const versions = await response.json();
        
        if (!Array.isArray(versions)) {
          console.error("Invalid response from version API:", versions);
          return null;
        }
        
        // Find a version that matches our game version
        const gameVersion = this.repositoryGameVersion || '1.20.1';
        const matchingVersion = versions.find(v => 
          v.game_versions?.includes(gameVersion)
        );
        
        if (matchingVersion && matchingVersion.files && matchingVersion.files.length > 0) {
          // Return the primary file URL
          const primaryFile = matchingVersion.files.find(f => f.primary) || matchingVersion.files[0];
          return {
            url: primaryFile.url,
            filename: primaryFile.filename,
            version: matchingVersion.version_number
          };
        }
        
        return null;
      } catch (error) {
        console.error("Error getting mod download URL:", error);
        return null;
      }
    },

    async loadAvailableMods() {
      try {
        this.isLoading = true;
        console.log("Starting to load available mods");
        
        let modrinthMods = [];
        let randomMods = [];
        
        // If there's a search query, use search API
        if (this.searchQuery) {
          modrinthMods = await this.fetchModrinthMods();
          console.log(`Fetched ${modrinthMods.length} mods from Modrinth search`);
        } else {
          // Otherwise fetch random mods
          randomMods = await this.fetchRandomMods();
          console.log(`Fetched ${randomMods.length} random mods from Modrinth`);
          modrinthMods = randomMods;
        }
        
        // Update available mods with API data
        this.availableMods = [...modrinthMods];
        console.log("Updated availableMods array:", this.availableMods.length, "mods");
        console.log("First few mods:", this.availableMods.slice(0, 3));
        
        // Switch to Available tab
        this.contentTab = 'available';
        
        // If no mods found, show an alert
        if (modrinthMods.length === 0) {
          console.log("No mods found from API");
          alert('No mods found matching your search criteria. Try a different search term or game version.');
        } else {
          console.log("Successfully loaded mods, should be visible now");
        }
      } catch (error) {
        console.error('Error loading mods:', error);
        alert('Failed to load mods from the repository. Please try again later.');
      } finally {
        this.isLoading = false;
      }
    },

    showModDetails(mod) {
      this.selectedMod = mod;
      this.showModDetailsDialog = true;
    },

    showContextMenu(event, mod) {
      this.contextMenu.show = true;
      this.contextMenu.x = event.clientX;
      this.contextMenu.y = event.clientY;
      this.contextMenu.mod = mod;
    },

    hideContextMenu() {
      this.contextMenu.show = false;
      this.contextMenu.mod = null;
    },

    installModContextMenu() {
      this.installMod(this.contextMenu.mod);
      this.hideContextMenu();
    },

    viewDetailsContextMenu() {
      this.showModDetails(this.contextMenu.mod);
      this.hideContextMenu();
    },

    viewModWebsiteContextMenu() {
      if (this.contextMenu.mod?.source === 'Modrinth') {
        window.open(`https://modrinth.com/mod/${this.contextMenu.mod.id}`, '_blank');
      } else if (this.contextMenu.mod?.source === 'CurseForge') {
        window.open(`https://www.curseforge.com/minecraft/mc-mods/${this.contextMenu.mod.id}`, '_blank');
      }
      this.hideContextMenu();
    },

    getServerFolderOptions() {
      // This function is not directly used in the template anymore,
      // but it's kept for potential future use or if the dialog needs it.
      // For now, it just returns a default option.
      return ['mods', 'plugins'];
    },

    // New methods for modpack-specific functionality
    getModpackColor(name) {
      const colors = {
        'Sodium': 'success',
        'Lithium': 'warning',
        'Starlight': 'info',
        'Create': 'primary',
        'JEI (Just Enough Items)': 'secondary'
      };
      return colors[name] || 'grey';
    },

    getModpackInitials(name) {
      return name.split(' ').map(word => word[0]).join('').substring(0, 2).toUpperCase();
    },

    getModpackTags(mod) {
      const tags = [];
      if (mod.category) {
        tags.push(mod.category);
      }
      if (mod.gameVersions && mod.gameVersions.length > 0) {
        tags.push(`v${mod.gameVersions[0]}`);
      }
      return tags;
    },

    formatDownloads(downloads) {
      if (downloads === null || downloads === undefined) return '0';
      return downloads.toLocaleString();
    },

    formatFollowers(followers) {
      if (followers === null || followers === undefined) return '0';
      return followers.toLocaleString();
    },

    getServerName(serverId) {
      const server = this.servers.find(s => s.id === serverId);
      return server ? server.name : 'Unknown Server';
    },

    onSearchInput() {
      // Clear any existing timeout
      if (this.searchTimeout) {
        clearTimeout(this.searchTimeout);
      }
      
      // Set a new timeout to debounce the search
      this.searchTimeout = setTimeout(() => {
        this.loadAvailableMods();
      }, 500); // Wait 500ms after user stops typing
    }
  }
}
</script>

<style scoped>
.mods-view {
  padding: 16px;
  position: relative;
}

/* Navigation Tabs */
.nav-tabs-toggle {
  background-color: rgba(30, 30, 30, 0.8);
  border-radius: 50px !important;
  overflow: hidden;
}
.nav-tabs-toggle .v-btn {
  text-transform: none;
  font-weight: 500;
  color: white;
}
.nav-tabs-toggle .v-btn--active {
  background-color: #4ade80 !important;
  color: #121212 !important;
}

/* Search and Filter Bar */
.search-filter-bar {
  background-color: rgba(30, 30, 30, 0.8);
  border-radius: 8px;
  padding: 16px;
}
.search-field {
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  flex-grow: 1;
  max-width: 400px;
}
.search-field :deep(.v-field__field) {
  height: 100%;
}
.filter-btn {
  background-color: rgba(255, 255, 255, 0.1);
  text-transform: none;
  border: 1px solid rgba(255, 255, 255, 0.1);
  min-width: 150px;
  padding: 0 16px;
}
.pagination-controls .v-btn {
  color: white;
  min-width: 32px;
}
.pagination-controls .v-btn--active {
  background-color: #4ade80 !important;
  color: #121212 !important;
}

/* Content Tabs */
.content-tabs-toggle {
  background-color: rgba(30, 30, 30, 0.8);
  border-radius: 50px !important;
  overflow: hidden;
}
.content-tabs-toggle .v-btn {
  text-transform: none;
  font-weight: 500;
  color: white;
}
.content-tabs-toggle .v-btn--active {
  background-color: #4ade80 !important;
  color: #121212 !important;
}

/* Modpack List */
.modpack-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.modpack-item {
  background-color: #1e1e1e;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid rgba(74, 222, 128, 0.05);
  transition: all 0.3s ease;
  position: relative;
}

.modpack-item::before {
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

.modpack-item:hover {
  border-color: rgba(74, 222, 128, 0.2);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  transform: translateY(-2px);
}

.modpack-item:hover::before {
  opacity: 1;
  background: linear-gradient(135deg, 
    rgba(74, 222, 128, 0.15) 0%, 
    rgba(74, 222, 128, 0.05) 30%, 
    rgba(74, 222, 128, 0) 60%);
}

.modpack-card {
  display: flex;
  align-items: center;
  padding: 16px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  position: relative;
  z-index: 10;
  pointer-events: auto;
}

.modpack-card:hover {
  background-color: rgba(255, 255, 255, 0.02);
}

/* Modpack Icon */
.modpack-icon {
  flex-shrink: 0;
  margin-right: 16px;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modpack-avatar {
  border: 2px solid rgba(255, 255, 255, 0.1);
  transition: opacity 0.2s ease;
}

.modpack-initials {
  font-size: 18px;
  font-weight: bold;
  color: white;
}

.modpack-initials-large {
  font-size: 24px;
  font-weight: bold;
  color: white;
}

/* Install button overlay on hover */
.install-overlay {
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

.modpack-icon:hover .modpack-avatar {
  opacity: 0.3;
}

.modpack-icon:hover .install-overlay {
  opacity: 1;
}

/* Modpack Details */
.modpack-details {
  flex-grow: 1;
  margin-right: 16px;
}

.modpack-title {
  font-size: 18px;
  font-weight: bold;
  color: white;
  margin-bottom: 4px;
}

.modpack-author {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 8px;
}

.modpack-description {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.8);
  margin-bottom: 8px;
  line-height: 1.4;
}

.modpack-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

/* Modpack Stats */
.modpack-stats {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 8px;
}

.stat-item {
  display: flex;
  align-items: center;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.7);
}

.stat-text {
  margin-left: 4px;
}

.install-btn {
  background-color: #4ade80 !important;
  color: #121212 !important;
  font-weight: 600;
  min-width: 100px;
}

.uninstall-btn {
  border-color: #ef4444 !important;
  color: #ef4444 !important;
  min-width: 100px;
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

/* Responsive Design */
@media (max-width: 768px) {
  .modpack-card {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .modpack-icon {
    margin-right: 0;
    margin-bottom: 12px;
  }
  
  .modpack-details {
    margin-right: 0;
    margin-bottom: 12px;
    width: 100%;
  }
  
  .modpack-stats {
    width: 100%;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }
  
  .search-filter-bar .d-flex {
    flex-direction: column;
    gap: 12px;
  }
  
  .search-field {
    max-width: none;
  }
}
</style> 