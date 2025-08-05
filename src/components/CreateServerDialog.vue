<template>
  <div>
    <v-dialog 
      v-model="dialog" 
      max-width="500px" 
      persistent
      :scrim="false"
      transition="dialog-bottom-transition"
    >
      <template v-slot:activator="{ props }">
        <v-btn 
          color="primary" 
          v-bind="props"
          rounded="lg"
          size="large"
          class="add-server-btn"
        >
          <v-icon color="white" class="mr-1">mdi-plus</v-icon>
          <span class="text-white">ADD SERVER</span>
        </v-btn>
      </template>
      
      <v-card class="create-server-dialog">
        <v-card-title class="text-h5 pb-4 pt-5 px-5">
          Creating an instance
          <v-btn icon="mdi-close" variant="text" size="small" @click="dialog = false" class="close-btn"></v-btn>
        </v-card-title>
        
        <v-divider></v-divider>
        
        <v-card-text class="pt-4">
          <!-- Server Type Selection -->
          <div class="server-type-selection mb-6">
            <v-btn-toggle v-model="serverType" mandatory class="server-type-toggle">
              <v-btn value="custom" prepend-icon="mdi-check" class="type-btn">
                Custom
              </v-btn>
              <v-btn value="fromExport" class="type-btn">
                Import From Export
              </v-btn>
              <v-btn value="fromLauncher" class="type-btn">
                Import From Launcher
              </v-btn>
              <v-btn value="fromEgg" class="type-btn">
                Install from Egg
              </v-btn>
            </v-btn-toggle>
          </div>
          
          <!-- Server Icon Selection -->
          <div class="server-icon-selection mb-6 d-flex">
            <div class="server-icon-preview">
              <v-avatar size="80" :color="serverIcon ? undefined : '#1e1e1e'" class="elevation-2">
                <v-icon v-if="!serverIcon" size="40" color="primary">mdi-cube-outline</v-icon>
                <v-img v-else :src="serverIcon" alt="Server Icon"></v-img>
              </v-avatar>
            </div>
            
            <div class="server-icon-actions d-flex flex-column ml-4 justify-center">
              <v-btn 
                prepend-icon="mdi-upload" 
                variant="outlined" 
                class="mb-2" 
                @click="selectIcon"
                size="small"
              >
                Select icon
              </v-btn>
              <v-btn 
                prepend-icon="mdi-close" 
                variant="outlined" 
                @click="removeIcon"
                size="small"
                :disabled="!serverIcon"
              >
                Remove icon
              </v-btn>
              <input 
                type="file" 
                ref="iconInput" 
                accept="image/*" 
                style="display: none" 
                @change="onIconSelected"
              />
            </div>
          </div>
          
          <!-- Server Name -->
          <div class="mb-6">
            <label class="text-subtitle-1 font-weight-medium mb-2 d-block">Name</label>
            <v-text-field
              v-model="serverName"
              placeholder="My Minecraft Server"
              variant="outlined"
              bg-color="#1e1e1e"
              hide-details
              density="comfortable"
            ></v-text-field>
          </div>
          
          <!-- Server Loader -->
          <div class="mb-6">
            <label class="text-subtitle-1 font-weight-medium mb-2 d-block">Loader</label>
            <div class="loader-grid">
              <v-btn 
                :variant="serverLoader === 'vanilla' ? 'elevated' : 'tonal'"
                :color="serverLoader === 'vanilla' ? 'primary' : undefined"
                @click="serverLoader = 'vanilla'"
                class="loader-btn"
                prepend-icon="mdi-check"
                :elevation="serverLoader === 'vanilla' ? 4 : 0"
              >
                Vanilla
              </v-btn>
              <v-btn 
                :variant="serverLoader === 'paper' ? 'elevated' : 'tonal'"
                :color="serverLoader === 'paper' ? 'primary' : undefined"
                @click="serverLoader = 'paper'"
                class="loader-btn"
                :elevation="serverLoader === 'paper' ? 4 : 0"
              >
                Paper
              </v-btn>
              <v-btn 
                :variant="serverLoader === 'fabric' ? 'elevated' : 'tonal'"
                :color="serverLoader === 'fabric' ? 'primary' : undefined"
                @click="serverLoader = 'fabric'"
                class="loader-btn"
                :elevation="serverLoader === 'fabric' ? 4 : 0"
              >
                Fabric
              </v-btn>
              <v-btn 
                :variant="serverLoader === 'forge' ? 'elevated' : 'tonal'"
                :color="serverLoader === 'forge' ? 'primary' : undefined"
                @click="serverLoader = 'forge'"
                class="loader-btn"
                :elevation="serverLoader === 'forge' ? 4 : 0"
              >
                Forge
              </v-btn>
              <v-btn 
                :variant="serverLoader === 'neoforge' ? 'elevated' : 'tonal'"
                :color="serverLoader === 'neoforge' ? 'primary' : undefined"
                @click="serverLoader = 'neoforge'"
                class="loader-btn"
                :elevation="serverLoader === 'neoforge' ? 4 : 0"
              >
                Neoforge
              </v-btn>
              <v-btn 
                :variant="serverLoader === 'pocketmine' ? 'elevated' : 'tonal'"
                :color="serverLoader === 'pocketmine' ? 'primary' : undefined"
                @click="serverLoader = 'pocketmine'"
                class="loader-btn"
                :elevation="serverLoader === 'pocketmine' ? 4 : 0"
              >
                Bedrock
              </v-btn>
            </div>
            
            <!-- Server Type Indicator -->
            <div class="server-type-indicator mt-3">
              <v-chip
                :color="serverTypeInfo.color"
                variant="flat"
                size="small"
                prepend-icon="serverTypeInfo.icon"
              >
                {{ serverTypeInfo.type }} Edition
              </v-chip>
              <div class="text-caption text-grey mt-1">
                {{ serverTypeInfo.description }}
              </div>
            </div>
            
            <!-- Bedrock Auto-Setup Notice -->
            <div v-if="isBedrockServer" class="mt-3">
              <v-alert
                type="success"
                variant="tonal"
                density="compact"
                class="mb-0"
              >
                <template v-slot:prepend>
                  <v-icon>mdi-check-circle</v-icon>
                </template>
                <div class="text-caption">
                  <strong>Auto-Setup:</strong> The PocketMine-MP PHP binary will be automatically downloaded and extracted during server creation!
                </div>
              </v-alert>
            </div>
          </div>
          
          <!-- Import File Selection (for import types) -->
          <div v-if="serverType !== 'custom'" class="mb-6">
            <label class="text-subtitle-1 font-weight-medium mb-2 d-block">
              {{ serverType === 'fromExport' ? 'ServerMint Export File' : 'Launcher Export File' }}
            </label>
            <div class="d-flex align-center">
              <v-text-field
                v-model="importFilePath"
                :placeholder="serverType === 'fromExport' ? 'Select ServerMint export ZIP file' : 'Select launcher export ZIP file'"
                variant="outlined"
                bg-color="#1e1e1e"
                hide-details
                density="comfortable"
                readonly
                class="flex-grow-1 mr-3"
              ></v-text-field>
              <v-btn 
                prepend-icon="mdi-folder-open" 
                variant="outlined" 
                @click="selectImportFile"
                size="large"
              >
                Browse
              </v-btn>
            </div>
            <div v-if="importFileInfo" class="mt-3">
              <v-alert
                type="info"
                variant="tonal"
                density="compact"
                class="mb-0"
              >
                <template v-slot:prepend>
                  <v-icon>mdi-information</v-icon>
                </template>
                <div class="text-caption">
                  <strong>Detected:</strong> {{ importFileInfo.server_name }}
                  <br v-if="importFileInfo.export_date">
                  <strong>Exported:</strong> {{ new Date(importFileInfo.export_date).toLocaleDateString() }}
                  <br>
                  <strong>Files:</strong> {{ importFileInfo.files.length }} files will be imported
                </div>
              </v-alert>
            </div>
          </div>
          
          <!-- Game Version (only for custom) -->
          <div v-if="serverType === 'custom'" class="mb-6">
            <label class="text-subtitle-1 font-weight-medium mb-2 d-block">Game version</label>
            <v-select
              v-model="gameVersion"
              :items="availableVersions"
              placeholder="Select game version"
              variant="outlined"
              bg-color="#1e1e1e"
              hide-details
              density="comfortable"
            ></v-select>
          </div>
        </v-card-text>
        
        <v-divider></v-divider>
        
        <v-card-actions class="pa-4">
          <v-btn 
            prepend-icon="mdi-code-brackets" 
            variant="outlined" 
            color="grey"
            @click="showAdvanced = !showAdvanced"
          >
            {{ showAdvanced ? 'Hide advanced' : 'Show advanced' }}
          </v-btn>
          <v-spacer></v-spacer>
          <v-btn 
            variant="outlined" 
            color="grey" 
            @click="dialog = false"
          >
            Cancel
          </v-btn>
          <v-btn 
            color="primary" 
            @click="createServer"
            :loading="isCreating"
            :disabled="!isFormValid"
          >
            {{ serverType === 'custom' ? 'Create' : 'Import' }}
          </v-btn>
        </v-card-actions>
        
        <!-- Advanced options panel -->
        <v-expand-transition>
          <div v-if="showAdvanced" class="advanced-options pa-4 pt-0">
            <v-divider class="mb-4"></v-divider>
            
            <div class="mb-4">
              <label class="text-subtitle-1 font-weight-medium mb-2 d-block">Server directory</label>
              <v-text-field
                v-model="serverDirectory"
                :placeholder="defaultServerPath"
                variant="outlined"
                bg-color="#1e1e1e"
                hide-details
                density="comfortable"
                append-inner-icon="mdi-folder-open-outline"
                @click:append-inner="selectDirectory"
              ></v-text-field>
            </div>
            
            <div class="mb-4">
              <label class="text-subtitle-1 font-weight-medium mb-2 d-block">Custom download URL (optional)</label>
              <v-text-field
                v-model="downloadUrl"
                placeholder="https://example.com/server.jar"
                variant="outlined"
                bg-color="#1e1e1e"
                hide-details
                density="comfortable"
                hint="Leave empty to use default URL"
                persistent-hint
              ></v-text-field>
            </div>
            
            <div class="mb-4">
              <label class="text-subtitle-1 font-weight-medium mb-2 d-block">Memory allocation (GB)</label>
              <div class="d-flex align-center">
                <v-slider
                  v-model="memoryAllocation"
                  :min="isBedrockServer ? 0.5 : 1"
                  :max="isBedrockServer ? 8 : 16"
                  :step="isBedrockServer ? 0.5 : 1"
                  thumb-label
                  class="memory-slider"
                  color="primary"
                ></v-slider>
                <span class="ml-2">{{ memoryAllocation }} GB</span>
              </div>
              <div class="text-caption text-grey mt-1">
                {{ isBedrockServer ? 'Bedrock servers typically use less memory than Java servers' : 'Recommended: 4-8 GB for most servers' }}
              </div>
            </div>
            
            <!-- PHP Requirement Notice for Bedrock -->
            <div v-if="isBedrockServer" class="mb-4">
              <v-alert
                type="info"
                variant="tonal"
                density="compact"
                class="mb-0"
              >
                <template v-slot:prepend>
                  <v-icon>mdi-information</v-icon>
                </template>
                <div class="text-caption">
                  <strong>Auto-Setup PHP:</strong> The correct PocketMine-MP PHP binary with all required extensions 
                  (curl, openssl, mbstring, zip, etc.) will be automatically downloaded and extracted during server creation.
                  <br><br>
                  <strong>No manual setup required!</strong>
                </div>
              </v-alert>
            </div>
            
            <div class="mb-4">
              <v-checkbox
                v-model="autoStart"
                label="Auto-start server after creation"
                hide-details
                color="primary"
              ></v-checkbox>
            </div>
          </div>
        </v-expand-transition>
      </v-card>
    </v-dialog>
    
    <!-- Backdrop overlay -->
    <div 
      class="modal-backdrop" 
      v-if="dialog"
      @click="dialog = false"
    ></div>
  </div>
</template>

<script>
import { store } from '../store.js';
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'CreateServerDialog',
  data() {
    return {
      dialog: false,
      serverType: 'custom',
      serverIcon: null,
      serverName: '',
      serverLoader: 'vanilla',
      gameVersion: null,
      showAdvanced: false,
      serverDirectory: '',
      downloadUrl: '',
      memoryAllocation: 4,
      autoStart: true,
      isCreating: false,
      store: store,
      importFilePath: '',
      importFileInfo: null,
      selectedImportFile: null
    }
  },
  computed: {
    isFormValid() {
      if (this.serverType === 'custom') {
        return this.serverName && this.gameVersion;
      } else {
        return this.serverName && this.importFilePath;
      }
    },
    defaultServerPath() {
      const serverDirName = this.serverName ? 
        this.serverName.toLowerCase()
          .replace(/\s+/g, '-')
          .replace(/[^a-z0-9\-_]/g, '')
          .replace(/-+/g, '-')
          .replace(/^-|-$/g, '') : 
        'my-minecraft-server';
      const basePath = this.store.settings.general.defaultServerPath || 'C:\\servermint\\servers';
      return `${basePath}/${serverDirName}`;
    },
    availableVersions() {
      // Return available versions based on selected loader
      const loaderType = this.serverLoader.toLowerCase();
      
      if (loaderType === 'vanilla') {
        return ['1.21.8', '1.21.2', '1.20.4', '1.19.4', '1.18.2', '1.16.5', '1.12.2'];
      } else if (loaderType === 'paper') {
        return ['1.21.2', '1.20.4', '1.19.4', '1.18.2', '1.16.5', '1.12.2'];
      } else if (loaderType === 'fabric') {
        return ['1.21.2', '1.20.1', '1.19.4', '1.18.2', '1.16.5'];
      } else if (loaderType === 'forge') {
        return ['1.20.4', '1.19.4', '1.18.2', '1.16.5', '1.12.2'];
      } else if (loaderType === 'neoforge') {
        return ['1.20.4'];
      } else if (loaderType === 'pocketmine') {
        // PocketMine-MP versions (these are release tags)
        return ['5.8.0', '5.7.0', '5.6.0', '5.5.0', '5.4.0', '5.3.0', '5.2.0', '5.1.0', '5.0.0'];
      }
      
      return ['1.21.8', '1.21.2', '1.20.4', '1.19.4', '1.18.2', '1.16.5', '1.12.2'];
    },
    isBedrockServer() {
      return this.serverLoader.toLowerCase() === 'pocketmine';
    },
    serverTypeInfo() {
      if (this.isBedrockServer) {
        return {
          type: 'Bedrock',
          color: 'orange',
          icon: 'mdi-cellphone',
          description: 'Runs on PocketMine-MP, compatible with Bedrock Edition'
        };
      } else {
        return {
          type: 'Java',
          color: 'primary',
          icon: 'mdi-language-java',
          description: 'Runs on Java, compatible with Java Edition'
        };
      }
    }
  },
  watch: {
         serverType() {
       // Reset import fields when switching to custom
       if (this.serverType === 'custom') {
         this.importFilePath = '';
         this.importFileInfo = null;
         this.selectedImportFile = null;
       }
     },
    serverLoader() {
      // Reset game version when loader changes
      this.gameVersion = null;
      
      // Set appropriate default memory allocation
      if (this.serverLoader.toLowerCase() === 'pocketmine') {
        this.memoryAllocation = 2; // 2GB default for Bedrock servers
      } else {
        this.memoryAllocation = this.store.settings.java.memory || 4; // Default for Java servers
      }
    },
    serverName(newName) {
      // Update server directory when name changes
      if (newName) {
        console.log('serverName changed to:', newName);
        // Sanitize folder name: lowercase, replace spaces with hyphens, remove special chars
        const serverDirName = newName.toLowerCase()
          .replace(/\s+/g, '-')
          .replace(/[^a-z0-9\-_]/g, '')
          .replace(/-+/g, '-')
          .replace(/^-|-$/g, '');
        console.log('serverDirName generated:', serverDirName);
        const basePath = this.store.settings.general.defaultServerPath || 'C:\\servermint\\servers';
        this.serverDirectory = `${basePath}/${serverDirName}`;
        console.log('serverDirectory set to:', this.serverDirectory);
      }
    }
  },
  mounted() {
    // Set default memory allocation from settings
    this.memoryAllocation = this.store.settings.java.memory || 4;
    
    // Set default game version from settings
    this.gameVersion = this.store.settings.general.defaultGameVersion || '1.21.2';
  },
  methods: {
    selectIcon() {
      this.$refs.iconInput.click();
    },
    onIconSelected(event) {
      const file = event.target.files[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (e) => {
          this.serverIcon = e.target.result;
        };
        reader.readAsDataURL(file);
      }
    },
    removeIcon() {
      this.serverIcon = null;
      this.$refs.iconInput.value = '';
    },
    selectDirectory() {
      if (!this.serverName) {
        window.showWarning('Server Name Required', 'Please enter a server name first');
        return;
      }
      
      const serverDirName = this.serverName.toLowerCase()
        .replace(/\s+/g, '-')
        .replace(/[^a-z0-9\-_]/g, '')
        .replace(/-+/g, '-')
        .replace(/^-|-$/g, '');
      const basePath = this.store.settings.general.defaultServerPath || 'C:\\servermint\\servers';
      this.serverDirectory = `${basePath}/${serverDirName}`;
    },
    async selectImportFile() {
      try {
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = '.zip';
                 input.onchange = async (event) => {
           const file = event.target.files[0];
           if (file) {
             this.importFilePath = file.name;
             this.selectedImportFile = file; 
 
             
             if (!this.serverName) {
               this.serverName = file.name.replace('.zip', '').replace(/[-_]/g, ' ');
             }
             
             // Create basic file info for display
             this.importFileInfo = {
               server_name: this.serverName,
               export_date: null,
               files: [
                 { name: 'server.jar', path: 'server.jar' },
                 { name: 'server.properties', path: 'server.properties' },
                 { name: 'eula.txt', path: 'eula.txt' },
                 { name: 'world/', path: 'world/' }
               ]
             };
           }
         };
        input.click();
      } catch (error) {
        console.error('Error selecting import file:', error);
        window.showError('File Selection Error', 'Failed to open file dialog.');
      }
    },
    async createServer() {
      if (!this.isFormValid) return;
      
      this.isCreating = true;
      
      try {
        // Generate server directory path if not specified
        if (!this.serverDirectory) {
          console.log('Generating server directory, serverName:', this.serverName);
          const serverDirName = this.serverName.toLowerCase()
            .replace(/\s+/g, '-')
            .replace(/[^a-z0-9\-_]/g, '')
            .replace(/-+/g, '-')
            .replace(/^-|-$/g, '');
          console.log('Generated serverDirName:', serverDirName);
          const basePath = this.store.settings.general.defaultServerPath || 'C:\\servermint\\servers';
          this.serverDirectory = `${basePath}/${serverDirName}`;
          console.log('Final serverDirectory:', this.serverDirectory);
        }
        
        let result;
        
        if (this.serverType === 'custom') {
          // Create server data object for custom server
          const serverData = {
            name: this.serverName,
            version: this.gameVersion,
            type: this.serverLoader, // Use full loader name
            path: this.serverDirectory,
            icon: this.serverIcon,
            memoryAllocation: this.memoryAllocation,
            autoStart: this.autoStart,
            downloadUrl: this.downloadUrl || null // Include downloadUrl if provided
          };
          
          // Create the server using the store
          result = await this.store.createServer(serverData);
                 } else {
                       // Handle import from ZIP
            try {
              // Use the stored file from the component data
              if (!this.selectedImportFile) {
                throw new Error('No import file selected');
              }
              
              const file = this.selectedImportFile;
              
              // Create temp file using a different approach to avoid memory issues
              const tempPath = await invoke('create_temp_file_from_blob', {
                file_name: file.name,
                file_size: file.size
              });
              
              // Write the file data in chunks to avoid memory issues
              const chunkSize = 1024 * 1024; // 1MB chunks
              const totalChunks = Math.ceil(file.size / chunkSize);
              
              for (let i = 0; i < totalChunks; i++) {
                const start = i * chunkSize;
                const end = Math.min(start + chunkSize, file.size);
                const chunk = file.slice(start, end);
                const arrayBuffer = await chunk.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);
                
                await invoke('write_temp_file_chunk', {
                  temp_path: tempPath,
                  chunk_index: i,
                  chunk_data: Array.from(uint8Array)
                });
              }
             
             // Import the server from ZIP
             await invoke('import_server_from_zip', {
               zip_path: tempPath,
               target_directory: this.serverDirectory
             });
             
             // Create server entry in the store
             const serverData = {
               name: this.serverName,
               version: '1.21.2', // Use a default version for imports
               type: 'vanilla', // Use vanilla as default type for imports
               path: this.serverDirectory,
               icon: this.serverIcon,
               memoryAllocation: this.memoryAllocation,
               autoStart: this.autoStart,
               downloadUrl: null
             };
             
             result = await this.store.createServer(serverData);
             
           } catch (error) {
             console.error('Error importing server:', error);
             window.showError('Import Failed', `Failed to import server: ${error.message || error}`);
             this.isCreating = false;
             return;
           }
         }
        
        if (result.success) {
          // Show success toast
          const action = this.serverType === 'custom' ? 'Created' : 'Imported';
          window.showSuccess(`Server ${action}!`, `"${this.serverName}" has been ${action.toLowerCase()} successfully.`);
          
          // Close the dialog
          this.dialog = false;
          
          // Reset form
          this.resetForm();
          
          // Emit event to notify that a server was created
          window.dispatchEvent(new CustomEvent('server-created'));
          
          // Auto-start server if enabled
          if (this.autoStart) {
            setTimeout(() => {
              this.store.startServer(result.server.id);
            }, 1000);
          }
        } else {
          console.error('Error creating server:', result.error);
          window.showError('Server Creation Failed', result.error);
        }
      } catch (error) {
        console.error('Error creating server:', error);
        window.showError('Server Creation Failed', error.message || 'Unknown error occurred');
      } finally {
        this.isCreating = false;
      }
    },
    resetForm() {
      this.serverType = 'custom';
      this.serverIcon = null;
      this.serverName = '';
      this.serverLoader = 'vanilla';
      this.gameVersion = this.store.settings.general.defaultGameVersion || '1.21.2';
      this.showAdvanced = false;
      this.serverDirectory = '';
      this.downloadUrl = ''; // Reset downloadUrl
      this.memoryAllocation = this.store.settings.java.memory || 4;
      this.autoStart = false; // Don't auto-start by default
      this.importFilePath = '';
      this.importFileInfo = null;
      this.selectedImportFile = null;
    }
  }
}
</script>

<style scoped>
.create-server-dialog {
  background-color: #121212;
  border-radius: 12px;
  border: none;
  overflow: hidden;
  position: relative;
  z-index: 1001;
}
.close-btn {
  position: absolute;
  right: 12px;
  top: 12px;
}
.server-type-toggle {
  width: 100%;
  border-radius: 8px;
  overflow: hidden;
  background-color: #1e1e1e;
}
.type-btn {
  flex-grow: 1;
  border-radius: 0;
  text-transform: none;
}
.loader-toggle {
  width: 100%;
  border-radius: 8px;
  overflow: hidden;
  background-color: #1e1e1e;
}
.loader-btn {
  flex-grow: 1;
  border-radius: 0;
  text-transform: none;
}
.server-icon-preview {
  display: flex;
  justify-content: center;
  align-items: center;
}
.server-type-indicator {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
.server-type-indicator .v-chip {
  font-weight: 600;
  letter-spacing: 0.5px;
}
.memory-slider {
  max-width: 300px;
}
.advanced-options {
  background-color: #121212;
  border-bottom-left-radius: 12px;
  border-bottom-right-radius: 12px;
}
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100%;
  height: 100%;
  backdrop-filter: blur(8px);
  z-index: 1000;
}
.add-server-btn {
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.5px;
  padding: 0 20px;
  height: 42px;
}
.loader-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  width: 100%;
}
.loader-btn {
  text-transform: none;
  min-height: 48px;
  font-size: 14px;
  font-weight: 500;
  border-radius: 8px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style> 