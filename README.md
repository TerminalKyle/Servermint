# ServerMint

A modern, open-source Minecraft server management application built with love, passion, and care using Tauri and Vue.js. ServerMint transforms your game development workflow by providing powerful tools that are easy to use, helping you create and manage game servers for development and multiplayer testing without the technical overwhelm.

![ServerMint Screenshot](public/servermint.png)

## What is ServerMint?

ServerMint is your comprehensive toolkit for game server management. Whether you're testing mods locally or scaling to production, ServerMint handles the complexity so you can focus on what matters most. Built with modern technologies and designed for developers who demand both power and simplicity.

### Key Features

- **One-Click Server Setup**: Create local game servers with just a few clicks for any Minecraft version
- **Multiple Server Types**: Full support for Vanilla, Paper, Spigot, Fabric, Forge, Bukkit, and Mohist
- **Built-in SFTP/SSH Access**: Securely access and modify your servers with advanced management tools
- **Integrated Mod & Plugin Installer**: Browse and install mods and plugins with our integrated marketplace
- **Export & Deploy**: Export your server configurations and upload them to production environments seamlessly
- **Node/VPS Integration**: Connect your nodes/VPS to spin up servers with ease
- **Developer Tools**: Advanced tools for debugging, performance monitoring, and server optimization
- **Cross-Platform**: Works seamlessly on Windows 10/11, macOS, and Linux
- **Modern UI**: Beautiful, responsive interface with dark mode and intuitive design

## Automatic Updates

ServerMint includes a comprehensive automatic update system that keeps your application current:

- **Smart Update Checking**: Automatically checks for updates on startup and every 30 minutes
- **Background Downloads**: Downloads updates seamlessly in the background
- **User-Friendly Installation**: Installs updates with user confirmation and progress tracking
- **Seamless Restarts**: Handles app restarts automatically after installation
- **Detailed Notifications**: Keeps you informed with update details and progress

### Update Settings

Control your update experience through the intuitive Settings panel:

- **Automatic Update Checking**: Enable/disable automatic update detection
- **Auto-Download**: Automatically download updates when available
- **Auto-Install**: Automatically install downloaded updates



## Development

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Tauri CLI v2

### Quick Start

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd servermint
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Start development server**:
   ```bash
   npx tauri dev
   ```

### Building

**Development build**:
```bash
npx tauri dev
```

**Production build**:
```bash
npm run tauri:build
```

### Project Structure

```
servermint/
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

We believe in the power of open source and community collaboration. ServerMint is built with love and maintained with care, and we welcome contributions from developers who share our passion for creating amazing tools.

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** with care and attention to detail
4. **Test thoroughly** to ensure everything works as expected
5. **Submit a pull request** with a clear description of your changes

### Code of Conduct

We maintain a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## License

[Add your license here]

## Support & Community

ServerMint is more than just software - it's a community of passionate developers and gamers. We're here to help you succeed.

### Get Help

- **Documentation**: [https://servermint.app/](https://servermint.app/)
- **Discord Community**: [discord.gg/servermint](https://discord.gg/servermint)
- **GitHub Issues**: [https://github.com/your-repo/issues](https://github.com/your-repo/issues)
- **Website**: [https://servermint.app/](https://servermint.app/)

### Stay Connected

- Join our Discord for real-time updates and support
- Follow us on GitHub for the latest releases
- Check out our website for tutorials and guides

---

**Made with love by the ServerMint Team**

ServerMint is actively maintained and regularly updated to ensure compatibility with the latest Windows 10/11 systems and Minecraft versions. We're committed to providing the best possible experience for game server management.
