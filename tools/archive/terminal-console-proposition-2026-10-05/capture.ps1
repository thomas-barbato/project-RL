param([switch]$ProposalOnly)
$ErrorActionPreference = 'Stop'
$taskRoot = (Resolve-Path -LiteralPath "$PSScriptRoot/../..").Path
$taskExe = Join-Path $taskRoot 'target/debug/project-rl.exe'
$taskScenes = @('terminal-console-proposal-violet', 'terminal-console-proposal-blue', 'terminal-console-proposal-green', 'terminal-console-proposal-red', 'terminal-console-proposal-violet-small', 'terminal-hud-live', 'crawl-idle', 'crawl-idle-reduced', 'terminal-inventory-live-hands', 'ux-death')
if ($ProposalOnly) { $taskScenes = $taskScenes | Where-Object { $_ -like 'terminal-console-proposal-*' } }
foreach ($taskScene in $taskScenes) {
    $taskOutput = Join-Path $PSScriptRoot "captures/$taskScene"
    $taskLog = Join-Path $PSScriptRoot "capture-$taskScene.log"
    $taskError = Join-Path $PSScriptRoot "capture-$taskScene.err"
    $taskProcess = Start-Process -FilePath $taskExe -ArgumentList @("--ui-cold-$taskScene", $taskOutput) -WorkingDirectory $taskRoot -WindowStyle Hidden -RedirectStandardOutput $taskLog -RedirectStandardError $taskError -PassThru -Wait
    if ($taskProcess.ExitCode -ne 0) { throw "Capture $taskScene failed: $($taskProcess.ExitCode)" }
    Write-Output "$taskScene captured"
}
