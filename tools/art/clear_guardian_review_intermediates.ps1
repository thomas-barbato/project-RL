# Narrow cleanup for the interrupted ANIM-S46 native body prototype only.
$ErrorActionPreference = 'Stop'
$reviewRoot = 'C:\Users\User\Desktop\project-RL\artifacts\refonte-agent-2026-10-06\skill-animation-review-93'
if ((Resolve-Path -LiteralPath $reviewRoot).Path -ne $reviewRoot) { throw 'Racine inattendue.' }
if ((Get-Item -LiteralPath $reviewRoot).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Lien inattendu.' }
$plan = Get-Content -LiteralPath (Join-Path $reviewRoot 'cleanup-plan.json') -Raw | ConvertFrom-Json
$names = @('_qa-guardian.png','_qa-traqueur-current.png','_qa-traqueur-source.png','_capture.stdout.log','_capture.stderr.log','_wall.stdout.log','_wall.stderr.log','animations-before-wall.json','validation-before-wall.json','capture-before-wall.json')
$verified = @()
foreach ($name in $names) {
    $target = Join-Path $reviewRoot $name
    if (-not (Test-Path -LiteralPath $target)) { continue }
    $item = Get-Item -LiteralPath $target
    if ($item.PSIsContainer -or $item.DirectoryName -ne $reviewRoot -or ($item.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw 'Fichier inattendu.' }
    $record = @($plan.files | Where-Object { $_.path -eq $item.FullName })
    if ($record.Count -ne 1 -or $record[0].bytes -ne $item.Length) { throw 'Plan different.' }
    if ((Get-FileHash -LiteralPath $item.FullName -Algorithm SHA256).Hash -ne $record[0].sha256) { throw 'Empreinte differente.' }
    $verified += $item.FullName
}
foreach ($target in $verified) { Remove-Item -LiteralPath $target -Force }
Write-Output "Intermediaires verifies retires : $($verified.Count) fichiers."
