#!/bin/ash
# Fabric MC Installation Script
#
# Server Files: /mnt/server
mkdir -p /mnt/server
cd /mnt/server

if [ -z "$MINECRAFT_VERSION" ]; then
  MINECRAFT_VERSION="1.21.2"
fi

if [ -z "$FABRIC_LOADER_VERSION" ] || [ "$FABRIC_LOADER_VERSION" == "latest" ]; then
  FABRIC_LOADER_VERSION=$(curl -sSL https://meta.fabricmc.net/v2/versions/loader | jq -r '.[0].version')
fi

echo -e "Installing Fabric for Minecraft version: $MINECRAFT_VERSION"
echo -e "Fabric Loader version: $FABRIC_LOADER_VERSION"

# Get the installer URL
INSTALLER_URL="https://meta.fabricmc.net/v2/versions/loader/${MINECRAFT_VERSION}/${FABRIC_LOADER_VERSION}/server/json"

echo -e "Downloading Fabric installer from: $INSTALLER_URL"

# Download the installer
curl -o fabric-installer.jar $INSTALLER_URL

# Run the installer
echo -e "Running Fabric installer..."
java -jar fabric-installer.jar server -downloadMinecraft

# Clean up installer
rm fabric-installer.jar

echo -e "Install Complete" 