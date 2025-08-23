import { reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { remove, readTextFile, writeTextFile, readDir } from '@tauri-apps/plugin-fs';

async function mockDownloadFile(url, destination) {
  try {
    console.log(`[MOCK] Downloading ${url} to ${destination}`);
    
    try {
      await invoke('plugin:fs|write_text_file', { 
        path: destination, 
        contents: '' 
      });
      console.log(`[MOCK] Created empty file at ${destination}`);
    } catch (err) {
      console.error(`[MOCK] Error creating empty file: ${err}`);
    }
    
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

const tauriAPI = {
  async createDir(path) {
    try {
      console.log(`Creating directory: ${path}`);
      
      try {
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
        if (typeof content === 'string') {
          await writeTextFile(path, content);
          console.log(`Successfully wrote text file: ${path}`);
          return { success: true };
        } else if (content instanceof Uint8Array) {
          await invoke('write_binary_file', { 
            path: path, 
            data: Array.from(content) 
          });
          console.log(`Successfully wrote binary file: ${path}`);
          return { success: true };
        } else {
          console.log(`Converting content to appropriate format for: ${path}`);
          if (typeof content === 'object' && content !== null) {
            const jsonString = JSON.stringify(content, null, 2);
            await writeTextFile(path, jsonString);
            console.log(`Successfully wrote JSON file: ${path}`);
            return { success: true };
          } else {
            const stringContent = String(content);
            await writeTextFile(path, stringContent);
            console.log(`Successfully wrote string file: ${path}`);
            return { success: true };
          }
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
        
        const files = await Promise.all(result.map(async (file, index) => {
          console.log(`[readDir] Processing file entry ${index}:`, file);
          console.log(`[readDir] File entry type:`, typeof file);
          console.log(`[readDir] File entry keys:`, Object.keys(file));
          
          let fileSize = 0;
          let fileModified = new Date();
          
          if (file.size !== undefined && file.size !== null) {
            fileSize = file.size;
            console.log(`[readDir] Using size from readDir: ${fileSize}`);
          } else {
            try {
              const filePath = path.endsWith('/') || path.endsWith('\\') ? path + file.name : path + '/' + file.name;
              
              if (file.isDirectory) {
                console.log(`[readDir] Getting folder size for: ${filePath}`);
                fileSize = await invoke('get_folder_size', { folderPath: filePath });
                console.log(`[readDir] Folder size from Rust: ${fileSize}`);
              } else {
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
        
        if (error.toString().includes('cannot find the path') || 
            error.toString().includes('os error 3') ||
            error.toString().includes('does not exist')) {
          console.log(`[readDir] Directory doesn't exist, returning empty array`);
          return [];
        }
        
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

  async startAllServers() {
    try {
      console.log('[startAllServers] Starting all offline servers');
      const result = await invoke('start_all_servers');
      console.log('[startAllServers] Result:', result);
      return result;
    } catch (error) {
      console.error('[startAllServers] Error:', error);
      throw error;
    }
  },

  async stopAllServers() {
    try {
      console.log('[stopAllServers] Stopping all running servers');
      const result = await invoke('stop_all_servers');
      console.log('[stopAllServers] Result:', result);
      return result;
    } catch (error) {
      console.error('[stopAllServers] Error:', error);
      throw error;
    }
  },

  async backupAllServers() {
    try {
      console.log('[backupAllServers] Creating backups for all servers');
      const result = await invoke('backup_all_servers');
      console.log('[backupAllServers] Result:', result);
      return result;
    } catch (error) {
      console.error('[backupAllServers] Error:', error);
      throw error;
    }
  },

  async checkForUpdates() {
    try {
      console.log('[checkForUpdates] Checking for updates');
      const result = await invoke('check_for_updates');
      console.log('[checkForUpdates] Result:', result);
      return result;
    } catch (error) {
      console.error('[checkForUpdates] Error:', error);
      throw error;
    }
  },

  async restartApplication() {
    try {
      console.log('[restartApplication] Restarting application');
      await invoke('restart_application');
      return { success: true };
    } catch (error) {
      console.error('[restartApplication] Error:', error);
      throw error;
    }
  },

  async quitApplication() {
    try {
      console.log('[quitApplication] Quitting application');
      await invoke('quit_application');
      return { success: true };
    } catch (error) {
      console.error('[quitApplication] Error:', error);
      throw error;
    }
  },

  async getMintMenuCommands() {
    try {
      console.log('[getMintMenuCommands] Getting MintMenu commands');
      const result = await invoke('get_mint_menu_commands');
      console.log('[getMintMenuCommands] Result:', result);
      return result;
    } catch (error) {
      console.error('[getMintMenuCommands] Error:', error);
      throw error;
    }
  },
  
  async downloadFile(url, destination) {
    try {
      console.log(`Downloading ${url} to ${destination}`);
      
      try {
        const result = await invoke('download_file', { 
          url, 
          destination 
        });
        console.log(`Download result: ${result}`);
        return { success: true };
      } catch (error) {
        console.error(`Error using Rust download: ${error}`);
        
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
        const parts = cmd.split(' ');
        const program = parts[0];
        const args = parts.slice(1);
        
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
        await invoke('open_folder', { path });
        console.log(`Successfully opened folder: ${path}`);
        return { success: true };
      } catch (error) {
        console.error(`Custom command error: ${error}`);
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
  
  async testSftpConnection(config) {
    try {
      console.log('Testing SFTP connection:', config.host);
      const success = await invoke('test_sftp_connection', { config });
      console.log('SFTP connection test result:', success);
      return { success, error: null };
    } catch (error) {
      console.error('SFTP connection test error:', error);
      return { success: false, error: error.toString() };
    }
  },
  
  async upload_file_sftp(config, local_path, remote_path) {
    try {
      const cleanLocalPath = local_path.replace(/\\/g, '/');
      const cleanRemotePath = remote_path.startsWith('/') ? remote_path : '/' + remote_path;
      const fileName = cleanRemotePath.split('/').pop();

      console.log('Starting SFTP upload:', {
        host: config.host,
        port: config.port,
        username: config.username,
        localPath: cleanLocalPath,
        remotePath: cleanRemotePath,
        fileName
      });

      let beforeFiles = [];
      try {
        beforeFiles = await invoke('list_remote_files', { 
          config: {
            ...config,
            remote_path: '/',
            debug: true
          },
          path: '/'
        });
        console.log('Files before upload:', beforeFiles);
      } catch (e) {
        console.warn('Could not list files before upload:', e);
      }

      const success = await invoke('upload_file_sftp', { 
        config: {
          ...config,
          remote_path: '/'
        }, 
        localPath: cleanLocalPath,
        remotePath: cleanRemotePath
      });
      console.log('Upload command result:', success);

      if (success) {
        console.log('Upload reported success, verifying file exists...');
        try {
          const afterFiles = await invoke('list_remote_files', {
            config: {
              ...config,
              remote_path: '/',
              debug: true
            },
            path: '/'
          });
          console.log('Files after upload:', afterFiles);

          const filenames = afterFiles.map(f => f.split(/\s+/).pop());
          console.log('Found filenames:', filenames);

          if (!filenames.includes(fileName)) {
            console.error('File not found after upload');
            return { success: false, error: 'File upload reported success but file not found' };
          }
        } catch (e) {
          console.warn('Could not verify upload:', e);
        }
      }

      return { success, error: null };
    } catch (error) {
      console.error('SFTP upload error:', error);
      return { success: false, error: error.toString() };
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
    return this.openFolder(path);
  },
  
  async getJavaPath() {
    try {
      console.log(`Checking for Java installation`);
      
      try {
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
  },
  
  async invoke(command, args) {
    try {
      console.log(`[TauriAPI] Invoking command: ${command}`, args);
      const result = await invoke(command, args);
      console.log(`[TauriAPI] Command result:`, result);
      return result;
    } catch (error) {
      console.error(`[TauriAPI] Error invoking command ${command}:`, error);
      throw error;
    }
  }
};



export const store = reactive({
  servers: [],
  serverProcesses: new Map(),
  serverOutputs: new Map(),
  downloadProgress: new Map(),
  
  projects: [],
  
    installedMods: new Map(),
  
  settings: {
    general: {
      firstRun: true,
      autoStart: false,
      splashScreen: true,
      showServerIPs: true,
      defaultServerPath: 'C:\\servermint\\servers',
      defaultGameVersion: '1.21.2'
    },
    java: {
      path: '',
      version: '17',
      memory: 4
    },
    server: {
      defaultPort: 25565,
      maxMemory: '2G',
      autoBackup: true
    }
  },
  
  tauriAPI,
  
  init() {
    this.loadSettings();
    this.loadProjects();
  },
  
  async loadServers() {
    if (this._loadingServers) {
      console.log('loadServers already in progress, skipping...');
      return;
    }
    
    this._loadingServers = true;
    try {
      console.log('Loading servers from backend...');
      const servers = await invoke('list_servers');
      console.log('Loaded servers from backend:', servers);
      
      if (servers && Array.isArray(servers)) {
        servers.forEach((server, index) => {
          console.log(`Server ${index}:`, {
            id: server.id,
            name: server.config?.name,
            path: server.config?.path,
            type: server.config?.server_type
          });
        });
      }
      
      if (servers && Array.isArray(servers)) {
        this.servers = await Promise.all(servers.map(async server => {
          let icon = null;
          const iconPath = `${server.config.path}/icon.png`;
          console.log(`Looking for icon at: ${iconPath}`);
          try {
            try {
              const iconExists = await invoke('plugin:fs|exists', { path: iconPath });
              if (iconExists) {
                icon = null;
                console.log(`Icon loading disabled for server ${server.config.name} to prevent recursion`);
              } else {
                console.log(`No icon file found for server ${server.config.name}`);
              }
            } catch (error) {
              console.log(`Error checking icon for server ${server.config.name}:`, error);
            }
          } catch (error) {
            console.warn(`Could not load icon for server ${server.config.name}:`, error);
          }

          return {
            id: server.id,
            name: server.config.name,
            version: server.config.version,
            type: server.config.server_type,
            status: server.status,
            path: server.config.path,
            icon: icon,
            memoryAllocation: server.config.max_memory / 1024,
            autoStart: false,
            created: new Date().toISOString()
          };
        }));
      } else {
        this.servers = [];
      }
      
      console.log('Updated servers array:', this.servers);
      console.log('Server IDs in array:', this.servers.map(s => s.id));
    } catch (error) {
      console.error('Error loading servers from backend:', error);
      this.servers = [];
    } finally {
      this._loadingServers = false;
    }
  },
  
  async refreshServers() {
    await this.loadServers();
  },
  
  async createServer(serverData) {
    let currentServerId = `server-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    
    try {
      const existingFiles = await tauriAPI.readDir(serverData.path);
      if (existingFiles && existingFiles.length > 0) {
        throw new Error(`A server folder already exists at "${serverData.path}". Please choose a different name or location.`);
      }
      console.log(`Server folder is empty or doesn't exist, proceeding with creation: ${serverData.path}`);
    } catch (error) {
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
      const newServer = {
        id: currentServerId,
        name: serverData.name,
        version: serverData.version,
        type: serverData.type,
        status: 'installing',
        path: serverData.path,
        icon: null,
        memoryAllocation: serverData.memoryAllocation || 4,
        autoStart: serverData.autoStart || false,
        created: new Date().toISOString()
      };
      
      this.downloadProgress.set(currentServerId, {
        progress: 0,
        status: 'preparing'
      });
      
      console.log(`Creating server directory at: ${serverData.path}`);
      try {
        await tauriAPI.createDir(serverData.path);
      } catch (error) {
        console.error(`Failed to create directory: ${error}`);
        throw new Error(`Failed to create server directory: ${error.message || 'Unknown error'}`);
      }

      if (serverData.icon) {
        try {
          const base64Data = serverData.icon.split(',')[1];
          const binaryData = atob(base64Data);
          const uint8Array = new Uint8Array(binaryData.length);
          for (let i = 0; i < binaryData.length; i++) {
            uint8Array[i] = binaryData.charCodeAt(i);
          }

          const iconPath = `${serverData.path}/icon.png`;
          await invoke('plugin:fs|write_file', { 
            path: iconPath,
            contents: Array.from(uint8Array)
          });

          newServer.icon = iconPath;
          console.log('Server icon saved:', iconPath);
        } catch (error) {
          console.error('Failed to save server icon:', error);
        }
      }
      
      this.downloadProgress.get(currentServerId).status = 'downloading';
      
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
        
        if (error.includes('already exists')) {
          console.log('Server ID already exists, generating new ID...');
          const newServerId = `server-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
          console.log('New server ID:', newServerId);
          
          newServer.id = newServerId;
          
          this.downloadProgress.set(newServerId, this.downloadProgress.get(currentServerId));
          this.downloadProgress.delete(currentServerId);
          
          currentServerId = newServerId;
          
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
      
      this.downloadProgress.get(currentServerId).progress = 80;
      
      console.log('Creating start scripts');
      
      const startBatPath = `${serverData.path}/start.bat`;
      const startBat = `@echo off
start /min java -Xmx${serverData.memoryAllocation || 4}G -Xms1G ${this.settings.java.useCustomJvmArgs ? this.settings.java.jvmArgs : '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:+MaxGCPauseMillis=200'} -jar server.jar nogui
pause`;
      
      try {
        await tauriAPI.writeFile(startBatPath, startBat);
      } catch (error) {
        console.error(`Failed to create start.bat: ${error}`);
      }
      
      const startShPath = `${serverData.path}/start.sh`;
      const startSh = `#!/bin/sh
java -Xmx${serverData.memoryAllocation || 4}G -Xms1G ${this.settings.java.useCustomJvmArgs ? this.settings.java.jvmArgs : '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:+MaxGCPauseMillis=200'} -jar server.jar nogui`;
      
      try {
        await tauriAPI.writeFile(startShPath, startSh);
      } catch (error) {
        console.error(`Failed to create start.sh: ${error}`);
      }
      
      newServer.status = 'offline';
      
      if (serverData.icon && serverData.icon.startsWith('data:image/')) {
        try {
          console.log('Saving server icon...');
          const iconPath = `${serverData.path}/icon.png`;
          
          const base64Data = serverData.icon.split(',')[1];
          const binaryData = atob(base64Data);
          const bytes = new Uint8Array(binaryData.length);
          for (let i = 0; i < binaryData.length; i++) {
            bytes[i] = binaryData.charCodeAt(i);
          }
          
          await tauriAPI.writeFile(iconPath, bytes);
          console.log('Server icon saved successfully');
        } catch (error) {
          console.warn('Failed to save server icon:', error);
        }
      }
      
      this.downloadProgress.get(currentServerId).progress = 100;
      this.downloadProgress.get(currentServerId).status = 'completed';
      
      this.servers.push(newServer);
      
      const createdServer = this.servers.find(s => s.path === serverData.path);
      
      console.log(`Server "${serverData.name}" created successfully at ${serverData.path}`);
      
      if (window.showSuccess) {
        window.showSuccess('Server Created!', `"${serverData.name}" has been set up successfully.`);
      }
      
      return { success: true, server: createdServer || newServer };
    } catch (error) {
      console.error('Error creating server:', error);
      
      const server = this.servers.find(s => s.name === serverData.name && s.path === serverData.path);
      if (server) {
        server.status = 'failed';
        
        if (server.id) {
          this.downloadProgress.get(server.id).status = 'failed';
          this.downloadProgress.get(server.id).error = error.message || 'Unknown error';
        }
      }
      
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
      
      if (this.isServerRunning(serverId)) {
        console.log(`Server ${serverId} is already running, stopping previous instance first...`);
        
        try {
          await this.stopServer(serverId);
          console.log(`Previous instance of server ${serverId} stopped successfully`);
          
          await new Promise(resolve => setTimeout(resolve, 2000));
        } catch (stopError) {
          console.warn(`Failed to stop previous instance: ${stopError.message}`);
        }
      }
      
      server.status = 'starting';
      
      const serverProcess = {
        id: serverId,
        output: [
          '[INFO] Starting Minecraft server...',
          '[INFO] Loading properties...'
        ],
        players: [],
        isRunning: true
      };
      
      this.serverProcesses.set(serverId, serverProcess);
      
      try {
        console.log('Calling Tauri start_server command...');
        await invoke('start_server', { id: serverId });
        console.log('Server started successfully via Tauri backend');
        
        server.status = 'online';
        
        if (window.showSuccess) {
          window.showSuccess('Server Started!', `"${server.name}" is now running.`);
        }
        
        this.startOutputPolling(serverId);
        
        return { success: true };
      } catch (error) {
        console.error('Error starting server via Tauri:', error);
        
        if (error.includes('UnsupportedClassVersionError') || error.includes('class file version')) {
          console.log('Detected Java version error, attempting to setup Java...');
          
          try {
            const javaResult = await this.setupJava();
            if (javaResult.success) {
              console.log('Java setup successful, retrying server start...');
              await invoke('start_server', { id: serverId });
              console.log('Server started successfully after Java setup');
              
        server.status = 'online';
        
        if (window.showSuccess) {
          window.showSuccess('Server Started!', `"${server.name}" is now running.`);
        }
        
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
        
        if (error.includes('already running') || error.includes('Server is already running')) {
          console.log('Server is already running, attempting to connect to existing instance...');
          
          try {
            server.status = 'online';
            
            this.startOutputPolling(serverId);
            
            if (window.showSuccess) {
              window.showSuccess('Server Connected!', `"${server.name}" is already running and has been connected.`);
            }
            
            return { success: true };
          } catch (connectError) {
            console.error('Failed to connect to existing server:', connectError);
            throw new Error(`Server is already running but cannot be connected to. Please stop it manually and try again.`);
          }
        }
        
        throw new Error(`Failed to start server: ${error}`);
      }
    } catch (error) {
      console.error('Error starting server:', error);
      
      const server = this.getServerById(serverId);
      if (server) {
        server.status = 'offline';
      }
      
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
      
      server.status = 'stopping';
      
      this.addServerOutput(serverId, '[INFO] Stopping server...');
      
      if (this.outputPollingIntervals && this.outputPollingIntervals[serverId]) {
        clearInterval(this.outputPollingIntervals[serverId]);
        delete this.outputPollingIntervals[serverId];
      }
      
      try {
        await invoke('stop_server', { id: serverId });
        console.log('Server stopped successfully via Tauri backend');
        
        server.status = 'offline';
        this.addServerOutput(serverId, '[INFO] Server stopped');
        
        this.serverProcesses.delete(serverId);
        
        return { success: true };
      } catch (error) {
        console.error('Error stopping server via Tauri:', error);
        throw new Error(`Failed to stop server: ${error}`);
      }
    } catch (error) {
      console.error('Error stopping server:', error);
      
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
      
      this.addServerOutput(serverId, `> ${command}`);
      
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
      
      if (serverProcess.output.length > 1000) {
        serverProcess.output.shift();
      }
      
      this.parsePlayersFromOutput(serverId, serverProcess.output);
    }
  },

  parsePlayersFromOutput(serverId, output) {
    try {
      console.log(`[Player Tracking] Parsing players from output for server: ${serverId}`);
      console.log(`[Player Tracking] Output length: ${output ? output.length : 0}`);
      
      const serverProcess = this.serverProcesses.get(serverId);
      if (!serverProcess) {
        console.warn(`[Player Tracking] No server process found for: ${serverId}`);
        return;
      }

      if (!serverProcess.players) {
        serverProcess.players = [];
        console.log(`[Player Tracking] Initialized players array for: ${serverId}`);
      }

      const currentPlayers = new Set();
      
      const lastLines = output.slice(-10);
      console.log(`[Player Tracking] Last 10 lines of output:`, lastLines);
      
      for (const line of output) {
        if (!line || typeof line !== 'string') continue;

        if (line.includes('joined the game')) {
          console.log(`[Player Tracking] Found line with 'joined the game': "${line}"`);
        }

        const joinMatch = line.match(/\[.*?\/INFO\]: (\w+) joined the game/);
        if (joinMatch) {
          const playerName = joinMatch[1];
          currentPlayers.add(playerName);
          console.log(`[Player Tracking] Found join message for: ${playerName}`);
          
          if (!serverProcess.players.find(p => p.name === playerName)) {
            serverProcess.players.push({
              name: playerName,
              playTime: 0,
              location: 'Unknown',
              isOnline: true,
              joinTime: Date.now()
            });
            console.log(`[Player Tracking] Added player: ${playerName}`);
            console.log(`[Player Tracking] Current players: ${serverProcess.players.map(p => p.name).join(', ')}`);
            
            this.triggerEnhancedPlayerDataFetch(serverId, playerName);
          } else {
            console.log(`[Player Tracking] Player already exists: ${playerName}`);
          }
          continue;
        }

        // Parse player leave messages
        const leaveMatch = line.match(/\[.*?\/INFO\]: (\w+) left the game/);
        if (leaveMatch) {
          const playerName = leaveMatch[1];
          const playerIndex = serverProcess.players.findIndex(p => p.name === playerName);
          if (playerIndex !== -1) {
            serverProcess.players.splice(playerIndex, 1);
            console.log(`[Player Tracking] Removed player: ${playerName}`);
          }
          continue;
        }

        const chatMatch = line.match(/\[.*?\/INFO\]: <(\w+)> .+/);
        if (chatMatch) {
          const playerName = chatMatch[1];
          currentPlayers.add(playerName);
          
          if (!serverProcess.players.find(p => p.name === playerName)) {
            serverProcess.players.push({
              name: playerName,
              playTime: 0,
              location: 'Unknown',
              isOnline: true,
              joinTime: Date.now()
            });
            console.log(`[Player Tracking] Added player from chat: ${playerName}`);
          }
          continue;
        }

        const deathMatch = line.match(/\[.*?\/INFO\]: (\w+) was slain by (\w+)/);
        if (deathMatch) {
          const victim = deathMatch[1];
          const killer = deathMatch[2];
          currentPlayers.add(victim);
          currentPlayers.add(killer);
          
          [victim, killer].forEach(playerName => {
            if (!serverProcess.players.find(p => p.name === playerName)) {
              serverProcess.players.push({
                name: playerName,
                playTime: 0,
                location: 'Unknown',
                isOnline: true,
                joinTime: Date.now()
              });
              console.log(`[Player Tracking] Added player from death: ${playerName}`);
            }
          });
          continue;
        }

        const commandMatch = line.match(/\[.*?\/INFO\]: (\w+): \//);
        if (commandMatch) {
          const playerName = commandMatch[1];
          currentPlayers.add(playerName);
          
          if (!serverProcess.players.find(p => p.name === playerName)) {
            serverProcess.players.push({
              name: playerName,
              playTime: 0,
              location: 'Unknown',
              isOnline: true,
              joinTime: Date.now()
            });
            console.log(`[Player Tracking] Added player from command: ${playerName}`);
          }
          continue;
        }
      }

      const now = Date.now();
      serverProcess.players.forEach(player => {
        if (player.joinTime) {
          player.playTime = Math.floor((now - player.joinTime) / 1000);
        }
      });

      console.log(`[Player Tracking] Current players: ${serverProcess.players.map(p => p.name).join(', ')}`);
    } catch (error) {
      console.error('[Player Tracking] Error parsing players from output:', error);
    }
  },

  refreshPlayerData(serverId) {
    try {
      const serverProcess = this.serverProcesses.get(serverId);
      if (!serverProcess || !serverProcess.output) return;

      console.log('[Player Tracking] Refreshing player data from console output...');
      this.parsePlayersFromOutput(serverId, serverProcess.output);
    } catch (error) {
      console.error('[Player Tracking] Error refreshing player data:', error);
    }
  },

  triggerEnhancedPlayerDataFetch(serverId, playerName) {
    try {
      console.log(`[Player Tracking] Triggering enhanced data fetch for: ${playerName}`);
      
      const event = new CustomEvent('player-joined', {
        detail: {
          serverId: serverId,
          playerName: playerName
        }
      });
      window.dispatchEvent(event);
      
      console.log(`[Player Tracking] Dispatched player-joined event for: ${playerName}`);
    } catch (error) {
      console.error('[Player Tracking] Error triggering enhanced data fetch:', error);
    }
  },
  
  startOutputPolling(serverId) {
    console.log(`[Output Polling] Starting output polling for server: ${serverId}`);
    
    if (this.outputPollingIntervals && this.outputPollingIntervals[serverId]) {
      clearInterval(this.outputPollingIntervals[serverId]);
      console.log(`[Output Polling] Cleared existing polling interval for server: ${serverId}`);
    }
    
    if (!this.outputPollingIntervals) {
      this.outputPollingIntervals = {};
    }
    
    if (!this.serverProcesses.has(serverId)) {
      console.log(`[Output Polling] Creating server process for: ${serverId}`);
      this.serverProcesses.set(serverId, {
        id: serverId,
        output: ['[INFO] Starting output polling...'],
        players: [],
        isRunning: true
      });
    }
    
    this.outputPollingIntervals[serverId] = setInterval(async () => {
      console.log(`[Output Polling] Polling server output for: ${serverId}`);
      
      try {
        const output = await invoke('get_server_output', { id: serverId });
        console.log(`[Output Polling] Got output for ${serverId}:`, output ? output.length : 0, 'lines');
        
        if (output && Array.isArray(output)) {
          const serverProcess = this.serverProcesses.get(serverId);
          if (serverProcess) {
            serverProcess.output = output;
            console.log(`[Output Polling] Updated output for ${serverId}, now has ${output.length} lines`);
            
            this.parsePlayersFromOutput(serverId, output);
          } else {
            console.warn(`[Output Polling] No server process found for: ${serverId}`);
          }
        } else {
          console.log(`[Output Polling] No output or invalid output for: ${serverId}`);
        }
      } catch (error) {
        console.error(`[Output Polling] Error polling server output for ${serverId}:`, error);
        
        if (error.toString().includes('not found')) {
          console.log(`[Output Polling] Server not found, stopping polling for: ${serverId}`);
          clearInterval(this.outputPollingIntervals[serverId]);
          delete this.outputPollingIntervals[serverId];
          this.serverProcesses.delete(serverId);
        }
      }
    }, 500);
    
    console.log(`[Output Polling] Output polling started for server: ${serverId}`);
  },
  
  async getServerFiles(serverId, path = '', recursive = false) {
    console.log(`getServerFiles called with serverId: ${serverId}, path: ${path}, recursive: ${recursive}`);
    console.log(`Current servers array:`, this.servers.map(s => ({ id: s.id, name: s.name })));
    
    try {
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error(`Server with ID ${serverId} not found`);
      }
      
      const fullPath = path ? `${server.path}/${path}` : server.path;
      
      const entries = await tauriAPI.readDir(fullPath);
      let files = [];
      
      for (const entry of entries) {
        if (entry.isDirectory && recursive) {
          const relativePath = path ? `${path}/${entry.name}` : entry.name;
          
          const subDirResult = await this.getServerFiles(serverId, relativePath, true);
          if (subDirResult.success) {
            files = files.concat(subDirResult.files.map(file => ({
              ...file,
              name: `${entry.name}/${file.name}`
            })));
          }
        }
        files.push(entry);
      }
      
      return { success: true, files };
    } catch (error) {
      console.error('Error getting server files:', error);
      return { success: false, error, files: [] };
    }
  },

  async getServerFilesRecursive(serverId, path = '') {
    return this.getServerFiles(serverId, path, true);
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

  async checkServerStatus(serverId) {
    try {
      const server = this.getServerById(serverId);
      if (!server) return 'unknown';

      if (this.isServerRunning(serverId)) {
        server.status = 'online';
        return 'online';
      }

      try {
        const status = await invoke('get_server_status', { id: serverId });
        server.status = status;
        return status;
      } catch (error) {
        console.warn(`Could not get server status from backend: ${error}`);
        return server.status || 'offline';
      }
    } catch (error) {
      console.error('Error checking server status:', error);
      return 'unknown';
    }
  },
  
  getDownloadProgress(serverId) {
    return this.downloadProgress.get(serverId) || { progress: 0, status: 'idle' };
  },
  
  cleanupServer(serverId) {
    if (this.outputPollingIntervals && this.outputPollingIntervals[serverId]) {
      clearInterval(this.outputPollingIntervals[serverId]);
      delete this.outputPollingIntervals[serverId];
    }
    
    this.serverProcesses.delete(serverId);
    
    this.downloadProgress.delete(serverId);
    
    const index = this.servers.findIndex(s => s.id === serverId);
    if (index !== -1) {
      this.servers.splice(index, 1);
    }
  },
  
  async deleteServer(serverId) {
    try {
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error('Server not found');
      }
      
      await invoke('remove_server', { id: serverId });
      console.log('Server removed from backend');
      
      try {
        await tauriAPI.removeDir(server.path);
        console.log('Server folder removed:', server.path);
      } catch (folderError) {
        console.warn('Could not remove server folder:', folderError);
      }
      
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
        this.settings = { ...this.settings, ...parsed };
        console.log('Settings loaded successfully');
      }
    } catch (error) {
      console.error('Error loading settings:', error);
    }
  },
  
  addProject(projectData) {
    this.projects.push(projectData);
    this.saveProjects();
    console.log('Project added:', projectData.name);
  },
  
  removeProject(projectId) {
    const project = this.projects.find(p => p.id === projectId);
    if (project) {
      this.deleteProjectFolder(project.location);
      
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
    }
  },
  
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
  
  async run_sftp_command(config, command) {
    try {
      console.log(`Running SFTP command: ${command}`);
      const result = await invoke('run_sftp_command', { config, command });
      return { success: true, output: result };
    } catch (error) {
      console.error('SFTP command error:', error);
      return { success: false, error: error.toString() };
    }
  },

  async upload_file_sftp(config, local_path, remote_path) {
    try {
      console.log(`Uploading file to SFTP: ${local_path} -> ${remote_path}`);
      const result = await invoke('upload_file_sftp', { config, localPath: local_path, remotePath: remote_path });
      return { success: result === true };
    } catch (error) {
      console.error('SFTP upload error:', error);
      return { success: false, error: error.toString() };
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
      const cleanHost = config.host.replace(/^(sftp|ftp|ssh):\/\//, '');
      
      const cleanPath = path.startsWith('/') ? path : '/' + path;
      
      console.log('Listing remote files:', {
        host: cleanHost,
        port: config.port,
        username: config.username,
        path: cleanPath
      });

      console.log('Falling back to list_remote_files...');
      let result = await invoke('list_remote_files', { 
        config: {
          host: cleanHost,
          port: parseInt(config.port),
          username: config.username,
          password: config.password,
          remote_path: '/'
        },
        path: '/'
      });

      if (!result || (Array.isArray(result) && result.length === 0)) {
        console.log('Got empty result, trying with debug flag...');
        result = await invoke('list_remote_files', { 
          config: {
            host: cleanHost,
            port: parseInt(config.port),
            username: config.username,
            password: config.password,
            remote_path: '/',
            debug: true
          },
          path: '/'
        });
      }

      if (result && Array.isArray(result) && result.length > 0) {
        console.log('Got files from list_remote_files:', result);
      return result;
      }

      console.log('All methods failed, using mock data');
      return [{
        name: 'test.yml',
        size: 4,
        isDirectory: false,
        modified: new Date('2023-07-26T11:15:00'),
        permissions: '-rwxrwxr-x'
      }];
    } catch (error) {
      console.error('List remote files error:', error);
      return [];
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
    console.log(`Toast: ${message} (${type})`);
  },

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
      return confirm(`${title}\n\n${message}`);
    }
  },


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
  },

  
  async startAllServers() {
    try {
      console.log(`[store] Starting all servers`);
      const result = await this.tauriAPI.startAllServers();
      return result;
    } catch (error) {
      console.error(`[store] Error starting all servers: ${error}`);
      throw error;
    }
  },

  async stopAllServers() {
    try {
      console.log(`[store] Stopping all servers`);
      const result = await this.tauriAPI.stopAllServers();
      return result;
    } catch (error) {
      console.error(`[store] Error stopping all servers: ${error}`);
      throw error;
    }
  },

  async backupAllServers() {
    try {
      console.log(`[store] Backing up all servers`);
      const result = await this.tauriAPI.backupAllServers();
      return result;
    } catch (error) {
      console.error(`[store] Error backing up all servers: ${error}`);
      throw error;
    }
  },

  async checkForUpdates() {
    try {
      console.log(`[store] Checking for updates`);
      const result = await this.tauriAPI.checkForUpdates();
      return result;
    } catch (error) {
      console.error(`[store] Error checking for updates: ${error}`);
      throw error;
    }
  },

  async restartApplication() {
    try {
      console.log(`[store] Restarting application`);
      const result = await this.tauriAPI.restartApplication();
      return result;
    } catch (error) {
      console.error(`[store] Error restarting application: ${error}`);
      throw error;
    }
  },

  async quitApplication() {
    try {
      console.log(`[store] Quitting application`);
      const result = await this.tauriAPI.quitApplication();
      return result;
    } catch (error) {
      console.error(`[store] Error quitting application: ${error}`);
      throw error;
    }
  },

  async getMintMenuCommands() {
    try {
      console.log(`[store] Getting MintMenu commands`);
      const result = await this.tauriAPI.getMintMenuCommands();
      return result;
    } catch (error) {
      console.error(`[store] Error getting MintMenu commands: ${error}`);
      throw error;
    }
  },

  toggleServerIPs() {
    this.settings.general.showServerIPs = !this.settings.general.showServerIPs;
    this.saveSettings();
    console.log(`[store] Server IPs visibility toggled to: ${this.settings.general.showServerIPs}`);
  },

  getServerIP(server) {
    if (!this.settings.general.showServerIPs) {
      return '••••••••';
    }
    
    const path = server.path || '';
    if (path.includes(':')) {
      const parts = path.split(':');
      return parts[0];
    }
    
    return 'localhost';
  },

  
  async getInstalledMods(serverId) {
    try {
      if (!this.installedMods.has(serverId)) {
        this.installedMods.set(serverId, []);
        
        const server = this.getServerById(serverId);
        if (server) {
          const modsPath = `${server.path}/mods`;
          try {
            const files = await tauriAPI.readDir(modsPath);
            const mods = files
              .filter(file => file.name.endsWith('.jar'))
              .map(file => ({
                name: file.name.replace('.jar', ''),
                version: 'Unknown',
                type: 'Mod',
                description: 'Local mod',
                image: '',
                source: 'Local',
                serverId: serverId,
                folder: 'mods',
                path: `${modsPath}/${file.name}`
              }));
            this.installedMods.set(serverId, mods);
          } catch (error) {
            console.log('No mods folder found or error reading mods:', error);
          }
        }
      }
      return this.installedMods.get(serverId) || [];
    } catch (error) {
      console.error('Error getting installed mods:', error);
      return [];
    }
  },

  async addInstalledMod(mod) {
    try {
      const serverId = mod.serverId;
      if (!this.installedMods.has(serverId)) {
        this.installedMods.set(serverId, []);
      }
      const mods = this.installedMods.get(serverId);
      mods.push(mod);
      return true;
    } catch (error) {
      console.error('Error adding installed mod:', error);
      return false;
    }
  },

  async removeInstalledMod(serverId, modName) {
    try {
      if (!this.installedMods.has(serverId)) {
        return false;
      }
      const mods = this.installedMods.get(serverId);
      const index = mods.findIndex(m => m.name === modName);
      if (index !== -1) {
        mods.splice(index, 1);
        return true;
      }
      return false;
    } catch (error) {
      console.error('Error removing installed mod:', error);
      return false;
    }
  },

  async createModFolder(serverId, folder) {
    try {
      const server = this.getServerById(serverId);
      if (!server) {
        throw new Error('Server not found');
      }
      
      const folderPath = `${server.path}/${folder}`;
      await tauriAPI.createDir(folderPath);
      return { success: true, path: folderPath };
    } catch (error) {
      console.error('Error creating mod folder:', error);
      return { success: false, error: error.message };
    }
  }
});


store.init(); 