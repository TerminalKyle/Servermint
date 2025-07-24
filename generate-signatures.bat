@echo off
echo Generating signatures for ServerMint update files...

REM Set the private key environment variable
set TAURI_SIGNING_PRIVATE_KEY=dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5YzloZjVQSGhyZkNoY2ZZRldhRm56K0gyc1BMSjFLYWhCQmY2WjdkWk9Dd0FBQkFBQUFBQUFBQUFBQUlBQUFBQU00dEVtbDR4RWZkTytLU3UzYkZzR0lFOVYrSTh5MjVEOGFLRDhadHpEeS94MzdRcnFYRnNZb1Y5ME1KalF3bzJUeFdKNXpNOUsweHB2SWJqVGQzVmZvTVNpTjhuMzJySjFFOEdDSG1IZ2lsT2hHbHh6dzd2SWRWMnlyYUR1cVZVS0ErMUdZd1Uyd3c9Cg==

echo.
echo Generating signatures for Windows build...

REM Windows signature
if exist "src-tauri\target\release\bundle\msi\ServerMint_0.1.1_x64-setup.msi" (
    echo Windows signature:
    npx tauri signer sign "src-tauri\target\release\bundle\msi\ServerMint_0.1.1_x64-setup.msi"
    echo.
) else (
    echo Windows MSI file not found. Checking for other Windows formats...
    if exist "src-tauri\target\release\bundle\nsis\ServerMint_0.1.1_x64-setup.exe" (
        echo Windows NSIS signature:
        npx tauri signer sign "src-tauri\target\release\bundle\nsis\ServerMint_0.1.1_x64-setup.exe"
        echo.
    ) else (
        echo No Windows installer found. Please build the application first.
    )
)

echo.
echo Note: For macOS and Linux builds, you'll need to set up cross-compilation.
echo See the UPDATER_SETUP.md guide for cross-platform build instructions.

echo.
echo Copy the signatures above and update your updates.json file.
echo.
pause 