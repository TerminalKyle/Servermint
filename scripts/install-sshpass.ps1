# Check if sshpass is already installed
$sshpassPath = Join-Path $env:TEMP "sshpass.exe"

if (-not (Test-Path $sshpassPath)) {
    Write-Host "Downloading sshpass..."
    $url = "https://github.com/darkguy2008/sshpass-windows/releases/download/1.09/sshpass.exe"
    Invoke-WebRequest -Uri $url -OutFile $sshpassPath
}

# Add sshpass to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if (-not $userPath.Contains($env:TEMP)) {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$env:TEMP", "User")
}

Write-Host "sshpass installed successfully!" 