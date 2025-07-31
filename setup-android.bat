@echo off
echo ServerMint Android Development Setup
echo ====================================
echo.

REM Check if PowerShell is available
powershell -Command "Get-Host" >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: PowerShell is required to run this setup script.
    echo Please install PowerShell and try again.
    pause
    exit /b 1
)

REM Run the PowerShell setup script
powershell -ExecutionPolicy Bypass -File "scripts\setup-android.ps1"

if %errorlevel% neq 0 (
    echo.
    echo Setup failed. Please check the error messages above.
    pause
    exit /b 1
)

echo.
echo Setup completed successfully!
echo.
echo To start Android development:
echo 1. Connect an Android device with USB debugging enabled
echo 2. Or start an Android emulator
echo 3. Run: npm run tauri:android:dev
echo.
pause 