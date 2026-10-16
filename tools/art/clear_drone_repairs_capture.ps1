$ErrorActionPreference = 'Stop'
$repairProject = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$repairReview = Join-Path $repairProject 'artifacts/refonte-agent-2026-10-06/skill-animation-review-95'
$repairExpected = 'C:\Users\User\Desktop\project-RL\artifacts\refonte-agent-2026-10-06\skill-animation-review-95'
if ((Resolve-Path -LiteralPath $repairReview).Path -ne $repairExpected) { throw 'Racine de revue inattendue.' }
if ((Get-Item -LiteralPath $repairReview).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien inattendu.' }
$repairProof = Get-Content -Raw -LiteralPath (Join-Path $repairReview 'validation.json') | ConvertFrom-Json
if ($repairProof.checks.Count -ne 22) { throw 'Revue incomplete.' }
foreach ($repairCheck in $repairProof.checks) {
    if ($repairCheck.path -notmatch '^(DP01|DR01)-(DROITE|GAUCHE|HAUT|BAS|HOSTILE|PLAFOND|CLOISON|RAPPELE|DETRUIT|ENERGIE|ATTENTE)\.webp$') { throw 'Atlas inattendu.' }
    $repairPacked = Join-Path $repairReview $repairCheck.path
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $repairPacked).Hash.ToLowerInvariant() -ne $repairCheck.sha256) { throw 'Atlas non verifie.' }
}
$repairCapture = Join-Path $repairReview '_capture'
$repairFiles = @()
$repairDirectories = @()
if (Test-Path -LiteralPath $repairCapture) {
    if ((Resolve-Path -LiteralPath $repairCapture).Path -ne (Join-Path $repairExpected '_capture')) { throw 'Capture hors revue.' }
    $repairStack = [Collections.Generic.Stack[IO.DirectoryInfo]]::new()
    $repairStack.Push((Get-Item -LiteralPath $repairCapture))
    while ($repairStack.Count -gt 0) {
        $repairDirectory = $repairStack.Pop()
        if ($repairDirectory.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien dans la capture.' }
        $repairDirectories += $repairDirectory.FullName
        foreach ($repairChild in (Get-ChildItem -LiteralPath $repairDirectory.FullName -Force)) {
            if (-not $repairChild.FullName.StartsWith($repairCapture + '\',[StringComparison]::OrdinalIgnoreCase) -or ($repairChild.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Chemin hors capture.' }
            if ($repairChild.PSIsContainer) { $repairStack.Push($repairChild) }
            elseif ($repairChild.Extension -in @('.ppm','.json')) { $repairFiles += $repairChild }
            else { throw 'Fichier de capture inattendu.' }
        }
    }
}
foreach ($repairName in @('capture-stdout.log','capture-stderr.log','DP01-native-qa.png','DR01-native-qa.png')) {
    $repairExtra = Join-Path $repairReview $repairName
    if (Test-Path -LiteralPath $repairExtra) {
        $repairItem = Get-Item -LiteralPath $repairExtra
        if ($repairItem.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien de controle inattendu.' }
        $repairFiles += $repairItem
    }
}
$repairBytes = ($repairFiles | Measure-Object Length -Sum).Sum
foreach ($repairFile in $repairFiles) { Remove-Item -LiteralPath $repairFile.FullName -Force }
foreach ($repairDirectory in ($repairDirectories | Sort-Object Length -Descending)) {
    if (@(Get-ChildItem -LiteralPath $repairDirectory -Force).Count -ne 0) { throw 'Capture encore occupee.' }
    Remove-Item -LiteralPath $repairDirectory
}
@{date='2026-10-15';removed_files=$repairFiles.Count;removed_bytes=$repairBytes;verified_atlases=22;preserved='Compact native animations, accepted assets, pending previews, code, documentation and verification receipts.'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $repairReview 'cleanup.json') -Encoding utf8
Write-Output "Captures temporaires retirees : $($repairFiles.Count) fichiers, $repairBytes octets."
