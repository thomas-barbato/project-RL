param([switch] $Apply)

$ErrorActionPreference = 'Stop'
$cleanupRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$report = Join-Path $cleanupRoot 'docs/maintenance/nettoyage-2026-10-25'
$plan = Get-Content -LiteralPath (Join-Path $report 'plan.json') -Raw | ConvertFrom-Json
if ($plan.status -ne 'ready_after_validation' -or $plan.root -ne $cleanupRoot) {
    throw 'Le plan ne correspond pas au projet valide.'
}
$protected = @('map-editor', 'refonte-agent-2026-10-06', 'walk-preview-review-01', 'itemisation-loot-01')
$reviewRoots = @(
    'docs/reviews/refonte-histoire-2026-10-16/street-repairs-review-09/proofs/',
    'docs/reviews/player-walk-10/', 'docs/reviews/player-walk-11/',
    'docs/reviews/player-walk-12/'
)
$checked = [Collections.Generic.List[object]]::new()
$parents = @{}

# Check the resolved absolute paths, all ancestors and the complete contents
# before the first operation. Neither junctions nor newly added files are removed.
foreach ($target in $plan.targets) {
    $relative = [string]$target.path
    if ($relative -match '(^|[/\\])\.\.([/\\]|$)|:' -or [IO.Path]::IsPathRooted($relative)) {
        throw "Chemin relatif interdit : $relative"
    }
    $absolute = [IO.Path]::GetFullPath((Join-Path $cleanupRoot $relative))
    if (-not $absolute.StartsWith($cleanupRoot + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw "Cible hors du projet : $absolute"
    }
    $normalized = $relative.Replace('\', '/')
    $allowed = $false
    if ($normalized.StartsWith('artifacts/')) {
        $first = $normalized.Split('/')[1]
        $allowed = $first -notin $protected
    }
    foreach ($prefix in $reviewRoots) {
        if ($normalized.StartsWith($prefix) -and [IO.Path]::GetExtension($normalized) -in @('.png', '.gif')) {
            $allowed = $true
        }
    }
    if ($normalized -eq 'docs/reviews/player-walk-11/audit/audit.json' -or
        $normalized -eq 'target/flycheck0' -or
        ($normalized.StartsWith('tools/') -and $normalized.EndsWith('/__pycache__'))) {
        $allowed = $true
    }
    if (-not $allowed) { throw "Cible hors du perimetre : $relative" }
    $parent = $absolute
    while ($parent -ne $cleanupRoot) {
        if ($parents.ContainsKey($parent)) { break }
        $entry = Get-Item -LiteralPath $parent -Force
        if (($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "Lien ou jonction interdit : $parent"
        }
        $parents[$parent] = $true
        $parent = [IO.Path]::GetDirectoryName($parent)
    }
    $item = Get-Item -LiteralPath $absolute -Force
    if ($item.PSIsContainer) {
        $entries = @(Get-ChildItem -LiteralPath $absolute -Recurse -Force)
        if ($entries | Where-Object { ($_.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0 }) {
            throw "La cible contient un lien : $absolute"
        }
        $actual = @($entries | Where-Object { -not $_.PSIsContainer })
    } else {
        $actual = @($item)
    }
    if ($actual.Count -ne @($target.files).Count) { throw "Contenu modifie : $absolute" }
    foreach ($row in $target.files) {
        $file = [IO.Path]::GetFullPath((Join-Path $cleanupRoot ([string]$row.path)))
        if ($file -ne $absolute -and -not $file.StartsWith($absolute + '\', [StringComparison]::OrdinalIgnoreCase)) {
            throw "Fichier hors de la cible : $file"
        }
        $entry = Get-Item -LiteralPath $file -Force
        if ($entry.PSIsContainer -or ($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0 -or
            $entry.Length -ne $row.bytes -or (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash -ne $row.sha256) {
            throw "Fichier modifie ou lie : $file"
        }
    }
    $checked.Add([pscustomobject]@{ path = $relative; absolute = $absolute; directory = $item.PSIsContainer;
        files = @($target.files).Count; bytes = [long]$target.bytes })
}
if (-not $Apply) {
    @{ status = 'checked'; targets = $checked.Count; files = $plan.file_count; bytes = $plan.bytes } | ConvertTo-Json -Compress
    exit 0
}

Add-Type -AssemblyName Microsoft.VisualBasic
$completed = [Collections.Generic.List[object]]::new()
foreach ($target in $checked) {
    if ($target.directory) {
        [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteDirectory($target.absolute,
            [Microsoft.VisualBasic.FileIO.UIOption]::OnlyErrorDialogs,
            [Microsoft.VisualBasic.FileIO.RecycleOption]::SendToRecycleBin,
            [Microsoft.VisualBasic.FileIO.UICancelOption]::ThrowException)
    } else {
        [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteFile($target.absolute,
            [Microsoft.VisualBasic.FileIO.UIOption]::OnlyErrorDialogs,
            [Microsoft.VisualBasic.FileIO.RecycleOption]::SendToRecycleBin,
            [Microsoft.VisualBasic.FileIO.UICancelOption]::ThrowException)
    }
    if (Test-Path -LiteralPath $target.absolute) { throw "Cible toujours presente : $($target.absolute)" }
    $completed.Add([pscustomobject]@{path = $target.path; files = $target.files; bytes = $target.bytes})
    @($completed) | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $report 'recycled.json') -Encoding utf8
}
@{ status = 'completed'; recycled_targets = $completed.Count; recycled_files = $plan.file_count;
    bytes_removed_from_project = $plan.bytes; recycle_bin_emptied = $false } |
    ConvertTo-Json | Set-Content -LiteralPath (Join-Path $report 'recycle-result.json') -Encoding utf8
Get-Content -LiteralPath (Join-Path $report 'recycle-result.json') -Raw
