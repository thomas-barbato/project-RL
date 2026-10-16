$ErrorActionPreference = 'Stop'
$reviewRoot = $PSScriptRoot
$projectRoot = (Resolve-Path (Join-Path $reviewRoot '../../..')).Path
$reviewBinary = Join-Path $projectRoot 'target/debug/project-rl.exe'
if (-not (Test-Path -LiteralPath $reviewBinary)) { throw 'Compiler le projet en mode debug avant de lancer cet aperçu.' }
Push-Location -LiteralPath $projectRoot
try { & $reviewBinary '--apercu-laboratoire-competences' }
finally { Pop-Location }
