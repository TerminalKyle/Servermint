<template>
  <div class="egg-manager">
    <v-container fluid>
      <v-row>
        <v-col cols="12">
          <v-card>
            <v-card-title class="d-flex align-center justify-space-between">
              <div>
                <v-icon class="mr-2">mdi-egg</v-icon>
                Egg Manager
              </div>
              <v-btn
                color="primary"
                @click="showCreateEggDialog = true"
                prepend-icon="mdi-plus"
                class="mr-2"
              >
                Create Custom Egg
              </v-btn>
              <v-btn
                color="secondary"
                @click="showImportEggDialog = true"
                prepend-icon="mdi-download"
              >
                Import Egg
              </v-btn>
            </v-card-title>
            
            <v-card-text>
              <v-tabs v-model="activeTab" grow>
                <v-tab value="all">All Eggs</v-tab>
                <v-tab value="minecraft">Minecraft</v-tab>
                <v-tab value="bedrock">Bedrock</v-tab>
                <v-tab value="custom">Custom</v-tab>
              </v-tabs>
              
              <v-window v-model="activeTab">
                <v-window-item value="all">
                  <egg-grid :eggs="allEggs" @install="installFromEgg" @view="viewEgg" />
                </v-window-item>
                
                <v-window-item value="minecraft">
                  <egg-grid :eggs="minecraftEggs" @install="installFromEgg" @view="viewEgg" />
                </v-window-item>
                
                <v-window-item value="bedrock">
                  <egg-grid :eggs="bedrockEggs" @install="installFromEgg" @view="viewEgg" />
                </v-window-item>
                
                <v-window-item value="custom">
                  <egg-grid :eggs="customEggs" @install="installFromEgg" @view="viewEgg" />
                </v-window-item>
              </v-window>
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Create Custom Egg Dialog -->
    <v-dialog v-model="showCreateEggDialog" max-width="800px">
      <v-card>
        <v-card-title>Create Custom Egg</v-card-title>
        <v-card-text>
          <v-form ref="createEggForm" v-model="createEggFormValid">
            <v-row>
              <v-col cols="6">
                <v-text-field
                  v-model="newEgg.id"
                  label="Egg ID"
                  :rules="[v => !!v || 'Egg ID is required']"
                  required
                />
              </v-col>
              <v-col cols="6">
                <v-text-field
                  v-model="newEgg.name"
                  label="Egg Name"
                  :rules="[v => !!v || 'Egg name is required']"
                  required
                />
              </v-col>
            </v-row>
            
            <v-textarea
              v-model="newEgg.description"
              label="Description"
              :rules="[v => !!v || 'Description is required']"
              required
            />
            
            <v-row>
              <v-col cols="6">
                <v-text-field
                  v-model="newEgg.author"
                  label="Author"
                  :rules="[v => !!v || 'Author is required']"
                  required
                />
              </v-col>
              <v-col cols="6">
                <v-text-field
                  v-model="newEgg.category"
                  label="Category"
                  :rules="[v => !!v || 'Category is required']"
                  required
                />
              </v-col>
            </v-row>
            
            <v-textarea
              v-model="newEgg.script"
              label="Installation Script"
              :rules="[v => !!v || 'Installation script is required']"
              required
              rows="10"
              hint="Shell script that will be executed to install the server"
            />
            
            <v-text-field
              v-model="newEgg.startup_command"
              label="Startup Command"
              :rules="[v => !!v || 'Startup command is required']"
              required
              hint="Command to start the server (e.g., java -jar server.jar)"
            />
          </v-form>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="showCreateEggDialog = false">Cancel</v-btn>
          <v-btn
            color="primary"
            @click="createCustomEgg"
            :disabled="!createEggFormValid"
          >
            Create Egg
          </v-btn>
        </v-card-actions>
      </v-card>
         </v-dialog>

     <!-- Import Egg Dialog -->
     <v-dialog v-model="showImportEggDialog" max-width="600px">
       <v-card>
         <v-card-title>Import Egg from JSON</v-card-title>
         <v-card-text>
           <v-form ref="importForm" v-model="importFormValid">
             <v-file-input
               v-model="importFile"
               label="Select Egg JSON File"
               accept=".json"
               :rules="[v => !!v || 'Please select a JSON file']"
               required
               prepend-icon="mdi-file-json"
               hint="Select a Pterodactyl egg JSON file to import"
               persistent-hint
             />
             
             <div v-if="importPreview" class="mt-4">
               <v-alert
                 type="info"
                 variant="tonal"
                 density="compact"
                 class="mb-3"
               >
                 <template v-slot:prepend>
                   <v-icon>mdi-information</v-icon>
                 </template>
                 <div class="text-caption">
                   <strong>Preview:</strong> {{ importPreview.name }} by {{ importPreview.author }}
                   <br>
                   <strong>Description:</strong> {{ importPreview.description }}
                 </div>
               </v-alert>
             </div>
           </v-form>
         </v-card-text>
         <v-card-actions>
           <v-spacer />
           <v-btn @click="showImportEggDialog = false">Cancel</v-btn>
           <v-btn
             color="primary"
             @click="importEgg"
             :disabled="!importFormValid || !importPreview"
             :loading="importing"
           >
             Import Egg
           </v-btn>
         </v-card-actions>
       </v-card>
     </v-dialog>

     <!-- Install Server Dialog -->
    <v-dialog v-model="showInstallDialog" max-width="600px">
      <v-card>
        <v-card-title>Install Server from Egg</v-card-title>
        <v-card-text>
          <v-form ref="installForm" v-model="installFormValid">
            <v-text-field
              v-model="installConfig.serverName"
              label="Server Name"
              :rules="[v => !!v || 'Server name is required']"
              required
            />
            
            <v-text-field
              v-model="installConfig.serverPath"
              label="Server Path"
              :rules="[v => !!v || 'Server path is required']"
              required
              hint="Directory where the server will be installed"
            />
            
            <v-divider class="my-4" />
            
            <h3 class="text-h6 mb-3">Egg Variables</h3>
            <div v-for="variable in selectedEgg?.variables" :key="variable.name" class="mb-3">
              <v-text-field
                v-model="installConfig.variables[variable.name]"
                :label="variable.name"
                :hint="variable.description"
                :rules="getVariableRules(variable)"
                :required="variable.rules.includes('required')"
                persistent-hint
              />
            </div>
          </v-form>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="showInstallDialog = false">Cancel</v-btn>
          <v-btn
            color="primary"
            @click="installServer"
            :disabled="!installFormValid"
            :loading="installing"
          >
            Install Server
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Egg Details Dialog -->
    <v-dialog v-model="showEggDetails" max-width="800px">
      <v-card v-if="selectedEgg">
        <v-card-title>{{ selectedEgg.name }}</v-card-title>
        <v-card-text>
          <v-row>
            <v-col cols="6">
              <strong>Author:</strong> {{ selectedEgg.author }}
            </v-col>
            <v-col cols="6">
              <strong>Category:</strong> {{ selectedEgg.category }}
            </v-col>
          </v-row>
          
          <v-row>
            <v-col cols="12">
              <strong>Description:</strong>
              <p>{{ selectedEgg.description }}</p>
            </v-col>
          </v-row>
          
          <v-row>
            <v-col cols="12">
              <strong>Startup Command:</strong>
              <v-chip class="ma-1">{{ selectedEgg.startup_command }}</v-chip>
            </v-col>
          </v-row>
          
          <v-row v-if="selectedEgg.variables.length > 0">
            <v-col cols="12">
              <strong>Variables:</strong>
              <v-list>
                <v-list-item v-for="variable in selectedEgg.variables" :key="variable.name">
                  <v-list-item-title>{{ variable.name }}</v-list-item-title>
                  <v-list-item-subtitle>{{ variable.description }}</v-list-item-subtitle>
                  <template v-slot:append>
                    <v-chip size="small" color="primary">{{ variable.default_value }}</v-chip>
                  </template>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
          
          <v-row>
            <v-col cols="12">
              <strong>Installation Script:</strong>
              <v-textarea
                :model-value="selectedEgg.script"
                readonly
                rows="8"
                variant="outlined"
                class="mt-2"
              />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn @click="showEggDetails = false">Close</v-btn>
          <v-btn
            color="primary"
            @click="installFromEgg(selectedEgg)"
          >
            Install Server
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'EggManagerView',
  components: {
    EggGrid: {
      props: ['eggs'],
      emits: ['install', 'view'],
      template: `
        <div class="egg-grid">
          <v-row>
            <v-col v-for="egg in eggs" :key="egg.id" cols="12" sm="6" md="4" lg="3">
              <v-card class="egg-card" height="200">
                <v-card-title class="text-truncate">
                  <v-icon class="mr-2">mdi-egg</v-icon>
                  {{ egg.name }}
                </v-card-title>
                
                <v-card-text>
                  <p class="text-body-2 text-truncate">{{ egg.description }}</p>
                  <v-chip size="small" color="primary" class="ma-1">
                    {{ egg.category }}
                  </v-chip>
                  <v-chip size="small" color="secondary" class="ma-1">
                    {{ egg.author }}
                  </v-chip>
                </v-card-text>
                
                <v-card-actions>
                  <v-btn
                    size="small"
                    variant="text"
                    @click="$emit('view', egg)"
                  >
                    View
                  </v-btn>
                  <v-spacer />
                  <v-btn
                    size="small"
                    color="primary"
                    @click="$emit('install', egg)"
                  >
                    Install
                  </v-btn>
                </v-card-actions>
              </v-card>
            </v-col>
          </v-row>
        </div>
      `
    }
  },
  data() {
    return {
      eggs: [],
      activeTab: 'all',
      showCreateEggDialog: false,
      showImportEggDialog: false,
      showInstallDialog: false,
      showEggDetails: false,
      selectedEgg: null,
             installing: false,
       importing: false,
       createEggFormValid: false,
       importFormValid: false,
       installFormValid: false,
       importFile: null,
       importPreview: null,
      newEgg: {
        id: '',
        name: '',
        description: '',
        author: '',
        version: '1.0.0',
        category: '',
        script: '',
        startup_command: '',
        variables: []
      },
      installConfig: {
        serverName: '',
        serverPath: '',
        variables: {}
      }
    };
  },
  computed: {
    allEggs() {
      return this.eggs;
    },
    minecraftEggs() {
      return this.eggs.filter(egg => egg.category.toLowerCase() === 'minecraft');
    },
    bedrockEggs() {
      return this.eggs.filter(egg => egg.category.toLowerCase() === 'bedrock');
    },
    customEggs() {
      return this.eggs.filter(egg => egg.author !== 'ServerMint');
    }
  },
     async mounted() {
     await this.loadEggs();
   },
   watch: {
     importFile(newFile) {
       if (newFile) {
         this.parseImportFile(newFile);
       } else {
         this.importPreview = null;
       }
     }
   },
  methods: {
    async loadEggs() {
      try {
        this.eggs = await invoke('list_eggs');
      } catch (error) {
        console.error('Failed to load eggs:', error);
        this.$toast.error('Failed to load eggs');
      }
    },
    
    viewEgg(egg) {
      this.selectedEgg = egg;
      this.showEggDetails = true;
    },
    
    installFromEgg(egg) {
      this.selectedEgg = egg;
      this.installConfig = {
        serverName: '',
        serverPath: `C:/servermint/servers/${egg.name.toLowerCase().replace(/\s+/g, '-')}`,
        variables: {}
      };
      
      // Initialize variables with default values
      egg.variables.forEach(variable => {
        this.installConfig.variables[variable.name] = variable.default_value;
      });
      
      this.showInstallDialog = true;
    },
    
    getVariableRules(variable) {
      const rules = [];
      
      if (variable.rules.includes('required')) {
        rules.push(v => !!v || `${variable.name} is required`);
      }
      
      if (variable.rules.includes('numeric')) {
        rules.push(v => !isNaN(v) || `${variable.name} must be a number`);
      }
      
      return rules;
    },
    
    async createCustomEgg() {
      try {
        await invoke('add_custom_egg', { egg: this.newEgg });
        this.showCreateEggDialog = false;
        await this.loadEggs();
        this.$toast.success('Custom egg created successfully');
        
        // Reset form
        this.newEgg = {
          id: '',
          name: '',
          description: '',
          author: '',
          version: '1.0.0',
          category: '',
          script: '',
          startup_command: '',
          variables: []
        };
      } catch (error) {
        console.error('Failed to create egg:', error);
        this.$toast.error('Failed to create egg');
      }
    },
    
    async installServer() {
      this.installing = true;
      
      try {
        const environment = {
          SERVER_NAME: this.installConfig.serverName,
          SERVER_PATH: this.installConfig.serverPath
        };
        
        await invoke('install_server_from_egg', {
          eggId: this.selectedEgg.id,
          serverPath: this.installConfig.serverPath,
          variables: this.installConfig.variables,
          environment
        });
        
        this.showInstallDialog = false;
        this.$toast.success('Server installed successfully');
        
        // Emit event to parent to refresh servers list
        this.$emit('server-installed');
        
      } catch (error) {
        console.error('Failed to install server:', error);
        this.$toast.error(`Failed to install server: ${error}`);
      } finally {
        this.installing = false;
      }
     },
       
       async importEgg() {
         this.importing = true;
         
         try {
           if (!this.importFile || !this.importPreview) {
             throw new Error('No file selected or preview not available');
           }
           
           // Convert Pterodactyl egg format to ServerMint format
           const serverMintEgg = this.convertPterodactylEgg(this.importPreview);
           
           // Add the egg to ServerMint
           await invoke('add_custom_egg', { egg: serverMintEgg });
           
           this.showImportEggDialog = false;
           await this.loadEggs();
           this.$toast.success('Egg imported successfully');
           
           // Reset import form
           this.importFile = null;
           this.importPreview = null;
           
         } catch (error) {
           console.error('Failed to import egg:', error);
           const errorMessage = error?.message || error?.toString() || 'Unknown error occurred';
           this.$toast.error(`Failed to import egg: ${errorMessage}`);
         } finally {
           this.importing = false;
         }
       },
       
       convertPterodactylEgg(pterodactylEgg) {
         // Add null checks to prevent undefined access
         if (!pterodactylEgg || typeof pterodactylEgg !== 'object') {
           throw new Error('Invalid egg data provided');
         }
         
         const serverMintEgg = {
           id: (pterodactylEgg.name || 'unknown').toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, ''),
           name: pterodactylEgg.name || 'Unknown Egg',
           description: pterodactylEgg.description || 'No description available',
           author: pterodactylEgg.author || 'Unknown',
           version: '1.0.0',
           category: this.detectCategory(pterodactylEgg.name || '', pterodactylEgg.description || ''),
           script: pterodactylEgg.scripts?.installation?.script || '',
           startup_command: pterodactylEgg.startup || '',
           variables: [],
           file_denylist: pterodactylEgg.file_denylist || [],
           file_allowlist: [],
           features: pterodactylEgg.features || [],
           docker_image: null,
           docker_startup: null
         };
         
         // Convert variables
         if (pterodactylEgg.variables) {
           serverMintEgg.variables = pterodactylEgg.variables.map(variable => ({
             name: variable.name,
             description: variable.description,
             default_value: variable.default_value || '',
             env_variable: variable.env_variable,
             rules: variable.rules || 'required|string',
             field_type: variable.field_type || 'text',
             is_viewable: variable.user_viewable !== false,
             is_rules: false
           }));
         }
         
         return serverMintEgg;
       },
       
       detectCategory(name, description) {
         // Ensure we have valid strings to work with
         const safeName = (name || '').toString();
         const safeDescription = (description || '').toString();
         const text = (safeName + ' ' + safeDescription).toLowerCase();
         
         if (text.includes('minecraft') || text.includes('forge') || text.includes('fabric') || text.includes('paper') || text.includes('spigot') || text.includes('bukkit')) {
           return 'Minecraft';
         } else if (text.includes('bedrock') || text.includes('pocketmine')) {
           return 'Bedrock';
         } else if (text.includes('ark') || text.includes('rust') || text.includes('valheim') || text.includes('terraria')) {
           return 'Survival';
         } else if (text.includes('csgo') || text.includes('cs2') || text.includes('tf2') || text.includes('garry')) {
           return 'FPS';
         } else {
           return 'Other';
         }
       },
       
       parseImportFile(file) {
         if (!file) {
           this.importPreview = null;
           return;
         }
         
         const reader = new FileReader();
         reader.onload = (e) => {
           try {
             if (!e.target || !e.target.result) {
               throw new Error('No file content to parse');
             }
             
             const jsonData = JSON.parse(e.target.result);
             
             // Validate that the parsed data has the expected structure
             if (!jsonData || typeof jsonData !== 'object') {
               throw new Error('Invalid JSON structure');
             }
             
             this.importPreview = jsonData;
           } catch (error) {
             console.error('Failed to parse JSON file:', error);
             this.$toast.error('Invalid JSON file');
             this.importPreview = null;
           }
         };
         
         reader.onerror = () => {
           console.error('Failed to read file');
           this.$toast.error('Failed to read file');
           this.importPreview = null;
         };
         
         reader.readAsText(file);
       }
     }
   }
</script>

<style scoped>
.egg-manager {
  height: 100%;
  overflow-y: auto;
}

.egg-card {
  transition: transform 0.2s ease-in-out;
}

.egg-card:hover {
  transform: translateY(-2px);
}

.egg-grid {
  margin-top: 16px;
}
</style> 