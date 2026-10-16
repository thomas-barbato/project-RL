param([switch]$Quick, [string]$CaptureDirectory = 'captures')
$ErrorActionPreference = 'Stop'
$taskRoot = (Resolve-Path -LiteralPath "$PSScriptRoot/../..").Path
$taskExe = Join-Path $taskRoot 'target/debug/project-rl.exe'
$taskScenes = @(
    'terminal-hud-live-violet', 'terminal-inventory-live-hands-equipped-violet',
    'terminal-hud-live-blue', 'terminal-inventory-live-hands-equipped-blue',
    'terminal-hud-live-green', 'terminal-inventory-live-hands-equipped-green',
    'terminal-hud-live-red', 'terminal-inventory-live-hands-equipped-red',
    'terminal-hud-live-violet-small', 'terminal-inventory-live-hands-violet-small',
    'terminal-hud-live-violet-largeui', 'terminal-inventory-live-hands-violet-largeui',
    'terminal-hud-live-violet-wide', 'terminal-inventory-live-bag-blue', 'ux-death'
)
if ($Quick) { $taskScenes = $taskScenes[0..1] }
foreach ($taskScene in $taskScenes) {
    $taskOutput = Join-Path $PSScriptRoot "$CaptureDirectory/$taskScene"
    $taskProcess = Start-Process -FilePath $taskExe -ArgumentList @("--ui-cold-$taskScene", $taskOutput) -WorkingDirectory $taskRoot -WindowStyle Hidden -RedirectStandardOutput "$PSScriptRoot/capture-$taskScene.log" -RedirectStandardError "$PSScriptRoot/capture-$taskScene.err" -PassThru -Wait
    if ($taskProcess.ExitCode -ne 0) { throw "Capture $taskScene failed: $($taskProcess.ExitCode)" }
    Write-Output "$taskScene captured"
}
