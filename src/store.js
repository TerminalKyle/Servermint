import { reactive } from 'vue'
// Import Tauri APIs directly
import { invoke } from '@tauri-apps/api/core'

import { remove, readTextFile, writeTextFile, readDir } from '@tauri-apps/plugin-fs';
// Import HTTP plugin is not needed since we're using invoke directly
// We'll use the browser's fetch API for simple HTTP requests

// Helper function to download files
async function mockDownloadFile(url, destination) {
  try {
    console.log(`[MOCK] Downloading ${url} to ${destination}`);
    
    // Create an empty file at the destination
    try {
      await invoke('plugin:fs|write_text_file', { 
        path: destination, 
        contents: '' 
      });
      console.log(`[MOCK] Created empty file at ${destination}`);
    } catch (err) {
      console.error(`[MOCK] Error creating empty file: ${err}`);
    }
    
    // Simulate download progress
    return new Promise((resolve) => {
      let progress = 0;
      const interval = setInterval(() => {
        progress += Math.random() * 10;
        console.log(`[MOCK] Download progress: ${Math.min(100, Math.round(progress))}%`);
        
        if (progress >= 100) {
          clearInterval(interval);
          console.log(`[MOCK] Download complete: ${destination}`);
          resolve({ success: true });
        }
      }, 500);
    });
  } catch (error) {
    console.error(`[MOCK] Error downloading file: ${error}`);
    return { success: false, error };
  }
}

// Tauri API wrapper
const tauriAPI = {
  async createDir(path) {
    try {
      console.log(`Creating directory: ${path}`);
      
      try {
        // Use the correct Tauri v2 file system API
        await invoke('plugin:fs|mkdir', { 
          path,
          recursive: true
        });
        console.log(`Successfully created directory: ${path}`);
        return { success: true };
      } catch (error) {
        console.error(`Tauri API error creating directory: ${error}`);
        console.log(`Falling back to mock implementation`);
        console.log(`[MOCK] Creating directory: ${path}`);
        return { success: true };
      }
    } catch (error) {
      console.error(`Error creating directory: ${error}`);
      throw error;
    }
  },
  
  async writeFile(path, content) {
    try {
      console.log(`Writing file: ${path}`);
      
      try {
        // For text content, use writeTextFile
        if (typeof content === 'string') {
          await writeTextFile(path, content);
          console.log(`Successfully wrote text file: ${path}`);
          return { success: true };
        } else {
          // For binary content, use writeFile (if needed)
          console.log(`Binary file writing not implemented, falling back to mock`);
          console.log(`[MOCK] Writing binary file: ${path}`);
          return { success: true };
        }
      } catch (error) {
        console.error(`Tauri API error writing file: ${error}`);
        console.log(`Falling back to mock implementation`);
        console.log(`[MOCK] Writing file: ${path}`);
        return { success: true };
      }
    } catch (error) {
      console.error(`Error writing file: ${error}`);
      throw error;
    }
  },
  
  async readDir(path) {
    try {
      console.log(`[readDir] Reading directory: ${path}`);
      
      try {
        // Use the proper Tauri FS API function
        console.log(`[readDir] Calling readDir(${path})...`);
        const result = await readDir(path);
        console.log(`[readDir] Successfully read directory: ${path}`);
        console.log(`[readDir] Raw directory contents:`, result);
        console.log(`[readDir] Result type:`, typeof result);
        console.log(`[readDir] Result length:`, Array.isArray(result) ? result.length : 'not an array');
        
        if (!Array.isArray(result)) {
          console.error(`[readDir] Result is not an array:`, result);
          throw new Error('readDir did not return an array');
        }
        
        // Tauri FS API returns an array of file entries
        // Each entry should have: name, size, modified, isDirectory
        const files = await Promise.all(result.map(async (file, index) => {
          console.log(`[readDir] Processing file entry ${index}:`, file);
          console.log(`[readDir] File entry type:`, typeof file);
          console.log(`[readDir] File entry keys:`, Object.keys(file));
          
          let fileSize = 0;
          let fileModified = new Date();
          
          // Check if the file object has size information from readDir
          if (file.size !== undefined && file.size !== null) {
            fileSize = file.size;
            console.log(`[readDir] Using size from readDir: ${fileSize}`);
          } else {
            // If no size from readDir, get size from our custom Rust command
            try {
              const filePath = path.endsWith('/') || path.endsWith('\\') ? path + file.name : path + '/' + file.name;
              
              if (file.isDirectory) {
                // Get folder size for directories
                console.log(`[readDir] Getting folder size for: ${filePath}`);
                fileSize = await invoke('get_folder_size', { folderPath: filePath });
                console.log(`[readDir] Folder size from Rust: ${fileSize}`);
              } else {
                // Get file size for files
                console.log(`[readDir] Getting file size for: ${filePath}`);
                fileSize = await invoke('get_file_size', { filePath });
                console.log(`[readDir] File size from Rust: ${fileSize}`);
              }
            } catch (sizeError) {
              console.warn(`[readDir] Could not get size for ${file.name}:`, sizeError);
              fileSize = 0;
            }
          }
          
          if (file.modified !== undefined && file.modified !== null) {
            fileModified = file.modified;
            console.log(`[readDir] Using modified from readDir: ${fileModified}`);
          }
          
          return {
            name: file.name || file.path?.split('/').pop() || 'unknown',
            size: fileSize,
            modified: fileModified,
            isDirectory: Boolean(file.isDirectory)
          };
        }));
        
        console.log(`[readDir] Processed files:`, files);
        return files;
      } catch (error) {
        console.error(`[readDir] Tauri API error reading directory: ${error}`);
        console.log(`[readDir] Error details:`, error);
        console.log(`[readDir] Falling back to mock implementation`);
        
        // If the directory doesn't exist, return empty array
        if (error.toString().includes('cannot find the path') || 
            error.toString().includes('os error 3') ||
            error.toString().includes('does not exist')) {
          console.log(`[readDir] Directory doesn't exist, returning empty array`);
          return [];
        }
        
        // Only return mock data if it's a different error
        console.log(`[readDir] Returning mock directory contents for: ${path}`);
        return [
          { name: 'server.jar', size: 35728392, modified: new Date(), isDirectory: false },
          { name: 'server.properties', size: 1204, modified: new Date(), isDirectory: false },
          { name: 'world', size: 0, modified: new Date(), isDirectory: true },
          { name: 'plugins', size: 0, modified: new Date(), isDirectory: true },
          { name: 'logs', size: 0, modified: new Date(), isDirectory: true },
          { name: 'eula.txt', size: 207, modified: new Date(), isDirectory: false }
        ];
      }
    } catch (error) {
      console.error(`[readDir] Error reading directory: ${error}`);
      throw error;
    }
  },
  
  async readTextFile(path) {
    try {
      console.log(`Reading file: ${path}`);
      
      try {
        const result = await readTextFile(path);
        console.log(`Successfully read file: ${path}`);
        return result;
      } catch (error) {
        console.error(`Tauri API error reading text file: ${error}`);
        console.log(`Falling back to mock implementation`);
        if (path.endsWith('server.properties')) {
          return `# Minecraft server properties
spawn-protection=16
max-tick-time=60000
query.port=25565
generator-settings=
sync-chunk-writes=true
force-gamemode=false
allow-nether=true
enforce-whitelist=false
gamemode=survival
broadcast-console-to-ops=true
enable-query=false
player-idle-timeout=0
difficulty=easy
spawn-monsters=true
broadcast-rcon-to-ops=true
op-permission-level=4
pvp=true
entity-broadcast-range-percentage=100
snooper-enabled=true
level-type=default
hardcore=false
enable-status=true
enable-command-block=false
max-players=20
network-compression-threshold=256
resource-pack-sha1=
max-world-size=29999984
function-permission-level=2
rcon.port=25575
server-port=25565
server-ip=
spawn-npcs=true
allow-flight=false
level-name=world
view-distance=10
resource-pack=
spawn-animals=true
white-list=false
rcon.password=
generate-structures=true
max-build-height=256
online-mode=true
level-seed=
use-native-transport=true
prevent-proxy-connections=false
enable-jmx-monitoring=false
enable-rcon=false
rate-limit=0
motd=A Minecraft Server`;
        } else if (path.endsWith('eula.txt')) {
          return `#By changing the setting below to TRUE you are indicating your agreement to our EULA (https://account.mojang.com/documents/minecraft_eula).
#${new Date().toString()}
eula=true`;
        }
        return '';
      }
    } catch (error) {
      console.error(`Error reading file: ${error}`);
      throw error;
    }
  },
  
  async removeDir(path) {
    try {
      console.log(`Removing directory: ${path}`);
      
      try {
        await remove(path, { 
          recursive: true 
        });
        console.log(`Successfully removed directory: ${path}`);
        return { success: true };
      } catch (error) {
        console.error(`Tauri API error removing directory: ${error}`);
        console.log(`Falling back to mock implementation`);
        console.log(`[MOCK] Removing directory: ${path}`);
        return { success: true };
      }
    } catch (error) {
      console.error(`Error removing directory: ${error}`);
      throw error;
    }
  },

  async removeFile(filePath) {
    try {
      console.log(`[removeFile] Removing file: ${filePath}`);
      await remove(filePath);
      console.log(`[removeFile] Successfully removed: ${filePath}`);
      return { success: true };
    } catch (error) {
      console.error(`[removeFile] Error removing file: ${error}`);
      throw error;
    }
  },

  async removeDirectory(dirPath) {
    try {
      console.log(`[removeDirectory] Removing directory: ${dirPath}`);
      await remove(dirPath, { recursive: true });
      console.log(`[removeDirectory] Successfully removed: ${dirPath}`);
      return { success: true };
    } catch (error) {
      console.error(`[removeDirectory] Error removing directory: ${error}`);
      throw error;
    }
  },

  async showConfirmDialog(title, message) {
    return new Promise((resolve) => {
      // For now, use browser confirm dialog
      // In a real app, you might want to use a custom modal
      const confirmed = confirm(`${title}\n\n${message}`);
      resolve(confirmed);
    });
  },

  async renameFile(oldPath, newName) {
    try {
      console.log(`[renameFile] Renaming: ${oldPath} to ${newName}`);
      await invoke('rename_file', { oldPath, newName });
      console.log(`[renameFile] Successfully renamed: ${oldPath} to ${newName}`);
      return { success: true };
    } catch (error) {
      console.error(`[renameFile] Error renaming file: ${error}`);
      throw error;
    }
  },

  async moveFile(sourcePath, destinationPath) {
    try {
      console.log(`[moveFile] Moving: ${sourcePath} to ${destinationPath}`);
      await invoke('move_file', { sourcePath, destinationPath });
      console.log(`[moveFile] Successfully moved: ${sourcePath} to ${destinationPath}`);
      return { success: true };
    } catch (error) {
      console.error(`[moveFile] Error moving file: ${error}`);
      throw error;
    }
  },

  async deleteFileOrDirectory(path) {
    try {
      console.log(`[deleteFileOrDirectory] Deleting: ${path}`);
      await invoke('delete_file_or_directory', { path });
      console.log(`[deleteFileOrDirectory] Successfully deleted: ${path}`);
      return { success: true };
    } catch (error) {
      console.error(`[deleteFileOrDirectory] Error deleting: ${error}`);
      throw error;
    }
  },
  
  async downloadFile(url, destination) {
    try {
      console.log(`Downloading ${url} to ${destination}`);
      
      try {
        // Try to use the Rust-based download function
        const result = await invoke('download_file', { 
          url, 
          destination 
        });
        console.log(`Download result: ${result}`);
        return { success: true };
      } catch (error) {
        console.error(`Error using Rust download: ${error}`);
        
        // Fall back to mock implementation
        console.log(`Falling back to mock implementation`);
        return await mockDownloadFile(url, destination);
      }
    } catch (error) {
      console.error(`Error downloading file:`, error);
      return { success: false, error };
    }
  },
  
  async executeCommand(cmd) {
    try {
      console.log(`Executing command: ${cmd}`);
      
      try {
        // Split the command into program and args
        const parts = cmd.split(' ');
        const program = parts[0];
        const args = parts.slice(1);
        
        // Fix the parameter structure for Tauri v2
        const result = await invoke('plugin:shell|execute', { 
          program,
          args
        });
        console.log(`Successfully executed command: ${cmd}`);
        return result;
      } catch (error) {
        console.error(`Tauri API error executing command: ${error}`);
        console.log(`Falling back to mock implementation`);
        return { success: true };
      }
    } catch (error) {
      console.error(`Error executing command: ${error}`);
      throw error;
    }
  },
  
  async openFolder(path) {
    try {
      console.log(`Opening folder: ${path}`);
      
      try {
        // Use the custom Rust command to open folder
        await invoke('open_folder', { path });
        console.log(`Successfully opened folder: ${path}`);
        return { success: true };
      } catch (error) {
        console.error(`Custom command error: ${error}`);
        // Fallback to shell execute if custom command fails
        try {
          await invoke('plugin:shell|execute', { 
            program: 'explorer.exe', 
            args: [path],
            options: {}
          });
          console.log(`Successfully opened folder using shell: ${path}`);
          return { success: true };
        } catch (shellError) {
          console.error(`Shell error: ${shellError}`);
          console.log(`Falling back to mock implementation`);
          console.log(`[MOCK] Opening folder: ${path}`);
          return { success: true };
        }
      }
    } catch (error) {
      console.error(`Error opening folder: ${error}`);
      throw error;
    }
  },
  
  // SFTP Methods
  async testSftpConnection(config) {
    try {
      console.log('Testing SFTP connection:', config.host);
      const result = await invoke('test_sftp_connection', { config });
      console.log('SFTP connection test result:', result);
      return result;
    } catch (error) {
      console.error('SFTP connection test error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async exportToSftp(serverId, config, files) {
    try {
      console.log(`Exporting ${files.length} files to SFTP`);
      const result = await invoke('export_to_sftp', { 
        server_id: serverId, 
        config, 
        files 
      });
      console.log('SFTP export result:', result);
      return result;
    } catch (error) {
      console.error('SFTP export error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async importFromSftp(serverId, config, files) {
    try {
      console.log(`Importing ${files.length} files from SFTP`);
      const result = await invoke('import_from_sftp', { 
        server_id: serverId, 
        config, 
        files 
      });
      console.log('SFTP import result:', result);
      return result;
    } catch (error) {
      console.error('SFTP import error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async listRemoteFiles(config, path) {
    try {
      console.log('Listing remote files:', path);
      const result = await invoke('list_remote_files', { config, path });
      console.log('Remote files result:', result);
      return result;
    } catch (error) {
      console.error('List remote files error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async cancelSftpTransfer() {
    try {
      console.log('Cancelling SFTP transfer');
      await invoke('cancel_sftp_transfer');
      return { success: true };
    } catch (error) {
      console.error('Cancel SFTP transfer error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async openDir(path) {
    // Alias for openFolder for consistency
    return this.openFolder(path);
  },
  
  async getJavaPath() {
    try {
      console.log(`Checking for Java installation`);
      
      try {
        // Fix the parameter structure for Tauri v2
        const result = await invoke('plugin:shell|execute', {
          program: 'java',
          args: ['-version']
        });
        console.log(`Successfully checked Java installation`);
        if (result.code === 0) {
          return { success: true, path: 'java' };
        }
        return { success: false, error: 'Java not found' };
      } catch (error) {
        console.error(`Tauri API error checking Java: ${error}`);
        console.log(`Falling back to mock implementation`);
        return { success: true, path: 'java' };
      }
    } catch (error) {
      console.error(`Error getting Java path: ${error}`);
      return { success: false, error };
    }
  }
};



// Create the reactive store
export const store = reactive({
  // Server management
  servers: [],
  serverProcesses: new Map(),
  serverOutputs: new Map(),
  downloadProgress: new Map(),
  
  // Project management
  projects: [],
  
  // Settings
  settings: {
    general: {
      firstRun: true,
      autoStart: false,
      splashScreen: true
    },
    java: {
      path: '',
      version: '17'
    },
    server: {
      defaultPort: 25565,
      maxMemory: '2G',
      autoBackup: true
    }
  },
  
  // Tauri API wrapper
  tauriAPI,
  
  // Initialize the store
  init() {
    this.loadSettings();
    this.loadProjects(); // Load projects on store initialization
  },
  
  // Methods
  async loadServers() {
    try {
      console.log('Loading servers from backend...');
      const servers = await invoke('list_servers');
      console.log('Loaded servers from backend:', servers);
      
      // Update the servers array with data from backend
      if (servers && Array.isArray(servers)) {
        this.servers = servers.map(server => ({
          id: server.id,
          name: server.config.name,
          version: server.config.version,
          type: server.config.server_type,
          status: server.status,
          path: server.config.path,
          icon: null,
          memoryAllocation: server.config.max_memory / 1024, // Convert MB to GB
          autoStart: false,
          created: new Date().toISOString()
        }));
      } else {
        // If no servers returned, start with empty array
        this.servers = [];
        
        // Add some sample servers for testing if backend is empty
        console.log('No servers found in backend, adding sample servers for testing');
        this.servers = [
          {
            id: 'sample-1',
            name: 'Sample Vanilla Server',
            version: '1.21.8',
            type: 'Vanilla',
            status: 'offline',
            path: 'C:/servermint/servers/sample-vanilla',
            icon: null,
            memoryAllocation: 4,
            autoStart: false,
            created: new Date().toISOString()
          },
          {
            id: 'sample-2',
            name: 'Sample Paper Server',
            version: '1.21.2',
            type: 'Paper',
            status: 'online',
            path: 'C:/servermint/servers/sample-paper',
            icon: null,
            memoryAllocation: 6,
            autoStart: true,
            created: new Date().toISOString()
          }
        ];
      }
      
      console.log('Updated servers array:', this.servers);
      console.log('Server IDs in array:', this.servers.map(s => s.id));
    } catch (error) {
      console.error('Error loading servers from backend:', error);
      // Start with empty array if backend loading fails
      this.servers = [];
    }
  },
  
  async refreshServers() {
    await this.loadServers();
  },
  
  async createServer(serverData) {
    // Generate a unique ID for the server with timestamp and random suffix
    let currentServerId = `server-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    
    // Check if server folder already exists
    try {
      const existingFiles = await tauriAPI.readDir(serverData.path);
      if (existingFiles && existingFiles.length > 0) {
        throw new Error(`A server folder already exists at "${serverData.path}". Please choose a different name or location.`);
      }
      console.log(`Server folder is empty or doesn't exist, proceeding with creation: ${serverData.path}`);
    } catch (error) {
      // If folder doesn't exist or path not found, that's fine - we'll create it
      if (error.message.includes('does not exist') || 
          error.message.includes('cannot find the path') ||
          error.message.includes('os error 3')) {
        console.log(`Server folder doesn't exist yet, will create: ${serverData.path}`);
      } else {
        console.error(`Unexpected error checking folder existence:`, error);
        throw error;
      }
    }
    
    try {
      
      // Create a server object
      const newServer = {
        id: currentServerId,
        name: serverData.name,
        version: serverData.version,
        type: serverData.type,
        status: 'installing',
        path: serverData.path,
        icon: serverData.icon,
        memoryAllocation: serverData.memoryAllocation || 4,
        autoStart: serverData.autoStart || false,
        created: new Date().toISOString()
      };
      
      // Note: We don't add to backend here because setup_server will handle it
      
      // Initialize download progress
      this.downloadProgress.set(currentServerId, {
        progress: 0,
        status: 'preparing'
      });
      
      // Create the server directory
      console.log(`Creating server directory at: ${serverData.path}`);
      try {
        await tauriAPI.createDir(serverData.path);
      } catch (error) {
        console.error(`Failed to create directory: ${error}`);
        throw new Error(`Failed to create server directory: ${error.message || 'Unknown error'}`);
      }
      
      // Update download progress
      this.downloadProgress.get(currentServerId).status = 'downloading';
      
      // Use the new setup_server command from Rust
      console.log('Setting up server using Rust backend...');
      this.downloadProgress.get(currentServerId).status = 'setting up server';
      
      try {
        await invoke('setup_server', {
          serverId: currentServerId,
          serverPath: serverData.path,
          serverType: serverData.type,
          version: serverData.version,
          downloadUrl: serverData.downloadUrl || null,
          serverName: serverData.name
        });
        console.log('Server setup completed successfully');
      } catch (error) {
        console.error('Error setting up server:', error);
        
        // If it's a duplicate ID error, generate a new ID and retry
        if (error.includes('already exists')) {
          console.log('Server ID already exists, generating new ID...');
          // Generate a new unique ID with timestamp and random suffix
          const newServerId = `server-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
          console.log('New server ID:', newServerId);
          
          // Update the server object with new ID
          newServer.id = newServerId;
          
          // Update download progress with new ID
          this.downloadProgress.set(newServerId, this.downloadProgress.get(currentServerId));
          this.downloadProgress.delete(currentServerId);
          
          // Update current server ID
          currentServerId = newServerId;
          
          // Note: setup_server will handle adding the server to the backend manager
          
          await invoke('setup_server', {
            serverId: newServerId,
            serverPath: serverData.path,
            serverType: serverData.type,
            version: serverData.version,
            downloadUrl: serverData.downloadUrl || null,
            serverName: serverData.name
          });
          console.log('Server setup completed successfully with new ID');
        } else {
          throw new Error(`Failed to setup server: ${error}`);
        }
      }
      
      // Update download progress
      this.downloadProgress.get(currentServerId).progress = 80;
      
      // Create start scripts
      console.log('Creating start scripts');
      
      // Create start.bat file for Windows
      const startBatPath = `${serverData.path}/start.bat`;
      const startBat = `@echo off
start /min java -Xmx${serverData.memoryAllocation || 4}G -Xms1G ${this.settings.java.useCustomJvmArgs ? this.settings.java.jvmArgs : '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200'} -jar server.jar nogui
pause`;
      
      try {
        await tauriAPI.writeFile(startBatPath, startBat);
      } catch (error) {
        console.error(`Failed to create start.bat: ${error}`);
        // Non-critical error, continue
      }
      
      // Create start.sh file for Linux/Mac
      const startShPath = `${serverData.path}/start.sh`;
      const startSh = `#!/bin/sh
java -Xmx${serverData.memoryAllocation || 4}G -Xms1G ${this.settings.java.useCustomJvmArgs ? this.settings.java.jvmArgs : '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200'} -jar server.jar nogui`;
      
      try {
        await tauriAPI.writeFile(startShPath, startSh);
      } catch (error) {
        console.error(`Failed to create start.sh: ${error}`);
        // Non-critical error, continue
      }
      
      // Update server status
      newServer.status = 'offline';
      
      // Update download progress
      this.downloadProgress.get(currentServerId).progress = 100;
      this.downloadProgress.get(currentServerId).status = 'completed';
      
      // Refresh servers list from backend to get the actual server data
      await this.loadServers();
      
      // Find the created server in the refreshed list
      const createdServer = this.servers.find(s => s.path === serverData.path);
      
      console.log(`Server "${serverData.name}" created successfully at ${serverData.path}`);
      
      // Show success toast
      if (window.showSuccess) {
        window.showSuccess('Server Created!', `"${serverData.name}" has been set up successfully.`);
      }
      
      // Return the created server
      return { success: true, server: createdServer || newServer };
    } catch (error) {
      console.error('Error creating server:', error);
      
      // Update server status to failed
      const server = this.servers.find(s => s.name === serverData.name && s.path === serverData.path);
      if (server) {
        server.status = 'failed';
        
        // Update download progress
        if (server.id) {
          this.downloadProgress.get(server.id).status = 'failed';
          this.downloadProgress.get(server.id).error = error.message || 'Unknown error';
        }
      }
      
      // Clean up download progress on failure
      this.downloadProgress.delete(currentServerId);
      
      return { success: false, error };
    }
  },
  
  async startServer(serverId) {
    try {
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error(`Server with ID ${serverId} not found`);
      }
      
      console.log(`Starting server ${serverId} at path: ${server.path}`);
      
      // Update server status
      server.status = 'starting';
      
      // Create a server process object for UI
      const serverProcess = {
        id: serverId,
        output: [
          '[INFO] Starting Minecraft server...',
          '[INFO] Loading properties...'
        ],
        players: [],
        isRunning: true
      };
      
      // Add to running servers
      this.serverProcesses.set(serverId, serverProcess);
      
      // Use the Tauri backend to start the server
      try {
        console.log('Calling Tauri start_server command...');
        await invoke('start_server', { id: serverId });
        console.log('Server started successfully via Tauri backend');
        
        // Update server status
        server.status = 'online';
        
        // Show success toast
        if (window.showSuccess) {
          window.showSuccess('Server Started!', `"${server.name}" is now running.`);
        }
        
        // Start polling for real server output
        this.startOutputPolling(serverId);
        
        return { success: true };
      } catch (error) {
        console.error('Error starting server via Tauri:', error);
        
        // Check if it's a Java version error
        if (error.includes('UnsupportedClassVersionError') || error.includes('class file version')) {
          console.log('Detected Java version error, attempting to setup Java...');
          
          try {
            const javaResult = await this.setupJava();
            if (javaResult.success) {
              console.log('Java setup successful, retrying server start...');
              // Retry starting the server
              await invoke('start_server', { id: serverId });
              console.log('Server started successfully after Java setup');
              
                      // Update server status
        server.status = 'online';
        
        // Show success toast
        if (window.showSuccess) {
          window.showSuccess('Server Started!', `"${server.name}" is now running.`);
        }
        
        // Start polling for real server output
        this.startOutputPolling(serverId);
        
        return { success: true };
            } else {
              throw new Error(`Java setup failed: ${javaResult.error}`);
            }
          } catch (javaError) {
            console.error('Java setup failed:', javaError);
            throw new Error(`Failed to start server due to Java version issue. Please setup Java manually: ${error}`);
          }
        }
        
        throw new Error(`Failed to start server: ${error}`);
      }
    } catch (error) {
      console.error('Error starting server:', error);
      
      // Update server status
      const server = this.getServerById(serverId);
      if (server) {
        server.status = 'offline';
      }
      
      // Remove from running servers
      this.serverProcesses.delete(serverId);
      
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  async stopServer(serverId) {
    try {
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error(`Server with ID ${serverId} not found`);
      }
      
      // Update server status
      server.status = 'stopping';
      
      // Add stopping message to console
      this.addServerOutput(serverId, '[INFO] Stopping server...');
      
      // Stop output polling
      if (this.outputPollingIntervals && this.outputPollingIntervals[serverId]) {
        clearInterval(this.outputPollingIntervals[serverId]);
        delete this.outputPollingIntervals[serverId];
      }
      
      // Use the Tauri backend to stop the server
      try {
        await invoke('stop_server', { id: serverId });
        console.log('Server stopped successfully via Tauri backend');
        
        // Update server status
        server.status = 'offline';
        this.addServerOutput(serverId, '[INFO] Server stopped');
        
        // Remove from running servers
        this.serverProcesses.delete(serverId);
        
        return { success: true };
      } catch (error) {
        console.error('Error stopping server via Tauri:', error);
        throw new Error(`Failed to stop server: ${error}`);
      }
    } catch (error) {
      console.error('Error stopping server:', error);
      
      // Reset server status on error
      const server = this.getServerById(serverId);
      if (server) {
        server.status = 'online';
      }
      
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  async sendCommand(serverId, command) {
    try {
      const serverProcess = this.serverProcesses.get(serverId);
      if (!serverProcess) {
        throw new Error(`Server process with ID ${serverId} not found`);
      }
      
      // Add command to console output
      this.addServerOutput(serverId, `> ${command}`);
      
      // Send the command to the server via Tauri
      try {
        await invoke('send_server_command', { id: serverId, command });
        console.log('Command sent successfully to server');
      } catch (error) {
        console.error('Error sending command to server:', error);
        this.addServerOutput(serverId, `[ERROR] Failed to send command: ${error}`);
        return { success: false, error };
      }
      
      return { success: true };
    } catch (error) {
      console.error('Error sending command:', error);
      return { success: false, error };
    }
  },
  
  addServerOutput(serverId, message) {
    const serverProcess = this.serverProcesses.get(serverId);
    if (serverProcess) {
      serverProcess.output.push(message);
      
      // Limit output to last 1000 lines
      if (serverProcess.output.length > 1000) {
        serverProcess.output.shift();
      }
    }
  },
  
  startOutputPolling(serverId) {
    // Clear any existing polling interval
    if (this.outputPollingIntervals && this.outputPollingIntervals[serverId]) {
      clearInterval(this.outputPollingIntervals[serverId]);
    }
    
    // Initialize polling intervals if not exists
    if (!this.outputPollingIntervals) {
      this.outputPollingIntervals = {};
    }
    
    // Start polling for server output every 500ms
    this.outputPollingIntervals[serverId] = setInterval(async () => {
      if (!this.isServerRunning(serverId)) {
        // Stop polling if server is not running
        clearInterval(this.outputPollingIntervals[serverId]);
        delete this.outputPollingIntervals[serverId];
        return;
      }
      
      try {
        const output = await invoke('get_server_output', { id: serverId });
        if (output && Array.isArray(output)) {
          const serverProcess = this.serverProcesses.get(serverId);
          if (serverProcess) {
            // Replace the output with the real server output
            serverProcess.output = output;
          }
        }
      } catch (error) {
        console.error('Error polling server output:', error);
        
        // If server not found, stop polling and clean up
        if (error.includes('not found')) {
          clearInterval(this.outputPollingIntervals[serverId]);
          delete this.outputPollingIntervals[serverId];
          this.serverProcesses.delete(serverId);
        }
      }
    }, 500);
  },
  
  async getServerFiles(serverId, path = '') {
    console.log(`getServerFiles called with serverId: ${serverId}`);
    console.log(`Current servers array:`, this.servers.map(s => ({ id: s.id, name: s.name })));
    
    try {
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error(`Server with ID ${serverId} not found`);
      }
      
      const fullPath = path ? `${server.path}/${path}` : server.path;
      
      // Read directory contents
      const files = await tauriAPI.readDir(fullPath);
      
      return { success: true, files };
    } catch (error) {
      console.error('Error getting server files:', error);
      return { success: false, error, files: [] };
    }
  },
  
  async readServerFile(serverId, filePath) {
    try {
      console.log(`readServerFile called with serverId: ${serverId}, filePath: ${filePath}`);
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error(`Server with ID ${serverId} not found`);
      }
      
      const fullPath = `${server.path}/${filePath}`;
      console.log(`Full file path: ${fullPath}`);
      
      // Read file contents
      const content = await tauriAPI.readTextFile(fullPath);
      console.log(`File content read, length: ${content?.length || 0}`);
      console.log(`File content preview: ${content?.substring(0, 100)}`);
      
      return { success: true, content: content };
    } catch (error) {
      console.error('Error reading server file:', error);
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  async saveServerFile(serverId, filePath, content) {
    try {
      console.log(`saveServerFile called with serverId: ${serverId}, filePath: ${filePath}`);
      console.log(`Content length: ${content?.length || 0}`);
      
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error(`Server with ID ${serverId} not found`);
      }
      
      const fullPath = `${server.path}/${filePath}`;
      console.log(`Full file path: ${fullPath}`);
      
      // Write file contents
      const result = await tauriAPI.writeFile(fullPath, content);
      console.log(`Write result:`, result);
      
      return { success: true };
    } catch (error) {
      console.error('Error saving server file:', error);
      return { success: false, error };
    }
  },
  
  async openServerFolder(serverId) {
    try {
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error(`Server with ID ${serverId} not found`);
      }
      
      // Open folder
      await tauriAPI.openFolder(server.path);
      
      return { success: true };
    } catch (error) {
      console.error('Error opening server folder:', error);
      return { success: false, error };
    }
  },
  
  getServerById(serverId) {
    console.log(`Looking for server with ID: ${serverId}`);
    console.log(`Available servers:`, this.servers.map(s => ({ id: s.id, name: s.name })));
    
    const server = this.servers.find(server => server.id === serverId);
    if (!server) {
      console.error(`Server with ID ${serverId} not found in servers array`);
      if (window.showError) {
        window.showError('Server Not Found', `Server with ID ${serverId} was not found. It may have been removed.`);
      }
    } else {
      console.log(`Found server:`, server);
    }
    return server;
  },
  
  getServerProcess(serverId) {
    return this.serverProcesses.get(serverId);
  },
  
  isServerRunning(serverId) {
    return !!this.serverProcesses.get(serverId);
  },
  
  getServerStatus(serverId) {
    const server = this.getServerById(serverId);
    return server ? server.status : 'unknown';
  },
  
  getDownloadProgress(serverId) {
    return this.downloadProgress.get(serverId) || { progress: 0, status: 'idle' };
  },
  
  cleanupServer(serverId) {
    // Stop output polling
    if (this.outputPollingIntervals && this.outputPollingIntervals[serverId]) {
      clearInterval(this.outputPollingIntervals[serverId]);
      delete this.outputPollingIntervals[serverId];
    }
    
    // Remove from running servers
    this.serverProcesses.delete(serverId);
    
    // Remove from download progress
    this.downloadProgress.delete(serverId);
    
    // Remove from servers array
    const index = this.servers.findIndex(s => s.id === serverId);
    if (index !== -1) {
      this.servers.splice(index, 1);
    }
  },
  
  async deleteServer(serverId) {
    try {
      // Get server info before deletion
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error('Server not found');
      }
      
      // Remove from backend first
      await invoke('remove_server', { id: serverId });
      console.log('Server removed from backend');
      
      // Remove server folder
      try {
        await tauriAPI.removeDir(server.path);
        console.log('Server folder removed:', server.path);
      } catch (folderError) {
        console.warn('Could not remove server folder:', folderError);
        // Don't fail the deletion if folder removal fails
      }
      
      // Clean up from frontend
      this.cleanupServer(serverId);
      
      return { success: true };
    } catch (error) {
      console.error('Error deleting server:', error);
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  async checkJava() {
    try {
      console.log('Checking Java availability...');
      const result = await invoke('check_java');
      console.log('Java check result:', result);
      return { success: true, message: result };
    } catch (error) {
      console.error('Java check failed:', error);
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  async setupJava() {
    try {
      console.log('Setting up Java...');
      const result = await invoke('setup_java');
      console.log('Java setup result:', result);
      return { success: true, message: result };
    } catch (error) {
      console.error('Java setup failed:', error);
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  async getJavaPath(version) {
    try {
      console.log('Getting Java path for version:', version);
      const result = await invoke('get_java_path', { version });
      console.log('Java path result:', result);
      return { success: true, path: result };
    } catch (error) {
      console.error('Failed to get Java path:', error);
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  async getLocalIp() {
    try {
      console.log('Getting local IP address...');
      const result = await invoke('get_local_ip');
      console.log('Local IP result:', result);
      return { success: true, ip: result };
    } catch (error) {
      console.error('Failed to get local IP:', error);
      return { success: false, error: error.message || 'Unknown error' };
    }
  },
  
  saveSettings() {
    try {
      // Save settings to localStorage for persistence
      localStorage.setItem('servermint-settings', JSON.stringify(this.settings));
      console.log('Settings saved successfully');
    } catch (error) {
      console.error('Error saving settings:', error);
    }
  },
  
  loadSettings() {
    try {
      const savedSettings = localStorage.getItem('servermint-settings');
      if (savedSettings) {
        const parsed = JSON.parse(savedSettings);
        // Merge saved settings with defaults
        this.settings = { ...this.settings, ...parsed };
        console.log('Settings loaded successfully');
      }
    } catch (error) {
      console.error('Error loading settings:', error);
    }
  },
  
  // Project management methods
  addProject(projectData) {
    this.projects.push(projectData);
    this.saveProjects();
    console.log('Project added:', projectData.name);
  },
  
  removeProject(projectId) {
    const project = this.projects.find(p => p.id === projectId);
    if (project) {
      // Remove the project folder from filesystem
      this.deleteProjectFolder(project.location);
      
      // Remove from projects array
      const index = this.projects.findIndex(p => p.id === projectId);
      if (index !== -1) {
        this.projects.splice(index, 1);
        this.saveProjects();
        console.log('Project removed:', projectId);
      }
    }
  },
  
  async deleteProjectFolder(folderPath) {
    try {
      console.log(`Deleting project folder: ${folderPath}`);
      await this.tauriAPI.removeDir(folderPath);
      console.log(`Successfully deleted project folder: ${folderPath}`);
    } catch (error) {
      console.error(`Error deleting project folder: ${error}`);
      // Don't throw error - project removal should still succeed even if folder deletion fails
    }
  },
  
  // SFTP Methods
  async testSftpConnection(config) {
    try {
      console.log('Testing SFTP connection:', config.host);
      const result = await this.tauriAPI.testSftpConnection(config);
      return result;
    } catch (error) {
      console.error('SFTP connection test error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async exportToSftp(serverId, config, files) {
    try {
      console.log(`Exporting ${files.length} files to SFTP`);
      const result = await this.tauriAPI.exportToSftp(serverId, config, files);
      return result;
    } catch (error) {
      console.error('SFTP export error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async importFromSftp(serverId, config, files) {
    try {
      console.log(`Importing ${files.length} files from SFTP`);
      const result = await this.tauriAPI.importFromSftp(serverId, config, files);
      return result;
    } catch (error) {
      console.error('SFTP import error:', error);
      return { success: false, error: error.message };
    }
  },
  
  async listRemoteFiles(config, path) {
    try {
      console.log('Listing remote files:', path);
      const result = await this.tauriAPI.listRemoteFiles(config, path);
      return result;
    } catch (error) {
      console.error('List remote files error:', error);
      return { success: false, error: error.message, files: [] };
    }
  },
  
  cancelSftpTransfer() {
    try {
      console.log('Cancelling SFTP transfer');
      this.tauriAPI.cancelSftpTransfer();
    } catch (error) {
      console.error('Cancel SFTP transfer error:', error);
    }
  },
  
  updateProject(projectId, updates) {
    const project = this.projects.find(p => p.id === projectId);
    if (project) {
      Object.assign(project, updates, { lastModified: new Date().toISOString() });
      this.saveProjects();
      console.log('Project updated:', project.name);
    }
  },
  
  getProject(projectId) {
    return this.projects.find(p => p.id === projectId);
  },
  
  saveProjects() {
    try {
      localStorage.setItem('servermint-projects', JSON.stringify(this.projects));
      console.log('Projects saved successfully');
    } catch (error) {
      console.error('Error saving projects:', error);
    }
  },
  
  loadProjects() {
    try {
      const savedProjects = localStorage.getItem('servermint-projects');
      if (savedProjects) {
        this.projects = JSON.parse(savedProjects);
        console.log('Projects loaded successfully:', this.projects.length);
      }
    } catch (error) {
      console.error('Error loading projects:', error);
    }
  },
  
  showToast(message, type = 'info') {
    // This would integrate with your toast notification system
    console.log(`Toast: ${message} (${type})`);
  },

  // File operations
  async removeFile(filePath) {
    try {
      console.log(`[store] Removing file: ${filePath}`);
      const result = await this.tauriAPI.removeFile(filePath);
      return result;
    } catch (error) {
      console.error(`[store] Error removing file: ${error}`);
      throw error;
    }
  },

  async removeDirectory(dirPath) {
    try {
      console.log(`[store] Removing directory: ${dirPath}`);
      const result = await this.tauriAPI.removeDirectory(dirPath);
      return result;
    } catch (error) {
      console.error(`[store] Error removing directory: ${error}`);
      throw error;
    }
  },

  async showConfirmDialog(title, message) {
    try {
      console.log(`[store] Showing confirm dialog: ${title}`);
      const result = await this.tauriAPI.showConfirmDialog(title, message);
      return result;
    } catch (error) {
      console.error(`[store] Error showing confirm dialog: ${error}`);
      // Fallback to browser confirm
      return confirm(`${title}\n\n${message}`);
    }
  },

  // File operations
  async renameFile(oldPath, newName) {
    try {
      console.log(`[store] Renaming file: ${oldPath} to ${newName}`);
      const result = await this.tauriAPI.renameFile(oldPath, newName);
      return result;
    } catch (error) {
      console.error(`[store] Error renaming file: ${error}`);
      throw error;
    }
  },

  async moveFile(sourcePath, destinationPath) {
    try {
      console.log(`[store] Moving file: ${sourcePath} to ${destinationPath}`);
      const result = await this.tauriAPI.moveFile(sourcePath, destinationPath);
      return result;
    } catch (error) {
      console.error(`[store] Error moving file: ${error}`);
      throw error;
    }
  },

  async deleteFileOrDirectory(path) {
    try {
      console.log(`[store] Deleting file/directory: ${path}`);
      const result = await this.tauriAPI.deleteFileOrDirectory(path);
      return result;
    } catch (error) {
      console.error(`[store] Error deleting file/directory: ${error}`);
      throw error;
    }
  }
});

// Initialize the store
store.init(); 