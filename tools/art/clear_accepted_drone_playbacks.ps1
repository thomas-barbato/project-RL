# Narrow pruning of old accepted review atlases after ANIM-S46 packaging.
$ErrorActionPreference = 'Stop'
$baseRoot = 'C:\Users\User\Desktop\project-RL\artifacts\refonte-agent-2026-10-06'
$reviewRoot = Join-Path $baseRoot 'skill-animation-review-93'
$previewRoot = Join-Path $baseRoot 'apercu-animations-data'
foreach ($root in @($reviewRoot,$previewRoot)) {
    if ((Resolve-Path -LiteralPath $root).Path -ne $root -or ((Get-Item -LiteralPath $root).Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Racine inattendue.' }
}
$validation = Get-Content -LiteralPath (Join-Path $reviewRoot 'validation.json') -Raw | ConvertFrom-Json
if ($validation.scenes -ne 26 -or @($validation.checks | Where-Object { -not $_.pixel_equality }).Count -ne 0) { throw 'Nouvelle lecture non verifiee.' }
$plan = Get-Content -LiteralPath (Join-Path $reviewRoot 'obsolete-playback-cleanup-plan.json') -Raw | ConvertFrom-Json
$verified = @()
foreach ($record in $plan.files) {
    $item = Get-Item -LiteralPath $record.path
    if ($item.PSIsContainer -or $item.DirectoryName -ne $previewRoot -or $item.Extension -ne '.webp' -or ($item.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Hors apercus.' }
    if ((Resolve-Path -LiteralPath $item.FullName).Path -ne $item.FullName) { throw 'Chemin inattendu.' }
    if ($item.Length -ne $record.bytes -or (Get-FileHash -LiteralPath $item.FullName -Algorithm SHA256).Hash -ne $record.sha256) { throw 'Plan different.' }
    $receiptPath = [IO.Path]::GetFullPath((Join-Path (Split-Path -Parent $baseRoot | Split-Path -Parent) $record.accepted_review))
    $allowedReceipts = @(84,85,91,92 | ForEach-Object { Join-Path $baseRoot "skill-animation-review-$_\visual-acceptance-01.json" })
    if ($receiptPath -notin $allowedReceipts) { throw 'Recu de validation inattendu.' }
    $receipt = Get-Content -LiteralPath $receiptPath -Raw | ConvertFrom-Json
    if ($record.sha256 -notin @($receipt.accepted_atlas_sha256.PSObject.Properties.Value)) { throw 'Lecture non validee.' }
    $verified += $item.FullName
}
foreach ($target in $verified) { Remove-Item -LiteralPath $target -Force }
[ordered]@{ completed=$true; removed_files=$verified.Count; removed_bytes=$plan.total_bytes; scope=$plan.scope; plan='obsolete-playback-cleanup-plan.json' } |
    ConvertTo-Json | Set-Content -LiteralPath (Join-Path $reviewRoot 'obsolete-playback-cleanup.json') -Encoding utf8
Write-Output "Anciens apercus valides retires : $($verified.Count) fichiers."
