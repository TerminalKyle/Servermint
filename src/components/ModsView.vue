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
              <v-btn variant="outlined" color="secondary" v-bind="props" class="filter-btn mr-3" rounded="lg">
                <span class="mr-1 text-white">Version: {{ selectedVersion }}</span>
                <v-icon size="small" color="white">mdi-chevron-down</v-icon>
              </v-btn>
            </template>
            <v-list density="compact" bg-color="surface" rounded="lg">
              <v-list-item 
                v-for="version in gameVersions" 
                :key="version"
                @click="selectVersion(version)"
                :active="version === selectedVersion"
                class="version-item"
              >
                <v-list-item-title>{{ version }}</v-list-item-title>
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

    <!-- Action Buttons -->
    <div class="d-flex justify-end mb-4 gap-2">
      <v-btn 
        color="secondary" 
        variant="flat" 
        prepend-icon="mdi-package-variant"
        @click="showModpackInstaller = true"
      >
        Install Modpack
      </v-btn>
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
                  <v-chip size="x-small" variant="flat" color="success" class="mr-1 mb-1">Installed</v-chip>
                  <v-chip size="x-small" variant="flat" color="grey-darken-3" class="mr-1 mb-1">{{ mod.source }}</v-chip>
                  <v-chip v-if="mod.loaders && mod.loaders.length > 0" size="x-small" variant="flat" color="info" class="mr-1 mb-1">
                    {{ mod.loaders.join(', ') }}
                  </v-chip>
                  <v-chip v-if="mod.game_versions && mod.game_versions.length > 0" size="x-small" variant="flat" color="warning" class="mr-1 mb-1">
                    {{ mod.game_versions[0] }}
                  </v-chip>
                  <v-chip v-if="mod.type" size="x-small" variant="flat" color="primary" class="mr-1 mb-1">
                    {{ mod.type }}
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
        <v-card-title class="text-h6 pa-4">Install {{ pendingModInstall?.name }}</v-card-title>
        
        <v-card-text class="pa-4">
          <div class="mb-6">
            <div class="text-subtitle-1 mb-2">Select Server</div>
            <v-select
              v-model="targetServerId"
              :items="servers"
              item-title="name"
              item-value="id"
              variant="outlined"
              density="comfortable"
              placeholder="Choose a server"
              bg-color="#2A2A2A"
              class="mb-4"
              :error-messages="!targetServerId ? 'Please select a server' : ''"
            ></v-select>

            <div class="text-subtitle-1 mb-2">Select Version</div>
            <v-select
              v-model="selectedModVersion"
              :items="availableModVersions"
              variant="outlined"
              density="comfortable"
              placeholder="Choose a version"
              bg-color="#2A2A2A"
              class="mb-4"
              :error-messages="!selectedModVersion ? 'Please select a version' : ''"
              :loading="loadingVersions"
              :disabled="loadingVersions"
            ></v-select>

          <div class="text-subtitle-1 mb-2">Installation Location</div>
            <v-text-field
              v-model="targetFolder"
              variant="outlined"
              density="comfortable"
              placeholder="Enter folder path (e.g., mods, plugins)"
              bg-color="#2A2A2A"
              class="mb-2"
              :error-messages="!targetFolder ? 'Please specify installation location' : ''"
              hint="Common folders: mods, plugins, resourcepacks"
              persistent-hint
            ></v-text-field>

            <div class="text-caption mt-2 mb-4">
              <v-icon size="small" color="info" class="mr-1">mdi-information</v-icon>
              Folder will be created if it doesn't exist
            </div>

            <v-expand-transition>
              <div v-if="targetServerId && targetFolder" class="installation-details pa-3 rounded bg-surface">
                <div class="text-caption mb-2">Installation Summary:</div>
                <div class="d-flex align-center mb-1">
                  <v-icon size="small" color="white" class="mr-2">mdi-server</v-icon>
                  <span>Server: {{ getServerName(targetServerId) }}</span>
                </div>
                <div class="d-flex align-center mb-1">
                  <v-icon size="small" color="white" class="mr-2">mdi-folder</v-icon>
                  <span>Location: {{ targetFolder }}</span>
                </div>
                <div class="d-flex align-center">
                  <v-icon size="small" color="white" class="mr-2">mdi-package-variant</v-icon>
                  <span>Mod: {{ pendingModInstall?.name }} ({{ pendingModInstall?.version }})</span>
                </div>
              </div>
            </v-expand-transition>
          </div>
        </v-card-text>

        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn 
            color="error" 
            variant="text" 
            @click="serverSelectionDialog = false"
            :disabled="installLoading"
          >
            Cancel
          </v-btn>
          <v-btn
            color="primary"
            @click="confirmModInstall"
            :loading="installLoading"
            :disabled="!targetServerId || !targetFolder"
          >
            <v-icon class="mr-2">mdi-download</v-icon>
            Install
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

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
    
    <v-dialog v-model="showModpackInstaller" max-width="900px" :persistent="modpackInstalling">
      <ModpackInstaller 
        @close="showModpackInstaller = false" 
        @install-start="modpackInstalling = true"
        @install-end="modpackInstalling = false"
      />
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
import { store } from '../store.js'
import { invoke } from '@tauri-apps/api/core'
import ModpackInstaller from './ModpackInstaller.vue'

export default {
  name: 'ModsView',
  components: {
    ModpackInstaller
  },
  data() {
    return {
      activeTab: 'modpacks', 
      contentTab: 'available', 
      sortBy: 'Downloads',
      viewCount: 20,
      currentPage: 1,
      selectedServer: null,
      selectedType: 'All',
      selectedSource: 'All',
      selectedVersion: '1.20.1', 
      searchQuery: '',
      searchTimeout: null, 
      servers: [], 
      // Original data store for installed mods
      installedMods: [],
      availableMods: [], 
      
      serverSelectionDialog: false,
      pendingModInstall: null,
      targetServerId: null,
      targetFolder: null, 
      installLoading: false, 
      
      selectedModVersion: null,
      availableModVersions: [],
      loadingVersions: false,
      
      importLocalDialog: false,
      localModFile: null,
      localModTargetServer: null,
      localModTargetFolder: null, 
      
      showModDetailsDialog: false,
      selectedMod: null,
      
      contextMenu: {
        show: false,
        x: 0,
        y: 0,
        mod: null
      },
      
      showModpackInstaller: false,
      modpackInstalling: false,
      
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
        '1.21.1',
        '1.20',
        '1.19.4',
        '1.19.3',
        '1.19.2',
        '1.19.1',
        '1.19',
        '1.18.2',
        '1.18.1',
        '1.18',
        '1.17.1',
        '1.17',
        '1.16.5'
      ],
      
      isLoading: false,
      store: store 
    }
  },
  computed: {
    filteredInstalledMods() {
      return this.installedMods.filter(mod => {
        if (mod.serverId !== this.selectedServer) return false;
        
        if (this.selectedType !== 'All' && mod.type !== this.selectedType) return false;
        
        if (this.selectedSource !== 'All' && mod.source !== this.selectedSource) return false;
        
        if (this.searchQuery && !this.matchesSearch(mod, this.searchQuery)) return false;
        
        return true;
      });
    },
    
    filteredAvailableMods() {
      return this.availableMods.filter(mod => {
        if (this.selectedType !== 'All' && mod.type !== this.selectedType) return false;
        
        if (this.selectedSource !== 'All' && mod.source !== this.selectedSource) return false;
        
        if (this.searchQuery && !this.matchesSearch(mod, this.searchQuery)) return false;
        
        const alreadyInstalled = this.installedMods.some(installedMod => 
          installedMod.name === mod.name && 
          installedMod.serverId === this.selectedServer
        );
        
        return !alreadyInstalled;
      });
    }
  },
  async created() {
    await this.loadServers();
    
    if (this.selectedServer) {
      this.installedMods = await this.store.getInstalledMods(this.selectedServer);
    }
    
    this.loadAvailableMods();
  },
  
  watch: {
    async selectedServer(newServerId) {
      if (newServerId) {
        this.installedMods = await this.store.getInstalledMods(newServerId);
      } else {
        this.installedMods = [];
      }
    }
  },
  methods: {
    async loadServers() {
      try {
        await this.store.loadServers();
        this.servers = this.store.servers;
        
        if (this.servers.length > 0 && !this.selectedServer) {
          this.selectedServer = this.servers[0].id;
        }
        
        console.log('Loaded servers:', this.servers);
      } catch (error) {
        console.error('Error loading servers:', error);
        if (window.showError) {
          window.showError('Error', 'Failed to load servers. Please try again.');
        }
      }
    },

    // filterMods() { 
    // this.filteredInstalledMods = this.installedMods.filter(mod => {
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
      query = query.toLowerCase();
      
      return mod.name.toLowerCase().includes(query) ||
        mod.description.toLowerCase().includes(query) ||
        mod.type.toLowerCase().includes(query) ||
        mod.version.toLowerCase().includes(query) ||
        mod.source.toLowerCase().includes(query);
    },
    
    async installMod(mod) {
      console.log('=== Starting mod installation flow ===');
      console.log('Selected mod:', mod);
      
      this.pendingModInstall = mod;
      this.targetServerId = this.selectedServer; 
      this.targetFolder = 'mods'; 
      this.selectedModVersion = null; 
      
      await this.fetchModVersions(mod.id);
      
      console.log('Current state:', {
        pendingMod: this.pendingModInstall,
        targetServer: this.targetServerId,
        targetFolder: this.targetFolder,
        servers: this.servers,
        availableVersions: this.availableModVersions
      });
      
      this.serverSelectionDialog = true;
    },

    async fetchModVersions(modId) {
      this.loadingVersions = true;
      this.availableModVersions = [];
      
      try {
        const response = await fetch(`https://api.modrinth.com/v2/project/${modId}/version`);
        if (!response.ok) {
          throw new Error('Failed to fetch version information');
        }
        
        const versions = await response.json();
        
        this.availableModVersions = versions.map(version => ({
          title: `${version.version_number} (${version.game_versions.join(', ')})`,
          value: {
            version: version.version_number,
            url: version.files[0]?.url,
            loaders: version.loaders,
            game_versions: version.game_versions
          }
        }));
        
        const defaultVersion = versions.find(v => 
          v.game_versions.includes(this.selectedVersion)
        );
        
        if (defaultVersion) {
          this.selectedModVersion = {
            version: defaultVersion.version_number,
            url: defaultVersion.files[0]?.url,
            loaders: defaultVersion.loaders,
            game_versions: defaultVersion.game_versions
          };
        }
        
      } catch (error) {
        console.error('Error fetching mod versions:', error);
        if (window.showError) {
          window.showError('Error', 'Failed to fetch mod versions');
        }
      } finally {
        this.loadingVersions = false;
      }
    },
    
    async confirmModInstall() {
      console.log('=== Confirming mod installation ===');
      if (!this.pendingModInstall || !this.targetServerId || !this.selectedModVersion) {
        console.error('Missing required data:', {
          pendingMod: this.pendingModInstall,
          targetServer: this.targetServerId,
          selectedVersion: this.selectedModVersion
        });
        return;
      }

      this.installLoading = true;
      
      const modToInstall = {
        ...this.pendingModInstall,
        version: this.selectedModVersion.version,
        download_url: this.selectedModVersion.url,
        loaders: this.selectedModVersion.loaders,
        game_versions: this.selectedModVersion.game_versions
      };
      
      await this.downloadAndInstallMod(modToInstall);
    },
    
    async downloadAndInstallMod(mod) {
      try {
        this.installLoading = true;
        console.log('=== Starting mod installation process ===');
        console.log('Mod details:', mod);
        
        const server = this.servers.find(s => s.id === this.targetServerId);
        if (!server) {
          throw new Error('Selected server not found');
        }
        console.log('Server found:', server);
        
        let downloadUrl = mod.download_url;
        let version = mod.version;
        
        if (!downloadUrl && mod.id && mod.source === 'Modrinth') {
          console.log(`Fetching download URL for mod ${mod.name} (${mod.id})`);
          const downloadInfo = await this.getModDownloadUrl(mod.id);
          console.log('Download info received:', downloadInfo);
          
          if (downloadInfo) {
            downloadUrl = downloadInfo.url;
            version = downloadInfo.version;
            mod.loaders = downloadInfo.loaders;
            mod.game_versions = downloadInfo.game_versions;
            mod.type = downloadInfo.type;
            console.log('Using download URL:', downloadUrl);
            console.log('Using version:', version);
            console.log('Mod loaders:', mod.loaders);
            console.log('Game versions:', mod.game_versions);
            console.log('Project type:', mod.type);
          } else {
            throw new Error('Could not find a download URL for this mod');
          }
        }
        
        if (!downloadUrl) {
          throw new Error('No download URL available for this mod');
        }

        try {
          console.log('Creating mod folder...');
          const folderResult = await this.store.createModFolder(this.targetServerId, this.targetFolder);
          console.log('Folder creation result:', folderResult);
          
          if (!folderResult.success) {
            throw new Error(folderResult.error || 'Failed to create mod folder');
          }
          console.log(`Created/verified folder: ${this.targetFolder} for server: ${server.name}`);
        } catch (error) {
          console.error('Folder creation error:', error);
          throw new Error(`Failed to create installation folder: ${error.message}`);
        }
        
        console.log('=== Installation parameters ===');
        console.log(`Mod name: ${mod.name}`);
        console.log(`Download URL: ${downloadUrl}`);
        console.log(`Server name: ${server.name}`);
        console.log(`Server ID: ${this.targetServerId}`);
        console.log(`Server path: ${server.path}`);
        console.log(`Target folder: ${this.targetFolder}`);
        
        try {
          const filename = `${mod.name.replace(/[^a-zA-Z0-9-_]/g, '_')}-${version}.jar`;
          const params = {
            url: downloadUrl,
            serverId: this.targetServerId,
            serverPath: server.path,
            folder: this.targetFolder,
            filename: filename
          };
          
          console.log('Calling download_and_install_mod with params:', params);
          
          const result = await invoke('download_and_install_mod', params);
          console.log('Installation successful! File path:', result);
          
          try {
            const fileExists = await invoke('plugin:fs|exists', { 
              path: result 
            });
            console.log('File exists check:', fileExists);
            if (!fileExists) {
              throw new Error('File was not found after installation');
            }
          } catch (error) {
            console.error('File verification error:', error);
            throw new Error(`Failed to verify installed file: ${error.message}`);
          }
          
          const modToInstall = {
            ...mod,
            serverId: this.targetServerId,
            folder: this.targetFolder,
            version: version,
            download_url: downloadUrl,
            path: result,
            loaders: mod.loaders || [],
            game_versions: mod.game_versions || [],
            type: mod.type || 'unknown'
          };
          
          console.log('Adding mod to installed list:', modToInstall);
          
          const addResult = await this.store.addInstalledMod(modToInstall);
          console.log('Add to store result:', addResult);
          
          if (!addResult) {
            throw new Error('Failed to update installed mods list');
          }
          
          this.installedMods = await this.store.getInstalledMods(this.targetServerId);
          this.availableMods = this.availableMods.filter(m => m.id !== mod.id);
          
          console.log('Updated installed mods:', this.installedMods);
          
          this.serverSelectionDialog = false;
          this.pendingModInstall = null;
          
          const successMessage = `${mod.name} has been installed successfully to ${server.name}`;
          console.log('Installation complete:', successMessage);
          
          if (window.showSuccess) {
            window.showSuccess('Mod Installed', successMessage);
          } else {
            alert(successMessage);
          }
          
          console.log('=== Mod installation completed successfully ===');
          
        } catch (error) {
          console.error('Download/install error:', error);
          throw new Error(`Failed to download and install mod: ${error.message || error}`);
        }
      } catch (error) {
        console.error('=== Mod installation failed ===');
        console.error('Error details:', error);
        if (window.showError) {
          window.showError('Installation Failed', error.message);
        } else {
          alert(`Failed to install ${mod.name}: ${error.message}`);
        }
      } finally {
        this.installLoading = false;
      }
    },

    async uninstallMod(mod) {
      try {
        const server = this.servers.find(s => s.id === mod.serverId);
        if (!server) {
          throw new Error('Server not found');
        }

        const modPath = `${server.path}/${mod.folder}/${mod.name.replace(/[^a-zA-Z0-9-_]/g, '_')}-${mod.version}.jar`;
        
        try {
          await invoke('remove_file', { path: modPath });
          console.log('Mod file removed successfully');
        } catch (error) {
          throw new Error(`Failed to remove mod file: ${error.message}`);
        }

        const removeResult = await this.store.removeInstalledMod(mod.serverId, mod.name);
        if (!removeResult) {
          throw new Error('Failed to update installed mods list');
        }

        this.installedMods = await this.store.getInstalledMods(mod.serverId);

        if (mod.source !== 'Local') {
          const availableMod = {
            name: mod.name,
            version: mod.version,
            type: mod.type,
            description: mod.description,
            image: mod.image,
            source: mod.source,
            id: mod.id
          };

          if (!this.availableMods.some(m => m.name === availableMod.name)) {
            this.availableMods.push(availableMod);
          }
        }

        if (window.showSuccess) {
          window.showSuccess(
            'Mod Uninstalled',
            `${mod.name} has been removed from ${server.name}`
          );
        } else {
          alert(`Mod ${mod.name} uninstalled successfully from ${server.name}`);
        }
      } catch (error) {
        console.error('Error uninstalling mod:', error);
        if (window.showError) {
          window.showError('Uninstall Failed', error.message);
        } else {
          alert(`Failed to uninstall ${mod.name}: ${error.message}`);
        }
      }
    },
    
    checkForUpdates(mod) {
      console.log('Check for updates', mod.name);
      
      alert(`No updates available for ${mod.name}`);
    },
    
    importLocalMod() {
      this.localModFile = null;
      this.localModTargetServer = this.selectedServer; 
      this.localModTargetFolder = 'mods'; 
      this.importLocalDialog = true;
    },
    
    confirmLocalModImport() {
      if (!this.localModFile || !this.localModTargetServer) return;
      
      const fileName = this.localModFile.name;
      
      const newMod = {
        name: fileName.replace('.jar', ''),
        version: '1.0.0', 
        type: 'Mod', 
        description: 'Imported local mod',
        image: '',
        source: 'Local',
        serverId: this.localModTargetServer,
        folder: this.localModTargetFolder 
      };
      
      this.installedMods.push(newMod);
      
      this.importLocalDialog = false;
      
      alert(`Local mod ${newMod.name} imported successfully to ${this.getServerName(newMod.serverId)}`);
    },
    
    searchRepository() {
      console.log('Searching repository with:', {
        query: this.repositorySearch,
        source: this.repositorySource,
        category: this.repositoryCategory,
        gameVersion: this.repositoryGameVersion
      });
      
      const query = this.repositorySearch.toLowerCase();
      this.repositoryMods = this.repositoryMods.filter(mod => {
        if (this.repositorySource !== 'All Sources' && mod.source !== this.repositorySource) {
          return false;
        }
        
        if (this.repositoryCategory !== 'All' && mod.category !== this.repositoryCategory) {
          return false;
        }
        
        if (!mod.gameVersions.includes(this.repositoryGameVersion)) {
          return false;
        }
        
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
      const gameVersion = encodeURIComponent('1.20.1');
      const limit = 100; 
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
          slug: mod.slug,  
          name: mod.title,
          version: mod.latest_version || 'Unknown',
          type: 'Mod',
          description: mod.description || 'No description available',
          image: mod.icon_url || '',
          source: 'Modrinth',
          download_url: '', 
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
      const baseUrl = 'https://api.modrinth.com/v2/search';
      
      const params = new URLSearchParams({
        limit: '70',  
        index: this.searchQuery ? 'relevance' : 'downloads',  
        offset: '0',
        facets: JSON.stringify([["project_type:mod"], [`versions:${this.selectedVersion}`]]),
      });

      if (this.searchQuery) {
        params.append('query', this.searchQuery);
      }

      const url = `${baseUrl}?${params.toString()}`;
      console.log("Fetching mods from Modrinth URL:", url);
      
      try {
        const response = await fetch(url);
        const data = await response.json();
        
        console.log("Modrinth API response:", data);
        
        if (!data.hits || !Array.isArray(data.hits) || data.hits.length === 0) {
          console.log("No mods found");
          return [];
        }
        
        const mods = data.hits.map(mod => {
          console.log('Processing mod:', mod);
          return {
            id: mod.project_id,
            slug: mod.slug,
            name: mod.title,
            version: mod.latest_version || 'Unknown',
            type: 'Mod',
            description: mod.description || 'No description available',
            image: mod.icon_url || '',
            source: 'Modrinth',
            download_url: '', 
            category: mod.categories?.[0] || 'Utility',
            downloads: mod.downloads || 0,
            gameVersions: mod.game_versions || [this.selectedVersion],
            author: mod.author || 'Unknown',
            followers: mod.followers || 0
          };
        });
        
        console.log("Processed mods:", mods);
        console.log("Sample mod object:", mods[0]);
        return mods;
      } catch (error) {
        console.error("Error fetching mods:", error);
        return [];
      }
    },

    async loadAvailableMods() {
      try {
        this.isLoading = true;
        console.log("Starting to load available mods");
        
        let modrinthMods = [];
        
        modrinthMods = await this.fetchRandomMods();
        console.log(`Fetched ${modrinthMods.length} mods from Modrinth`);
        
        this.availableMods = [...modrinthMods];
        console.log("Updated availableMods array:", this.availableMods.length, "mods");
        console.log("First few mods:", this.availableMods.slice(0, 3));
        
        this.contentTab = 'available';
        
        if (modrinthMods.length === 0) {
          console.log("No mods found from API");
          if (this.searchQuery) {
            alert('No mods found matching your search criteria. Try a different search term.');
          } else {
            alert('Failed to load mods. Please try again.');
          }
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
      return ['mods', 'plugins'];
    },

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
      if (this.searchTimeout) {
        clearTimeout(this.searchTimeout);
      }
      
      this.searchTimeout = setTimeout(() => {
        this.loadAvailableMods();
      }, 500); 
    },

    selectVersion(version) {
      console.log('Selecting version:', version);
      this.selectedVersion = version;
      this.loadAvailableMods(); 
    },

    async getModDownloadUrl(modId) {
      try {
        const projectResponse = await fetch(`https://api.modrinth.com/v2/project/${modId}`);
        if (!projectResponse.ok) {
          throw new Error('Failed to fetch project information');
        }
        const projectData = await projectResponse.json();


        const versionsResponse = await fetch(`https://api.modrinth.com/v2/project/${modId}/version`);
        if (!versionsResponse.ok) {
          throw new Error('Failed to fetch version information');
        }
        const versions = await versionsResponse.json();

        const latestVersion = versions[0];
        if (!latestVersion) {
          throw new Error('No versions found for this mod');
        }

        const primaryFile = latestVersion.files.find(f => f.primary) || latestVersion.files[0];
        if (!primaryFile) {
          throw new Error('No download file found');
        }

        return {
          url: primaryFile.url,
          version: latestVersion.version_number,
          loaders: latestVersion.loaders,
          game_versions: latestVersion.game_versions,
          type: projectData.project_type
        };
      } catch (error) {
        console.error('Error fetching mod download info:', error);
        return null;
      }
    }
  }
}
</script>

<style scoped>
.mods-view {
  padding: 16px;
  position: relative;
}

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