@echo off
echo Testing update process...

REM Temporarily change version to test update detection
echo Setting version to 0.1.1 for testing...
node scripts/update-release.js 0.1.1 "Test update"

echo.
echo Version set to 0.1.1 - now test the update in your app
echo The app should detect an update to 0.1.2
echo.
echo To restore the original version, run:
echo node scripts/update-release.js 0.1.2 "Bug Fixes + UI Improvements"
echo.
pause 