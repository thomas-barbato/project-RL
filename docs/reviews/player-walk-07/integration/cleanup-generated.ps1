param([switch]$Execute)
$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../../..')).Path
if ($projectRoot -ne 'C:\Users\User\Desktop\project-RL') { throw 'Unexpected workspace' }
$artifactRoot = (Resolve-Path -LiteralPath (Join-Path $projectRoot 'artifacts')).Path
$protectedMapEditor = Join-Path $artifactRoot 'map-editor'
$candidates = [System.Collections.Generic.List[string]]::new()
foreach ($name in @('player-walk-fix-02','player-walk-fix-03','player-walk-integration-01','player-walk-review-01','player-walk-review-02','player-walk-review-03','player-walk-review-04','player-walk-review-05','player-walk-review-06')) {
    $candidate = Join-Path $artifactRoot $name
    if (Test-Path -LiteralPath $candidate) { $candidates.Add($candidate) }
}
$current = Join-Path $artifactRoot 'player-walk-review-07'
foreach ($item in Get-ChildItem -LiteralPath $current -Force) {
    if ($item.Name -in @('recipe-final','preview-data','raster-fixtures.json')) { continue }
    $candidates.Add($item.FullName)
}
foreach ($item in Get-ChildItem -LiteralPath (Join-Path $current 'preview-data') -File) {
    if ($item.Name -like 'creatures*') { $candidates.Add($item.FullName) }
}
$plan = @()
foreach ($candidate in $candidates) {
    $resolved = (Resolve-Path -LiteralPath $candidate).Path
    if (-not $resolved.StartsWith($artifactRoot + '\', [StringComparison]::OrdinalIgnoreCase)) { throw "Escaped artifact directory: $resolved" }
    if ($resolved.StartsWith($protectedMapEditor, [StringComparison]::OrdinalIgnoreCase)) { throw 'Protected map editor' }
    $item = Get-Item -LiteralPath $resolved -Force
    if (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw "Linked target: $resolved" }
    if ($item.PSIsContainer) {
        $children = @(Get-ChildItem -LiteralPath $resolved -Recurse -Force)
        if (@($children | Where-Object { ($_.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0 }).Count -gt 0) { throw "Linked child: $resolved" }
        $files = @($children | Where-Object { -not $_.PSIsContainer })
    } else { $files = @($item) }
    $bytes = ($files | Measure-Object Length -Sum).Sum
    $plan += [pscustomobject]@{path=$resolved; files=$files.Count; bytes=[long]$bytes}
}
$plan | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'cleanup-plan.json') -Encoding utf8
if (-not $Execute) {
    [pscustomobject]@{targets=$plan.Count;files=($plan|Measure-Object files -Sum).Sum;bytes=($plan|Measure-Object bytes -Sum).Sum;mode='plan_only'} | ConvertTo-Json
    exit
}
# All targets above were resolved, checked against the workspace and scanned
# for links before any move. Native sources and pending player recipes stay.
Add-Type -AssemblyName Microsoft.VisualBasic
foreach ($target in $plan) {
    if (Test-Path -LiteralPath $target.path -PathType Container) {
        [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteDirectory($target.path,
            [Microsoft.VisualBasic.FileIO.UIOption]::OnlyErrorDialogs,
            [Microsoft.VisualBasic.FileIO.RecycleOption]::SendToRecycleBin)
    } else {
        [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteFile($target.path,
            [Microsoft.VisualBasic.FileIO.UIOption]::OnlyErrorDialogs,
            [Microsoft.VisualBasic.FileIO.RecycleOption]::SendToRecycleBin)
    }
    if (Test-Path -LiteralPath $target.path) { throw "Target remains: $($target.path)" }
}
$result = [pscustomobject]@{mode='recycle_bin';removed_targets=$plan.Count;removed_files=($plan|Measure-Object files -Sum).Sum;removed_project_bytes=($plan|Measure-Object bytes -Sum).Sum;disk_space_reclaimed=$false;source_art_preserved=$true;pending_player_compositions_preserved=$true;map_editor_preserved=$true}
$result | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'cleanup-result.json') -Encoding utf8
$result | ConvertTo-Json
