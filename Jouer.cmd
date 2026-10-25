@echo off
cd /d "%~dp0"
cargo build --locked --release --bin project-rl
if errorlevel 1 (
  pause
  exit /b 1
)
"%~dp0target\release\project-rl.exe" %*
