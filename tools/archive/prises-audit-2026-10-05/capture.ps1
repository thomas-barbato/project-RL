$ErrorActionPreference = 'Stop'
$taskRoot = (Resolve-Path -LiteralPath "$PSScriptRoot/../..").Path
$taskExe = Join-Path $taskRoot 'target/debug/project-rl.exe'
$taskScenes = @('crawl-weapons', 'crawl-grips-one-1', 'crawl-grips-one-2', 'crawl-grips-two-1', 'crawl-grips-two-2', 'crawl-grips-dual', 'terminal-hud-live', 'terminal-inventory-live-equipped', 'terminal-inventory-live-hands', 'ux-death')
foreach ($taskScene in $taskScenes) {
    $taskOutput = Join-Path $PSScriptRoot "captures/final-$taskScene"
    if (Test-Path -LiteralPath (Join-Path $taskOutput 'cold-start.png')) { continue }
    $taskLog = Join-Path $PSScriptRoot "capture-$taskScene.log"
    $taskError = Join-Path $PSScriptRoot "capture-$taskScene.err"
    $taskProcess = Start-Process -FilePath $taskExe -ArgumentList @("--ui-cold-$taskScene", $taskOutput) -WorkingDirectory $taskRoot -WindowStyle Hidden -RedirectStandardOutput $taskLog -RedirectStandardError $taskError -PassThru -Wait
    if ($taskProcess.ExitCode -ne 0) { throw "Capture $taskScene failed: $($taskProcess.ExitCode)" }
    Write-Output "$taskScene captured"
}
