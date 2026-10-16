$ErrorActionPreference = 'Stop'
$heavyProject = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$heavyBinary = Join-Path $heavyProject 'target/debug/project-rl.exe'
if (-not (Test-Path -LiteralPath $heavyBinary)) { throw 'Compiler le projet en mode debug avant cet aperçu.' }
Push-Location -LiteralPath $heavyProject
try { & $heavyBinary '--apercu-laboratoire-competences' '--lot-armes-lourdes-suite' }
finally { Pop-Location }
