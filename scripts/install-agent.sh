#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Print Servermint logo
echo -e "${GREEN}"
echo "  _____                           __  __ _       _   "
echo " / ____|                         |  \/  (_)     | |  "
echo "| (___   ___ _ ____   _____ _ __| \  / |_ _ __ | |_ "
echo " \___ \ / _ \ '__\ \ / / _ \ '__| |\/| | | '_ \| __|"
echo " ____) |  __/ |   \ V /  __/ |  | |  | | | | | | |_ "
echo "|_____/ \___|_|    \_/ \___|_|  |_|  |_|_|_| |_|\__|"
echo -e "${NC}"
echo -e "${BLUE}VPS Agent Installer${NC}"
echo ""

# Check if running as root
if [ "$(id -u)" -ne 0 ]; then
    echo -e "${RED}Error: This script must be run as root${NC}"
    echo "Please run with sudo or as root user"
    exit 1
fi

# Check system requirements
echo -e "${BLUE}Checking system requirements...${NC}"

# Check for required commands
for cmd in curl wget systemctl java; do
    if ! command -v $cmd &> /dev/null; then
        echo -e "${YELLOW}$cmd is not installed. Installing...${NC}"
        apt-get update
        apt-get install -y $cmd
    fi
done

# Check Java version
JAVA_VERSION=$(java -version 2>&1 | head -n 1 | cut -d'"' -f2 | cut -d'.' -f1)
if [ "$JAVA_VERSION" -lt 17 ]; then
    echo -e "${YELLOW}Java 17 or higher is required. Installing OpenJDK 17...${NC}"
    apt-get update
    apt-get install -y openjdk-17-jre-headless
fi

echo -e "${GREEN}✓ System requirements satisfied${NC}"

# Create directories
echo -e "${BLUE}Creating directories...${NC}"
mkdir -p /opt/servermint
mkdir -p /var/lib/servermint/servers
mkdir -p /etc/servermint
echo -e "${GREEN}✓ Directories created${NC}"

# Download and install Rust agent
echo -e "${BLUE}Setting up Servermint agent...${NC}"

# Download latest agent binary
echo -e "${BLUE}Downloading agent binary...${NC}"
AGENT_URL="https://releases.servermint.app/latest/servermint-agent-linux-x64"
curl -sSL -o /opt/servermint/servermint-agent "$AGENT_URL"
chmod +x /opt/servermint/servermint-agent

# Create a run script
cat > /opt/servermint/run-agent.sh << 'EOF'
#!/bin/bash
cd /opt/servermint

# Create log file
touch agent.log
chmod 644 agent.log

echo "$(date): Starting Servermint agent..." >> agent.log

# Check for network connectivity
echo "$(date): Checking network connectivity..." >> agent.log
ping -c 1 google.com > /dev/null 2>&1
if [ $? -ne 0 ]; then
  echo "$(date): No network connectivity, waiting..." >> agent.log
  sleep 30
  # Try one more time before giving up
  ping -c 1 google.com > /dev/null 2>&1
  if [ $? -ne 0 ]; then
    echo "$(date): Still no network connectivity, continuing anyway..." >> agent.log
  fi
fi

# Run the agent
echo "$(date): Starting agent..." >> agent.log
exec ./servermint-agent >> agent.log 2>&1
EOF

chmod +x /opt/servermint/run-agent.sh

echo -e "${GREEN}✓ Agent setup complete${NC}"

# Prompt for pairing token
if [ -z "$TOKEN" ]; then
    echo -e "${BLUE}Please enter the pairing token from the Servermint desktop app:${NC}"
    read -p "> " TOKEN

    if [ -z "$TOKEN" ]; then
        echo -e "${RED}Error: Token cannot be empty${NC}"
        exit 1
    fi
else
    echo -e "${BLUE}Using provided token: $TOKEN${NC}"
fi

# Create config file with central WebSocket server
echo -e "${BLUE}Creating configuration...${NC}"
cat > /etc/servermint/config.toml << EOF
[agent]
name = "$(hostname)"
token = "$TOKEN"
ws_url = "wss://relay.servermint.app/ws"

[servers]
directory = "/var/lib/servermint/servers"
EOF

echo -e "${GREEN}✓ Configuration created${NC}"

# Create systemd service
echo -e "${BLUE}Creating systemd service...${NC}"
cat > /etc/systemd/system/servermint-agent.service << EOF
[Unit]
Description=Servermint Agent
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/servermint
ExecStart=/opt/servermint/run-agent.sh
Restart=always
RestartSec=10
Environment="SERVERMINT_CONFIG=/etc/servermint/config.toml"
Environment="SERVERMINT_SERVERS_DIR=/var/lib/servermint/servers"
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
EOF

echo -e "${GREEN}✓ Service created${NC}"

# Enable and start service
echo -e "${BLUE}Enabling and starting service...${NC}"
systemctl daemon-reload
systemctl enable servermint-agent

# Ensure proper permissions
echo -e "${BLUE}Setting permissions...${NC}"
chmod -R 755 /opt/servermint
chown -R root:root /opt/servermint

# Start the service
echo -e "${BLUE}Starting service...${NC}"
systemctl restart servermint-agent

# Check if service started successfully
sleep 2
if systemctl is-active --quiet servermint-agent; then
    echo -e "${GREEN}✓ Servermint agent started successfully${NC}"
else
    echo -e "${RED}Error: Failed to start Servermint agent${NC}"
    echo "Checking for errors in the agent logs:"
    journalctl -u servermint-agent -n 20 --no-pager
    
    echo -e "${YELLOW}Attempting to start the agent manually for debugging...${NC}"
    cd /opt/servermint && ./servermint-agent
fi

# Create firewall rules
echo -e "${BLUE}Configuring firewall...${NC}"
if command -v ufw &> /dev/null; then
    ufw allow 25565/tcp comment "Minecraft Server"
    echo -e "${GREEN}✓ Firewall configured (ufw)${NC}"
elif command -v firewall-cmd &> /dev/null; then
    firewall-cmd --permanent --add-port=25565/tcp
    firewall-cmd --reload
    echo -e "${GREEN}✓ Firewall configured (firewalld)${NC}"
else
    echo -e "${YELLOW}Warning: Could not configure firewall automatically${NC}"
    echo "Please manually open port 25565 for Minecraft servers"
fi

# Print success message
echo ""
echo -e "${GREEN}Installation completed successfully!${NC}"
echo -e "${BLUE}The Servermint agent is now running and connected to your desktop app.${NC}"
echo ""
echo -e "Server directories: ${YELLOW}/var/lib/servermint/servers${NC}"
echo -e "Configuration: ${YELLOW}/etc/servermint/config.toml${NC}"
echo -e "Service logs: ${YELLOW}journalctl -u servermint-agent${NC}"
echo ""
echo -e "${BLUE}You can now create and manage servers on this VPS from the Servermint desktop app.${NC}"
echo ""
echo -e "${YELLOW}The agent will automatically connect to the Servermint relay server.${NC}"
echo -e "${YELLOW}No additional firewall or port forwarding configuration is needed.${NC}" 