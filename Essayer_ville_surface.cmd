@echo off
cd /d "%~dp0"
if not exist "target\debug\examples\map_editor.exe" (
    cargo build --locked --example map_editor
    if errorlevel 1 exit /b 1
)
"target\debug\examples\map_editor.exe" --urban-review
