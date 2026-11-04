@echo off
setlocal
cd /d "%~dp0"
if not defined PROJECT_RL_SOFTWARE_CURSOR set "PROJECT_RL_SOFTWARE_CURSOR=1"
cargo build --locked --release --bin project-rl
if errorlevel 1 (
  pause
  exit /b 1
)
"%~dp0target\release\project-rl.exe" %*
