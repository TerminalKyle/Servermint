use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use zip::write::FileOptions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileToZip {
    pub name: String,
    pub path: String,
}

#[tauri::command]
pub async fn export_server_zip(server_id: String, files: Vec<FileToZip>, server_name: String) -> Result<(), String> {
    // Get the server directory from the first file's path
    let first_file = files.first().ok_or("No files to export")?;
    let server_dir = std::path::Path::new(&first_file.path)
        .parent()
        .ok_or("Invalid file path")?;
    
    // Create exports directory if it doesn't exist
    let exports_dir = server_dir.join("exports");
    std::fs::create_dir_all(&exports_dir).map_err(|e| format!("Failed to create exports directory: {}", e))?;
    
    // Create the zip file in the exports directory
    let zip_path = exports_dir.join(format!("{}-{}.zip", server_name, chrono::Local::now().format("%Y%m%d-%H%M%S")));
    println!("Creating ZIP file at: {:?}", zip_path);
    println!("Files to include: {:?}", files);
    let file = File::create(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Add metadata file
    let metadata = serde_json::json!({
        "server_id": server_id,
        "server_name": server_name,
        "export_date": chrono::Utc::now().to_rfc3339(),
        "files": files
    });
    zip.start_file("servermint.json", options).map_err(|e| e.to_string())?;
    zip.write_all(metadata.to_string().as_bytes()).map_err(|e| e.to_string())?;

    // Add server files
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