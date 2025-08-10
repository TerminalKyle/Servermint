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
              A new version of ServerMint is available:
              <strong>v{{ updateInfo.version }}</strong>
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
            v-if="!downloading"
            color="primary"
            @click="downloadAndInstallUpdate"
            :loading="downloading"
          >
            Download & Install
          </v-btn>
          <v-btn
            variant="outlined"
            @click="dismissUpdate"
            :disabled="downloading"
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
        <v-btn variant="text" @click="showUpdateDialog = true">View</v-btn>
        <v-btn variant="text" @click="showUpdateSnackbar = false">Dismiss</v-btn>
      </template>
    </v-snackbar>
  </div>
</template>

<script>
import { check, Update } from '@tauri-apps/plugin-updater';
import { markRaw } from 'vue';

export default {
  name: 'UpdateManager',
  data() {
    return {
      showUpdateDialog: false,
      showUpdateSnackbar: false,
      downloading: false,
      downloadProgress: 0,
      contentLength: null,
      updateInfo: { version: '', body: '' },
    };
  },
  created() {
    // Store update resource outside of Vue's reactivity
    this._updateResource = null;
  },
  mounted() {
    // Only run inside Tauri; noop in plain browser
    if (!this.isTauri()) return;
    // Small delay to ensure backend is ready
    setTimeout(() => this.checkForUpdates().catch(() => {}), 800);
  },
  beforeUnmount() {
    this.cleanupUpdateResource();
  },
  methods: {
    isTauri() {
      return typeof window !== 'undefined' && (window.__TAURI__ || window.__TAURI_INTERNALS__);
    },
    async checkForUpdates() {
      try {
        const update = await check({ timeout: 30000 });
        if (!update) return;

        // Store the raw update instance
        this._updateResource = markRaw(update);

        this.updateInfo.version = update.version || '';
        this.updateInfo.body = update.body || '';
        this.showUpdateSnackbar = true;
      } catch (err) {
        // Silently ignore in dev/offline; optionally log
        console.warn('Updater check failed:', err);
      }
    },
    async downloadAndInstallUpdate() {
      if (!this._updateResource) return;
      this.downloading = true;
      this.downloadProgress = 0;
      this.contentLength = null;

      let downloaded = 0;

      try {
        await this._updateResource.downloadAndInstall((evt) => {
          if (evt.event === 'Started') {
            this.contentLength = evt.data?.contentLength ?? null;
            downloaded = 0;
            this.downloadProgress = 0;
          } else if (evt.event === 'Progress') {
            downloaded += evt.data.chunkLength || 0;
            if (this.contentLength && this.contentLength > 0) {
              this.downloadProgress = Math.min(100, (downloaded / this.contentLength) * 100);
            }
          } else if (evt.event === 'Finished') {
            this.downloadProgress = 100;
          }
        }, { timeout: 300000 });
        // On Windows, app exits automatically during install (per docs).
        // On other platforms, installation completes without mandatory restart.
        this.showUpdateDialog = false;
        this.showUpdateSnackbar = false;
        if (window.showSuccess) {
          window.showSuccess('Update installed', 'ServerMint will restart or be ready on next launch.');
        }
      } catch (err) {
        console.error('Update install failed:', err);
        if (window.showError) {
          window.showError('Update failed', String(err));
        }
      } finally {
        this.downloading = false;
        this.cleanupUpdateResource();
      }
    },
    dismissUpdate() {
      this.showUpdateDialog = false;
      this.showUpdateSnackbar = false;
      this.cleanupUpdateResource();
    },
    async cleanupUpdateResource() {
      try {
        if (this._updateResource instanceof Update) {
          await this._updateResource.close?.();
        }
      } catch (err) {
        // Ignore cleanup errors - non-critical during unmount
      }
      this._updateResource = null;
    }
  }
};
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
