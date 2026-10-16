$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path (Join-Path $PSScriptRoot '../../..')).Path
$reviewBinary = Join-Path $projectRoot 'target/debug/project-rl.exe'
foreach ($skillId in @('SG00','FE04','FR06')) {
    $captureTarget = Join-Path $PSScriptRoot ('window-' + $skillId)
    if (Test-Path -LiteralPath $captureTarget) { throw 'Ce dossier de capture existe déjà.' }
    $captureProcess = Start-Process -FilePath $reviewBinary -ArgumentList @('--ui-cold-agent-skills-bastion',$captureTarget,('--capture-agent-skill=' + $skillId)) -WorkingDirectory $projectRoot -WindowStyle Hidden -PassThru -Wait
    if ($captureProcess.ExitCode -ne 0) { throw "Capture interrompue : $($captureProcess.ExitCode)" }
}
