@echo off
setlocal
pushd "%~dp0..\..\.."
if not exist "target\debug\project-rl.exe" (
  echo Compiler le projet en mode debug avant cet apercu.
  pause
  popd
  exit /b 1
)
"target\debug\project-rl.exe" --apercu-laboratoire-competences --lot-portees-zones-vision
popd
