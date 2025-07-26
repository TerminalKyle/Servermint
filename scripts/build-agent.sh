#!/bin/bash

# Install Rust if not installed
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install build dependencies
echo "Installing build dependencies..."
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Build agent
echo "Building agent..."
cd /tmp/servermint-agent
cargo build --release

# Copy binary to install location
echo "Installing binary..."
sudo mkdir -p /opt/servermint/agent
sudo cp target/release/servermint-agent /opt/servermint/agent/
sudo chmod +x /opt/servermint/agent/servermint-agent

echo "Build complete! Binary is at /opt/servermint/agent/servermint-agent" 