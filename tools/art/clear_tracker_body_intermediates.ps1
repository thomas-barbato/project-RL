$ErrorActionPreference = 'Stop'
$trackerProject = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$trackerReview = Join-Path $trackerProject 'artifacts/refonte-agent-2026-10-06/skill-animation-review-96'
$trackerExpected = 'C:\Users\User\Desktop\project-RL\artifacts\refonte-agent-2026-10-06\skill-animation-review-96'
if ((Resolve-Path -LiteralPath $trackerReview).Path -ne $trackerExpected) { throw 'Revue inattendue.' }
if ((Get-Item -LiteralPath $trackerReview).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien inattendu.' }
$trackerProof = Get-Content -Raw -LiteralPath (Join-Path $trackerReview 'validation.json') | ConvertFrom-Json
foreach ($trackerScene in @(@('scene-before.png',$trackerProof.scene_before_sha256),@('scene-after.png',$trackerProof.scene_after_sha256))) {
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $trackerReview $trackerScene[0])).Hash.ToLowerInvariant() -ne $trackerScene[1]) { throw 'Composition non verifiee.' }
}
$trackerFiles = @()
$trackerCapture = Join-Path $trackerReview '_capture'
if (Test-Path -LiteralPath $trackerCapture) {
    if ((Resolve-Path -LiteralPath $trackerCapture).Path -ne (Join-Path $trackerExpected '_capture')) { throw 'Capture hors revue.' }
    if ((Get-Item -LiteralPath $trackerCapture).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien de capture.' }
    foreach ($trackerFile in Get-ChildItem -LiteralPath $trackerCapture -Force) {
        if ($trackerFile.Name -notin @('scene-before.png','scene-after.png') -or $trackerFile.PSIsContainer -or ($trackerFile.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Capture inattendue.' }
        $trackerFiles += $trackerFile
    }
}
foreach ($trackerName in @('capture-stdout.log','capture-stderr.log')) {
    $trackerLog = Join-Path $trackerReview $trackerName
    if (Test-Path -LiteralPath $trackerLog) {
        $trackerItem = Get-Item -LiteralPath $trackerLog
        if ($trackerItem.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien de journal.' }
        $trackerFiles += $trackerItem
    }
}
# Only derived atlases of the explicitly accepted repair review are retired.
# Its two complete animated previews and all code, sources and receipts remain.
$repairReview = Join-Path $trackerProject 'artifacts/refonte-agent-2026-10-06/skill-animation-review-95'
$repairExpected = 'C:\Users\User\Desktop\project-RL\artifacts\refonte-agent-2026-10-06\skill-animation-review-95'
if ((Resolve-Path -LiteralPath $repairReview).Path -ne $repairExpected -or ((Get-Item -LiteralPath $repairReview).Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Ancienne revue inattendue.' }
$repairReceipt = Get-Content -Raw -LiteralPath (Join-Path $repairReview 'visual-acceptance-01.json') | ConvertFrom-Json
if ($repairReceipt.review_id -ne 'ANIM-S48-ACCEPT01') { throw 'Validation explicite requise.' }
foreach ($repairInline in $repairReceipt.accepted_inline_sha256.PSObject.Properties) {
    if ($repairInline.Name -notin @('DP01-repair-blue-64.webp','DR01-repair-blue-64.webp')) { throw 'Lecture inattendue.' }
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $repairReview $repairInline.Name)).Hash.ToLowerInvariant() -ne $repairInline.Value) { throw 'Lecture complete non verifiee.' }
}
$currentTrackerPage = Get-Content -Raw -LiteralPath (Join-Path $trackerProject 'artifacts/refonte-agent-2026-10-06/apercu-animations.html')
$repairRetired = @()
foreach ($repairAtlas in $repairReceipt.accepted_atlas_sha256.PSObject.Properties) {
    if ($repairAtlas.Name -notmatch '^(DP01|DR01)-(DROITE|GAUCHE|HAUT|BAS|HOSTILE|PLAFOND|CLOISON|RAPPELE|DETRUIT|ENERGIE|ATTENTE)\.webp$') { throw 'Atlas inattendu.' }
    if ($currentTrackerPage.Contains('skill-animation-review-95/' + $repairAtlas.Name)) { throw 'Atlas encore necessaire a la page.' }
    $repairPath = Join-Path $repairReview $repairAtlas.Name
    if (Test-Path -LiteralPath $repairPath) {
        $repairItem = Get-Item -LiteralPath $repairPath
        if ($repairItem.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien d atlas.' }
        if ((Get-FileHash -Algorithm SHA256 -LiteralPath $repairPath).Hash.ToLowerInvariant() -ne $repairAtlas.Value) { throw 'Atlas modifie.' }
        $repairRetired += $repairItem
    }
}
$trackerBytes = ($trackerFiles | Measure-Object Length -Sum).Sum
$repairBytes = ($repairRetired | Measure-Object Length -Sum).Sum
foreach ($trackerFile in $trackerFiles) { Remove-Item -LiteralPath $trackerFile.FullName -Force }
if (Test-Path -LiteralPath $trackerCapture) {
    if (@(Get-ChildItem -LiteralPath $trackerCapture -Force).Count -ne 0) { throw 'Capture non vide.' }
    Remove-Item -LiteralPath $trackerCapture
}
foreach ($repairAtlas in $repairRetired) { Remove-Item -LiteralPath $repairAtlas.FullName -Force }
@{date='2026-10-15';accepted_atlas_cache_removed=$repairRetired.Count;removed_bytes=$repairBytes;complete_animations_preserved=@('DP01-repair-blue-64.webp','DR01-repair-blue-64.webp');acceptance='ANIM-S48-ACCEPT01'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $repairReview 'capture-cache-cleanup.json') -Encoding utf8
@{date='2026-10-15';temporary_capture_files_removed=$trackerFiles.Count;temporary_capture_bytes_removed=$trackerBytes;previous_accepted_atlases_removed=$repairRetired.Count;previous_atlas_bytes_removed=$repairBytes;preserved='New sources and current review compositions; existing assets, accepted animations, code, documentation and receipts.'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $trackerReview 'cleanup.json') -Encoding utf8
Write-Output "Nettoyage : $($trackerFiles.Count) captures/journaux, $($repairRetired.Count) atlas acceptes devenus inutiles."
