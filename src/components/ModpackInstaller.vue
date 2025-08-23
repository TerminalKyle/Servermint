<template>
  <div class="modpack-installer">
    <v-card color="#1E1E1E" class="pa-6 modpack-card">
      <div class="green-blob blob-1"></div>
      <div class="green-blob blob-2"></div>
      <div class="green-blob blob-3"></div>
      
      <v-card-title class="text-h5 font-weight-bold mb-4">
        <v-icon class="mr-3" color="primary">mdi-package-variant</v-icon>
        Modpack Auto-Installer
      </v-card-title>

      <v-card-text>
        <div class="text-body-1 mb-6">
          Import modpacks from CurseForge or Modrinth exports. Simply select your exported modpack file and choose a server to install it on.
        </div>

        <v-row>
          <v-col cols="12" md="6">
            <v-file-input
              v-model="selectedFile"
              accept=".zip,application/zip"
              label="Select Modpack File"
              prepend-icon=""
              variant="outlined"
              density="comfortable"
              bg-color="#2A2A2A"
              class="mb-4"
              :error-messages="fileError"
              @update:model-value="onFileSelected"
            >
              <template v-slot:prepend>
                <v-icon color="primary">mdi-file</v-icon>
              </template>
            </v-file-input>



            <v-select
              v-model="selectedServerId"
              :items="servers"
              item-title="name"
              item-value="id"
              label="Select Target Server"
              variant="outlined"
              density="comfortable"
              bg-color="#2A2A2A"
              class="mb-4"
              :error-messages="!selectedServerId ? 'Please select a server' : ''"
              :disabled="!selectedFile || installing"
            >
              <template v-slot:prepend>
                <v-icon color="primary">mdi-server</v-icon>
              </template>
            </v-select>

            <v-expand-transition>
              <div v-if="modpackInfo" class="modpack-preview pa-4 rounded bg-surface mb-4">
                <div class="text-h6 mb-3">Modpack Information</div>
                
                <div class="d-flex align-center mb-2">
                  <v-icon size="small" color="white" class="mr-2">mdi-package-variant</v-icon>
                  <span class="font-weight-medium">{{ modpackInfo.name }}</span>
                </div>
                <div class="d-flex align-center mb-2">
                  <v-icon size="small" color="white" class="mr-2">mdi-tag</v-icon>
                  <span>Version: {{ modpackInfo.version }}</span>
                </div>
                <div class="d-flex align-center mb-2">
                  <v-icon size="small" color="white" class="mr-2">mdi-account</v-icon>
                  <span>Author: {{ modpackInfo.author }}</span>
                </div>
                <div class="d-flex align-center mb-2">
                  <v-icon size="small" color="white" class="mr-2">mdi-minecraft</v-icon>
                  <span>Game Version: {{ modpackInfo.game_version }}</span>
                </div>
                <div class="d-flex align-center mb-2">
                  <v-icon size="small" color="white" class="mr-2">mdi-cube-outline</v-icon>
                  <span>Loader: {{ modpackInfo.loader }}</span>
                </div>
                <div class="d-flex align-center mb-2">
                  <v-icon size="small" color="white" class="mr-2">mdi-download</v-icon>
                  <span>Mods: {{ modpackInfo.mods.length }}</span>
                </div>
                
                <div v-if="modpackInfo.source === 'curseforge'" class="d-flex align-center mb-2">
                  <v-icon size="small" color="success" class="mr-2">mdi-check-circle</v-icon>
                  <span class="text-caption text-success">CurseForge mods will be downloaded automatically</span>
                </div>

                <v-divider class="my-3"></v-divider>
                
                <div class="text-body-2">{{ modpackInfo.description }}</div>
              </div>
            </v-expand-transition>

          </v-col>

          <v-col cols="12" md="6">
            <v-expand-transition>
              <div v-if="installing" class="installation-progress pa-4 rounded bg-surface">
                <div class="text-h6 mb-3">Installation Progress</div>
                
                <v-progress-linear
                  v-model="installProgress"
                  color="primary"
                  height="20"
                  rounded
                  class="mb-3"
                >
                  <template v-slot:default="{ value }">
                    <strong>{{ Math.ceil(value) }}%</strong>
                  </template>
                </v-progress-linear>
                
                <div class="text-body-2">{{ currentStep }}</div>
                
                <div v-if="installLog.length > 0" class="install-log mt-3">
                  <div class="text-caption mb-2">Installation Log:</div>
                  <div class="log-container pa-2 rounded bg-black">
                    <div v-for="(log, index) in installLog" :key="index" class="log-entry text-caption">
                      {{ log }}
                    </div>
                  </div>
                </div>
              </div>
            </v-expand-transition>

            <v-expand-transition>
              <div v-if="modpackInfo && !installing" class="mod-list-preview pa-4 rounded bg-surface">
                <div class="d-flex justify-space-between align-center mb-3">
                  <div class="text-h6">Mods to be Installed ({{ modpackInfo.mods.length }})</div>
                  <div class="text-caption text-grey">
                    {{ currentModsPage }} / {{ totalModsPages }}
                  </div>
                </div>
                
                <v-list density="compact" bg-color="transparent" class="mod-list-container">
                  <v-list-item
                    v-for="(mod, index) in paginatedMods"
                    :key="`mod-${currentModsPage}-${index}`"
                    class="px-0"
                  >
                    <template v-slot:prepend>
                      <v-icon size="small" color="primary">mdi-package-variant-closed</v-icon>
                    </template>
                    
                    <v-list-item-title class="text-body-2">
                      {{ getModDisplayName(mod) }}
                    </v-list-item-title>
                    <v-list-item-subtitle class="text-caption">
                      {{ mod.filename }}
                      <span v-if="mod.project_id" class="text-grey ml-1">(ID: {{ mod.project_id }})</span>
                    </v-list-item-subtitle>
                  </v-list-item>
                </v-list>
                
                <div v-if="totalModsPages > 1" class="pagination-controls d-flex justify-center align-center mt-2 gap-2">
                  <v-btn
                    size="x-small"
                    variant="outlined"
                    :disabled="!hasPrevPage"
                    @click="goToPrevPage"
                    color="primary"
                  >
                    <v-icon size="small">mdi-chevron-left</v-icon>
                    Prev
                  </v-btn>
                  
                  <div class="text-caption mx-2">
                    {{ currentModsPage }} / {{ totalModsPages }}
                  </div>
                  
                  <v-btn
                    size="x-small"
                    variant="outlined"
                    :disabled="!hasNextPage"
                    @click="goToNextPage"
                    color="primary"
                  >
                    Next
                    <v-icon size="small">mdi-chevron-right</v-icon>
                  </v-btn>
                </div>
              </div>
            </v-expand-transition>
          </v-col>
        </v-row>
      </v-card-text>

      <v-card-actions class="pa-6 pt-0">
        <v-spacer></v-spacer>
        
        <v-btn
          color="error"
          variant="text"
          @click="handleCancel"
          :disabled="installing"
        >
          Cancel
        </v-btn>
        
        <v-btn
          color="primary"
          variant="flat"
          :prepend-icon="installing ? undefined : 'mdi-download'"
          @click="installModpack"
          :loading="installing"
          :disabled="!canInstall"
        >
          {{ installing ? 'Installing...' : 'Install Modpack' }}
        </v-btn>
      </v-card-actions>
    </v-card>

    <v-dialog v-model="showSuccessDialog" max-width="500px">
      <v-card color="#1E1E1E">
        <v-card-title class="text-h6 pa-4">
          <v-icon color="success" class="mr-2">mdi-check-circle</v-icon>
          Installation Complete
        </v-card-title>
        
        <v-card-text class="pa-4">
          <div class="text-body-1 mb-3">
            The modpack <strong>{{ modpackInfo?.name }}</strong> has been successfully installed to 
            <strong>{{ getServerName(selectedServerId) }}</strong>!
          </div>
          
          <div class="text-body-2 text-grey">
            You can now start your server and the modpack will be ready to use.
          </div>
        </v-card-text>
        
        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn color="primary" @click="showSuccessDialog = false">
            Continue
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script>
import { store } from '../store.js'
import { invoke } from '@tauri-apps/api/core'

export default {
  name: 'ModpackInstaller',
  
  data() {
    return {
      selectedFile: null,
      selectedServerId: null,
      modpackInfo: null,
      installing: false,
      installProgress: 0,
      currentStep: '',
      installLog: [],
      fileError: '',
      showSuccessDialog: false,
      servers: [],
      
      modsPerPage: 8,
      currentModsPage: 1
    }
  },
  
  computed: {
    canInstall() {
      return this.selectedFile && this.selectedServerId && this.modpackInfo && !this.installing
    },
    
    paginatedMods() {
      if (!this.modpackInfo || !this.modpackInfo.mods) return []
      
      const startIndex = (this.currentModsPage - 1) * this.modsPerPage
      const endIndex = startIndex + this.modsPerPage
      return this.modpackInfo.mods.slice(startIndex, endIndex)
    },
    
    totalModsPages() {
      if (!this.modpackInfo || !this.modpackInfo.mods) return 0
      return Math.ceil(this.modpackInfo.mods.length / this.modsPerPage)
    },
    
    hasNextPage() {
      return this.currentModsPage < this.totalModsPages
    },
    
    hasPrevPage() {
      return this.currentModsPage > 1
    }
  },
  
  async created() {
    await this.loadServers()
  },
  
  methods: {
    async loadServers() {
      try {
        await store.loadServers()
        this.servers = store.servers
      } catch (error) {
        console.error('Error loading servers:', error)
      }
    },
    
    onFileSelected(file) {
      this.fileError = ''
      this.modpackInfo = null
      
      if (!file) return
      
      if (!(file instanceof File)) {
        console.log('Ignoring non-File object:', file)
        return
      }
      
      console.log('File selected:', file)
      
      console.log('File object properties:', {
        name: file.name,
        type: file.type,
        size: file.size,
        lastModified: file.lastModified
      })
      
      if (!file.name) {
        this.fileError = 'Please select a valid file (no name)'
        console.log('Validation failed: file.name is falsy')
        return
      }
      
      const fileName = file.name.toLowerCase()
      console.log('File name (lowercase):', fileName)
      
      if (!fileName.endsWith('.zip')) {
        this.fileError = 'Please select a valid ZIP file (.zip extension required)'
        console.log('Validation failed: file does not end with .zip')
        return
      }
      
      console.log('File validation passed, proceeding to preview')
      
      this.previewModpack(file)
    },
    
    async previewModpack(file) {
      try {
        this.currentStep = 'Analyzing modpack...'
        this.currentModsPage = 1
        
        console.log('File object:', file)
        console.log('File path:', file.path)
        console.log('File name:', file.name)
        
        const tempPath = await this.createTempFile(file)
        console.log('Created temp file at:', tempPath)
        
        const result = await invoke('analyze_modpack_file', {
          modpackPath: tempPath
        })
        
        this.modpackInfo = result
        
        this.currentStep = ''
        
        console.log('Modpack analysis complete:', this.modpackInfo)
        console.log('Pagination debug:', {
          totalMods: this.modpackInfo.mods.length,
          totalPages: this.totalModsPages,
          currentPage: this.currentModsPage,
          hasNext: this.hasNextPage,
          hasPrev: this.hasPrevPage,
          paginatedMods: this.paginatedMods.length
        })
      } catch (error) {
        console.error('Error previewing modpack:', error)
        this.fileError = `Failed to analyze modpack file: ${error.message}`
        this.currentStep = ''
      }
    },
    
    async createTempFile(file) {
      const arrayBuffer = await this.fileToArrayBuffer(file)
      const tempPath = await invoke('create_temp_file', {
        fileName: file.name,
        fileData: Array.from(new Uint8Array(arrayBuffer))
      })
      
      return tempPath
    },
    
    async fileToArrayBuffer(file) {
      return new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.onload = () => resolve(reader.result)
        reader.onerror = reject
        reader.readAsArrayBuffer(file)
      })
    },
    
    async installModpack() {
      if (!this.canInstall) return
      
      try {
        this.installing = true
        this.installProgress = 0
        this.installLog = []

        this.$emit('install-start')
        
        const server = this.servers.find(s => s.id === this.selectedServerId)
        if (!server) {
          throw new Error('Selected server not found')
        }
        
        this.addLog('Starting modpack installation...')
        this.installProgress = 10
        
        const tempPath = await this.createTempFile(this.selectedFile)
        this.addLog(`Modpack file: ${this.selectedFile.name}`)
        this.addLog(`Target server: ${server.name}`)
        this.installProgress = 20
        
        this.currentStep = 'Installing modpack...'
        this.installProgress = 30
        
        await invoke('install_modpack_from_file', {
          modpackPath: tempPath,
          serverId: this.selectedServerId,
          serverPath: server.path
        })
        
        this.installProgress = 100
        
        this.addLog('Modpack installation completed successfully!')
        this.installProgress = 100
        this.currentStep = 'Installation complete'
        
        this.showSuccessDialog = true
        
      } catch (error) {
        console.error('Error installing modpack:', error)
        this.addLog(`Error: ${error.message}`)
        this.currentStep = 'Installation failed'
        
        if (window.showError) {
          window.showError('Installation Failed', error.message)
        } else {
          alert(`Failed to install modpack: ${error.message}`)
        }
      } finally {
        this.installing = false
        this.$emit('install-end')
      }
    },
    
    addLog(message) {
      this.installLog.push(`[${new Date().toLocaleTimeString()}] ${message}`)
      if (this.installLog.length > 50) {
        this.installLog = this.installLog.slice(-50)
      }
    },
    
    getServerName(serverId) {
      const server = this.servers.find(s => s.id === serverId)
      return server ? server.name : 'Unknown Server'
    },
    
    getModDisplayName(mod) {
      if (mod.name.startsWith('mod-')) {
        const filename = mod.filename.replace('.jar', '')
        if (/^\d+$/.test(filename)) {
          return `Mod ${mod.project_id || filename}`
        }
        return filename
      }
      return mod.name
    },
    
    resetForm() {
      this.selectedFile = null
      this.selectedServerId = null
      this.modpackInfo = null
      this.installing = false
      this.installProgress = 0
      this.currentStep = ''
      this.installLog = []
      this.fileError = ''
      this.showSuccessDialog = false
      this.currentModsPage = 1
    },
    
    goToPrevPage() {
      console.log('Going to previous page, current:', this.currentModsPage, 'hasPrev:', this.hasPrevPage)
      if (this.hasPrevPage) {
        this.currentModsPage--
        console.log('Now on page:', this.currentModsPage)
      }
    },
    
    goToNextPage() {
      if (this.hasNextPage) {
        this.currentModsPage++
        console.log('Now on page:', this.currentModsPage)
      }
    },
    
    handleCancel() {
      if (this.installing) {
        return
      }
      this.resetForm()
      this.$emit('close')
    }
  }
}
</script>

<style scoped>
.modpack-installer {
  max-width: 800px;
  margin: 0 auto;
}

.modpack-card {
  position: relative;
  overflow: hidden;
}

.green-blob {
  position: absolute;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(74, 222, 128, 0.3) 0%, rgba(74, 222, 128, 0.1) 70%, transparent 100%);
  filter: blur(2px);
  z-index: 0;
}

.blob-1 {
  width: 120px;
  height: 120px;
  top: -60px;
  right: -60px;
  animation: float 6s ease-in-out infinite;
}

.blob-2 {
  width: 80px;
  height: 80px;
  bottom: -40px;
  left: -40px;
  animation: float 8s ease-in-out infinite reverse;
}

.blob-3 {
  width: 60px;
  height: 60px;
  top: 50%;
  right: 10%;
  animation: float 10s ease-in-out infinite;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px) rotate(0deg);
  }
  50% {
    transform: translateY(-20px) rotate(180deg);
  }
}

.modpack-preview,
.installation-progress,
.mod-list-preview {
  background-color: rgba(42, 42, 42, 0.5) !important;
  border: 1px solid rgba(74, 222, 128, 0.1);
}

.mod-list-container {
  max-height: 250px;
  overflow-y: auto;
}

.log-container {
  max-height: 200px;
  overflow-y: auto;
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
}

.log-entry {
  margin-bottom: 2px;
  color: #4ade80;
}

.install-log {
  max-height: 300px;
  overflow-y: auto;
}

.v-list-item {
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.v-list-item:last-child {
  border-bottom: none;
}

.pagination-controls {
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  padding-top: 16px;
}
</style> 