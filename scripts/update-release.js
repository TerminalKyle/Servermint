#!/usr/bin/env node

/**
 * Update Release Script for ServerMint
 * 
 * This script helps automate the process of creating a new release and updating
 * the updates.json file for the Tauri updater.
 * 
 * Usage:
 *   node scripts/update-release.js <version> <notes>
 * 
 * Example:
 *   node scripts/update-release.js 0.1.1 "Bug fixes and performance improvements"
 */

const fs = require('fs');
const path = require('path');

// Get command line arguments
const args = process.argv.slice(2);
const version = args[0];
const notes = args[1] || 'Bug fixes and improvements';

if (!version) {
  console.error('Usage: node scripts/update-release.js <version> [notes]');
  console.error('Example: node scripts/update-release.js 0.1.1 "Bug fixes and performance improvements"');
  process.exit(1);
}

// Validate version format (simple check)
if (!/^\d+\.\d+\.\d+$/.test(version)) {
  console.error('Error: Version must be in format X.Y.Z (e.g., 0.1.1)');
  process.exit(1);
}

// Update Cargo.toml
function updateCargoToml() {
  const cargoPath = path.join(__dirname, '..', 'src-tauri', 'Cargo.toml');
  const cargoContent = fs.readFileSync(cargoPath, 'utf8');
  
  // Update version in Cargo.toml
  const updatedCargo = cargoContent.replace(
    /^version = "[\d.]+"/m,
    `version = "${version}"`
  );
  
  fs.writeFileSync(cargoPath, updatedCargo);
  console.log(`✅ Updated Cargo.toml version to ${version}`);
}

// Update tauri.conf.json
function updateTauriConfig() {
  const configPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');
  const configContent = fs.readFileSync(configPath, 'utf8');
  const config = JSON.parse(configContent);
  
  config.version = version;
  
  fs.writeFileSync(configPath, JSON.stringify(config, null, 2));
  console.log(`✅ Updated tauri.conf.json version to ${version}`);
}

// Update package.json
function updatePackageJson() {
  const packagePath = path.join(__dirname, '..', 'package.json');
  const packageContent = fs.readFileSync(packagePath, 'utf8');
  const packageJson = JSON.parse(packageContent);
  
  packageJson.version = version;
  
  fs.writeFileSync(packagePath, JSON.stringify(packageJson, null, 2));
  console.log(`✅ Updated package.json version to ${version}`);
}

// Update updates.json
function updateUpdatesJson() {
  const updatesPath = path.join(__dirname, '..', 'updates.json');
  const currentDate = new Date().toISOString();
  
  const updatesData = {
    version: version,
    notes: notes,
    pub_date: currentDate,
    platforms: {
      "darwin-x86_64": {
        "signature": "YOUR_SIGNATURE_HERE",
        "url": `https://releases.servermint.gg/ServerMint_${version}_x64.dmg`
      },
      "darwin-aarch64": {
        "signature": "YOUR_SIGNATURE_HERE",
        "url": `https://releases.servermint.gg/ServerMint_${version}_aarch64.dmg`
      },
      "linux-x86_64": {
        "signature": "YOUR_SIGNATURE_HERE",
        "url": `https://releases.servermint.gg/ServerMint_${version}_amd64.AppImage`
      },
      "windows-x86_64": {
        "signature": "YOUR_SIGNATURE_HERE",
        "url": `https://releases.servermint.gg/ServerMint_${version}_x64-setup.exe`
      }
    }
  };
  
  fs.writeFileSync(updatesPath, JSON.stringify(updatesData, null, 2));
  console.log(`✅ Updated updates.json for version ${version}`);
}

// Main execution
console.log(`🚀 Preparing release for version ${version}`);
console.log(`📝 Release notes: ${notes}`);
console.log('');

try {
  updateCargoToml();
  updateTauriConfig();
  updatePackageJson();
  updateUpdatesJson();
  
  console.log('');
  console.log('🎉 Version update completed successfully!');
  console.log('');
  console.log('Next steps:');
  console.log('1. Build the application: npm run tauri:build');
  console.log('2. Sign the release files: tauri signer sign ~/.tauri/servermint.key <file>');
  console.log('3. Generate signatures for updates.json');
  console.log('4. Upload release files to your server');
  console.log('5. Upload the updated updates.json to your server');
  console.log('');
  console.log('⚠️  Remember to:');
  console.log('   - Replace "YOUR_SIGNATURE_HERE" in updates.json with actual signatures');
  console.log('   - Update the URLs in updates.json to match your actual release URLs');
  console.log('   - Test the update process before releasing to users');
  
} catch (error) {
  console.error('❌ Error updating version:', error.message);
  process.exit(1);
} 