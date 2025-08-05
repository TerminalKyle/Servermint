#!/bin/ash
# PocketMine-MP Installation Script
#
# Server Files: /mnt/server
mkdir -p /mnt/server
cd /mnt/server

if [ -z "$POCKETMINE_VERSION" ] || [ "$POCKETMINE_VERSION" == "latest" ]; then
  POCKETMINE_VERSION=$(curl -sSL https://api.github.com/repos/pmmp/PocketMine-MP/releases/latest | jq -r '.tag_name')
fi

echo -e "Installing PocketMine-MP version: $POCKETMINE_VERSION"

# Download PocketMine-MP
DOWNLOAD_URL="https://github.com/pmmp/PocketMine-MP/releases/download/${POCKETMINE_VERSION}/PocketMine-MP.phar"

echo -e "Downloading PocketMine-MP from: $DOWNLOAD_URL"
curl -o PocketMine-MP.phar $DOWNLOAD_URL

# Download PHP binary for Windows
echo -e "Downloading PHP binary..."
PHP_URL="https://github.com/pmmp/PHP-Binaries/releases/download/pm5-php-8.3-latest/PHP-8.3-Windows-x64-PM5.zip"
curl -o php.zip $PHP_URL

# Extract PHP binary
echo -e "Extracting PHP binary..."
unzip -q php.zip -d .
rm php.zip

echo -e "Install Complete" 