$ErrorActionPreference = 'Stop'
$tauntProject = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$tauntReview = Join-Path $tauntProject 'artifacts/refonte-agent-2026-10-06/skill-animation-review-94'
$tauntExpected = 'C:\Users\User\Desktop\project-RL\artifacts\refonte-agent-2026-10-06\skill-animation-review-94'
if ((Resolve-Path -LiteralPath $tauntReview).Path -ne $tauntExpected) { throw 'Racine de revue inattendue.' }
if ((Get-Item -LiteralPath $tauntReview).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien inattendu.' }
$tauntProof = Get-Content -Raw -LiteralPath (Join-Path $tauntReview 'validation.json') | ConvertFrom-Json
foreach ($tauntCheck in $tauntProof.checks) {
    $tauntName = $tauntCheck.variant.Replace('D66-TAUNT-A01-','') + '.webp'
    $tauntPacked = Join-Path $tauntReview $tauntName
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $tauntPacked).Hash.ToLowerInvariant() -ne $tauntCheck.sha256) { throw 'Atlas non verifie.' }
}
$tauntIcon = Join-Path $tauntReview 'D66-TAUNT-I01-native32.png'
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $tauntIcon).Hash.ToLowerInvariant() -ne $tauntProof.icon_sha256) { throw 'Icone non verifiee.' }
$tauntCapture = Join-Path $tauntReview '_capture'
$tauntFiles = @()
$tauntDirectories = @()
if (Test-Path -LiteralPath $tauntCapture) {
    if ((Resolve-Path -LiteralPath $tauntCapture).Path -ne (Join-Path $tauntExpected '_capture')) { throw 'Capture hors revue.' }
    $tauntStack = [Collections.Generic.Stack[IO.DirectoryInfo]]::new()
    $tauntStack.Push((Get-Item -LiteralPath $tauntCapture))
    while ($tauntStack.Count -gt 0) {
        $tauntDirectory = $tauntStack.Pop()
        if ($tauntDirectory.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien dans la capture.' }
        $tauntDirectories += $tauntDirectory.FullName
        foreach ($tauntChild in (Get-ChildItem -LiteralPath $tauntDirectory.FullName -Force)) {
            if (-not $tauntChild.FullName.StartsWith($tauntCapture + '\',[StringComparison]::OrdinalIgnoreCase) -or ($tauntChild.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Chemin hors capture.' }
            if ($tauntChild.PSIsContainer) { $tauntStack.Push($tauntChild) }
            elseif ($tauntChild.Extension -in @('.ppm','.png','.json')) { $tauntFiles += $tauntChild }
            else { throw 'Fichier de capture inattendu.' }
        }
    }
}
foreach ($tauntName in @('capture-stdout.log','capture-stderr.log','native-keyframes.png')) {
    $tauntExtra = Join-Path $tauntReview $tauntName
    if (Test-Path -LiteralPath $tauntExtra) {
        $tauntItem = Get-Item -LiteralPath $tauntExtra
        if ($tauntItem.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien de controle inattendu.' }
        $tauntFiles += $tauntItem
    }
}
$tauntBytes = ($tauntFiles | Measure-Object Length -Sum).Sum
foreach ($tauntFile in $tauntFiles) { Remove-Item -LiteralPath $tauntFile.FullName -Force }
foreach ($tauntDirectory in ($tauntDirectories | Sort-Object Length -Descending)) {
    if (@(Get-ChildItem -LiteralPath $tauntDirectory -Force).Count -ne 0) { throw 'Capture encore occupee.' }
    Remove-Item -LiteralPath $tauntDirectory
}
@{date='2026-10-15';removed_files=$tauntFiles.Count;removed_bytes=$tauntBytes;verified_atlases=$tauntProof.checks.Count;preserved='Native compact animations, icon, sources, code, rules and verification receipts.'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $tauntReview 'cleanup.json') -Encoding utf8
Write-Output "Captures temporaires retirees : $($tauntFiles.Count) fichiers, $tauntBytes octets."
