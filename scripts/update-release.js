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
  
  // Try to read existing signature from the built file
  let signature = "YOUR_SIGNATURE_HERE";
  const signaturePath = path.join(__dirname, '..', 'src-tauri', 'target', 'release', 'bundle', 'nsis', `servermint_${version}_x64-setup.exe.sig`);
  
  if (fs.existsSync(signaturePath)) {
    try {
      signature = fs.readFileSync(signaturePath, 'utf8').trim();
      console.log(`✅ Found signature for version ${version}`);
    } catch (error) {
      console.warn(`⚠️  Could not read signature file: ${error.message}`);
    }
  } else {
    console.warn(`⚠️  Signature file not found: ${signaturePath}`);
    console.warn('   Make sure to build the application first with: npm run tauri:build');
  }
  
  const updatesData = {
    version: version,
    notes: notes,
    pub_date: currentDate,
    platforms: {
      "windows-x86_64": {
        "signature": signature,
        "url": `https://releases.servermint.app/servermint_${version}_x64-setup.exe`
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
  console.log('2. Run this script again to update signatures: node scripts/update-release.js ' + version);
  console.log('3. Upload release files to your server');
  console.log('4. Upload the updated updates.json to your server');
  console.log('');
  console.log('⚠️  Remember to:');
  console.log('   - Make sure the signature file exists before running this script');
  console.log('   - Test the update process before releasing to users');
  console.log('   - Verify that signatures are unique for each version');
  
} catch (error) {
  console.error('❌ Error updating version:', error.message);
  process.exit(1);
} 