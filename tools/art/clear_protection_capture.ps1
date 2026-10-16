$ErrorActionPreference = 'Stop'
$captureRoot = 'C:\Users\User\Desktop\project-RL\artifacts\refonte-agent-2026-10-06\skill-animation-review-76\_capture'
if (-not (Test-Path -LiteralPath $captureRoot)) { exit 0 }
if ((Resolve-Path -LiteralPath $captureRoot).Path -ne $captureRoot) { throw 'Racine inattendue.' }
$pending = [Collections.Generic.Stack[IO.DirectoryInfo]]::new()
$pending.Push((Get-Item -LiteralPath $captureRoot))
$files = @()
$directories = @($captureRoot)
while ($pending.Count -gt 0) {
    $directory = $pending.Pop()
    if ($directory.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien inattendu.' }
    foreach ($child in (Get-ChildItem -LiteralPath $directory.FullName -Force)) {
        if (-not $child.FullName.StartsWith($captureRoot + '\',[StringComparison]::OrdinalIgnoreCase) -or ($child.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Hors capture.' }
        if ($child.PSIsContainer) { $directories += $child.FullName; $pending.Push($child) }
        elseif ($child.Extension -in @('.png','.json')) { $files += $child.FullName }
        else { throw 'Fichier inattendu dans les captures.' }
    }
}
foreach ($file in $files) { Remove-Item -LiteralPath $file -Force }
foreach ($directory in ($directories | Sort-Object Length -Descending)) {
    if (@(Get-ChildItem -LiteralPath $directory -Force).Count -ne 0) { throw 'Dossier encore occupe.' }
    Remove-Item -LiteralPath $directory
}
Write-Output "Captures temporaires retirees : $($files.Count) fichiers."
