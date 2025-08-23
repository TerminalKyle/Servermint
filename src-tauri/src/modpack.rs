use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModpackInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub game_version: String,
    pub loader: String,
    pub source: String, // "curseforge" or "modrinth"
    pub mods: Vec<ModInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub file_id: Option<String>,
    pub project_id: Option<String>,
    pub url: Option<String>,
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeManifest {
    pub minecraft: CurseForgeMinecraft,
    pub manifestType: String,
    pub manifestVersion: u32,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<CurseForgeFile>,
    pub overrides: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeMinecraft {
    pub version: String,
    pub modLoaders: Vec<CurseForgeModLoader>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeModLoader {
    pub id: String,
    pub primary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeFile {
    pub fileID: u32,
    pub projectID: u32,
    pub required: bool,
    pub fileName: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeProject {
    pub id: u32,
    pub name: String,
    pub summary: Option<String>,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeFileInfo {
    pub id: u32,
    pub displayName: String,
    pub fileName: String,
    pub downloadUrl: Option<String>,
    pub gameVersions: Vec<String>,
    pub releaseType: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthIndex {
    pub formatVersion: u32,
    pub game: String,
    pub versionId: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<ModrinthFile>,
    pub dependencies: ModrinthDependencies,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthFile {
    pub path: String,
    pub hashes: HashMap<String, String>,
    pub env: Option<ModrinthEnv>,
    pub downloads: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthEnv {
    pub client: String,
    pub server: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthDependencies {
    pub minecraft: String,
    pub forge: Option<String>,
    pub fabric_loader: Option<String>,
}

pub async fn install_modpack(
    modpack_path: String,
    server_path: String,
    server_id: String,
) -> Result<ModpackInfo, String> {
    println!("=== Starting modpack installation ===");
    println!("Modpack path: {}", modpack_path);
    println!("Server path: {}", server_path);
    println!("Server ID: {}", server_id);

    let modpack_info = detect_and_parse_modpack(&modpack_path).await?;
    println!("Detected modpack: {} v{} ({})", modpack_info.name, modpack_info.version, modpack_info.source);

    create_server_directories(&server_path)?;

    let temp_dir = extract_modpack(&modpack_path)?;

    match modpack_info.source.as_str() {
        "curseforge" => install_curseforge_modpack(&modpack_info, &temp_dir, &server_path).await?,
        "modrinth" => install_modrinth_modpack(&modpack_info, &temp_dir, &server_path).await?,
        _ => return Err(format!("Unsupported modpack source: {}", modpack_info.source)),
    }

    if let Err(e) = fs::remove_dir_all(&temp_dir) {
        println!("Warning: Failed to clean up temp directory: {}", e);
    }

    println!("=== Modpack installation completed successfully ===");
    Ok(modpack_info)
}

async fn detect_and_parse_modpack(modpack_path: &str) -> Result<ModpackInfo, String> {
    let path = Path::new(modpack_path);
    if !path.exists() {
        return Err(format!("Modpack file not found: {}", modpack_path));
    }

    let file = File::open(path)
        .map_err(|e| format!("Failed to open modpack file: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    if matches!(archive.by_name("manifest.json"), Ok(_)) {
        let manifest_content = {
            let mut manifest_file = archive.by_name("manifest.json")
                .map_err(|e| format!("Failed to read manifest.json: {}", e))?;
            let mut content = String::new();
            manifest_file.read_to_string(&mut content)
                .map_err(|e| format!("Failed to read manifest content: {}", e))?;
            content
        };
        return parse_curseforge_manifest_from_content(&manifest_content).await;
    }

    if matches!(archive.by_name("modrinth.index.json"), Ok(_)) {
        return parse_modrinth_index(&mut archive);
    }

    Err("Could not detect modpack format. Expected manifest.json (CurseForge) or modrinth.index.json (Modrinth)".to_string())
}

async fn parse_curseforge_manifest_from_content(manifest_content: &str) -> Result<ModpackInfo, String> {
    let manifest: CurseForgeManifest = serde_json::from_str(manifest_content)
        .map_err(|e| format!("Failed to parse manifest.json: {}", e))?;

    let mut mods = Vec::new();
    for file in &manifest.files {
        println!("Debug: Project ID: {}, File ID: {}, FileName: {:?}", 
                 file.projectID, file.fileID, file.fileName);
        
        mods.push(ModInfo {
            name: format!("mod-{}", file.projectID),
            version: "unknown".to_string(),
            file_id: Some(file.fileID.to_string()),
            project_id: Some(file.projectID.to_string()),
            url: None,
            filename: file.fileName.clone().unwrap_or_else(|| format!("{}.jar", file.projectID)),
        });
    }

    let loader = manifest.minecraft.modLoaders
        .iter()
        .find(|l| l.primary)
        .map(|l| l.id.clone())
        .unwrap_or_else(|| "unknown".to_string());

    let modpack_info = ModpackInfo {
        name: manifest.name,
        version: manifest.version,
        author: manifest.author,
        description: "CurseForge modpack".to_string(),
        game_version: manifest.minecraft.version,
        loader,
        source: "curseforge".to_string(),
        mods,
    };

    let mut enriched_mods = modpack_info.mods;
    enrich_curseforge_mods(&mut enriched_mods).await?;

    Ok(ModpackInfo {
        mods: enriched_mods,
        ..modpack_info
    })
}

async fn parse_curseforge_manifest(archive: &mut ZipArchive<File>) -> Result<ModpackInfo, String> {
    let mut manifest_file = archive.by_name("manifest.json")
        .map_err(|e| format!("Failed to read manifest.json: {}", e))?;

    let mut manifest_content = String::new();
    manifest_file.read_to_string(&mut manifest_content)
        .map_err(|e| format!("Failed to read manifest content: {}", e))?;

    parse_curseforge_manifest_from_content(&manifest_content).await
}

fn parse_modrinth_index(archive: &mut ZipArchive<File>) -> Result<ModpackInfo, String> {
    let mut index_file = archive.by_name("modrinth.index.json")
        .map_err(|e| format!("Failed to read modrinth.index.json: {}", e))?;

    let mut index_content = String::new();
    index_file.read_to_string(&mut index_content)
        .map_err(|e| format!("Failed to read index content: {}", e))?;

    let index: ModrinthIndex = serde_json::from_str(&index_content)
        .map_err(|e| format!("Failed to parse modrinth.index.json: {}", e))?;

    let mut mods = Vec::new();
    for file in &index.files {
        if file.path.ends_with(".jar") {
            let filename = Path::new(&file.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown.jar")
                .to_string();

            mods.push(ModInfo {
                name: filename.replace(".jar", ""),
                version: "unknown".to_string(),
                file_id: None,
                project_id: None,
                url: file.downloads.first().cloned(),
                filename,
            });
        }
    }

    let loader = if index.dependencies.forge.is_some() {
        "forge".to_string()
    } else if index.dependencies.fabric_loader.is_some() {
        "fabric".to_string()
    } else {
        "unknown".to_string()
    };

    Ok(ModpackInfo {
        name: index.name,
        version: index.versionId,
        author: "Unknown".to_string(),
        description: index.summary.unwrap_or_else(|| "Modrinth modpack".to_string()),
        game_version: index.dependencies.minecraft,
        loader,
        source: "modrinth".to_string(),
        mods,
    })
}

fn create_server_directories(server_path: &str) -> Result<(), String> {
    let dirs = ["mods", "config", "resourcepacks", "datapacks", "world"];
    
    for dir in &dirs {
        let dir_path = format!("{}/{}", server_path, dir);
        fs::create_dir_all(&dir_path)
            .map_err(|e| format!("Failed to create directory {}: {}", dir_path, e))?;
    }

    println!("Created server directories");
    Ok(())
}

fn extract_modpack(modpack_path: &str) -> Result<PathBuf, String> {
    let temp_dir = std::env::temp_dir().join(format!("servermint_modpack_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()));

    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;

    let file = File::open(modpack_path)
        .map_err(|e| format!("Failed to open modpack file: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to access file in zip: {}", e))?;

        let outpath = temp_dir.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
            }

            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("Failed to create file: {}", e))?;

            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }

    println!("Extracted modpack to: {:?}", temp_dir);
    Ok(temp_dir)
}

async fn install_curseforge_modpack(
    modpack_info: &ModpackInfo,
    temp_dir: &Path,
    server_path: &str,
) -> Result<(), String> {
    println!("Installing CurseForge modpack...");

    let overrides_path = temp_dir.join("overrides");
    if overrides_path.exists() {
        copy_directory_recursive(&overrides_path, Path::new(server_path))
            .map_err(|e| format!("Failed to copy overrides: {}", e))?;
        println!("Copied overrides folder");
    }

    let mods_dir = format!("{}/mods", server_path);
    fs::create_dir_all(&mods_dir)
        .map_err(|e| format!("Failed to create mods directory: {}", e))?;

    println!("Downloading {} mods from CurseForge...", modpack_info.mods.len());
    
    for (index, mod_info) in modpack_info.mods.iter().enumerate() {
        println!("Downloading mod {}/{}: {}", index + 1, modpack_info.mods.len(), mod_info.name);
        
        if let Some(file_id_str) = &mod_info.file_id {
            if let Ok(file_id) = file_id_str.parse::<u32>() {
                let mod_path = format!("{}/{}", mods_dir, mod_info.filename);
                
                let url_patterns = vec![
                    get_curseforge_url_pattern1(0, file_id, &mod_info.filename),
                    get_curseforge_url_pattern2(0, file_id, &mod_info.filename),
                    get_curseforge_url_pattern3(0, file_id, &mod_info.filename),
                ];
                
                let mut download_success = false;
                for (pattern_num, download_url) in url_patterns.iter().enumerate() {
                    println!("Trying pattern {} for {}: {}", pattern_num + 1, mod_info.name, download_url);
                    
                    match download_file(download_url.clone(), mod_path.clone()).await {
                        Ok(_) => {
                            println!("Successfully downloaded: {} using pattern {}", mod_info.filename, pattern_num + 1);
                            download_success = true;
                            break;
                        }
                        Err(e) => {
                            println!("Pattern {} failed for {}: {}", pattern_num + 1, mod_info.filename, e);
                        }
                    }
                }
                
                if !download_success {
                    println!("Warning: All download patterns failed for {}", mod_info.name);
                }
            }
        } else {
            println!("Warning: No file ID available for {}", mod_info.name);
        }
    }

    println!("CurseForge modpack installation completed!");
    Ok(())
}

async fn install_modrinth_modpack(
    modpack_info: &ModpackInfo,
    temp_dir: &Path,
    server_path: &str,
) -> Result<(), String> {
    println!("Installing Modrinth modpack...");

    let overrides_path = temp_dir.join("overrides");
    if overrides_path.exists() {
        copy_directory_recursive(&overrides_path, Path::new(server_path))
            .map_err(|e| format!("Failed to copy overrides: {}", e))?;
        println!("Copied overrides folder");
    }

    for mod_info in &modpack_info.mods {
        if let Some(url) = &mod_info.url {
            let mod_path = format!("{}/mods/{}", server_path, mod_info.filename);
            
            println!("Downloading mod: {} from {}", mod_info.name, url);
            
            match download_file(url.clone(), mod_path.clone()).await {
                Ok(_) => println!("Successfully downloaded: {}", mod_info.filename),
                Err(e) => println!("Warning: Failed to download {}: {}", mod_info.filename, e),
            }
        }
    }

    Ok(())
}

fn copy_directory_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if src.is_dir() {
        if !dst.exists() {
            fs::create_dir_all(dst)
                .map_err(|e| format!("Failed to create destination directory: {}", e))?;
        }

        for entry in fs::read_dir(src)
            .map_err(|e| format!("Failed to read source directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let ty = entry.file_type()
                .map_err(|e| format!("Failed to get file type: {}", e))?;
            
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if ty.is_dir() {
                copy_directory_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            }
        }
    }
    Ok(())
}

async fn download_file(url: String, destination: String) -> Result<String, String> {
    use crate::server::download_file;
    download_file(url, destination).await
}

fn get_curseforge_download_url_with_filename(project_id: u32, file_id: u32, filename: &str) -> String {
    format!("https://edge.forgecdn.net/files/{}/{}/{}", file_id / 1000, file_id % 1000, filename)
}

fn get_curseforge_alternative_url(project_id: u32, file_id: u32, filename: &str) -> String {
    format!("https://files.forgecdn.net/files/{}/{}/{}", file_id / 1000, file_id % 1000, filename)
}

fn get_curseforge_url_pattern1(project_id: u32, file_id: u32, filename: &str) -> String {
    format!("https://edge.forgecdn.net/files/{}/{}/{}", file_id / 1000, file_id % 1000, filename)
}

fn get_curseforge_url_pattern2(project_id: u32, file_id: u32, filename: &str) -> String {
    format!("https://files.forgecdn.net/files/{}/{}/{}", file_id / 1000, file_id % 1000, filename)
}

fn get_curseforge_url_pattern3(project_id: u32, file_id: u32, filename: &str) -> String {
    format!("https://mediafilez.forgecdn.net/files/{}/{}/{}", file_id / 1000, file_id % 1000, filename)
}

async fn get_curseforge_file_info_with_api(file_id: u32) -> Option<String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.curseforge.com/v1/mods/files/{}", file_id);
    
    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header("x-api-key", "5b9a642d-ee9e-4ec5-a2aa-b21537d17168")
        .header("User-Agent", "ServerMint/1.0")
        .send()
        .await.ok()?;
    
    if response.status().is_success() {
        if let Ok(api_response) = response.json::<CurseForgeApiResponse<CurseForgeFileInfo>>().await {
            return Some(api_response.data.fileName);
        }
    }
    
    None
}

async fn enrich_curseforge_mods(mods: &mut Vec<ModInfo>) -> Result<(), String> {
    println!("Enriching CurseForge mods with actual filenames...");
    
    let mut success_count = 0;
    
    for mod_info in mods.iter_mut() {
        if let Some(project_id_str) = &mod_info.project_id {
            if let Ok(project_id) = project_id_str.parse::<u32>() {
                if mod_info.name.starts_with("mod-") {
                    mod_info.name = format!("CurseForge Mod {}", project_id);
                }
                
                if let Some(file_id_str) = &mod_info.file_id {
                    if let Ok(file_id) = file_id_str.parse::<u32>() {
                        if let Some(actual_filename) = get_curseforge_file_info_with_api(file_id).await {
                            let filename_clone = actual_filename.clone();
                            mod_info.filename = actual_filename;
                            success_count += 1;
                            println!("Got actual filename for {}: {}", mod_info.name, filename_clone);
                        } else {
                            println!("Could not get filename for {}, using fallback", mod_info.name);
                        }
                    }
                }
            }
        }
    }
    
    println!("Successfully got filenames for {}/{} mods", success_count, mods.len());
    Ok(())
}

#[tauri::command]
pub async fn install_modpack_from_file(
    modpack_path: String,
    server_id: String,
    server_path: String,
) -> Result<ModpackInfo, String> {
    install_modpack(modpack_path, server_path, server_id).await
}

#[tauri::command]
pub async fn analyze_modpack_file(
    modpack_path: String,
) -> Result<ModpackInfo, String> {
    println!("=== Analyzing modpack file ===");
    println!("Modpack path: {}", modpack_path);

    let modpack_info = detect_and_parse_modpack(&modpack_path).await?;
    println!("Detected modpack: {} v{} ({})", modpack_info.name, modpack_info.version, modpack_info.source);
    
    Ok(modpack_info)
} 