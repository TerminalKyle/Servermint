# ServerMint

A modern Minecraft server management application built with Tauri and Vue.js.

## Features

- **Server Management**: Create, start, stop, and manage Minecraft servers
- **Multiple Server Types**: Support for Vanilla, Paper, Spigot, and more
- **File Management**: Edit server files directly in the app
- **Automatic Updates**: Built-in updater for seamless app updates
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Modern UI**: Beautiful, responsive interface with dark mode

## Automatic Updates

ServerMint includes a comprehensive automatic update system that:

- **Checks for updates** automatically on app startup and every 30 minutes
- **Downloads updates** in the background when available
- **Installs updates** automatically with user confirmation
- **Notifies users** with update details and progress
- **Restarts the app** seamlessly after installation

### Update Settings

Users can control update behavior through the Settings panel:

- **Check for updates automatically**: Enable/disable automatic update checking
- **Automatically download updates**: Auto-download when updates are available
- **Automatically install updates**: Auto-install downloaded updates

### For Developers

To set up automatic updates for your own releases:

1. **Generate code signing keys**:
   ```bash
   tauri signer generate -w ~/.tauri/servermint.key
   tauri signer generate -w ~/.tauri/servermint.key -p
   ```

2. **Update configuration** in `src-tauri/tauri.conf.json`:
   ```json
   {
     "updater": {
       "active": true,
       "endpoints": ["https://your-domain.com/updates.json"],
       "dialog": true,
       "pubkey": "YOUR_PUBLIC_KEY_HERE"
     }
   }
   ```

3. **Use the release script**:
   ```bash
   node scripts/update-release.js 0.1.1 "Bug fixes and improvements"
   ```

4. **Build and sign** your release:
   ```bash
   npm run tauri:build
   tauri signer sign ~/.tauri/servermint.key path/to/release/file
   ```

5. **Upload** to your server and update `updates.json`

See `UPDATER_SETUP.md` for detailed setup instructions.

## Development

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Tauri CLI v2

### Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd manticore
   ```

2. **Install dependencies**:
   ```bash
npm install
```

3. **Start development server**:
   ```bash
   npm run tauri:dev
```

### Building

**Development build**:
```bash
npm run tauri:dev
```

**Production build**:
```bash
npm run tauri:build
```

### Project Structure

```
manticore/
├── src/                    # Vue.js frontend
│   ├── components/         # Vue components
│   ├── App.vue            # Main app component
│   └── store.js           # State management
├── src-tauri/             # Rust backend
│   ├── src/               # Rust source code
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── scripts/               # Build and release scripts
├── updates.json           # Update manifest
└── UPDATER_SETUP.md       # Detailed updater setup guide
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

[Add your license here]

## Support

For issues and questions:
- [GitHub Issues](https://github.com/your-repo/issues)
- [Documentation](https://your-docs-url.com)
