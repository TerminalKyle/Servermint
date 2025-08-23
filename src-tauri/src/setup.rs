use std::fs;
use std::path::Path;

pub fn ensure_app_directories() -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = Path::new("C:/servermint");
    if !app_dir.exists() {
        fs::create_dir_all(app_dir)?;
        println!("Created main application directory: {:?}", app_dir);
    }

    let servers_dir = app_dir.join("servers");
    if !servers_dir.exists() {
        fs::create_dir_all(&servers_dir)?;
        println!("Created servers directory: {:?}", servers_dir);
    }

    // Create any other required directories here
    // For example:
    // let backups_dir = app_dir.join("backups");
    // if !backups_dir.exists() {
    //     fs::create_dir_all(&backups_dir)?;
    //     println!("Created backups directory: {:?}", backups_dir);
    // }

    Ok(())
} 