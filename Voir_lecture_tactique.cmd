@echo off
cd /d "%~dp0"
cargo build --locked --bin project-rl
if errorlevel 1 (
  echo Impossible de compiler l'essai.
  pause
  exit /b 1
)
echo Essai jouable avec Lecture tactique apprise. Sauvegarde de test separee.
echo F2 : afficher / masquer vision et bruit. E : installations. Echap : menu.
"target\debug\project-rl.exe" --essai-lecture-tactique
