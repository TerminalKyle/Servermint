# Release Guide

## For Future Releases

When you want to release a new version, just follow this process:

1. **Update version**: `node scripts/update-release.js 0.1.2 "New features"`
2. **Build and sign**: `.\build-and-sign.bat`
3. **Fix signature**: Run these two commands:
   ```powershell
   $env:TAURI_SIGNING_PRIVATE_KEY="YOUR_PRIVATE_KEY"
   node scripts/fix-signature.js
   ```
4. **Upload files** to your server

## Files to Upload

- `updates.json` → `https://releases.servermint.app/updates.json`
- `servermint_0.1.2_x64-setup.exe` → `https://releases.servermint.app/ServerMint_0.1.2_x64-setup.exe`

The updater will automatically detect new versions and prompt users to update!

## Note
- Always run the signature fix commands separately in PowerShell
- Replace YOUR_PRIVATE_KEY with your actual Tauri signing key
- The fix-signature script will automatically update updates.json with the correct signature