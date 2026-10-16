$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
if ($projectRoot -ne 'C:\Users\User\Desktop\project-RL') { throw 'Racine de projet inattendue.' }
$reviewRoot = Join-Path $projectRoot 'artifacts/refonte-agent-2026-10-06/skill-animation-review-102'
$rawRoot = (Resolve-Path -LiteralPath (Join-Path $reviewRoot '_capture')).Path
$expectedRoot = [IO.Path]::GetFullPath((Join-Path $reviewRoot '_capture'))
if ($rawRoot -ne $expectedRoot) { throw 'Dossier de capture inattendu.' }
$plan = Get-Content -LiteralPath (Join-Path $reviewRoot 'cleanup-plan.json') -Raw | ConvertFrom-Json
$actual = @(Get-ChildItem -LiteralPath $rawRoot -Recurse -Force)
if (@($actual | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }).Count) { throw 'Lien symbolique interdit dans la capture.' }
if (@($actual | Where-Object { -not $_.PSIsContainer }).Count -ne @($plan.files).Count) { throw 'Le contenu de capture a changé.' }
foreach ($entry in $plan.files) {
    $path = [IO.Path]::GetFullPath((Join-Path $rawRoot $entry.path))
    if (-not $path.StartsWith($rawRoot + '\', [StringComparison]::OrdinalIgnoreCase)) { throw 'Chemin hors capture.' }
    if ((Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $entry.sha256) { throw 'Empreinte de capture modifiée.' }
}
foreach ($entry in $plan.files) { Remove-Item -LiteralPath (Join-Path $rawRoot $entry.path) }
foreach ($directory in @($actual | Where-Object PSIsContainer | Sort-Object { $_.FullName.Length } -Descending)) {
    if (@(Get-ChildItem -LiteralPath $directory.FullName -Force).Count) { throw 'Sous-dossier de capture non vide.' }
    Remove-Item -LiteralPath $directory.FullName
}
if (@(Get-ChildItem -LiteralPath $rawRoot -Force).Count) { throw 'Capture encore non vide.' }
Remove-Item -LiteralPath $rawRoot
[PSCustomObject]@{date='2026-10-15';status='completed';removed_files=@($plan.files).Count;bytes_removed=$plan.bytes;retained='Native lossless animations, compact metadata, source, code and documentation'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $reviewRoot 'capture-cleanup.json') -Encoding utf8






