# Automatic Updater Setup Guide

This guide explains how to set up automatic updates for your ServerMint Tauri application.

## Overview

The automatic updater system includes:
- **Automatic update checking** on app startup and every 30 minutes
- **Automatic download** of updates when available
- **User notifications** with update details and progress
- **Automatic installation** with app restart

## Prerequisites

1. **Tauri CLI v2** installed
2. **Code signing certificate** for your platform
3. **Web server** to host your updates.json and release files

## Setup Steps

### 1. Generate Code Signing Keys

First, generate your code signing keys:

```bash
# Generate private key
tauri signer generate -w ~/.tauri/servermint.key

# Generate public key
tauri signer generate -w ~/.tauri/servermint.key -p
```

### 2. Update Configuration

#### Update `src-tauri/tauri.conf.json`:

Replace the placeholder values in the updater configuration:

```json
{
  "updater": {
    "active": true,
    "endpoints": [
      "https://your-domain.com/updates.json"
    ],
    "dialog": true,
    "pubkey": "YOUR_PUBLIC_KEY_HERE"
  }
}
```

#### Update `updates.json`:

Replace the placeholder values with your actual information:

```json
{
  "version": "0.1.1",
  "notes": "Bug fixes and performance improvements",
  "pub_date": "2024-01-15T00:00:00Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "YOUR_SIGNATURE_HERE",
      "url": "https://your-domain.com/ServerMint_0.1.1_x64.dmg"
    },
    "darwin-aarch64": {
      "signature": "YOUR_SIGNATURE_HERE",
      "url": "https://your-domain.com/ServerMint_0.1.1_aarch64.dmg"
    },
    "linux-x86_64": {
      "signature": "YOUR_SIGNATURE_HERE",
      "url": "https://your-domain.com/ServerMint_0.1.1_amd64.AppImage"
    },
    "windows-x86_64": {
      "signature": "YOUR_SIGNATURE_HERE",
      "url": "https://your-domain.com/ServerMint_0.1.1_x64-setup.exe"
    }
  }
}
```

### 3. Build and Sign Your Application

#### Build the application:

```bash
npm run tauri:build
```

#### Sign the release files:

```bash
# For each platform, sign the release file
tauri signer sign ~/.tauri/servermint.key path/to/your/release/file
```

### 4. Generate Update Signatures

For each platform's release file, generate the signature:

```bash
# Example for Windows
tauri signer sign ~/.tauri/servermint.key src-tauri/target/release/bundle/msi/ServerMint_0.1.1_x64-setup.msi

# Example for macOS
tauri signer sign ~/.tauri/servermint.key src-tauri/target/release/bundle/dmg/ServerMint_0.1.1_x64.dmg

# Example for Linux
tauri signer sign ~/.tauri/servermint.key src-tauri/target/release/bundle/appimage/ServerMint_0.1.1_amd64.AppImage
```

### 5. Host Your Updates

1. **Upload your release files** to your web server
2. **Upload the `updates.json`** file to your web server
3. **Ensure HTTPS** is enabled (required for security)

### 6. Update Your Version

When releasing a new version:

1. **Update the version** in `src-tauri/Cargo.toml`
2. **Update the version** in `src-tauri/tauri.conf.json`
3. **Build and sign** the new release
4. **Update `updates.json`** with the new version information
5. **Upload** the new release files and updated `updates.json`

## How It Works

### Automatic Update Checking

The app automatically checks for updates:
- **On startup** - When the app launches
- **Every 30 minutes** - While the app is running
- **Minimum 5-minute interval** - Prevents excessive checking

### Update Flow

1. **Check for updates** - App queries your `updates.json` endpoint
2. **Compare versions** - If a newer version is available, show notification
3. **Auto-download** - Automatically download the update (if enabled)
4. **User notification** - Show progress and installation options
5. **Auto-install** - Install and restart the app (if enabled)

### User Experience

- **Non-intrusive notifications** - Snackbar notifications for available updates
- **Progress tracking** - Visual progress bar during download
- **User control** - Users can choose to install later or dismiss
- **Automatic restart** - App restarts automatically after installation

## Customization

### Update Settings

You can customize the update behavior by modifying the `UpdateManager.vue` component:

```javascript
shouldAutoDownload() {
  // Check user settings for auto-download preference
  return this.store.settings.updates.autoDownload;
},

shouldAutoInstall() {
  // Check user settings for auto-install preference
  return this.store.settings.updates.autoInstall;
}
```

### Update Intervals

Modify the check intervals in `UpdateManager.vue`:

```javascript
// Check every 30 minutes (default)
this.checkInterval = setInterval(() => {
  this.checkForUpdates();
}, 30 * 60 * 1000);

// Minimum 5 minutes between checks
if (now - this.lastCheckTime < 5 * 60 * 1000) {
  return;
}
```

## Security Considerations

1. **HTTPS Required** - Update endpoints must use HTTPS
2. **Code Signing** - All releases must be properly signed
3. **Signature Verification** - Tauri verifies signatures before installation
4. **Public Key** - Include your public key in the configuration

## Troubleshooting

### Common Issues

1. **Updates not found** - Check your `updates.json` format and endpoint URL
2. **Signature verification failed** - Ensure your release files are properly signed
3. **Download fails** - Check your web server configuration and file URLs
4. **Installation fails** - Verify file permissions and antivirus settings

### Debug Mode

Enable debug logging to troubleshoot issues:

```rust
// In src-tauri/src/lib.rs
if cfg!(debug_assertions) {
  app.handle().plugin(
    tauri_plugin_log::Builder::default()
      .level(log::LevelFilter::Debug)  // Change to Debug
      .build(),
  )?;
}
```

## Testing

### Local Testing

1. **Build a test version** with a higher version number
2. **Host updates.json locally** or on a test server
3. **Test the update flow** with the lower version app

### Production Testing

1. **Deploy to a small group** first
2. **Monitor update success rates**
3. **Check user feedback** on the update process

## Best Practices

1. **Incremental versioning** - Use semantic versioning (e.g., 0.1.0, 0.1.1)
2. **Release notes** - Include meaningful update notes in `updates.json`
3. **Rollback plan** - Keep previous versions available
4. **User communication** - Inform users about major updates
5. **Testing** - Always test updates before wide release

## Support

For issues with the Tauri updater plugin, refer to:
- [Tauri Updater Documentation](https://v2.tauri.app/plugin/updater/)
- [Tauri Community Discord](https://discord.gg/tauri)
- [GitHub Issues](https://github.com/tauri-apps/plugins-workspace) 