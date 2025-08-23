<template>
  <div class="premade-servers-view">
    <div class="d-flex align-center mb-6">
      <h2 class="text-h5 font-weight-bold">Premade Servers</h2>
      <v-spacer></v-spacer>
      <v-btn
        color="primary"
        prepend-icon="mdi-refresh"
        variant="outlined"
        @click="refreshServers"
        :loading="isRefreshing"
      >
        Refresh
      </v-btn>
    </div>

    <div class="filter-section mb-4">
      <div class="d-flex align-center">
        <v-btn-toggle v-model="activeTab" color="primary" density="comfortable" mandatory rounded="lg" class="filter-tabs">
          <v-btn value="all" variant="text" class="px-4 py-2">
            <v-icon color="white" class="mr-2">mdi-view-grid</v-icon>All Servers
          </v-btn>
          <v-btn value="survival" variant="text" class="px-4 py-2">
            <v-icon color="white" class="mr-2">mdi-sword-cross</v-icon>Survival
          </v-btn>
          <v-btn value="creative" variant="text" class="px-4 py-2">
            <v-icon color="white" class="mr-2">mdi-palette</v-icon>Creative
          </v-btn>
          <v-btn value="minigames" variant="text" class="px-4 py-2">
            <v-icon color="white" class="mr-2">mdi-gamepad-variant</v-icon>Mini Games
          </v-btn>
          <v-btn value="modded" variant="text" class="px-4 py-2">
            <v-icon color="white" class="mr-2">mdi-puzzle</v-icon>Modded
          </v-btn>
        </v-btn-toggle>
      </div>
    </div>

    <div class="d-flex align-center mb-6 search-filter-container">
      <v-text-field
        v-model="searchQuery"
        prepend-inner-icon="mdi-magnify"
        placeholder="Search premade servers..."
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
            <span class="mr-1 text-white">Sort by: Popularity</span>
            <v-icon size="small" color="white">mdi-chevron-down</v-icon>
          </v-btn>
        </template>
        <v-list density="compact" bg-color="surface" rounded="lg">
          <v-list-item value="popularity">
            <v-list-item-title>Popularity</v-list-item-title>
          </v-list-item>
          <v-list-item value="name">
            <v-list-item-title>Name</v-list-item-title>
          </v-list-item>
          <v-list-item value="version">
            <v-list-item-title>Version</v-list-item-title>
          </v-list-item>
          <v-list-item value="rating">
            <v-list-item-title>Rating</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
      
      <v-btn
        variant="outlined"
        color="secondary"
        class="filter-btn"
        rounded="lg"
        @click="showAdvancedFilters = !showAdvancedFilters"
      >
        <v-icon size="small" color="white" class="mr-1">mdi-filter-variant</v-icon>
        <span class="text-white">Filters</span>
      </v-btn>
    </div>

    <v-expand-transition>
      <div v-if="showAdvancedFilters" class="advanced-filters mb-4">
        <v-card class="pa-4" bg-color="rgba(30, 30, 30, 0.8)">
          <v-row>
            <v-col cols="12" md="3">
              <v-select
                v-model="filters.version"
                label="Minecraft Version"
                :items="minecraftVersions"
                variant="outlined"
                density="comfortable"
                bg-color="rgba(255, 255, 255, 0.1)"
                clearable
              ></v-select>
            </v-col>
            <v-col cols="12" md="3">
              <v-select
                v-model="filters.serverType"
                label="Server Type"
                :items="serverTypes"
                variant="outlined"
                density="comfortable"
                bg-color="rgba(255, 255, 255, 0.1)"
                clearable
              ></v-select>
            </v-col>
            <v-col cols="12" md="3">
              <v-select
                v-model="filters.rating"
                label="Minimum Rating"
                :items="ratingOptions"
                variant="outlined"
                density="comfortable"
                bg-color="rgba(255, 255, 255, 0.1)"
                clearable
              ></v-select>
            </v-col>
            <v-col cols="12" md="3">
              <v-select
                v-model="filters.downloads"
                label="Downloads"
                :items="downloadOptions"
                variant="outlined"
                density="comfortable"
                bg-color="rgba(255, 255, 255, 0.1)"
                clearable
              ></v-select>
            </v-col>
          </v-row>
        </v-card>
      </div>
    </v-expand-transition>

    <v-row>
      <v-col 
        v-for="server in filteredServers" 
        :key="server.id" 
        cols="12" 
        sm="6" 
        md="4" 
        lg="3"
      >
        <v-card class="server-card" @click="viewServerDetails(server)">
          <div class="server-image-container">
            <v-img
              :src="server.image || '/servermint.png'"
              :alt="server.name"
              height="200"
              cover
              class="server-image"
            >
              <template v-slot:placeholder>
                <div class="d-flex align-center justify-center fill-height">
                  <v-icon size="64" color="grey">mdi-server</v-icon>
                </div>
              </template>
            </v-img>
            
            <v-chip
              size="small"
              :color="getServerTypeColor(server.type)"
              class="server-type-badge"
              variant="flat"
            >
              {{ server.type }}
            </v-chip>
            
            <div class="rating-badge">
              <v-icon size="small" color="amber">mdi-star</v-icon>
              <span class="rating-text">{{ server.rating }}</span>
            </div>
          </div>
          
          <v-card-item>
            <v-card-title class="text-h6 font-weight-bold">{{ server.name }}</v-card-title>
            <v-card-subtitle class="text-caption">
              <div class="d-flex align-center mb-1">
                <v-icon size="small" class="mr-1">mdi-minecraft</v-icon>
                <span>{{ server.version }}</span>
              </div>
              <div class="d-flex align-center">
                <v-icon size="small" class="mr-1">mdi-download</v-icon>
                <span>{{ formatDownloads(server.downloads) }} downloads</span>
              </div>
            </v-card-subtitle>
          </v-card-item>
          
          <v-card-text class="pt-0">
            <p class="text-caption text-truncate">{{ server.description }}</p>
            
            <div class="server-tags mt-2">
              <v-chip
                v-for="tag in server.tags.slice(0, 3)"
                :key="tag"
                size="x-small"
                variant="outlined"
                color="primary"
                class="mr-1 mb-1"
              >
                {{ tag }}
              </v-chip>
              <v-chip
                v-if="server.tags.length > 3"
                size="x-small"
                variant="outlined"
                color="grey"
                class="mr-1 mb-1"
              >
                +{{ server.tags.length - 3 }}
              </v-chip>
            </div>
          </v-card-text>
          
          <v-card-actions>
            <v-btn
              color="primary"
              variant="flat"
              size="small"
              @click.stop="downloadServer(server)"
              :loading="server.downloading"
              prepend-icon="mdi-download"
            >
              Download
            </v-btn>
            <v-spacer></v-spacer>
            <v-btn
              variant="text"
              size="small"
              @click.stop="viewServerDetails(server)"
              prepend-icon="mdi-information"
            >
              Details
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-row v-if="filteredServers.length === 0">
      <v-col cols="12" class="text-center">
        <v-icon size="64" color="grey" class="mb-4">mdi-server-off</v-icon>
        <h3 class="text-h6 mb-2">No servers found</h3>
        <p class="text-caption text-medium-emphasis">
          Try adjusting your search or filters to find more servers.
        </p>
      </v-col>
    </v-row>

    <v-dialog v-model="showServerDialog" max-width="600">
      <v-card v-if="selectedServer">
        <v-img
          :src="selectedServer.image || '/servermint.png'"
          height="200"
          cover
        ></v-img>
        
        <v-card-item>
          <v-card-title class="text-h5">{{ selectedServer.name }}</v-card-title>
          <v-card-subtitle>
            <div class="d-flex align-center mb-2">
              <v-icon size="small" class="mr-1">mdi-minecraft</v-icon>
              <span>{{ selectedServer.version }}</span>
              <v-chip size="small" :color="getServerTypeColor(selectedServer.type)" class="ml-2">
                {{ selectedServer.type }}
              </v-chip>
            </div>
            <div class="d-flex align-center">
              <v-icon size="small" class="mr-1">mdi-star</v-icon>
              <span>{{ selectedServer.rating }} ({{ selectedServer.reviews }} reviews)</span>
            </div>
          </v-card-subtitle>
        </v-card-item>
        
        <v-card-text>
          <p>{{ selectedServer.description }}</p>
          
          <h4 class="text-subtitle-1 font-weight-bold mt-4 mb-2">Features</h4>
          <ul class="text-caption">
            <li v-for="feature in selectedServer.features" :key="feature">{{ feature }}</li>
          </ul>
          
          <h4 class="text-subtitle-1 font-weight-bold mt-4 mb-2">Requirements</h4>
          <div class="d-flex align-center mb-1">
            <v-icon size="small" class="mr-1">mdi-memory</v-icon>
            <span class="text-caption">RAM: {{ selectedServer.requirements.ram }}GB</span>
          </div>
          <div class="d-flex align-center mb-1">
            <v-icon size="small" class="mr-1">mdi-cpu-64-bit</v-icon>
            <span class="text-caption">CPU: {{ selectedServer.requirements.cpu }}</span>
          </div>
          <div class="d-flex align-center">
            <v-icon size="small" class="mr-1">mdi-harddisk</v-icon>
            <span class="text-caption">Storage: {{ selectedServer.requirements.storage }}GB</span>
          </div>
        </v-card-text>
        
        <v-card-actions>
          <v-btn
            color="primary"
            variant="flat"
            @click="downloadServer(selectedServer)"
            :loading="selectedServer.downloading"
            prepend-icon="mdi-download"
          >
            Download Server
          </v-btn>
          <v-spacer></v-spacer>
          <v-btn variant="outlined" @click="showServerDialog = false">
            Close
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script>
export default {
  name: 'PremadeServersView',
  data() {
    return {
      activeTab: 'all',
      searchQuery: '',
      showAdvancedFilters: false,
      showServerDialog: false,
      selectedServer: null,
      isRefreshing: false,
      filters: {
        version: null,
        serverType: null,
        rating: null,
        downloads: null
      },
      minecraftVersions: ['1.21.2', '1.20.4', '1.20.1', '1.19.4', '1.19.2', '1.18.2', '1.16.5'],
      serverTypes: ['Vanilla', 'Paper', 'Spigot', 'Forge', 'Fabric', 'Neoforge'],
      ratingOptions: ['4.5+', '4.0+', '3.5+', '3.0+'],
      downloadOptions: ['10K+', '50K+', '100K+', '500K+', '1M+'],   
      premadeServers: [
        {
          id: 1,
          name: 'Survival Paradise',
          version: '1.20.4',
          type: 'Paper',
          description: 'A complete survival server setup with economy, protection, and essential plugins.',
          image: '/servermint.png',
          rating: 4.8,
          reviews: 1247,
          downloads: 15420,
          tags: ['Survival', 'Economy', 'Protection', 'Essentials'],
          features: [
            'Economy system with shops and trading',
            'Land protection with WorldGuard',
            'EssentialsX for basic commands',
            'Auto-backup system',
            'Anti-grief protection'
          ],
          requirements: {
            ram: 4,
            cpu: 'Dual-core 2.0GHz',
            storage: 10
          },
          downloading: false
        },
        {
          id: 2,
          name: 'Creative Builders',
          version: '1.20.4',
          type: 'Paper',
          description: 'Perfect for creative building with WorldEdit, VoxelSniper, and building tools.',
          image: '/servermint.png',
          rating: 4.6,
          reviews: 892,
          downloads: 8765,
          tags: ['Creative', 'Building', 'WorldEdit', 'VoxelSniper'],
          features: [
            'WorldEdit for advanced building',
            'VoxelSniper for terrain editing',
            'Plot system for organized building',
            'Creative mode tools',
            'Building competitions support'
          ],
          requirements: {
            ram: 6,
            cpu: 'Quad-core 2.5GHz',
            storage: 15
          },
          downloading: false
        },
        {
          id: 3,
          name: 'SkyBlock Adventure',
          version: '1.20.4',
          type: 'Paper',
          description: 'Complete SkyBlock server with challenges, progression, and custom plugins.',
          image: '/servermint.png',
          rating: 4.9,
          reviews: 2156,
          downloads: 23410,
          tags: ['SkyBlock', 'Challenges', 'Progression', 'Custom'],
          features: [
            'Custom SkyBlock challenges',
            'Progression system',
            'Island management',
            'Custom recipes and items',
            'Leaderboards and achievements'
          ],
          requirements: {
            ram: 8,
            cpu: 'Quad-core 3.0GHz',
            storage: 20
          },
          downloading: false
        },
        {
          id: 4,
          name: 'Modded Adventure',
          version: '1.19.4',
          type: 'Forge',
          description: 'Adventure-focused modpack with exploration, magic, and technology mods.',
          image: '/servermint.png',
          rating: 4.7,
          reviews: 1567,
          downloads: 12340,
          tags: ['Modded', 'Adventure', 'Magic', 'Technology'],
          features: [
            'Curse of Binding modpack',
            'Magic mods (Thaumcraft, Botania)',
            'Technology mods (Thermal Expansion, Mekanism)',
            'Adventure mods (Twilight Forest)',
            'Custom quest system'
          ],
          requirements: {
            ram: 12,
            cpu: 'Quad-core 3.5GHz',
            storage: 30
          },
          downloading: false
        },
        {
          id: 5,
          name: 'Mini Games Hub',
          version: '1.20.4',
          type: 'Paper',
          description: 'Multi-game server with BedWars, SkyWars, and other popular mini-games.',
          image: '/servermint.png',
          rating: 4.5,
          reviews: 987,
          downloads: 6540,
          tags: ['Mini Games', 'BedWars', 'SkyWars', 'PvP'],
          features: [
            'BedWars with custom maps',
            'SkyWars with kits',
            'TNT Run and Parkour',
            'Lobby system',
            'Statistics tracking'
          ],
          requirements: {
            ram: 10,
            cpu: 'Quad-core 3.0GHz',
            storage: 25
          },
          downloading: false
        },
        {
          id: 6,
          name: 'RPG Adventure',
          version: '1.20.4',
          type: 'Paper',
          description: 'RPG server with classes, skills, dungeons, and custom quests.',
          image: '/servermint.png',
          rating: 4.4,
          reviews: 743,
          downloads: 5430,
          tags: ['RPG', 'Classes', 'Skills', 'Dungeons'],
          features: [
            'Custom RPG classes and skills',
            'Dungeon system with bosses',
            'Quest system with rewards',
            'Custom items and weapons',
            'Party system for group play'
          ],
          requirements: {
            ram: 8,
            cpu: 'Quad-core 2.8GHz',
            storage: 20
          },
          downloading: false
        },
        {
          id: 7,
          name: 'Vanilla Enhanced',
          version: '1.21.2',
          type: 'Vanilla',
          description: 'Pure vanilla server with quality-of-life improvements and datapacks.',
          image: '/servermint.png',
          rating: 4.3,
          reviews: 456,
          downloads: 3210,
          tags: ['Vanilla', 'Datapacks', 'QoL', 'Simple'],
          features: [
            'Vanilla gameplay with datapacks',
            'Quality of life improvements',
            'Custom crafting recipes',
            'Enhanced mob drops',
            'Simple and lightweight'
          ],
          requirements: {
            ram: 4,
            cpu: 'Dual-core 2.0GHz',
            storage: 8
          },
          downloading: false
        },
        {
          id: 8,
          name: 'Fabric Performance',
          version: '1.20.4',
          type: 'Fabric',
          description: 'High-performance server using Fabric with optimization mods.',
          image: '/servermint.png',
          rating: 4.6,
          reviews: 678,
          downloads: 4320,
          tags: ['Fabric', 'Performance', 'Optimization', 'Lightweight'],
          features: [
            'Fabric with performance mods',
            'Lithium and Phosphor for optimization',
            'Lightweight plugin alternatives',
            'Fast startup and low resource usage',
            'Modern Minecraft features'
          ],
          requirements: {
            ram: 6,
            cpu: 'Quad-core 2.5GHz',
            storage: 12
          },
          downloading: false
        }
      ]
    }
  },
  computed: {
    filteredServers() {
      let filtered = [...this.premadeServers];
      
      if (this.activeTab === 'survival') {
        filtered = filtered.filter(server => 
          server.tags.some(tag => tag.toLowerCase().includes('survival'))
        );
      } else if (this.activeTab === 'creative') {
        filtered = filtered.filter(server => 
          server.tags.some(tag => tag.toLowerCase().includes('creative'))
        );
      } else if (this.activeTab === 'minigames') {
        filtered = filtered.filter(server => 
          server.tags.some(tag => tag.toLowerCase().includes('mini') || tag.toLowerCase().includes('pvp'))
        );
      } else if (this.activeTab === 'modded') {
        filtered = filtered.filter(server => 
          server.tags.some(tag => tag.toLowerCase().includes('modded'))
        );
      }
      
      if (this.searchQuery) {
        const query = this.searchQuery.toLowerCase();
        filtered = filtered.filter(server => 
          server.name.toLowerCase().includes(query) ||
          server.description.toLowerCase().includes(query) ||
          server.tags.some(tag => tag.toLowerCase().includes(query))
        );
      }
      
      if (this.filters.version) {
        filtered = filtered.filter(server => server.version === this.filters.version);
      }
      
      if (this.filters.serverType) {
        filtered = filtered.filter(server => server.type === this.filters.serverType);
      }
      
      if (this.filters.rating) {
        const minRating = parseFloat(this.filters.rating);
        filtered = filtered.filter(server => server.rating >= minRating);
      }
      
      if (this.filters.downloads) {
        const minDownloads = this.parseDownloadFilter(this.filters.downloads);
        filtered = filtered.filter(server => server.downloads >= minDownloads);
      }
      
      return filtered;
    }
  },
  methods: {
    refreshServers() {
      this.isRefreshing = true;
      setTimeout(() => {
        this.isRefreshing = false;
      }, 1000);
    },
    
    getServerTypeColor(type) {
      const colors = {
        'Vanilla': 'grey',
        'Paper': 'blue',
        'Spigot': 'orange',
        'Forge': 'purple',
        'Fabric': 'green',
        'Neoforge': 'deep-purple'
      };
      return colors[type] || 'grey';
    },
    
    formatDownloads(downloads) {
      if (downloads >= 1000000) {
        return (downloads / 1000000).toFixed(1) + 'M';
      } else if (downloads >= 1000) {
        return (downloads / 1000).toFixed(1) + 'K';
      }
      return downloads.toString();
    },
    
    parseDownloadFilter(filter) {
      const multipliers = {
        '10K+': 10000,
        '50K+': 50000,
        '100K+': 100000,
        '500K+': 500000,
        '1M+': 1000000
      };
      return multipliers[filter] || 0;
    },
    
    viewServerDetails(server) {
      this.selectedServer = server;
      this.showServerDialog = true;
    },
    
    async downloadServer(server) {
      server.downloading = true;
      
      try {
        await new Promise(resolve => setTimeout(resolve, 2000));
        
        console.log(`Downloading server: ${server.name}`);
        
        this.$emit('server-downloaded', server);
        
      } catch (error) {
        console.error('Error downloading server:', error);
      } finally {
        server.downloading = false;
      }
    }
  }
}
</script>

<style scoped>
.premade-servers-view {
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
  min-width: 120px;
  padding: 0 16px;
}

.server-card {
  background-color: #1e1e1e;
  border-radius: 12px;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(74, 222, 128, 0.05);
  cursor: pointer;
}

.server-card:hover {
  transform: translateY(-4px);
  border-color: rgba(74, 222, 128, 0.2);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
}

.server-image-container {
  position: relative;
}

.server-image {
  border-radius: 12px 12px 0 0;
}

.server-type-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  font-weight: 600;
}

.rating-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background-color: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 4px 8px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  font-size: 12px;
  font-weight: 600;
}

.rating-text {
  margin-left: 2px;
}

.server-tags {
  display: flex;
  flex-wrap: wrap;
}

.server-tags .v-chip {
  font-size: 10px;
  height: 20px;
}

.advanced-filters {
  transition: all 0.3s ease;
}
</style> 