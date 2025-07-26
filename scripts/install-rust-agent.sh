#!/bin/bash

# Check if TOKEN is provided
if [ -z "$TOKEN" ]; then
    echo "Error: TOKEN environment variable not set"
    echo "Usage: TOKEN=sm-xxx... ./install-rust-agent.sh"
    exit 1
fi

# Stop old Node.js agent
echo "Stopping old Node.js agent..."
sudo systemctl stop servermint-agent || true

# Create directories
echo "Creating directories..."
sudo mkdir -p /opt/servermint/agent
sudo mkdir -p /etc/servermint
sudo mkdir -p /var/lib/servermint/servers

# Create config file
echo "Creating config file..."
cat << EOF | sudo tee /etc/servermint/config.toml
[agent]
name = "VPS Node"
token = "$TOKEN"
ws_url = "wss://relay.servermint.app/ws"

[servers]
directory = "/var/lib/servermint/servers"
EOF

# Assuming servermint-agent binary is in current directory
echo "Installing agent binary..."
sudo cp servermint-agent /opt/servermint/agent/
sudo chmod +x /opt/servermint/agent/servermint-agent

# Create systemd service
echo "Creating systemd service..."
cat << EOF | sudo tee /etc/systemd/system/servermint-agent.service
[Unit]
Description=Servermint Agent
After=network.target

[Service]
Type=simple
Environment="SERVERMINT_CONFIG=/etc/servermint/config.toml"
Environment="RUST_LOG=info"
ExecStart=/opt/servermint/agent/servermint-agent
Restart=always
RestartSec=5
User=root

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd and start agent
echo "Starting agent..."
sudo systemctl daemon-reload
sudo systemctl enable servermint-agent
sudo systemctl start servermint-agent

echo "Installation complete! Agent should be running."
echo "Check status with: sudo systemctl status servermint-agent"
echo "View logs with: sudo journalctl -u servermint-agent -f" 