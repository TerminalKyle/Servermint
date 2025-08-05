#!/usr/bin/env node

/**
 * Fix Signature Script for ServerMint
 * 
 * This script downloads the file from the URL and generates a new signature
 * to fix signature verification issues.
 */

const fs = require('fs');
const path = require('path');
const https = require('https');

const version = '0.1.2';
const url = `https://releases.servermint.app/servermint_${version}_x64-setup.exe`;
const localFilePath = path.join(__dirname, '..', 'temp-download.exe');

console.log(`🔍 Downloading file from: ${url}`);
console.log(`📁 Saving to: ${localFilePath}`);

// Download the file
function downloadFile(url, filepath) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(filepath);
    
    https.get(url, (response) => {
      if (response.statusCode !== 200) {
        reject(new Error(`HTTP ${response.statusCode}: ${response.statusMessage}`));
        return;
      }
      
      response.pipe(file);
      
      file.on('finish', () => {
        file.close();
        console.log('✅ File downloaded successfully');
        resolve();
      });
      
      file.on('error', (err) => {
        fs.unlink(filepath, () => {}); // Delete the file if there was an error
        reject(err);
      });
    }).on('error', (err) => {
      reject(err);
    });
  });
}

// Generate signature using Tauri CLI
async function generateSignature(filepath) {
  const { exec } = require('child_process');
  
  return new Promise((resolve, reject) => {
    const command = `npx tauri signer sign --private-key "${process.env.TAURI_SIGNING_PRIVATE_KEY}" "${filepath}"`;
    
    console.log('🔐 Generating signature...');
    console.log(`Command: ${command}`);
    
    exec(command, (error, stdout, stderr) => {
      if (error) {
        console.error('❌ Error generating signature:', error);
        reject(error);
        return;
      }
      
      if (stderr) {
        console.warn('⚠️  Warning:', stderr);
      }
      
      console.log('✅ Signature generated successfully');
      console.log('Signature:', stdout.trim());
      resolve(stdout.trim());
    });
  });
}

// Update updates.json with new signature
function updateUpdatesJson(signature) {
  const updatesPath = path.join(__dirname, '..', 'updates.json');
  const currentDate = new Date().toISOString();
  
  const updatesData = {
    version: version,
    notes: "Bug Fixes + UI Improvements",
    pub_date: currentDate,
    platforms: {
      "windows-x86_64": {
        "signature": signature,
        "url": url
      }
    }
  };
  
  fs.writeFileSync(updatesPath, JSON.stringify(updatesData, null, 2));
  console.log(`✅ Updated updates.json with new signature`);
}

// Main execution
async function main() {
  try {
    // Check if TAURI_SIGNING_PRIVATE_KEY is set
    if (!process.env.TAURI_SIGNING_PRIVATE_KEY) {
      console.error('❌ TAURI_SIGNING_PRIVATE_KEY environment variable is not set');
      console.log('Please set it to your private key before running this script');
      process.exit(1);
    }
    
    // Download the file
    await downloadFile(url, localFilePath);
    
    // Generate new signature
    const signature = await generateSignature(localFilePath);
    
    // Update updates.json
    updateUpdatesJson(signature);
    
    // Clean up
    fs.unlinkSync(localFilePath);
    console.log('🧹 Cleaned up temporary file');
    
    console.log('');
    console.log('🎉 Signature fix completed!');
    console.log('The updates.json now has the correct signature for the file at the URL.');
    
  } catch (error) {
    console.error('❌ Error:', error.message);
    
    // Clean up on error
    if (fs.existsSync(localFilePath)) {
      fs.unlinkSync(localFilePath);
    }
    
    process.exit(1);
  }
}

main(); 