@echo off
cd /d "%~dp0"
if not exist "target\release\project-rl.exe" (
  cargo build --locked --release --bin project-rl
  if errorlevel 1 (
    echo Impossible de compiler l'essai d'expedition.
    pause
    exit /b 1
  )
)
if not exist "artifacts\essai-expedition-2026-10-02" mkdir "artifacts\essai-expedition-2026-10-02"
"target\release\project-rl.exe" --essai-expedition 2>"artifacts\essai-expedition-2026-10-02\derniere-erreur.log"
if errorlevel 1 (
  echo Le jeu s'est arrete. Voir artifacts\essai-expedition-2026-10-02\derniere-erreur.log
  pause
)
