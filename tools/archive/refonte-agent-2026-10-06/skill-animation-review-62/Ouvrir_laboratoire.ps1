$ErrorActionPreference = 'Stop'
$mineProject = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$mineBinary = Join-Path $mineProject 'target/debug/project-rl.exe'
if (-not (Test-Path -LiteralPath $mineBinary)) { throw 'Compiler le projet en mode debug avant cet aperçu.' }
Push-Location -LiteralPath $mineProject
try { & $mineBinary '--apercu-laboratoire-competences' '--lot-mines-variantes-v2' }
finally { Pop-Location }

