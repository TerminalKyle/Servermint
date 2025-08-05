@echo off
echo Building and signing ServerMint for updates...

REM Set the private key environment variable
set TAURI_SIGNING_PRIVATE_KEY=dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5YzloZjVQSGhyZkNoY2ZZRldhRm56K0gyc1BMSjFLYWhCQmY2WjdkWk9Dd0FBQkFBQUFBQUFBQUFBQUlBQUFBQU00dEVtbDR4RWZkTytLU3UzYkZzR0lFOVYrSTh5MjVEOGFLRDhadHpEeS94MzdRcnFYRnNZb1Y5ME1KalF3bzJUeFdKNXpNOUsweHB2SWJqVGQzVmZvTVNpTjhuMzJySjFFOEdDSG1IZ2lsT2hHbHh6dzd2SWRWMnlyYUR1cVZVS0ErMUdZd1Uyd3c9Cg==

echo Building for Windows...
npx tauri build

echo.
echo Building for other platforms (if you have cross-compilation set up)...
echo Note: Cross-platform builds require additional setup
echo For now, the Windows build is complete and ready for signing.

echo.
echo Build complete! Check the src-tauri/target/release/bundle directory for your signed files.
echo.
echo Next steps:
echo 1. Upload the files to your server at https://releases.servermint.app/
echo 2. Generate signatures for each platform
echo 3. Update updates.json with the real signatures
echo.
pause 