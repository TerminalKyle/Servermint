<template>
  <div>
    <v-row>
      <v-col cols="12">
        <v-card class="mb-4">
          <v-card-title class="d-flex align-center">
            <span>Server Backups</span>
            <v-spacer></v-spacer>
            <v-btn color="primary" prepend-icon="mdi-backup-restore">
              Create New Backup
            </v-btn>
          </v-card-title>
        </v-card>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" md="3">
        <v-card>
          <v-card-title>Filters</v-card-title>
          <v-card-text>
            <v-select
              label="Server"
              :items="servers"
              item-title="name"
              item-value="id"
              v-model="selectedServer"
              variant="outlined"
              density="compact"
              class="mb-2"
            ></v-select>
            
            <v-select
              label="Backup Type"
              :items="['All', 'World', 'Config', 'Full']"
              v-model="selectedType"
              variant="outlined"
              density="compact"
              class="mb-2"
            ></v-select>
            
            <v-text-field
              label="Search"
              v-model="searchQuery"
              prepend-inner-icon="mdi-magnify"
              variant="outlined"
              density="compact"
              class="mb-2"
            ></v-text-field>
            
            <v-checkbox
              label="Show auto-backups"
              v-model="showAutoBackups"
              hide-details
            ></v-checkbox>
          </v-card-text>
        </v-card>
        
        <v-card class="mt-4">
          <v-card-title>Backup Settings</v-card-title>
          <v-card-text>
            <v-switch
              v-model="autoBackupEnabled"
              label="Enable auto-backup"
              color="primary"
              hide-details
              class="mb-4"
            ></v-switch>
            
            <v-select
              label="Frequency"
              :items="['Daily', 'Weekly', 'Monthly']"
              v-model="backupFrequency"
              :disabled="!autoBackupEnabled"
              variant="outlined"
              density="compact"
              class="mb-2"
            ></v-select>
            
            <v-select
              label="Retention Policy"
              :items="['Keep 5 latest', 'Keep 10 latest', 'Keep 1 month', 'Keep 3 months', 'Keep all']"
              v-model="retentionPolicy"
              variant="outlined"
              density="compact"
              class="mb-2"
            ></v-select>
            
            <v-text-field
              label="Backup Location"
              v-model="backupLocation"
              variant="outlined"
              density="compact"
              class="mb-2"
              readonly
              append-icon="mdi-folder-open"
            ></v-text-field>
            
            <v-btn block color="primary" class="mt-2">
              Save Settings
            </v-btn>
          </v-card-text>
        </v-card>
      </v-col>
      
      <v-col cols="12" md="9">
        <v-data-table
          :headers="headers"
          :items="backups"
          :search="searchQuery"
          class="elevation-1"
        >
          <template #[`item.timestamp`]="{ item }">
            {{ new Date(item.timestamp).toLocaleString() }}
          </template>
          
          <template #[`item.size`]="{ item }">
            {{ formatSize(item.size) }}
          </template>
          
          <template #[`item.type`]="{ item }">
            <v-chip :color="getTypeColor(item.type)" size="small">
              {{ item.type }}
            </v-chip>
          </template>
          
          <template #[`item.actions`]="{ item }">
            <v-btn icon size="small" color="primary" @click="restoreBackup(item)">
              <v-icon color="white">mdi-restore</v-icon>
            </v-btn>
            <v-btn icon size="small" color="secondary" class="ml-2" @click="downloadBackup(item)">
              <v-icon color="white">mdi-download</v-icon>
            </v-btn>
            <v-btn icon size="small" color="error" class="ml-2" @click="deleteBackup(item)">
              <v-icon color="white">mdi-delete</v-icon>
            </v-btn>
          </template>
        </v-data-table>
      </v-col>
    </v-row>
  </div>
</template>

<script>
export default {
  name: 'BackupsView',
  data() {
    return {
      selectedServer: 1,
      selectedType: 'All',
      searchQuery: '',
      showAutoBackups: true,
      autoBackupEnabled: true,
      backupFrequency: 'Daily',
      retentionPolicy: 'Keep 10 latest',
      backupLocation: 'C:/minecraft/backups',
      headers: [
        { title: 'Name', key: 'name' },
        { title: 'Server', key: 'server' },
        { title: 'Type', key: 'type' },
        { title: 'Date', key: 'timestamp' },
        { title: 'Size', key: 'size' },
        { title: 'Actions', key: 'actions', sortable: false }
      ],
      servers: [
        { id: 1, name: 'Survival Server' },
        { id: 2, name: 'Creative Server' }
      ],
      backups: [
        {
          id: 1,
          name: 'Survival-Full-20230715',
          server: 'Survival Server',
          type: 'Full',
          timestamp: '2023-07-15T14:30:00',
          size: 1572864000, // 1.5 GB in bytes
          auto: false
        },
        {
          id: 2,
          name: 'Survival-World-20230720',
          server: 'Survival Server',
          type: 'World',
          timestamp: '2023-07-20T08:15:00',
          size: 1048576000, // 1 GB in bytes
          auto: true
        },
        {
          id: 3,
          name: 'Creative-Full-20230718',
          server: 'Creative Server',
          type: 'Full',
          timestamp: '2023-07-18T22:45:00',
          size: 524288000, // 500 MB in bytes
          auto: false
        },
        {
          id: 4,
          name: 'Survival-Config-20230721',
          server: 'Survival Server',
          type: 'Config',
          timestamp: '2023-07-21T16:20:00',
          size: 1048576, // 1 MB in bytes
          auto: false
        }
      ]
    }
  },
  methods: {
    formatSize(bytes) {
      const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
      if (bytes === 0) return '0 Bytes'
      const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)))
      return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i]
    },
    getTypeColor(type) {
      const colors = {
        'Full': 'purple',
        'World': 'green',
        'Config': 'blue'
      }
      return colors[type] || 'grey'
    },
    restoreBackup(backup) {
      // In a real app, this would communicate with Tauri to restore the backup
      console.log('Restore backup', backup.name)
    },
    downloadBackup(backup) {
      // In a real app, this would communicate with Tauri to download the backup
      console.log('Download backup', backup.name)
    },
    deleteBackup(backup) {
      // In a real app, this would communicate with Tauri to delete the backup
      console.log('Delete backup', backup.name)
    }
  }
}
</script> 

<style scoped>
/* Make all icons white */
.v-icon {
  color: white !important;
}
</style> 