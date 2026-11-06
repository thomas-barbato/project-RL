@echo off
setlocal
cd /d "%~dp0"
set "PROJECT_RL_DATA_DIR=%~dp0"
cargo build --locked --example north_city_building_review
if errorlevel 1 exit /b 1
"target\debug\examples\north_city_building_review.exe" %*
