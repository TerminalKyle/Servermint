# 🚀 Tauri Updater Setup Guide

## 📋 Prerequisites

1. **Your signing keys are already generated** ✅
2. **Public key is configured in tauri.conf.json** ✅
3. **Private key is ready for signing** ✅

## 🔧 Setup Steps

### 1. Set Environment Variables

**Option A: Use the provided script**
```bash
# Run the setup script
setup-updater.bat
```

**Option B: Set manually**
```bash
# Set the private key environment variable
set TAURI_SIGNING_PRIVATE_KEY=dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5YzloZjVQSGhyZkNoY2ZZRldhRm56K0gyc1BMSjFLYWhCQmY2WjdkWk9Dd0FBQkFBQUFBQUFBQUFBQUlBQUFBQU00dEVtbDR4RWZkTytLU3UzYkZzR0lFOVYrSTh5MjVEOGFLRDhadHpEeS94MzdRcnFYRnNZb1Y5ME1KalF3bzJUeFdKNXpNOUsweHB2SWJqVGQzVmZvTVNpTjhuMzJySjFFOEdDSG1IZ2lsT2hHbHh6dzd2SWRWMnlyYUR1cVZVS0ErMUdZd1Uyd3c9Cg==
```

### 2. Build and Sign Your Application

```bash
# Run the build and sign script
build-and-sign.bat
```

This will:
- Build for Windows (your current platform)
- Sign the Windows installer with your private key
- Create installers in `src-tauri/target/release/bundle/`

**Note:** For cross-platform builds (macOS, Linux), you'll need to set up cross-compilation. See the Cross-Platform Build section below.

### 3. Generate Signatures

```bash
# Run the signature generation script
generate-signatures.bat
```

This will output signatures like:
```
Windows signature:
dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHNpZ25hdHVyZQpSV1IxOU5nZU5HNk5nTTZPNjJlanptN1JBNFRoNVJsaEhuWDJxTSt5Q2V3eHJ6MVA5QVZVNVVaTAo=
```

**Note:** This will only generate signatures for Windows builds. For other platforms, see the Cross-Platform Build section.

### 4. Update Your updates.json

Replace the placeholder signatures with the real ones:

```json
{
  "version": "0.1.1",
  "notes": "Bug fixes and performance improvements",
  "pub_date": "2024-01-15T00:00:00Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "REAL_SIGNATURE_FROM_STEP_3",
      "url": "https://releases.servermint.gg/ServerMint_0.1.1_x64.dmg"
    },
    "darwin-aarch64": {
      "signature": "REAL_SIGNATURE_FROM_STEP_3",
      "url": "https://releases.servermint.gg/ServerMint_0.1.1_aarch64.dmg"
    },
    "linux-x86_64": {
      "signature": "REAL_SIGNATURE_FROM_STEP_3",
      "url": "https://releases.servermint.gg/ServerMint_0.1.1_amd64.AppImage"
    },
    "windows-x86_64": {
      "signature": "REAL_SIGNATURE_FROM_STEP_3",
      "url": "https://releases.servermint.gg/ServerMint_0.1.1_x64-setup.exe"
    }
  }
}
```

### 5. Upload Files to Your Server

Upload these files to `https://releases.servermint.gg/`:

**Windows:**
- `src-tauri/target/release/bundle/msi/ServerMint_0.1.1_x64-setup.msi` → `ServerMint_0.1.1_x64-setup.exe`
- OR `src-tauri/target/release/bundle/nsis/ServerMint_0.1.1_x64-setup.exe` → `ServerMint_0.1.1_x64-setup.exe`

**Update file:**
- `updates.json` → `updates.json`

**Note:** For macOS and Linux builds, see the Cross-Platform Build section below.

### 6. Test the Updater

1. **Build a test version** with a lower version number
2. **Run the app** and check for updates
3. **Verify** the update process works

## 🔄 Release Process

For each new release:

1. **Update version** in `src-tauri/tauri.conf.json`
2. **Update version** in `updates.json`
3. **Update pub_date** in `updates.json`
4. **Run** `build-and-sign.bat`
5. **Run** `generate-signatures.bat`
6. **Update** `updates.json` with new signatures
7. **Upload** all files to your server

## 🌐 Cross-Platform Builds

### macOS Builds

To build for macOS from Windows, you'll need:

1. **Install Rust targets:**
   ```bash
   rustup target add x86_64-apple-darwin
   rustup target add aarch64-apple-darwin
   ```

2. **Install cross-compilation tools:**
   ```bash
   # Install osxcross (requires WSL or Linux)
   # Or use GitHub Actions for automated builds
   ```

3. **Build for macOS:**
   ```bash
   npx tauri build --target x86_64-apple-darwin
   npx tauri build --target aarch64-apple-darwin
   ```

### Linux Builds

To build for Linux from Windows, you'll need:

1. **Install Rust targets:**
   ```bash
   rustup target add x86_64-unknown-linux-gnu
   ```

2. **Use WSL or Docker:**
   ```bash
   # In WSL or Linux environment
   npx tauri build --target x86_64-unknown-linux-gnu
   ```

### Alternative: GitHub Actions

For automated cross-platform builds, consider using GitHub Actions:

1. **Create `.github/workflows/build.yml`**
2. **Configure builds for all platforms**
3. **Automatically generate signatures**
4. **Upload to your release server**

This approach is recommended for production releases.

## 🛡️ Security Notes

- **Keep your private key secret** - never commit it to version control
- **Backup your private key** - if you lose it, you can't sign updates
- **Use HTTPS** for your update server
- **Verify signatures** before distributing updates

## 🐛 Troubleshooting

**"Signature verification failed"**
- Check that your public key in `tauri.conf.json` matches your private key
- Verify the signature in `updates.json` is correct
- Ensure the file URL is accessible

**"Update not found"**
- Check that `updates.json` is accessible at the configured URL
- Verify the version number is higher than the current version
- Ensure the platform-specific signature is correct

**"Build failed"**
- Make sure the `TAURI_SIGNING_PRIVATE_KEY` environment variable is set
- Check that all dependencies are installed
- Verify your Rust toolchain is up to date 