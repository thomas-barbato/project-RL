$ErrorActionPreference = 'Stop'
$projectRoot = 'C:\Users\User\Desktop\project-RL'
if ((Resolve-Path -LiteralPath $projectRoot).Path -ne $projectRoot) { throw 'Racine inattendue.' }
$reviewRoot = Join-Path $projectRoot 'artifacts\refonte-agent-2026-10-06\skill-animation-review-84'
$planPath = Join-Path $reviewRoot 'superseded-variant-cleanup-plan.json'
$plan = Get-Content -LiteralPath $planPath -Raw | ConvertFrom-Json
if ($plan.status -eq 'complete') { Write-Output 'Ancienne variante deja retiree.'; exit 0 }
if ($plan.remove_variant -ne 'SU06-A02' -or $plan.keep_variant -ne 'SU06-A03' -or $plan.files.Count -ne 14) { throw 'Plan inattendu.' }
$allowed = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
foreach ($direction in @('DROITE','GAUCHE','HAUT','BAS','ATTENTE','PERSISTANT','RECHARGE','SECOND')) {
    [void]$allowed.Add("artifacts/refonte-agent-2026-10-06/apercu-animations-data/SU06-A02-$direction.webp")
}
foreach ($name in @('animations.json','validation.json','browser-verification.json','cleanup.json')) {
    [void]$allowed.Add("artifacts/refonte-agent-2026-10-06/skill-animation-review-83/$name")
}
foreach ($name in @('survival_passives-A02.rs','fallback-preservation.json')) {
    [void]$allowed.Add("artifacts/refonte-agent-2026-10-06/skill-animation-review-84/$name")
}
$entries = Get-Content -LiteralPath (Join-Path $reviewRoot 'animations.json') -Raw | ConvertFrom-Json
if ($entries.Count -ne 8 -or @($entries | Where-Object { $_.variant -notlike 'SU06-A03-*' }).Count -ne 0) { throw 'La galerie depend encore de la variante retiree.' }
$viewer = [IO.File]::ReadAllText((Join-Path $projectRoot 'artifacts\refonte-agent-2026-10-06\apercu-animations.html'))
if ($viewer.Contains('SU06-A02') -or $viewer.Contains('id="version"')) { throw 'Comparateur encore present.' }
$files = @()
foreach ($entry in $plan.files) {
    if (-not $allowed.Remove([string]$entry.path)) { throw 'Cible non autorisee ou dupliquee.' }
    $resolved = (Resolve-Path -LiteralPath (Join-Path $projectRoot $entry.path)).Path
    if (-not $resolved.StartsWith($projectRoot + '\',[StringComparison]::OrdinalIgnoreCase)) { throw 'Hors projet.' }
    $file = Get-Item -LiteralPath $resolved
    if ($file.PSIsContainer -or ($file.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Cible inattendue.' }
    $parent = $file.Directory
    while ($parent.FullName -ne $projectRoot) {
        if ($parent.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien dans le chemin.' }
        $parent = $parent.Parent
    }
    if ($file.Length -ne $entry.bytes -or (Get-FileHash -LiteralPath $resolved -Algorithm SHA256).Hash -ne $entry.sha256) { throw 'Fichier modifie depuis le plan.' }
    $files += $file
}
if ($allowed.Count -ne 0) { throw 'Plan incomplet.' }
# All absolute paths and hashes have been checked before the first deletion.
foreach ($file in $files) { Remove-Item -LiteralPath $file.FullName -Force }
$plan.status = 'complete'
$plan | Add-Member -NotePropertyName removed_files -NotePropertyValue $files.Count
$plan | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $planPath -Encoding utf8
Write-Output "Ancienne variante retiree : $($files.Count) fichiers, $($plan.bytes) octets."
