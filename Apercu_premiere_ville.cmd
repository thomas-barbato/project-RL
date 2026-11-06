@echo off
setlocal
cd /d "%~dp0"
set "PROJECT_RL_DATA_DIR=%~dp0"
cargo build --locked --example city_cyberpunk12_review
if errorlevel 1 exit /b 1
"target\debug\examples\city_cyberpunk12_review.exe" %*
