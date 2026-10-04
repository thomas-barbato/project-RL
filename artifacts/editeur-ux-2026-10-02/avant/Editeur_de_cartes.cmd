@echo off
cd /d "%~dp0"
if not exist "target\release\examples\map_editor.exe" (
    cargo build --release --locked --example map_editor
    if errorlevel 1 exit /b 1
)
"target\release\examples\map_editor.exe" %*
