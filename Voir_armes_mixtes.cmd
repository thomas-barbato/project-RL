@echo off
cd /d "%~dp0"
if not exist "target\debug\project-rl.exe" (
  echo Compiler le jeu en mode debug avant cet essai.
  pause
  exit /b 1
)
start "Armes mixtes - laboratoire" "target\debug\project-rl.exe" --essai-armes-mixtes
