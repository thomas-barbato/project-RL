@echo off
cd /d "%~dp0"
cargo build --locked --bin project-rl --features dialogue-audio-trial
if errorlevel 1 (
  pause
  exit /b 1
)
start "Project RL - Essai dialogue terminal 03" "%~dp0target\debug\project-rl.exe" --essai-dialogue-terminal
