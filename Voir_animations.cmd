@echo off
cd /d "%~dp0"
cargo build --locked --bin project-rl
if errorlevel 1 (
  pause
  exit /b 1
)
start "Revue des animations" "%~dp0target\debug\project-rl.exe" --apercu-animations
