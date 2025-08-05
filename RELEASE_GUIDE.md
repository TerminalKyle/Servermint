# Release Guide

## For Future Releases

When you want to release a new version, just follow this process:

1. **Update version**: `node scripts/update-release.js 0.1.2 "New features"`
2. **Build and sign**: `.\build-and-sign.bat`
3. **Get the signature** from the generated `.sig` file
4. **Update `updates.json`** with the new signature
5. **Upload both files** to your server

## Files to Upload

- `updates.json` → `https://releases.servermint.app/updates.json`
- `servermint_0.1.2_x64-setup.exe` → `https://releases.servermint.app/ServerMint_0.1.2_x64-setup.exe`

The updater will automatically detect new versions and prompt users to update! 

$env:TAURI_SIGNING_PRIVATE_KEY="dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5YzloZjVQSGhyZkNoY2ZZRldhRm56K0gyc1BMSjFLYWhCQmY2WjdkWk9Dd0FBQkFBQUFBQUFBQUFBQUlBQUFBQU00dEVtbDR4RWZkTytLU3UzYkZzR0lFOVYrSTh5MjVEOGFLRDhadHpEeS94MzdRcnFYRnNZb1Y5ME1KalF3bzJUeFdKNXpNOUsweHB2SWJqVGQzVmZvTVNpTjhuMzJySjFFOEdDSG1IZ2lsT2hHbHh6dzd2SWRWMnlyYUR1cVZVS0ErMUdZd1Uyd3c9Cg=="; node scripts/fix-signature.js