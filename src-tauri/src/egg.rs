use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use tauri::State;
use tokio::process;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Egg {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub category: String,
    pub script: String,
    pub variables: Vec<EggVariable>,
    pub startup_command: String,
    pub file_denylist: Vec<String>,
    pub file_allowlist: Vec<String>,
    pub features: Vec<String>,
    pub docker_image: Option<String>,
    pub docker_startup: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggVariable {
    pub name: String,
    pub description: String,
    pub default_value: String,
    pub env_variable: String,
    pub rules: String,
    pub field_type: String,
    pub is_viewable: bool,
    pub is_rules: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggInstallation {
    pub egg_id: String,
    pub server_path: String,
    pub variables: HashMap<String, String>,
    pub environment: HashMap<String, String>,
}

impl Egg {
    pub fn new(
        id: String,
        name: String,
        description: String,
        author: String,
        version: String,
        category: String,
        script: String,
        startup_command: String,
    ) -> Self {
        Egg {
            id,
            name,
            description,
            author,
            version,
            category,
            script,
            variables: Vec::new(),
            startup_command,
            file_denylist: Vec::new(),
            file_allowlist: Vec::new(),
            features: Vec::new(),
            docker_image: None,
            docker_startup: None,
        }
    }

    pub fn add_variable(&mut self, variable: EggVariable) {
        self.variables.push(variable);
    }

    pub fn add_feature(&mut self, feature: String) {
        self.features.push(feature);
    }

    pub fn set_docker_image(&mut self, image: String) {
        self.docker_image = Some(image);
    }

    pub fn set_docker_startup(&mut self, startup: String) {
        self.docker_startup = Some(startup);
    }
}

#[derive(Clone)]
pub struct EggManager {
    eggs_dir: String,
    eggs: HashMap<String, Egg>,
}

impl EggManager {
    pub fn new() -> Result<Self, String> {
        let app_data_dir = std::env::var("APPDATA")
            .unwrap_or_else(|_| std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        let eggs_dir = format!("{}/ServerMint/eggs", app_data_dir);
        
        // Create eggs directory if it doesn't exist
        if !Path::new(&eggs_dir).exists() {
            fs::create_dir_all(&eggs_dir)
                .map_err(|e| format!("Failed to create eggs directory: {}", e))?;
        }

        let mut manager = EggManager {
            eggs_dir,
            eggs: HashMap::new(),
        };

        // Load built-in eggs
        manager.load_builtin_eggs()?;
        
        // Load custom eggs from disk
        manager.load_custom_eggs()?;

        Ok(manager)
    }

    fn load_builtin_eggs(&mut self) -> Result<(), String> {
        // Vanilla Minecraft Egg
        let mut vanilla_egg = Egg::new(
            "vanilla".to_string(),
            "Vanilla Minecraft".to_string(),
            "Official Minecraft server jar".to_string(),
            "ServerMint".to_string(),
            "1.0.0".to_string(),
            "Minecraft".to_string(),
            include_str!("eggs/vanilla.sh").to_string(),
            "java -Xms${MIN_MEMORY}M -Xmx${MAX_MEMORY}M -jar server.jar nogui".to_string(),
        );

        vanilla_egg.add_variable(EggVariable {
            name: "Server Version".to_string(),
            description: "Minecraft server version to install".to_string(),
            default_value: "latest".to_string(),
            env_variable: "VANILLA_VERSION".to_string(),
            rules: "required|string".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        vanilla_egg.add_variable(EggVariable {
            name: "Server JAR File".to_string(),
            description: "Name of the server jar file".to_string(),
            default_value: "server.jar".to_string(),
            env_variable: "SERVER_JARFILE".to_string(),
            rules: "required|string".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        vanilla_egg.add_variable(EggVariable {
            name: "Minimum Memory".to_string(),
            description: "Minimum memory allocation in MB".to_string(),
            default_value: "512".to_string(),
            env_variable: "MIN_MEMORY".to_string(),
            rules: "required|numeric|min:256|max:32768".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        vanilla_egg.add_variable(EggVariable {
            name: "Maximum Memory".to_string(),
            description: "Maximum memory allocation in MB".to_string(),
            default_value: "1024".to_string(),
            env_variable: "MAX_MEMORY".to_string(),
            rules: "required|numeric|min:512|max:32768".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        self.eggs.insert(vanilla_egg.id.clone(), vanilla_egg);

        // Paper Minecraft Egg
        let mut paper_egg = Egg::new(
            "paper".to_string(),
            "Paper Minecraft".to_string(),
            "High-performance Minecraft server with Spigot/Paper API".to_string(),
            "ServerMint".to_string(),
            "1.0.0".to_string(),
            "Minecraft".to_string(),
            include_str!("eggs/paper.sh").to_string(),
            "java -Xms${MIN_MEMORY}M -Xmx${MAX_MEMORY}M -jar server.jar nogui".to_string(),
        );

        paper_egg.add_variable(EggVariable {
            name: "Server Version".to_string(),
            description: "Paper server version to install".to_string(),
            default_value: "latest".to_string(),
            env_variable: "PAPER_VERSION".to_string(),
            rules: "required|string".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        paper_egg.add_variable(EggVariable {
            name: "Server JAR File".to_string(),
            description: "Name of the server jar file".to_string(),
            default_value: "server.jar".to_string(),
            env_variable: "SERVER_JARFILE".to_string(),
            rules: "required|string".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        self.eggs.insert(paper_egg.id.clone(), paper_egg);

        // Fabric Egg
        let mut fabric_egg = Egg::new(
            "fabric".to_string(),
            "Fabric Minecraft".to_string(),
            "Lightweight modding platform for Minecraft".to_string(),
            "ServerMint".to_string(),
            "1.0.0".to_string(),
            "Minecraft".to_string(),
            include_str!("eggs/fabric.sh").to_string(),
            "java -Xms${MIN_MEMORY}M -Xmx${MAX_MEMORY}M -jar server.jar nogui".to_string(),
        );

        fabric_egg.add_variable(EggVariable {
            name: "Minecraft Version".to_string(),
            description: "Minecraft version to use".to_string(),
            default_value: "1.21.2".to_string(),
            env_variable: "MINECRAFT_VERSION".to_string(),
            rules: "required|string".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        fabric_egg.add_variable(EggVariable {
            name: "Fabric Loader Version".to_string(),
            description: "Fabric loader version".to_string(),
            default_value: "latest".to_string(),
            env_variable: "FABRIC_LOADER_VERSION".to_string(),
            rules: "required|string".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        self.eggs.insert(fabric_egg.id.clone(), fabric_egg);

        // PocketMine-MP Egg
        let mut pocketmine_egg = Egg::new(
            "pocketmine".to_string(),
            "PocketMine-MP".to_string(),
            "Bedrock server software for Minecraft PE/Bedrock".to_string(),
            "ServerMint".to_string(),
            "1.0.0".to_string(),
            "Bedrock".to_string(),
            include_str!("eggs/pocketmine.sh").to_string(),
            "php PocketMine-MP.phar".to_string(),
        );

        pocketmine_egg.add_variable(EggVariable {
            name: "PocketMine Version".to_string(),
            description: "PocketMine-MP version to install".to_string(),
            default_value: "latest".to_string(),
            env_variable: "POCKETMINE_VERSION".to_string(),
            rules: "required|string".to_string(),
            field_type: "text".to_string(),
            is_viewable: true,
            is_rules: false,
        });

        self.eggs.insert(pocketmine_egg.id.clone(), pocketmine_egg);

        Ok(())
    }

    fn load_custom_eggs(&mut self) -> Result<(), String> {
        if let Ok(entries) = fs::read_dir(&self.eggs_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                        if let Ok(content) = fs::read_to_string(&path) {
                                                    if let Ok(egg) = serde_json::from_str::<Egg>(&content) {
                            let egg_name = egg.name.clone();
                            self.eggs.insert(egg.id.clone(), egg);
                            println!("Loaded custom egg: {}", egg_name);
                        }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_egg(&self, id: &str) -> Option<&Egg> {
        self.eggs.get(id)
    }

    pub fn list_eggs(&self) -> Vec<&Egg> {
        self.eggs.values().collect()
    }

    pub fn list_eggs_by_category(&self, category: &str) -> Vec<&Egg> {
        self.eggs.values()
            .filter(|egg| egg.category.to_lowercase() == category.to_lowercase())
            .collect()
    }

    pub fn add_custom_egg(&mut self, egg: Egg) -> Result<(), String> {
        let file_path = format!("{}/{}.json", self.eggs_dir, egg.id);
        let json = serde_json::to_string_pretty(&egg)
            .map_err(|e| format!("Failed to serialize egg: {}", e))?;
        
        fs::write(&file_path, json)
            .map_err(|e| format!("Failed to write egg file: {}", e))?;
        
        self.eggs.insert(egg.id.clone(), egg);
        Ok(())
    }

    pub fn remove_custom_egg(&mut self, id: &str) -> Result<(), String> {
        if let Some(egg) = self.eggs.get(id) {
            // Don't allow removal of built-in eggs
            if egg.author == "ServerMint" {
                return Err("Cannot remove built-in eggs".to_string());
            }
        }

        let file_path = format!("{}/{}.json", self.eggs_dir, id);
        if Path::new(&file_path).exists() {
            fs::remove_file(&file_path)
                .map_err(|e| format!("Failed to remove egg file: {}", e))?;
        }

        self.eggs.remove(id);
        Ok(())
    }

    pub async fn install_server_from_egg(
        &self,
        installation: EggInstallation,
    ) -> Result<String, String> {
        let egg = self.get_egg(&installation.egg_id)
            .ok_or_else(|| format!("Egg {} not found", installation.egg_id))?;

        // Create server directory
        fs::create_dir_all(&installation.server_path)
            .map_err(|e| format!("Failed to create server directory: {}", e))?;

        // Prepare environment variables
        let mut env_vars: HashMap<String, String> = installation.environment.clone();
        
        // Add egg variables to environment
        for variable in &egg.variables {
            let value = installation.variables
                .get(&variable.name)
                .unwrap_or(&variable.default_value)
                .clone();
            env_vars.insert(variable.env_variable.clone(), value);
        }

        // Add common environment variables
        env_vars.insert("SERVER_PATH".to_string(), installation.server_path.clone());
        env_vars.insert("PWD".to_string(), installation.server_path.clone());

        // Create and execute the installation script
        let script_path = format!("{}/install.sh", installation.server_path);
        
        // Write the script to file
        let script_content = self.process_script_template(&egg.script, &env_vars);
        fs::write(&script_path, script_content)
            .map_err(|e| format!("Failed to write installation script: {}", e))?;

        // Make script executable (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms)
                .map_err(|e| format!("Failed to set script permissions: {}", e))?;
        }

        // Execute the installation script
        let output = tokio::process::Command::new("sh")
            .arg(&script_path)
            .current_dir(&installation.server_path)
            .envs(env_vars.iter())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to execute installation script: {}", e))?;

        if output.status.success() {
            // Clean up the installation script
            let _ = fs::remove_file(&script_path);
            
            Ok(format!(
                "Server installed successfully from egg: {}\nOutput: {}",
                egg.name,
                String::from_utf8_lossy(&output.stdout)
            ))
        } else {
            let error_output = String::from_utf8_lossy(&output.stderr);
            Err(format!(
                "Installation failed for egg: {}\nError: {}",
                egg.name,
                error_output
            ))
        }
    }

    fn process_script_template(&self, script: &str, env_vars: &HashMap<String, String>) -> String {
        let mut processed_script = script.to_string();
        
        // Replace environment variables in the script
        for (key, value) in env_vars {
            let placeholder = format!("${{{}}}", key);
            processed_script = processed_script.replace(&placeholder, value);
        }

        processed_script
    }
}

// Global egg manager instance
lazy_static::lazy_static! {
    static ref EGG_MANAGER: Arc<Mutex<EggManager>> = {
        Arc::new(Mutex::new(EggManager::new().expect("Failed to initialize egg manager")))
    };
}

// Tauri commands
#[tauri::command]
pub fn list_eggs() -> Result<Vec<Egg>, String> {
    let manager = EGG_MANAGER.lock().map_err(|e| format!("Failed to lock egg manager: {}", e))?;
    Ok(manager.list_eggs().into_iter().cloned().collect())
}

#[tauri::command]
pub fn get_egg(egg_id: String) -> Result<Option<Egg>, String> {
    let manager = EGG_MANAGER.lock().map_err(|e| format!("Failed to lock egg manager: {}", e))?;
    Ok(manager.get_egg(&egg_id).cloned())
}

#[tauri::command]
pub fn list_eggs_by_category(category: String) -> Result<Vec<Egg>, String> {
    let manager = EGG_MANAGER.lock().map_err(|e| format!("Failed to lock egg manager: {}", e))?;
    Ok(manager.list_eggs_by_category(&category).into_iter().cloned().collect())
}

#[tauri::command]
pub fn add_custom_egg(egg: Egg) -> Result<(), String> {
    let mut manager = EGG_MANAGER.lock().map_err(|e| format!("Failed to lock egg manager: {}", e))?;
    manager.add_custom_egg(egg)
}

#[tauri::command]
pub fn remove_custom_egg(egg_id: String) -> Result<(), String> {
    let mut manager = EGG_MANAGER.lock().map_err(|e| format!("Failed to lock egg manager: {}", e))?;
    manager.remove_custom_egg(&egg_id)
}

#[tauri::command]
pub async fn install_server_from_egg(
    egg_id: String,
    server_path: String,
    variables: HashMap<String, String>,
    environment: HashMap<String, String>,
) -> Result<String, String> {
    let installation = EggInstallation {
        egg_id,
        server_path,
        variables,
        environment,
    };
    
    // Get the manager and clone it, then release the lock immediately
    let manager = {
        let guard = EGG_MANAGER.lock().map_err(|e| format!("Failed to lock egg manager: {}", e))?;
        guard.clone()
    };
    
    manager.install_server_from_egg(installation).await
} 