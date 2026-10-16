$ErrorActionPreference = 'Stop'
$tidyProject = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
if ($tidyProject -ne 'C:\Users\User\Desktop\project-RL') { throw 'Projet inattendu.' }
$tidyReview = Join-Path $tidyProject 'artifacts/refonte-agent-2026-10-06/skill-animation-review-96'
if ((Get-Item -LiteralPath $tidyReview).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien de revue inattendu.' }
$tidyReceipt = Get-Content -Raw -LiteralPath (Join-Path $tidyReview 'visual-acceptance-01.json') | ConvertFrom-Json
if ($tidyReceipt.review_id -ne 'D58-RES64-01-ACCEPT01') { throw 'Validation explicite requise.' }
foreach ($tidyAsset in $tidyReceipt.accepted_asset_sha256.PSObject.Properties) {
    if ($tidyAsset.Name -notin @('D58-BODY01-source.png','D58-BODY01-native128.png','scene-after.png','prompt.json')) { throw 'Ressource inattendue.' }
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $tidyReview $tidyAsset.Name)).Hash.ToLowerInvariant() -ne $tidyAsset.Value) { throw 'Ressource validee modifiee.' }
}
$tidyPage = Get-Content -Raw -LiteralPath (Join-Path $tidyProject 'artifacts/refonte-agent-2026-10-06/apercu-animations.html')
$tidyComparisons = @()
foreach ($tidyContext in $tidyReceipt.shown_review_context_sha256.PSObject.Properties) {
    if ($tidyContext.Name -notin @('D58-BODY01-comparison-native.png','D58-BODY01-comparison-zoom4.png','scene-before.png')) { throw 'Comparaison inattendue.' }
    if ($tidyPage.Contains($tidyContext.Name)) { throw 'Comparaison encore utilisee.' }
    $tidyPath = Join-Path $tidyReview $tidyContext.Name
    if (Test-Path -LiteralPath $tidyPath) {
        $tidyItem = Get-Item -LiteralPath $tidyPath
        if ($tidyItem.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien de comparaison.' }
        if ((Get-FileHash -Algorithm SHA256 -LiteralPath $tidyPath).Hash.ToLowerInvariant() -ne $tidyContext.Value) { throw 'Comparaison modifiee.' }
        $tidyComparisons += $tidyItem
    }
}
$tidyCache = Join-Path $tidyProject 'tools/art/__pycache__'
$tidyBytecode = @()
$tidySourceCount = 0
if (Test-Path -LiteralPath $tidyCache) {
    if ((Resolve-Path -LiteralPath $tidyCache).Path -ne 'C:\Users\User\Desktop\project-RL\tools\art\__pycache__') { throw 'Cache hors perimetre.' }
    if ((Get-Item -LiteralPath $tidyCache).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien de cache.' }
    foreach ($tidyItem in Get-ChildItem -LiteralPath $tidyCache -Force) {
        if ($tidyItem.PSIsContainer -or $tidyItem.Extension -ne '.pyc' -or ($tidyItem.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Contenu de cache inattendu.' }
        if (-not $tidyItem.FullName.StartsWith($tidyCache + '\',[StringComparison]::OrdinalIgnoreCase)) { throw 'Fichier hors cache.' }
        $tidySourceName = ($tidyItem.Name -replace '\.cpython-\d+\.pyc$','.py')
        if (Test-Path -LiteralPath (Join-Path (Split-Path -Parent $tidyCache) $tidySourceName)) { $tidySourceCount++ }
        $tidyBytecode += $tidyItem
    }
}
$tidyComparisonBytes = ($tidyComparisons | Measure-Object Length -Sum).Sum
$tidyCacheBytes = ($tidyBytecode | Measure-Object Length -Sum).Sum
foreach ($tidyItem in $tidyComparisons) { Remove-Item -LiteralPath $tidyItem.FullName -Force }
foreach ($tidyItem in $tidyBytecode) { Remove-Item -LiteralPath $tidyItem.FullName -Force }
if (Test-Path -LiteralPath $tidyCache) {
    if (@(Get-ChildItem -LiteralPath $tidyCache -Force).Count -ne 0) { throw 'Cache non vide.' }
    Remove-Item -LiteralPath $tidyCache
}
@{date='2026-10-15';review_acceptance='D58-RES64-01-ACCEPT01';comparison_files_removed=$tidyComparisons.Count;comparison_bytes_removed=$tidyComparisonBytes;python_cache_files_removed=$tidyBytecode.Count;python_cache_bytes_removed=$tidyCacheBytes;cache_entries_with_source_still_present=$tidySourceCount;total_bytes_removed=($tidyComparisonBytes+$tidyCacheBytes);preserved='Accepted body/source/current scene, all Python source scripts, game code, documentation, animation sources and approval receipts.'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $tidyReview 'accepted-cache-cleanup.json') -Encoding utf8
Write-Output "Nettoyage termine : $($tidyComparisons.Count) comparaisons et $($tidyBytecode.Count) fichiers de cache Python retires, $($tidyComparisonBytes+$tidyCacheBytes) octets."
