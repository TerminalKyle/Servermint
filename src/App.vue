<template>
  <!-- Splash screen -->
  <SplashScreen 
    v-if="shouldShowSplash" 
    @animation-complete="onSplashComplete"
  />
  
  <!-- Installer view for first run -->
  <InstallerView 
    v-if="showInstaller && !shouldShowSplash" 
    @installation-complete="onInstallationComplete"
  />
  
  <!-- Main app -->
  <v-app v-else-if="!shouldShowSplash" class="app-bg">
    <!-- MintMenu - Global Command Palette -->
    <MintMenu />
    <!-- Green glow elements -->
    <div class="glow-effect"></div>
    <div class="glow-effect-2"></div>
    <div class="glow-effect-3"></div>
    <div class="glow-effect-4"></div>
    
    <!-- Left sidebar -->
    <v-navigation-drawer
      :model-value="true"
      width="80"
      class="sidebar-nav"
      elevation="3"
      height="100vh"
      location="left"
    >
      <div class="d-flex flex-column h-100 pa-3">
        <div class="text-center py-4">
          <v-avatar size="40" class="mb-4 elevation-2">
            <v-img src="/servermint.png" alt="ServerMint Logo"></v-img>
          </v-avatar>
        </div>
        
        <v-list nav density="compact" class="sidebar-list">
          <v-list-item
            prepend-icon="mdi-server"
            value="servers"
            :active="$route.path === '/'"
            @click="$router.push('/')"
            active-color="primary"
            class="sidebar-item mb-2"
            :class="{ 'active-item': $route.path === '/' }"
            rounded="lg"
          ></v-list-item>
          
          <v-list-item
            prepend-icon="mdi-puzzle-outline"
            value="mods"
            :active="$route.path === '/mods'"
            @click="$router.push('/mods')"
            active-color="primary"
            class="sidebar-item mb-2"
            :class="{ 'active-item': $route.path === '/mods' }"
            rounded="lg"
          ></v-list-item>
          
          <v-list-item
            prepend-icon="mdi-plus-circle"
            value="plugin-setup"
            :active="$route.path === '/plugin-setup'"
            @click="$router.push('/plugin-setup')"
            active-color="primary"
            class="sidebar-item mb-2"
            :class="{ 'active-item': $route.path === '/plugin-setup' }"
            rounded="lg"
          ></v-list-item>
          
          <v-list-item
            prepend-icon="mdi-backup-restore"
            value="backups"
            :active="$route.path === '/backups'"
            @click="$router.push('/backups')"
            active-color="primary"
            class="sidebar-item mb-2"
            :class="{ 'active-item': $route.path === '/backups' }"
            rounded="lg"
          ></v-list-item>
          
          <v-list-item
            prepend-icon="mdi-server-network"
            value="nodes"
            :active="$route.path === '/nodes'"
            @click="$router.push('/nodes')"
            active-color="primary"
            class="sidebar-item mb-2"
            :class="{ 'active-item': $route.path === '/nodes' }"
            rounded="lg"
          ></v-list-item>
        </v-list>
        
        <v-spacer></v-spacer>
        
        <!-- MintMenu hint -->
        <div class="mint-menu-hint">
          <div class="hint-text">
            <span class="hint-label">Quick Actions</span>
            <div class="keyboard-shortcut">
              <kbd>{{ isMac ? '⌘' : 'Ctrl' }}</kbd>
              <span class="plus">+</span>
              <kbd>K</kbd>
            </div>
          </div>
        </div>
        
        <v-list nav density="compact" class="sidebar-list">
          <v-list-item class="settings-item">
            <SettingsModal @settings-updated="updateSettings" />
          </v-list-item>
        </v-list>
      </div>
    </v-navigation-drawer>

    <!-- Main content area -->
    <v-main class="main-content">
      <v-container fluid class="pa-6 main-container">
        <div class="d-flex align-center mb-6">
          <div>
            <h1 class="text-h5 font-weight-bold">{{ pageTitle }}</h1>
          </div>
          <v-spacer></v-spacer>
          
          <div class="d-flex align-center">
            <span class="text-caption text-medium-emphasis mr-2">Dark Mode</span>
            <v-switch
              density="compact"
              color="primary"
              hide-details
              model-value="true"
              class="mr-4"
            ></v-switch>
            
            <GuideButton 
              @restart-guide="restartGuide"
              class="mr-2"
            />
            
            <v-btn icon variant="text" class="mr-2 main-icon">
              <v-icon>mdi-bell-outline</v-icon>
              <v-badge
                color="primary"
                content="2"
                dot
                floating
                offset-x="3"
                offset-y="3"
              ></v-badge>
            </v-btn>
            
            <v-avatar size="36" color="surface" class="elevation-1">
              <v-icon color="white">mdi-account-circle</v-icon>
            </v-avatar>
          </div>
        </div>
        
        <router-view />
      </v-container>
    </v-main>
    
    <!-- Toast notifications -->
    <ToastNotification />
    
    <!-- Update manager -->
    <UpdateManager 
      @update-error="handleUpdateError"
      @update-installed="handleUpdateInstalled"
    />
    
    <!-- Guide overlay -->
    <GuideOverlay ref="guideOverlay" />
  </v-app>
</template>

<script>
import SettingsModal from './components/SettingsModal.vue'
import ToastNotification from './components/ToastNotification.vue'
import InstallerView from './components/InstallerView.vue'
import UpdateManager from './components/UpdateManager.vue'
import SplashScreen from './components/SplashScreen.vue'
import MintMenu from './components/MintMenu.vue'
import GuideOverlay from './components/GuideOverlay.vue'
import GuideButton from './components/GuideButton.vue'
import { store } from './store.js'

export default {
  name: 'App',
  components: {
    SettingsModal,
    ToastNotification,
    InstallerView,
    UpdateManager,
    SplashScreen,
    MintMenu,
    GuideOverlay,
    GuideButton
  },
  computed: {
    pageTitle() {
      const path = this.$route.path;
      
      if (path.startsWith('/server/')) {
        return 'Server Management';
      }
      
      const titles = {
        '/': 'Server Library',
        '/premade': 'Premade Servers',
        '/mods': 'Mods & Plugins',
        '/plugin-setup': 'Plugin Setup',
        '/backups': 'Backups',
        '/nodes': 'Server Nodes'
      };
      
      return titles[path] || 'ServerMint';
    },
    shouldShowSplash() {
      // Check if user has disabled splash screen in settings
      return this.showSplash;
    },
    isMac() {
      return navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    }
  },
  data() {
    return {
      store: store,
      showInstaller: false,
      showSplash: false
    }
  },
  watch: {
    shouldShowSplash(newVal, oldVal) {
      console.log('shouldShowSplash changed from', oldVal, 'to', newVal);
    }
  },
  async mounted() {
    // Load servers from backend when app starts (after splash)
    console.log('App mounted, loading servers...');
    try {
      await this.store.loadServers();
      console.log('Servers loaded on app startup');
    } catch (error) {
      console.error('Error loading servers on app startup:', error);
    }
  },
  methods: {
    onSplashComplete() {
      console.log('Splash complete event received');
      console.log('Setting showSplash to false');
      this.showSplash = false;
      console.log('showSplash is now:', this.showSplash);
      console.log('shouldShowSplash is now:', this.shouldShowSplash);
      // Check if this is the first run after splash
      this.checkFirstRun();
    },
    
    async checkFirstRun() {
      // Check if this is the first time running the app
      if (this.store.settings.general.firstRun !== false) {
        this.showInstaller = true;
      }
    },
    
    onInstallationComplete() {
      this.showInstaller = false;
      // Show a welcome message
      this.store.showToast('Welcome to ServerMint!', 'success');
    },
    
    updateSettings(settings) {
      console.log('Settings updated:', settings);
      // In a real app, this would update the app's settings
    },
    
    handleUpdateError(message) {
      this.store.showToast(message, 'error');
    },
    
    handleUpdateInstalled(message) {
      this.store.showToast(message, 'success');
    },
    
    restartGuide() {
      // Restart the guide
      this.$refs.guideOverlay.startGuide();
    }
  }
}
</script>

<style>
html, body {
  overflow: hidden;
}
#app {
  font-family: 'Roboto', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  height: 100vh;
}
.app-bg {
  background-color: #121212;
  position: relative;
  overflow: hidden;
}

/* Green glow background effects in corners and around the interface */
.app-bg::before,
.app-bg::after,
.app-bg .glow-effect,
.app-bg .glow-effect-2,
.app-bg .glow-effect-3,
.app-bg .glow-effect-4 {
  content: '';
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  z-index: 0;
  pointer-events: none;
}

/* Top-left corner */
.app-bg::before {
  background: radial-gradient(circle, rgba(74, 222, 128, 0.2) 0%, rgba(74, 222, 128, 0) 70%);
  width: 500px;
  height: 500px;
  top: -200px;
  left: -200px;
}

/* Bottom-right corner */
.app-bg::after {
  background: radial-gradient(circle, rgba(74, 222, 128, 0.2) 0%, rgba(74, 222, 128, 0) 70%);
  width: 500px;
  height: 500px;
  bottom: -200px;
  right: -200px;
}

/* Top-right corner */
.app-bg .glow-effect {
  background: radial-gradient(circle, rgba(74, 222, 128, 0.15) 0%, rgba(74, 222, 128, 0) 70%);
  width: 400px;
  height: 400px;
  top: -150px;
  right: -150px;
}

/* Bottom-left corner */
.app-bg .glow-effect-2 {
  background: radial-gradient(circle, rgba(74, 222, 128, 0.15) 0%, rgba(74, 222, 128, 0) 70%);
  width: 400px;
  height: 400px;
  bottom: -150px;
  left: -150px;
}

/* Middle-right area */
.app-bg .glow-effect-3 {
  background: radial-gradient(circle, rgba(74, 222, 128, 0.1) 0%, rgba(74, 222, 128, 0) 70%);
  width: 300px;
  height: 300px;
  top: 40%;
  right: -50px;
}

/* Middle area */
.app-bg .glow-effect-4 {
  background: radial-gradient(circle, rgba(74, 222, 128, 0.08) 0%, rgba(74, 222, 128, 0) 70%);
  width: 350px;
  height: 350px;
  top: 30%;
  left: 40%;
}

.sidebar-nav {
  background-color: rgba(18, 18, 18, 0.95) !important;
  border-right: 1px solid rgba(74, 222, 128, 0.1) !important;
  position: fixed !important;
  top: 0 !important;
  left: 0 !important;
  z-index: 1000 !important;
  height: 100vh !important;
  transform: none !important;
}
.sidebar-list {
  background-color: transparent;
}
.sidebar-item {
  min-height: 44px;
  border-radius: 8px;
  margin-left: 0;
  margin-right: 0;
  transition: all 0.2s ease;
}
.sidebar-item.active-item {
  background-color: rgba(74, 222, 128, 0.1);
  border-left: 3px solid var(--v-theme-primary);
}
.sidebar-item:hover:not(.active-item) {
  background-color: rgba(255, 255, 255, 0.05);
}
.sidebar-item .v-icon {
  transition: transform 0.2s ease;
}
.sidebar-item:hover .v-icon {
  transform: scale(1.1);
}
.settings-item {
  display: flex;
  justify-content: center;
  padding: 0;
  min-height: 44px;
}
.settings-item :deep(.settings-btn) {
  width: 100%;
  height: 100%;
  border-radius: 8px;
}
.settings-item :deep(.settings-btn:hover) {
  background-color: rgba(255, 255, 255, 0.05);
}

.mint-menu-hint {
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  margin-top: 8px;
  padding: 12px 8px;
  text-align: center;
}

.hint-text {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
}

.hint-label {
  color: rgba(255, 255, 255, 0.5);
  font-size: 10px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.keyboard-shortcut {
  display: flex;
  align-items: center;
  gap: 2px;
}

.keyboard-shortcut kbd {
  background-color: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 3px;
  padding: 1px 4px;
  font-size: 10px;
  font-family: 'JetBrains Mono', 'Consolas', monospace;
  color: rgba(255, 255, 255, 0.8);
  font-weight: 500;
}

.keyboard-shortcut .plus {
  color: rgba(255, 255, 255, 0.3);
  font-size: 9px;
  font-weight: 500;
  margin: 0 1px;
}
.main-container {
  max-width: 1600px;
  position: relative;
  z-index: 1;
}
.v-card {
  background-color: rgba(30, 30, 30, 0.95) !important;
  border: 1px solid rgba(74, 222, 128, 0.05);
}

/* Make non-sidebar icons white */
.main-content .v-icon {
  color: white !important;
}

/* Hide scrollbar but keep functionality */
::-webkit-scrollbar {
  width: 0;
  height: 0;
  display: none;
}

/* For Firefox */
* {
  scrollbar-width: none;
}

/* For IE and Edge */
* {
  -ms-overflow-style: none;
}

.main-content {
  overflow-y: auto;
  height: 100vh;
  margin-left: 80px !important;
}
</style>