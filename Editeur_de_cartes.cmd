@echo off
cd /d "%~dp0"
if not exist "target\release\examples\map_editor.exe" (
    cargo build --release --locked --example map_editor
    if errorlevel 1 exit /b 1
)
if not exist "artifacts\map-editor" mkdir "artifacts\map-editor"
"target\release\examples\map_editor.exe" %* 2>"artifacts\map-editor\derniere-erreur.log"
if errorlevel 1 (
    echo L'editeur s'est arrete. Le diagnostic est dans artifacts\map-editor\derniere-erreur.log
    pause
)
