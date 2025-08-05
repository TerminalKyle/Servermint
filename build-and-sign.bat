@echo off
echo Building and signing ServerMint for updates...

REM Set the private key environment variable
set TAURI_SIGNING_PRIVATE_KEY=dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5YzloZjVQSGhyZkNoY2ZZRldhRm56K0gyc1BMSjFLYWhCQmY2WjdkWk9Dd0FBQkFBQUFBQUFBQUFBQUlBQUFBQU00dEVtbDR4RWZkTytLU3UzYkZzR0lFOVYrSTh5MjVEOGFLRDhadHpEeS94MzdRcnFYRnNZb1Y5ME1KalF3bzJUeFdKNXpNOUsweHB2SWJqVGQzVmZvTVNpTjhuMzJySjFFOEdDSG1IZ2lsT2hHbHh6dzd2SWRWMnlyYUR1cVZVS0ErMUdZd1Uyd3c9Cg==

echo Cleaning previous builds...
cd src-tauri
cargo clean
cd ..

echo Building for Windows...
npx tauri build

echo.
echo Checking signature files...
if exist "src-tauri\target\release\bundle\nsis\servermint_*.exe.sig" (
    echo ✅ Signature files found
    for %%f in (src-tauri\target\release\bundle\nsis\servermint_*.exe.sig) do (
        echo Found signature: %%~nf
    )
) else (
    echo ❌ No signature files found
    echo This might indicate a signing issue
)

echo.
echo Build complete! Check the src-tauri/target/release/bundle directory for your signed files.
echo.
echo Next steps:
echo 1. Run: node scripts/update-release.js <version> "<notes>"
echo 2. Upload the files to your server at https://releases.servermint.app/
echo 3. Upload the updated updates.json to your server
echo.
pause 