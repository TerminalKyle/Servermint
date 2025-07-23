<template>
  <div>
    <!-- Update notification dialog -->
    <v-dialog v-model="showUpdateDialog" max-width="500" persistent>
      <v-card class="update-dialog">
        <v-card-title class="d-flex align-center">
          <v-icon color="primary" class="mr-3">mdi-update</v-icon>
          Update Available
        </v-card-title>
        
        <v-card-text>
          <div class="mb-4">
            <p class="text-body-1 mb-2">
              A new version of ServerMint is available: <strong>v{{ updateInfo.version }}</strong>
            </p>
            <p class="text-caption text-medium-emphasis">
              {{ updateInfo.body || 'Bug fixes and improvements' }}
            </p>
          </div>
          
          <v-progress-linear
            v-if="downloading"
            v-model="downloadProgress"
            color="primary"
            height="8"
            rounded
            class="mb-3"
          ></v-progress-linear>
          
          <div v-if="downloading" class="text-center text-caption text-medium-emphasis">
            Downloading update... {{ Math.round(downloadProgress) }}%
          </div>
        </v-card-text>
        
        <v-card-actions class="pa-4">
          <v-spacer></v-spacer>
          <v-btn
            v-if="!downloading && !downloaded"
            color="primary"
            @click="downloadUpdate"
            :loading="downloading"
          >
            Download & Install
          </v-btn>
          <v-btn
            v-if="downloaded"
            color="success"
            @click="installUpdate"
            :loading="installing"
          >
            Install Now
          </v-btn>
          <v-btn
            variant="outlined"
            @click="dismissUpdate"
            :disabled="downloading || installing"
          >
            Later
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    
    <!-- Update notification snackbar -->
    <v-snackbar
      v-model="showUpdateSnackbar"
      color="primary"
      timeout="10000"
      location="top"
    >
      <div class="d-flex align-center">
        <v-icon class="mr-2">mdi-update</v-icon>
        <span>Update available: v{{ updateInfo.version }}</span>
      </div>
      
      <template v-slot:actions>
        <v-btn
          variant="text"
          @click="showUpdateDialog = true"
        >
          View
        </v-btn>
        <v-btn
          variant="text"
          @click="showUpdateSnackbar = false"
        >
          Dismiss
        </v-btn>
      </template>
    </v-snackbar>
  </div>
</template>

<script>
import { check } from '@tauri-apps/plugin-updater'
import { invoke } from '@tauri-apps/api/core'
import { store } from '../store.js'

export default {
  name: 'UpdateManager',
  data() {
    return {
      showUpdateDialog: false,
      showUpdateSnackbar: false,
      downloading: false,
      downloaded: false,
      installing: false,
      downloadProgress: 0,
      updateInfo: {
        version: '',
        body: '',
        date: ''
      },
      checkInterval: null,
      lastCheckTime: 0
    }
  },
  async mounted() {
    // Check for updates on app start
    await this.checkForUpdates();
    
    // Set up periodic update checking (every 30 minutes)
    this.checkInterval = setInterval(() => {
      this.checkForUpdates();
    }, 30 * 60 * 1000);
  },
  beforeUnmount() {
    if (this.checkInterval) {
      clearInterval(this.checkInterval);
    }
  },
  methods: {
    async checkForUpdates() {
      try {
        const now = Date.now();
        // Don't check too frequently (minimum 5 minutes between checks)
        if (now - this.lastCheckTime < 5 * 60 * 1000) {
          return;
        }
        
        this.lastCheckTime = now;
        
        const update = await check();
        
        if (update) {
          this.updateInfo = {
            version: update.version,
            body: update.body,
            date: update.date
          };
          
          // Show notification
          this.showUpdateSnackbar = true;
          
          // Auto-download if enabled in settings
          if (this.shouldAutoDownload()) {
            this.downloadUpdate(update);
          }
        }
      } catch (error) {
        console.error('Error checking for updates:', error);
      }
    },
    
    shouldAutoDownload() {
      // Check if auto-download is enabled in settings
      return store.settings.general.autoDownloadUpdates !== false;
    },
    
    async downloadUpdate(update) {
      this.downloading = true;
      this.downloadProgress = 0;
      this.showUpdateDialog = true;
      
      try {
        let downloaded = 0;
        let contentLength = 0;
        
        // Download and install the update with progress tracking
        await update.downloadAndInstall((event) => {
          switch (event.event) {
            case 'Started':
              contentLength = event.data.contentLength;
              console.log(`Started downloading ${event.data.contentLength} bytes`);
              break;
            case 'Progress':
              downloaded += event.data.chunkLength;
              this.downloadProgress = Math.round((downloaded / contentLength) * 100);
              console.log(`Downloaded ${downloaded} from ${contentLength}`);
              break;
            case 'Finished':
              console.log('Download finished');
              this.downloadProgress = 100;
              break;
          }
        });
        
        this.downloaded = true;
        this.downloading = false;
        
        // Auto-install if enabled
        if (this.shouldAutoInstall()) {
          setTimeout(() => {
            this.installUpdate();
          }, 1000);
        }
      } catch (error) {
        this.downloading = false;
        console.error('Error downloading update:', error);
        this.$emit('update-error', 'Failed to download update');
      }
    },
    
    shouldAutoInstall() {
      // Check if auto-install is enabled in settings
      return store.settings.general.autoInstallUpdates !== false;
    },
    
    async installUpdate() {
      this.installing = true;
      
      try {
        // The update is already installed from downloadAndInstall
        console.log('Update installed successfully');
        this.$emit('update-installed', 'Update installed successfully');
        
        // Relaunch the app
        await invoke('plugin:process|relaunch');
      } catch (error) {
        this.installing = false;
        console.error('Error installing update:', error);
        this.$emit('update-error', 'Failed to install update');
      }
    },
    
    dismissUpdate() {
      this.showUpdateDialog = false;
      this.showUpdateSnackbar = false;
    }
  }
}
</script>

<style scoped>
.update-dialog {
  background-color: rgba(30, 30, 30, 0.95) !important;
  border: 1px solid rgba(74, 222, 128, 0.1);
}

.update-dialog .v-card-title {
  color: white;
  border-bottom: 1px solid rgba(74, 222, 128, 0.1);
}

.update-dialog .v-card-text {
  color: white;
}

.update-dialog .v-card-actions {
  border-top: 1px solid rgba(74, 222, 128, 0.1);
}
</style> 