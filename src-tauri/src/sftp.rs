use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use tokio::task;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SftpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub remote_path: String,
}

impl SftpConfig {
    fn create_temp_files(&self, commands: &[&str]) -> Result<(std::path::PathBuf, std::path::PathBuf), String> {
        let temp_dir = std::env::temp_dir();
        let batch_path = temp_dir.join("sftp_commands.txt");
        let ps_path = temp_dir.join("sftp_script.ps1");

        // Create batch file with commands
        fs::write(&batch_path, commands.join("\n"))
            .map_err(|e| format!("Failed to create batch file: {}", e))?;

        // Create PowerShell script that handles password input
        let ps_script = format!(
            r#"
            $password = "{}"
            $securePassword = ConvertTo-SecureString $password -AsPlainText -Force
            $username = "{}"
            $sftpHost = "{}"
            $port = {}
            $batchFile = "{}"
            
            # Download plink.exe and psftp.exe if not exists
            $psftp = Join-Path $env:TEMP "psftp.exe"
            $plink = Join-Path $env:TEMP "plink.exe"
            if (-not (Test-Path $psftp)) {{
                $url = "https://the.earth.li/~sgtatham/putty/latest/w64/psftp.exe"
                Invoke-WebRequest -Uri $url -OutFile $psftp
            }}
            if (-not (Test-Path $plink)) {{
                $url = "https://the.earth.li/~sgtatham/putty/latest/w64/plink.exe"
                Invoke-WebRequest -Uri $url -OutFile $plink
            }}

            # Get host key using plink in verbose mode
            $plinkOutput = & $plink -v -P $port -pw $password "$username@$sftpHost" "exit" 2>&1
            $hostKey = $plinkOutput | Select-String "Server host key is" | ForEach-Object {{ $_.ToString().Split(":")[1].Trim() }}
            
            if ($hostKey) {{
                # Create registry entry to auto-accept host key
                $regPath = "HKCU:\Software\SimonTatham\PuTTY\SshHostKeys"
                $keyName = "ssh-ed25519@$($port):$($sftpHost)"
                
                if (-not (Test-Path $regPath)) {{
                    New-Item -Path $regPath -Force | Out-Null
                }}
                New-ItemProperty -Path $regPath -Name $keyName -Value $hostKey -PropertyType String -Force | Out-Null
            }}

            # Run psftp command in batch mode
            $output = & $psftp -P $port -pw $password -batch -b $batchFile "$username@$sftpHost" 2>&1
            $output | ForEach-Object {{ Write-Host $_ }}
            "#,
            self.password,
            self.username,
            self.host,
            self.port,
            batch_path.display()
        );

        fs::write(&ps_path, ps_script)
            .map_err(|e| format!("Failed to create PowerShell script: {}", e))?;

        Ok((batch_path, ps_path))
    }

    fn run_sftp_command(&self, _batch_path: &Path, ps_path: &Path) -> Result<String, String> {
        let output = Command::new("powershell")
            .args([
                "-ExecutionPolicy", "Bypass",
                "-NoProfile",
                "-File", ps_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("Failed to execute PowerShell script: {}", e))?;

        // Combine stdout and stderr
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let combined = format!("{}{}", stdout, stderr);

        // Log the output for debugging
        println!("PowerShell output: {}", combined);

        // Check for common error patterns
        if combined.contains("No such host is known") {
            return Err("Could not resolve hostname. Please check your SFTP host.".to_string());
        }
        if combined.contains("Permission denied") {
            return Err("Permission denied. Please check your username and password.".to_string());
        }
        if combined.contains("Connection refused") {
            return Err("Connection refused. Please check your host and port.".to_string());
        }
        if combined.contains("FATAL ERROR") {
            return Err(format!("SFTP command failed: {}", combined.trim()));
        }

        // If we have any output that looks like a successful connection, consider it success
        if combined.contains("Remote working directory is") || 
           combined.contains("-rw-") || 
           combined.contains("-r--") || 
           combined.contains("drwx") {
            Ok(combined)
        } else if !combined.trim().is_empty() {
            Err(format!("Unexpected SFTP output: {}", combined.trim()))
        } else {
            Err("SFTP command failed with no output".to_string())
        }
    }

    fn cleanup_temp_files(&self, files: &[&Path]) {
        for file in files {
            let _ = fs::remove_file(file);
        }
    }
}

#[tauri::command]
pub async fn run_sftp_command(config: SftpConfig, command: String) -> Result<String, String> {
    task::spawn_blocking(move || {
        // Create temporary files with the provided command
        let (batch_path, ps_path) = config.create_temp_files(&[
            &command,
            "exit"
        ])?;

        // Run the command and get output
        let result = config.run_sftp_command(&batch_path, &ps_path);

        // Clean up temporary files
        config.cleanup_temp_files(&[&batch_path, &ps_path]);

        result
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn test_sftp_connection(config: SftpConfig) -> Result<bool, String> {
    task::spawn_blocking(move || {
        // Create temporary files
        let (batch_path, ps_path) = config.create_temp_files(&["pwd", "exit"])?;

        // Run the command and check result
        let result = config.run_sftp_command(&batch_path, &ps_path);

        // Clean up temporary files
        config.cleanup_temp_files(&[&batch_path, &ps_path]);

        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn upload_file_sftp(config: SftpConfig, local_path: String, remote_path: String) -> Result<bool, String> {
    task::spawn_blocking(move || {
        // Create temporary files with upload and verify commands
        let (batch_path, ps_path) = config.create_temp_files(&[
            &format!("put \"{}\" \"{}\"", local_path, remote_path),
            "ls",  // List files to verify upload
            "exit"
        ])?;

        // Run the command and check result
        let result = config.run_sftp_command(&batch_path, &ps_path);

        // Clean up temporary files
        config.cleanup_temp_files(&[&batch_path, &ps_path]);

        match result {
            Ok(output) => {
                // Get just the filename from the remote path
                let filename = std::path::Path::new(&remote_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&remote_path);

                // Extract filenames from directory listing
                let filenames: Vec<String> = output.lines()
                    .filter(|line| line.contains("-rw-") || line.contains("-r--") || line.contains("drwx") || line.contains("-rwx"))
                    .filter_map(|line| {
                        // Split on whitespace and get the last part which is the filename
                        line.split_whitespace().last().map(|s| s.to_string())
                    })
                    .collect();

                println!("Found files in directory: {:?}", filenames);
                println!("Looking for file: {}", filename);

                if filenames.iter().any(|name| name == filename) {
                    Ok(true)
                } else {
                    Err("File upload reported success but file not found in directory listing".to_string())
                }
            },
            Err(e) => Err(e),
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn download_file_sftp(config: SftpConfig, remote_path: String, local_path: String) -> Result<bool, String> {
    task::spawn_blocking(move || {
        // Create temporary files
        let (batch_path, ps_path) = config.create_temp_files(&[
            &format!("get \"{}\" \"{}\"", remote_path, local_path),
            "exit"
        ])?;

        // Run the command and check result
        let result = config.run_sftp_command(&batch_path, &ps_path);

        // Clean up temporary files
        config.cleanup_temp_files(&[&batch_path, &ps_path]);

        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn list_remote_files(config: SftpConfig, path: String) -> Result<Vec<String>, String> {
    task::spawn_blocking(move || {
        // Create temporary files with multiple commands
        let (batch_path, ps_path) = config.create_temp_files(&[
            "pwd",  // Show current directory
            "ls",   // List files
            "exit"
        ])?;

        // Run the command and parse output
        let result = config.run_sftp_command(&batch_path, &ps_path);

        // Clean up temporary files
        config.cleanup_temp_files(&[&batch_path, &ps_path]);

        match result {
            Ok(output) => {
                // Parse ls output into filenames
                let files: Vec<String> = output.lines()
                    .filter(|line| {
                        let line = line.trim();
                        // Keep only lines that look like file listings
                        line.contains("-rw-") || line.contains("-r--") || line.contains("drwx") || line.contains("-rwx")
                    })
                    .filter_map(|line| {
                        // Split on whitespace and get the last part which is the filename
                        line.split_whitespace().last().map(|s| s.to_string())
                    })
                    .collect();

                Ok(files)
            },
            Err(e) => Err(e),
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
} 