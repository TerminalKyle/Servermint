import { createApp } from 'vue'
import App from './App.vue'
import { createVuetify } from 'vuetify'
import { createRouter, createWebHistory } from 'vue-router'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'

import { invoke } from '@tauri-apps/api/core'
import { fetch } from '@tauri-apps/plugin-http'

console.log('ServerMint is starting up...');
console.log('Tauri invoke API is available:', typeof invoke === 'function');
console.log('Tauri HTTP fetch API is available:', typeof fetch === 'function');

import ServersView from './components/ServersView.vue'
import ModsView from './components/ModsView.vue'
import BackupsView from './components/BackupsView.vue'
import ServerManagementView from './components/ServerManagementView.vue'
import PremadeServersView from './components/PremadeServersView.vue'
import NodesView from './components/NodesView.vue'
import PluginSetupView from './components/PluginSetupView.vue'
import EggManagerView from './components/EggManagerView.vue'

const routes = [
  { path: '/', name: 'Servers', component: ServersView },
  { path: '/mods', name: 'Mods', component: ModsView },
  { path: '/plugin-setup', name: 'PluginSetup', component: PluginSetupView },
  { path: '/backups', name: 'Backups', component: BackupsView },
  { path: '/server/:serverId', name: 'ServerManagement', component: ServerManagementView },
  { path: '/premade', name: 'PremadeServers', component: PremadeServersView },
  { path: '/nodes', name: 'Nodes', component: NodesView },
  { path: '/eggs', name: 'Eggs', component: EggManagerView },
  { path: '/:pathMatch(.*)*', redirect: '/' } 
]

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
          primary: '#4ade80',     
          secondary: '#3f3f46',   
          accent: '#86efac',      
          background: '#121212',  
          surface: '#1e1e1e',     
          error: '#ef4444',       
          success: '#10b981',     
          warning: '#f59e0b',     
          info: '#3b82f6'         
        }
      }
    }
  },
  defaults: {
    global: {
      fontFamily: 'Space Grotesk, sans-serif'
    }
  }
})

createApp(App)
  .use(vuetify)
  .use(router)
  .mount('#app')
