@echo off
cd /d "%~dp0"
cargo build --locked --bin project-rl
if errorlevel 1 (
  echo Impossible de compiler l'apercu.
  pause
  exit /b 1
)
echo Apercu independant : aucun chargement ni enregistrement de partie.
echo F2 : afficher / masquer vision et bruit. Echap : fermer.
"target\debug\project-rl.exe" --apercu-lecture-tactique
