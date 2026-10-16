$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path (Join-Path $PSScriptRoot '../../..')).Path
$reviewBinary = Join-Path $projectRoot 'target/debug/project-rl.exe'
$captureTarget = Join-Path $PSScriptRoot 'native'
if (Test-Path -LiteralPath $captureTarget) { throw 'Ce dossier de capture existe déjà.' }
$captureProcess = Start-Process -FilePath $reviewBinary -ArgumentList @('--ui-cold-agent-skill-animations',$captureTarget,'--lot-portees-zones-vision') -WorkingDirectory $projectRoot -WindowStyle Hidden -PassThru -Wait
if ($captureProcess.ExitCode -ne 0) { throw "Capture interrompue : $($captureProcess.ExitCode)" }
