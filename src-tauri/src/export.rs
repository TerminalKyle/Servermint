use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use zip::write::FileOptions;
use zip::read::ZipArchive;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileToZip {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerMetadata {
    pub server_id: Option<String>,
    pub server_name: String,
    pub export_date: Option<String>,
    pub files: Vec<FileToZip>,
}

#[tauri::command]
pub async fn export_server_zip(server_id: String, files: Vec<FileToZip>, server_name: String) -> Result<(), String> {
    let first_file = files.first().ok_or("No files to export")?;
    let server_dir = std::path::Path::new(&first_file.path)
        .parent()
        .ok_or("Invalid file path")?;

    let exports_dir = server_dir.join("exports");
    std::fs::create_dir_all(&exports_dir).map_err(|e| format!("Failed to create exports directory: {}", e))?;
    
    let zip_path = exports_dir.join(format!("{}-{}.zip", server_name, chrono::Local::now().format("%Y%m%d-%H%M%S")));
    println!("Creating ZIP file at: {:?}", zip_path);
    println!("Files to include: {:?}", files);
    let file = File::create(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let metadata = serde_json::json!({
        "server_id": server_id,
        "server_name": server_name,
        "export_date": chrono::Utc::now().to_rfc3339(),
        "files": files
    });
    zip.start_file("servermint.json", options).map_err(|e| e.to_string())?;
    zip.write_all(metadata.to_string().as_bytes()).map_err(|e| e.to_string())?;

        
    for file_info in files {
        let mut file = File::open(&file_info.path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        zip.start_file(&file_info.name, options).map_err(|e| e.to_string())?;
        zip.write_all(&buffer).map_err(|e| e.to_string())?;
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn import_server_from_zip(zip_path: String, target_directory: String) -> Result<ServerMetadata, String> {
    println!("Importing server from ZIP: {}", zip_path);
    
    let file = File::open(&zip_path).map_err(|e| format!("Failed to open ZIP file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {}", e))?;
    
    std::fs::create_dir_all(&target_directory).map_err(|e| format!("Failed to create target directory: {}", e))?;
    
    let mut metadata: Option<ServerMetadata> = None;
    let mut extracted_files = Vec::new();
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to access file in ZIP: {}", e))?;
        let file_name = file.name().to_string();
        
        if file_name.starts_with("__MACOSX/") || file_name.starts_with(".DS_Store") {
            continue;
        }
        
        let outpath = PathBuf::from(&target_directory).join(&file_name);
        
        if file_name.ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
            }
            
            let mut outfile = File::create(&outpath).map_err(|e| format!("Failed to create file: {}", e))?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| format!("Failed to write file: {}", e))?;
            
            if file_name == "servermint.json" {
                let mut content = String::new();
                file.read_to_string(&mut content).map_err(|e| format!("Failed to read metadata: {}", e))?;
                metadata = serde_json::from_str(&content).map_err(|e| format!("Failed to parse metadata: {}", e))?;
            }
            
            extracted_files.push(FileToZip {
                name: file_name,
                path: outpath.to_string_lossy().to_string(),
            });
        }
    }
    
    if metadata.is_none() {
        metadata = Some(detect_launcher_type(&target_directory, extracted_files)?);
    }
    
    metadata.ok_or("Failed to extract metadata from ZIP".to_string())
}

fn detect_launcher_type(server_path: &str, files: Vec<FileToZip>) -> Result<ServerMetadata, String> {
    let server_properties_path = format!("{}/server.properties", server_path);
    
    let server_name = if std::path::Path::new(&server_properties_path).exists() {
        if let Ok(content) = std::fs::read_to_string(&server_properties_path) {
            for line in content.lines() {
                if line.starts_with("motd=") {
                    let motd = line[5..].trim();
                    if !motd.is_empty() && motd != "A Minecraft Server" {
                        return Ok(ServerMetadata {
                            server_id: None,
                            server_name: motd.to_string(),
                            export_date: None,
                            files,
                        });
                    }
                }
            }
        }
        "Imported Server".to_string()
    } else {
        "Imported Server".to_string()
    };
    
    Ok(ServerMetadata {
        server_id: None,
        server_name,
        export_date: None,
        files,
    })
} 