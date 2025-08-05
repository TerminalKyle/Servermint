#!/bin/ash
# Paper MC Installation Script
#
# Server Files: /mnt/server
mkdir -p /mnt/server
cd /mnt/server

LATEST_VERSION=`curl https://launchermeta.mojang.com/mc/game/version_manifest.json | jq -r '.latest.release'`
LATEST_SNAPSHOT_VERSION=`curl https://launchermeta.mojang.com/mc/game/version_manifest.json | jq -r '.latest.snapshot'`

echo -e "latest version is $LATEST_VERSION"
echo -e "latest snapshot is $LATEST_SNAPSHOT_VERSION"

if [ -z "$PAPER_VERSION" ] || [ "$PAPER_VERSION" == "latest" ]; then
  PAPER_VERSION=$LATEST_VERSION
fi

echo -e "Installing Paper version: $PAPER_VERSION"

# Get the latest build number for the specified version
BUILD_URL="https://papermc.io/api/v2/projects/paper/versions/${PAPER_VERSION}"
BUILD_NUMBER=$(curl -sSL $BUILD_URL | jq '.builds[-1]')

echo -e "Latest build number: $BUILD_NUMBER"

# Download the Paper jar
DOWNLOAD_URL="https://papermc.io/api/v2/projects/paper/versions/${PAPER_VERSION}/builds/${BUILD_NUMBER}/downloads/paper-${PAPER_VERSION}-${BUILD_NUMBER}.jar"

echo -e "Downloading Paper from: $DOWNLOAD_URL"
curl -o ${SERVER_JARFILE} $DOWNLOAD_URL

echo -e "Install Complete" 