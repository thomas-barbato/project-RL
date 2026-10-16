$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path (Join-Path $PSScriptRoot '../../..')).Path
$reviewBinary = Join-Path $projectRoot 'target/debug/project-rl.exe'
if (-not (Test-Path -LiteralPath $reviewBinary)) { throw 'Compiler le projet en mode debug avant cet aperçu.' }
Push-Location -LiteralPath $projectRoot
try { & $reviewBinary '--apercu-laboratoire-competences' '--lot-gel-statut' '--gel-status-v2' }
finally { Pop-Location }
