@echo off
cd /d "%~dp0\..\.."
cargo build --locked --bin project-rl
if errorlevel 1 exit /b 1
target\debug\project-rl.exe --apercu-canaux-egouts
