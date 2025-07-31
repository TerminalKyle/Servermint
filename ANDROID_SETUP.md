# Android Development Setup for ServerMint

This guide will help you set up Android development for ServerMint, allowing you to work on both PC and mobile platforms.

## Prerequisites

1. **Java Development Kit (JDK)** - You have Java 17 installed ✓
2. **Android SDK** - Need to install
3. **Android Device or Emulator** - For testing

## Installation Steps

### Option 1: Install Android Studio (Recommended)

1. **Download Android Studio**
   - Go to: https://developer.android.com/studio
   - Download and install Android Studio

2. **Install Android SDK**
   - Open Android Studio
   - Go to `Tools > SDK Manager`
   - Install the following:
     - Android SDK Platform-Tools
     - Android SDK Build-Tools (version 33.0.0 or higher)
     - Android SDK Tools
     - At least one Android SDK Platform (API level 21 or higher)

3. **Set Environment Variables**
   - Set `ANDROID_HOME` to your Android SDK path
   - Common locations:
     - `C:\Users\[YourUsername]\AppData\Local\Android\Sdk`
     - `C:\Android\Sdk`
   - Add to PATH:
     - `%ANDROID_HOME%\platform-tools`
     - `%ANDROID_HOME%\cmdline-tools\latest\bin`

### Option 2: Install Command Line Tools Only

1. **Download Android SDK Command Line Tools**
   - Go to: https://developer.android.com/studio#command-tools
   - Download the command line tools

2. **Extract and Setup**
   - Extract to `C:\Android\Sdk`
   - Set `ANDROID_HOME=C:\Android\Sdk`
   - Add to PATH: `C:\Android\Sdk\cmdline-tools\latest\bin`

## Quick Setup

Run the provided setup script to check your environment:

```bash
# Windows
setup-android.bat

# Or manually run PowerShell script
powershell -ExecutionPolicy Bypass -File "scripts\setup-android.ps1"
```

## Development Workflow

### 1. Initialize Android Project (First time only)
```bash
npm run tauri:android:init
```

### 2. Start Development Server
```bash
npm run tauri:android:dev
```

### 3. Build for Production
```bash
npm run tauri:android:build
```

## Device Setup

### Physical Android Device
1. Enable Developer Options:
   - Go to `Settings > About Phone`
   - Tap "Build Number" 7 times
2. Enable USB Debugging:
   - Go to `Settings > Developer Options`
   - Enable "USB Debugging"
3. Connect device via USB
4. Allow USB debugging when prompted

### Android Emulator
1. Open Android Studio
2. Go to `Tools > AVD Manager`
3. Create a new virtual device
4. Start the emulator

## Troubleshooting

### Common Issues

1. **"adb not found"**
   - Make sure Android SDK is installed
   - Check that `ANDROID_HOME` is set correctly
   - Verify platform-tools is in PATH

2. **"Build tools not found"**
   - Install Android SDK Build-Tools via SDK Manager
   - Make sure version 33.0.0 or higher is installed

3. **"Device not detected"**
   - Enable USB debugging on device
   - Install device drivers
   - Try different USB cable

4. **"Permission denied"**
   - Run as administrator if needed
   - Check file permissions

### Environment Variables

Make sure these are set in your system environment:

```bash
ANDROID_HOME=C:\Users\[YourUsername]\AppData\Local\Android\Sdk
PATH=%PATH%;%ANDROID_HOME%\platform-tools;%ANDROID_HOME%\cmdline-tools\latest\bin
```

## Project Structure

After initialization, you'll have:

```
src-tauri/
├── android/          # Android-specific files
├── gen/             # Generated Android files
└── tauri.conf.json  # Updated with mobile config
```

## Building for Distribution

To create an APK for distribution:

```bash
npm run tauri:android:build
```

The APK will be created in `src-tauri/gen/android/app/build/outputs/apk/release/`.

## Next Steps

1. Run `setup-android.bat` to verify your setup
2. Initialize the Android project: `npm run tauri:android:init`
3. Start development: `npm run tauri:android:dev`
4. Test on your device or emulator

Happy coding! 🚀 