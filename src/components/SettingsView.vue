<template>
  <div>
    <v-row>
      <v-col cols="12">
        <v-card class="mb-4">
          <v-card-title>Application Settings</v-card-title>
        </v-card>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" md="8">
        <v-card>
          <v-tabs v-model="tab" bg-color="surface">
            <v-tab value="general">General</v-tab>
            <v-tab value="java">Java Settings</v-tab>
            <v-tab value="minecraft">Minecraft</v-tab>
            <v-tab value="advanced">Advanced</v-tab>
          </v-tabs>
          
          <v-card-text>
            <v-window v-model="tab">
              <v-window-item value="general">
                <v-form class="mt-4">
                  <v-row>
                    <v-col cols="12">
                      <v-text-field
                        label="Default Server Directory"
                        v-model="settings.defaultServerDir"
                        variant="outlined"
                        append-icon="mdi-folder-open"
                      ></v-text-field>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-text-field
                        label="Default Backup Directory"
                        v-model="settings.defaultBackupDir"
                        variant="outlined"
                        append-icon="mdi-folder-open"
                      ></v-text-field>
                    </v-col>
                    
                    <v-col cols="12" md="6">
                      <v-switch
                        v-model="settings.startWithSystem"
                        label="Start with system"
                        color="primary"
                      ></v-switch>
                    </v-col>
                    
                    <v-col cols="12" md="6">
                      <v-switch
                        v-model="settings.minimizeToTray"
                        label="Minimize to tray"
                        color="primary"
                      ></v-switch>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-select
                        label="Theme"
                        v-model="settings.theme"
                        :items="['System Default', 'Light', 'Dark', 'Purple Dark']"
                        variant="outlined"
                      ></v-select>
                    </v-col>
                  </v-row>
                </v-form>
              </v-window-item>
              
              <v-window-item value="java">
                <v-form class="mt-4">
                  <v-row>
                    <v-col cols="12">
                      <v-text-field
                        label="Java Path"
                        v-model="settings.javaPath"
                        variant="outlined"
                        append-icon="mdi-folder-open"
                      ></v-text-field>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-text-field
                        label="Default JVM Arguments"
                        v-model="settings.jvmArgs"
                        variant="outlined"
                        hint="Arguments passed to the Java Virtual Machine"
                        persistent-hint
                      ></v-text-field>
                    </v-col>
                    
                    <v-col cols="12" md="6">
                      <v-text-field
                        label="Minimum Memory (MB)"
                        v-model="settings.minMemory"
                        type="number"
                        variant="outlined"
                      ></v-text-field>
                    </v-col>
                    
                    <v-col cols="12" md="6">
                      <v-text-field
                        label="Maximum Memory (MB)"
                        v-model="settings.maxMemory"
                        type="number"
                        variant="outlined"
                      ></v-text-field>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-btn color="primary" @click="detectJava">
                        Auto-detect Java Installations
                      </v-btn>
                    </v-col>
                  </v-row>
                </v-form>
              </v-window-item>
              
              <v-window-item value="minecraft">
                <v-form class="mt-4">
                  <v-row>
                    <v-col cols="12">
                      <v-text-field
                        label="Minecraft Directory"
                        v-model="settings.minecraftDir"
                        variant="outlined"
                        append-icon="mdi-folder-open"
                      ></v-text-field>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-select
                        label="Default Server Software"
                        v-model="settings.defaultServerSoftware"
                        :items="['Vanilla', 'Paper', 'Spigot', 'Forge', 'Fabric']"
                        variant="outlined"
                      ></v-select>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-select
                        label="Default Minecraft Version"
                        v-model="settings.defaultMinecraftVersion"
                        :items="['1.20.1', '1.19.4', '1.18.2', '1.17.1', '1.16.5']"
                        variant="outlined"
                      ></v-select>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-switch
                        v-model="settings.autoDownloadServerSoftware"
                        label="Auto-download server software when creating new server"
                        color="primary"
                      ></v-switch>
                    </v-col>
                  </v-row>
                </v-form>
              </v-window-item>
              
              <v-window-item value="advanced">
                <v-form class="mt-4">
                  <v-row>
                    <v-col cols="12">
                      <v-select
                        label="Log Level"
                        v-model="settings.logLevel"
                        :items="['Error', 'Warning', 'Info', 'Debug']"
                        variant="outlined"
                      ></v-select>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-switch
                        v-model="settings.enableCrashReporting"
                        label="Enable crash reporting"
                        color="primary"
                      ></v-switch>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-switch
                        v-model="settings.checkForUpdates"
                        label="Check for updates automatically"
                        color="primary"
                      ></v-switch>
                    </v-col>
                    
                    <v-col cols="12">
                      <v-btn color="error" @click="resetSettings">
                        Reset All Settings
                      </v-btn>
                    </v-col>
                  </v-row>
                </v-form>
              </v-window-item>
            </v-window>
          </v-card-text>
          
          <v-divider></v-divider>
          
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn color="secondary" @click="resetTab">
              Reset Tab
            </v-btn>
            <v-btn color="primary" @click="saveSettings">
              Save Settings
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
      
      <v-col cols="12" md="4">
        <v-card>
          <v-card-title>About</v-card-title>
          <v-card-text>
            <div class="text-center mb-4">
              <v-avatar size="100" color="primary" class="mb-2">
                <v-icon size="64" color="white">mdi-minecraft</v-icon>
              </v-avatar>
              <h2 class="text-h5">Manticore</h2>
              <p class="text-subtitle-1">Minecraft Server Manager</p>
              <p class="text-caption">Version 0.1.0</p>
            </div>
            
            <v-divider class="mb-4"></v-divider>
            
            <p class="text-body-2 mb-2">
              Manticore is a desktop application designed to help Minecraft server owners and modders easily manage multiple Minecraft servers locally on their machine.
            </p>
            
            <p class="text-body-2 mb-4">
              Built with Vue.js and Tauri.
            </p>
            
            <div class="d-flex justify-center">
              <v-btn color="primary" variant="text" prepend-icon="mdi-github">
                GitHub
              </v-btn>
              <v-btn color="primary" variant="text" prepend-icon="mdi-help-circle">
                Help
              </v-btn>
            </div>
          </v-card-text>
        </v-card>
        
        <v-card class="mt-4">
          <v-card-title>System Info</v-card-title>
          <v-card-text>
            <v-list density="compact">
              <v-list-item>
                <template v-slot:prepend>
                  <v-icon icon="mdi-desktop-classic"></v-icon>
                </template>
                <v-list-item-title>OS</v-list-item-title>
                <v-list-item-subtitle>{{ systemInfo.os }}</v-list-item-subtitle>
              </v-list-item>
              
              <v-list-item>
                <template v-slot:prepend>
                  <v-icon icon="mdi-memory"></v-icon>
                </template>
                <v-list-item-title>Memory</v-list-item-title>
                <v-list-item-subtitle>{{ systemInfo.memory }}</v-list-item-subtitle>
              </v-list-item>
              
              <v-list-item>
                <template v-slot:prepend>
                  <v-icon icon="mdi-coffee"></v-icon>
                </template>
                <v-list-item-title>Java Version</v-list-item-title>
                <v-list-item-subtitle>{{ systemInfo.javaVersion }}</v-list-item-subtitle>
              </v-list-item>
              
              <v-list-item>
                <template v-slot:prepend>
                  <v-icon icon="mdi-harddisk"></v-icon>
                </template>
                <v-list-item-title>Free Disk Space</v-list-item-title>
                <v-list-item-subtitle>{{ systemInfo.freeSpace }}</v-list-item-subtitle>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script>
export default {
  name: 'SettingsView',
  data() {
    return {
      tab: 'general',
      settings: {
        // General settings
        defaultServerDir: 'C:/minecraft/servers',
        defaultBackupDir: 'C:/minecraft/backups',
        startWithSystem: false,
        minimizeToTray: true,
        theme: 'Purple Dark',
        
        // Java settings
        javaPath: 'C:/Program Files/Java/jre1.8.0_301/bin/java.exe',
        jvmArgs: '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200',
        minMemory: 1024,
        maxMemory: 4096,
        
        // Minecraft settings
        minecraftDir: 'C:/Users/username/AppData/Roaming/.minecraft',
        defaultServerSoftware: 'Paper',
        defaultMinecraftVersion: '1.20.1',
        autoDownloadServerSoftware: true,
        
        // Advanced settings
        logLevel: 'Info',
        enableCrashReporting: true,
        checkForUpdates: true
      },
      systemInfo: {
        os: 'Windows 10 Pro 64-bit',
        memory: '16 GB',
        javaVersion: 'Java 17.0.2',
        freeSpace: '120 GB'
      }
    }
  },
  methods: {
    saveSettings() {
      // In a real app, this would communicate with Tauri to save settings
      console.log('Saving settings', this.settings)
    },
    resetTab() {
      // Reset only the current tab's settings
      console.log('Reset tab', this.tab)
    },
    resetSettings() {
      // Reset all settings
      console.log('Reset all settings')
    },
    detectJava() {
      // Auto-detect Java installations
      console.log('Detecting Java installations')
    }
  }
}
</script> 