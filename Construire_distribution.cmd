@echo off
cd /d "%~dp0"
python tools\distribution\package_release.py --build %*
if errorlevel 1 pause
