import { createApp } from 'vue'
import App from './App.vue'
import { createVuetify } from 'vuetify'
import { createRouter, createWebHistory } from 'vue-router'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'

// Import Tauri APIs - using v2 import style
import { invoke } from '@tauri-apps/api/core'
import { fetch } from '@tauri-apps/plugin-http'

// Log that we're attempting to use Tauri
console.log('ServerMint is starting up...');
console.log('Tauri invoke API is available:', typeof invoke === 'function');
console.log('Tauri HTTP fetch API is available:', typeof fetch === 'function');

// Import views
import ServersView from './components/ServersView.vue'
import ModsView from './components/ModsView.vue'
import BackupsView from './components/BackupsView.vue'
import ServerManagementView from './components/ServerManagementView.vue'
import PremadeServersView from './components/PremadeServersView.vue'
import NodesView from './components/NodesView.vue'
import PluginSetupView from './components/PluginSetupView.vue'

// Define routes
const routes = [
  { path: '/', name: 'Servers', component: ServersView },
  { path: '/mods', name: 'Mods', component: ModsView },
  { path: '/plugin-setup', name: 'PluginSetup', component: PluginSetupView },
  { path: '/backups', name: 'Backups', component: BackupsView },
  { path: '/server/:serverId', name: 'ServerManagement', component: ServerManagementView },
  { path: '/premade', name: 'PremadeServers', component: PremadeServersView },
  { path: '/nodes', name: 'Nodes', component: NodesView },
  { path: '/:pathMatch(.*)*', redirect: '/' } // Redirect any unmatched routes to home
]

// Create router
const router = createRouter({
  history: createWebHistory(),
  routes
})

const vuetify = createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: 'dark',
    themes: {
      dark: {
        dark: true,
        colors: {
          primary: '#4ade80',     // Green accent color
          secondary: '#3f3f46',   // Gray
          accent: '#86efac',      // Light green
          background: '#121212',  // Very dark gray/black
          surface: '#1e1e1e',     // Dark gray
          error: '#ef4444',       // Red
          success: '#10b981',     // Green
          warning: '#f59e0b',     // Amber
          info: '#3b82f6'         // Blue
        }
      }
    }
  }
})

createApp(App)
  .use(vuetify)
  .use(router)
  .mount('#app')
