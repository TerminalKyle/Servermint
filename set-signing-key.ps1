Write-Host "Setting TAURI_SIGNING_PRIVATE_KEY environment variable..." -ForegroundColor Green

# Set the private key environment variable
$env:TAURI_SIGNING_PRIVATE_KEY = "dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5YzloZjVQSGhyZkNoY2ZZRldhRm56K0gyc1BMSjFLYWhCQmY2WjdkWk9Dd0FBQkFBQUFBQUFBQUFBQUlBQUFBQU00dEVtbDR4RWZkTytLU3UzYkZzR0lFOVYrSTh5MjVEOGFLRDhadHpEeS94MzdRcnFYRnNZb1Y5ME1KalF3bzJUeFdKNXpNOUsweHB2SWJqVGQzVmZvTVNpTjhuMzJySjFFOEdDSG1IZ2lsT2hHbHh6dzd2SWRWMnlyYUR1cVZVS0ErMUdZd1Uyd3c9Cg=="

Write-Host "Environment variable set successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "You can now run your Tauri commands in this terminal session." -ForegroundColor Yellow
Write-Host ""
Write-Host "To verify the variable is set, run: `$env:TAURI_SIGNING_PRIVATE_KEY" -ForegroundColor Cyan
Write-Host ""
Read-Host "Press Enter to continue" 