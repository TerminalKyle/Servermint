#!/bin/ash
# Vanilla MC Installation Script
#
# Server Files: /mnt/server
mkdir -p /mnt/server
cd /mnt/server

LATEST_VERSION=`curl https://launchermeta.mojang.com/mc/game/version_manifest.json | jq -r '.latest.release'`
LATEST_SNAPSHOT_VERSION=`curl https://launchermeta.mojang.com/mc/game/version_manifest.json | jq -r '.latest.snapshot'`

echo -e "latest version is $LATEST_VERSION"
echo -e "latest snapshot is $LATEST_SNAPSHOT_VERSION"

if [ -z "$VANILLA_VERSION" ] || [ "$VANILLA_VERSION" == "latest" ]; then
  MANIFEST_URL=$(curl -sSL https://launchermeta.mojang.com/mc/game/version_manifest.json | jq --arg VERSION $LATEST_VERSION -r '.versions | .[] | select(.id== $VERSION )|.url')
elif [ "$VANILLA_VERSION" == "snapshot" ]; then
  MANIFEST_URL=$(curl -sSL https://launchermeta.mojang.com/mc/game/version_manifest.json | jq --arg VERSION $LATEST_SNAPSHOT_VERSION -r '.versions | .[] | select(.id== $VERSION )|.url')
else
  MANIFEST_URL=$(curl -sSL https://launchermeta.mojang.com/mc/game/version_manifest.json | jq --arg VERSION $VANILLA_VERSION -r '.versions | .[] | select(.id== $VERSION )|.url')
fi

DOWNLOAD_URL=$(curl ${MANIFEST_URL} | jq .downloads.server | jq -r '. | .url')

echo -e "running: curl -o ${SERVER_JARFILE} $DOWNLOAD_URL"
curl -o ${SERVER_JARFILE} $DOWNLOAD_URL

echo -e "Install Complete" 