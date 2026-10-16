$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
if ($projectRoot -ne 'C:\Users\User\Desktop\project-RL') { throw 'Racine inattendue.' }
$baseRoot = Join-Path $projectRoot 'artifacts/refonte-agent-2026-10-06'
$reviewRoot = (Resolve-Path -LiteralPath (Join-Path $baseRoot 'skill-animation-review-99')).Path
if ($reviewRoot -ne [IO.Path]::GetFullPath((Join-Path $baseRoot 'skill-animation-review-99'))) { throw 'Revue inattendue.' }
$gallery = Get-Content -LiteralPath (Join-Path $baseRoot 'apercu-animations.html') -Raw
if (-not $gallery.Contains('ANIM-S51') -or $gallery.Contains('skill-animation-review-99/')) { throw 'La galerie utilise encore les anciens atlas.' }
$acceptance = Get-Content -LiteralPath (Join-Path $reviewRoot 'visual-acceptance-01.json') -Raw | ConvertFrom-Json
foreach ($animation in $acceptance.standalone_animations) {
    $path = Join-Path $reviewRoot $animation.path
    if ((Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $animation.sha256) { throw 'Lecture autonome modifiée.' }
}
$removed = @()
foreach ($atlas in $acceptance.atlas_exports) {
    $path = [IO.Path]::GetFullPath((Join-Path $reviewRoot $atlas.path))
    if (-not $path.StartsWith($reviewRoot + '\', [StringComparison]::OrdinalIgnoreCase) -or [IO.Path]::GetDirectoryName($path) -ne $reviewRoot) { throw 'Atlas hors revue.' }
    if ((Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $atlas.sha256) { throw 'Atlas modifié.' }
    $removed += [PSCustomObject]@{path=$atlas.path;sha256=$atlas.sha256;bytes=(Get-Item -LiteralPath $path).Length}
}
foreach ($entry in $removed) { Remove-Item -LiteralPath (Join-Path $reviewRoot $entry.path) }
[PSCustomObject]@{
    date='2026-10-15';status='completed';removed_files=$removed.Count
    bytes_removed=($removed | Measure-Object -Property bytes -Sum).Sum
    retained='Accepted source, both full lossless animations, historical metadata and approval receipts'
    archived_manifest='animations.json records historical atlas references; regenerate from the native lab if needed'
    removed=$removed
} | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $reviewRoot 'accepted-atlas-cleanup.json') -Encoding utf8


