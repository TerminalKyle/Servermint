<template>
  <div>
    <v-dialog 
      v-model="dialog" 
      max-width="600px" 
      persistent
      :scrim="false"
      transition="dialog-bottom-transition"
      content-class="settings-dialog-container"
    >
      <template v-slot:activator="{ props }">
        <v-btn 
          icon 
          variant="text" 
          v-bind="props"
          class="settings-btn"
        >
          <v-icon>mdi-cog-outline</v-icon>
        </v-btn>
      </template>
      
      <v-card class="settings-dialog">
        <v-card-title class="text-h5 pb-4 pt-5 px-5 d-flex align-center">
          <v-icon class="mr-2">mdi-cog-outline</v-icon>
          Settings
          <v-btn icon="mdi-close" variant="text" size="small" @click="dialog = false" class="close-btn"></v-btn>
        </v-card-title>
        
        <v-divider></v-divider>
        
        <v-card-text class="pt-4">
          <v-tabs v-model="activeTab" color="primary">
            <v-tab value="general">GENERAL</v-tab>
            <v-tab value="appearance">APPEARANCE</v-tab>
            <v-tab value="java">JAVA</v-tab>
            <v-tab value="advanced">ADVANCED</v-tab>
          </v-tabs>
          
          <v-window v-model="activeTab" class="mt-4">
            <!-- General Settings -->
            <v-window-item value="general">
              <div class="settings-section">
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Application</h3>
                
                <v-checkbox
                  v-model="settings.general.startWithSystem"
                  label="Start with system"
                  color="primary"
                  hide-details
                  class="mb-3"
                ></v-checkbox>
                
                <v-checkbox
                  v-model="settings.general.checkForUpdates"
                  label="Check for updates automatically"
                  color="primary"
                  hide-details
                  class="mb-3"
                ></v-checkbox>
                
                <v-checkbox
                  v-model="settings.general.autoDownloadUpdates"
                  label="Automatically download updates"
                  color="primary"
                  hide-details
                  class="mb-3"
                ></v-checkbox>
                
                <v-checkbox
                  v-model="settings.general.autoInstallUpdates"
                  label="Automatically install updates"
                  color="primary"
                  hide-details
                  class="mb-3"
                ></v-checkbox>
                
                <v-checkbox
                  v-model="settings.general.minimizeToTray"
                  label="Minimize to system tray"
                  color="primary"
                  hide-details
                  class="mb-5"
                ></v-checkbox>
                
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Default Server Settings</h3>
                
                <v-text-field
                  v-model="settings.general.defaultServerPath"
                  label="Default server installation path"
                  variant="outlined"
                  density="comfortable"
                  bg-color="#1e1e1e"
                  append-inner-icon="mdi-folder-outline"
                  @click:append-inner="selectDefaultPath"
                  class="mb-3"
                ></v-text-field>
                
                <v-select
                  v-model="settings.general.defaultGameVersion"
                  label="Default game version"
                  :items="gameVersions"
                  variant="outlined"
                  density="comfortable"
                  bg-color="#1e1e1e"
                  class="mb-3"
                ></v-select>
              </div>
            </v-window-item>
            
            <!-- Appearance Settings -->
            <v-window-item value="appearance">
              <div class="settings-section">
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Theme</h3>
                
                <div class="d-flex align-center mb-4">
                  <span class="mr-4">Theme mode</span>
                  <v-switch
                    v-model="settings.appearance.darkMode"
                    color="primary"
                    hide-details
                    :label="settings.appearance.darkMode ? 'Dark' : 'Light'"
                  ></v-switch>
                </div>
                
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Accent Color</h3>
                
                <div class="d-flex flex-wrap gap-2 mb-4">
                  <v-btn
                    v-for="color in accentColors"
                    :key="color.value"
                    :color="color.value"
                    icon
                    size="large"
                    variant="flat"
                    :class="{ 'color-selected': settings.appearance.accentColor === color.value }"
                    @click="settings.appearance.accentColor = color.value"
                  >
                    <v-icon v-if="settings.appearance.accentColor === color.value">mdi-check</v-icon>
                  </v-btn>
                </div>
              </div>
            </v-window-item>
            
            <!-- Java Settings -->
            <v-window-item value="java">
              <div class="settings-section">
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Java Runtime</h3>
                
                <v-select
                  v-model="settings.java.javaPath"
                  label="Java installation"
                  :items="javaInstallations"
                  variant="outlined"
                  density="comfortable"
                  bg-color="#1e1e1e"
                  class="mb-3"
                ></v-select>
                
                <v-text-field
                  v-model="settings.java.customJavaPath"
                  label="Custom Java path"
                  variant="outlined"
                  density="comfortable"
                  bg-color="#1e1e1e"
                  append-inner-icon="mdi-folder-outline"
                  @click:append-inner="selectJavaPath"
                  class="mb-5"
                ></v-text-field>
                
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Memory Allocation</h3>
                
                <div class="mb-3">
                  <label class="d-block mb-2">Default memory allocation (MB)</label>
                  <div class="d-flex align-center">
                    <v-slider
                      v-model="settings.java.memory"
                      :min="1024"
                      :max="16384"
                      :step="512"
                      thumb-label
                      class="memory-slider"
                      color="primary"
                    ></v-slider>
                    <span class="ml-2">{{ settings.java.memory }} MB</span>
                  </div>
                </div>
                
                <v-checkbox
                  v-model="settings.java.useCustomJvmArgs"
                  label="Use custom JVM arguments"
                  color="primary"
                  hide-details
                  class="mb-3"
                ></v-checkbox>
                
                <v-textarea
                  v-model="settings.java.jvmArgs"
                  label="JVM arguments"
                  variant="outlined"
                  density="comfortable"
                  bg-color="#1e1e1e"
                  :disabled="!settings.java.useCustomJvmArgs"
                  rows="3"
                  class="mb-3"
                ></v-textarea>
              </div>
            </v-window-item>
            
            <!-- Advanced Settings -->
            <v-window-item value="advanced">
              <div class="settings-section">
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Data & Storage</h3>
                
                <v-text-field
                  v-model="settings.advanced.dataPath"
                  label="Application data path"
                  variant="outlined"
                  density="comfortable"
                  bg-color="#1e1e1e"
                  append-inner-icon="mdi-folder-outline"
                  @click:append-inner="selectDataPath"
                  class="mb-3"
                ></v-text-field>
                
                <div class="d-flex mb-5">
                  <v-btn color="error" variant="outlined" class="mr-2">Clear Cache</v-btn>
                  <v-btn color="error" variant="outlined">Reset All Settings</v-btn>
                </div>
                
                <h3 class="text-subtitle-1 font-weight-bold mb-3">Advanced Options</h3>
                
                <v-checkbox
                  v-model="settings.advanced.enableLogging"
                  label="Enable detailed logging"
                  color="primary"
                  hide-details
                  class="mb-3"
                ></v-checkbox>
                
                <v-checkbox
                  v-model="settings.advanced.enableDevMode"
                  label="Developer mode"
                  color="primary"
                  hide-details
                  class="mb-3"
                ></v-checkbox>
              </div>
            </v-window-item>
          </v-window>
        </v-card-text>
        
        <v-divider></v-divider>
        
        <v-card-actions class="pa-4">
          <v-btn 
            variant="outlined" 
            color="grey" 
            @click="dialog = false"
            class="text-uppercase"
          >
            Cancel
          </v-btn>
          <v-spacer></v-spacer>
          <v-btn 
            color="primary" 
            @click="saveSettings"
            class="text-uppercase"
          >
            Save
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script>
export default {
  name: 'SettingsModal',
  data() {
    return {
      dialog: false,
      activeTab: 'general',
      settings: {
        general: {
          startWithSystem: true,
          checkForUpdates: true,
          autoDownloadUpdates: true,
          autoInstallUpdates: true,
          minimizeToTray: false,
          defaultServerPath: 'C:/minecraft/servers',
          defaultGameVersion: '1.21.2'
        },
        appearance: {
          darkMode: true,
          accentColor: '#4ade80'
        },
        java: {
          javaPath: 'auto',
          customJavaPath: '',
          memory: 4096,
          useCustomJvmArgs: false,
          jvmArgs: '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200'
        },
        advanced: {
          dataPath: 'C:/Users/user/AppData/Roaming/ServerMint',
          enableLogging: false,
          enableDevMode: false
        }
      },
      gameVersions: ['1.21.2', '1.20.4', '1.20.1', '1.19.4', '1.19.2', '1.18.2', '1.16.5', '1.12.2'],
      javaInstallations: [
        { title: 'Auto-detect (recommended)', value: 'auto' },
        { title: 'Java 17 (C:/Program Files/Java/jdk-17)', value: 'java17' },
        { title: 'Java 8 (C:/Program Files/Java/jdk1.8.0_301)', value: 'java8' },
        { title: 'Custom', value: 'custom' }
      ],
      accentColors: [
        { name: 'Green', value: '#4ade80' },
        { name: 'Blue', value: '#3b82f6' },
        { name: 'Purple', value: '#8b5cf6' },
        { name: 'Pink', value: '#ec4899' },
        { name: 'Red', value: '#ef4444' },
        { name: 'Orange', value: '#f97316' },
        { name: 'Yellow', value: '#eab308' },
        { name: 'Teal', value: '#14b8a6' }
      ]
    }
  },
  mounted() {
    document.addEventListener('keydown', this.handleKeyDown);
  },
  beforeUnmount() {
    document.removeEventListener('keydown', this.handleKeyDown);
  },
  methods: {
    selectDefaultPath() {
      // In a real app, this would open a file picker
      console.log('Select default server path');
    },
    selectJavaPath() {
      // In a real app, this would open a file picker
      console.log('Select Java path');
    },
    selectDataPath() {
      // In a real app, this would open a file picker
      console.log('Select data path');
    },
    saveSettings() {
      // In a real app, this would save settings to storage
      console.log('Saving settings:', this.settings);
      this.$emit('settings-updated', this.settings);
      this.dialog = false;
    },
    handleKeyDown(event) {
      if (event.key === 'Escape' && this.dialog) {
        this.dialog = false;
      }
    }
  }
}
</script>

<style>
.v-dialog.settings-dialog-container {
  position: relative;
}

.v-dialog.settings-dialog-container::before {
  content: '';
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  z-index: -1;
  background-image: radial-gradient(circle at 50% 50%, rgba(74, 222, 128, 0.15) 0%, transparent 70%);
}
</style>

<style scoped>
.settings-dialog {
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
.settings-section {
  padding: 8px;
}
.memory-slider {
  max-width: 300px;
}
.color-selected {
  border: 2px solid white !important;
}
.gap-2 {
  gap: 8px;
}
</style> 