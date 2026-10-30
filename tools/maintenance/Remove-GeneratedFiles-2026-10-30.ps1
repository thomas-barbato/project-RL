param([ValidateSet('Delete', 'Recycle')][string] $Mode = 'Delete')

$ErrorActionPreference = 'Stop'
$cleanupWorkspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$cleanupReport = Join-Path $cleanupWorkspace 'docs/maintenance/nettoyage-2026-10-30'
$cleanupPlan = Get-Content -LiteralPath (Join-Path $cleanupReport 'plan.json') -Raw | ConvertFrom-Json
if ($cleanupPlan.root -ne $cleanupWorkspace -or $cleanupPlan.status -ne 'ready_after_release_build') {
    throw 'Manifest root or validation status mismatch.'
}
if (Get-Process -Name cargo,rustc,project-rl -ErrorAction SilentlyContinue) {
    throw 'A build or game is running; leave its files in place.'
}
$cleanupTargets = [Collections.Generic.List[object]]::new()
$cleanupParents = @{}
$cleanupFileCount = 0
$cleanupBytes = 0L
foreach ($cleanupEntry in $cleanupPlan.targets) {
    $cleanupAbsolute = [IO.Path]::GetFullPath((Join-Path $cleanupWorkspace $cleanupEntry.path))
    $cleanupArtifacts = Join-Path $cleanupWorkspace 'artifacts'
    $cleanupTools = Join-Path $cleanupWorkspace 'tools'
    $cleanupEditor = Join-Path $cleanupArtifacts 'map-editor'
    $cleanupAllowed = $cleanupAbsolute.StartsWith($cleanupArtifacts + '\', [StringComparison]::OrdinalIgnoreCase) -or
        ($cleanupAbsolute.StartsWith($cleanupTools + '\', [StringComparison]::OrdinalIgnoreCase) -and
         [IO.Path]::GetFileName($cleanupAbsolute) -eq '__pycache__') -or
        $cleanupAbsolute -eq (Join-Path $cleanupWorkspace 'target/flycheck0')
    if (-not $cleanupAllowed -or $cleanupAbsolute -eq $cleanupEditor -or
        $cleanupAbsolute.StartsWith($cleanupEditor + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw "Protected or unexpected target: $cleanupAbsolute"
    }
    $cleanupResolved = (Resolve-Path -LiteralPath $cleanupAbsolute).Path
    if ($cleanupResolved -ne $cleanupAbsolute -or -not $cleanupResolved.StartsWith($cleanupWorkspace + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw "Escaped target: $cleanupAbsolute"
    }
    $cleanupParent = $cleanupAbsolute
    while ($cleanupParent -ne $cleanupWorkspace) {
        if ($cleanupParents.ContainsKey($cleanupParent)) { break }
        $cleanupItem = Get-Item -LiteralPath $cleanupParent -Force
        if ($cleanupItem.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Linked path: $cleanupParent" }
        $cleanupParents[$cleanupParent] = $true
        $cleanupParent = [IO.Path]::GetDirectoryName($cleanupParent)
    }
    $cleanupItem = Get-Item -LiteralPath $cleanupAbsolute -Force
    $cleanupActual = if ($cleanupItem.PSIsContainer) {
        @(Get-ChildItem -LiteralPath $cleanupAbsolute -Recurse -Force)
    } else { @($cleanupItem) }
    if ($cleanupActual | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }) {
        throw "Linked descendant in $cleanupAbsolute"
    }
    $cleanupActualFiles = @($cleanupActual | Where-Object { -not $_.PSIsContainer })
    if ($cleanupActualFiles.Count -ne @($cleanupEntry.files).Count) { throw "File count changed: $cleanupAbsolute" }
    foreach ($cleanupFile in $cleanupEntry.files) {
        $cleanupFilePath = [IO.Path]::GetFullPath((Join-Path $cleanupWorkspace $cleanupFile.path))
        if ($cleanupFilePath -ne $cleanupAbsolute -and -not $cleanupFilePath.StartsWith($cleanupAbsolute + '\', [StringComparison]::OrdinalIgnoreCase)) {
            throw 'Manifest file escapes target.'
        }
        $cleanupInfo = Get-Item -LiteralPath $cleanupFilePath -Force
        if ($cleanupInfo.PSIsContainer -or $cleanupInfo.Length -ne $cleanupFile.bytes -or
            (Get-FileHash -LiteralPath $cleanupFilePath -Algorithm SHA256).Hash -ne $cleanupFile.sha256) {
            throw "File changed since inventory: $cleanupFilePath"
        }
    }
    $cleanupTargets.Add([pscustomobject]@{ Path = $cleanupAbsolute; Entry = $cleanupEntry; Directory = $cleanupItem.PSIsContainer })
}
if ($Mode -eq 'Recycle') { Add-Type -AssemblyName Microsoft.VisualBasic }
foreach ($cleanupTarget in $cleanupTargets) {
    if ($Mode -eq 'Recycle') {
        if ($cleanupTarget.Directory) {
            [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteDirectory($cleanupTarget.Path,
                [Microsoft.VisualBasic.FileIO.UIOption]::OnlyErrorDialogs,
                [Microsoft.VisualBasic.FileIO.RecycleOption]::SendToRecycleBin)
        } else {
            [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteFile($cleanupTarget.Path,
                [Microsoft.VisualBasic.FileIO.UIOption]::OnlyErrorDialogs,
                [Microsoft.VisualBasic.FileIO.RecycleOption]::SendToRecycleBin)
        }
    } else {
        Remove-Item -LiteralPath $cleanupTarget.Path -Recurse -Force
    }
    if (Test-Path -LiteralPath $cleanupTarget.Path) { throw "Target still present: $($cleanupTarget.Path)" }
    $cleanupFileCount += @($cleanupTarget.Entry.files).Count
    $cleanupBytes += $cleanupTarget.Entry.bytes
}
$cleanupResult = [ordered]@{
    status = 'completed'
    mode = $Mode
    files_removed_from_workspace = $cleanupFileCount
    bytes_removed_from_workspace = $cleanupBytes
    bytes_permanently_deleted = $(if ($Mode -eq 'Delete') { $cleanupBytes } else { 0 })
    recycle_bin_emptied = $false
    targets = $cleanupTargets.Count
}
$cleanupResult | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $cleanupReport 'exports-result.json') -Encoding utf8
$cleanupResult | ConvertTo-Json -Compress
