@echo off
cd /d "%~dp0"
cargo build --locked --release --bin project-rl
if errorlevel 1 (
  echo Impossible de compiler l'essai des interactions.
  pause
  exit /b 1
)
echo Sauvegarde independante : essai-interactions.json
echo Nouvelle partie : gagnez la ville puis l'annexe pres des archives.
echo Un terminal indique le chemin. E pour interagir, echap pour fermer.
"target\release\project-rl.exe" --essai-interactions
