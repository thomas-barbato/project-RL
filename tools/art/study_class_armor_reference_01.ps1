# Read-only inspection of the user's installed Diablo II archives.
# Run with Windows PowerShell 32-bit. Never launches the game or accesses saves.
$ErrorActionPreference = 'Stop'
if ([IntPtr]::Size -ne 4) { throw 'Use Windows PowerShell in SysWOW64.' }
$gameRoot = 'D:\JEUX\Diablo 2 old\Diablo II'
$projectRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..'))
$workRoot = Join-Path $projectRoot 'artifacts\class-armor-review-01\diablo2-reference'
[IO.Directory]::CreateDirectory($workRoot) | Out-Null
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class ClassArmorMpqRead {
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", CharSet=CharSet.Ansi)]
 public static extern bool SFileOpenArchive(string path, uint priority, uint flags, out IntPtr handle);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", CharSet=CharSet.Ansi)]
 public static extern bool SFileOpenFileEx(IntPtr archive, string name, uint scope, out IntPtr handle);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern uint SFileGetFileSize(IntPtr file, IntPtr high);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern bool SFileReadFile(IntPtr file, byte[] bytes, uint count, out uint read, IntPtr overlapped);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern bool SFileCloseFile(IntPtr file);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern bool SFileCloseArchive(IntPtr archive);
}
'@
$report = @()
foreach ($archiveName in @('patch_d2.mpq', 'd2exp.mpq', 'd2data.mpq', 'd2char.mpq')) {
 $archive = [IntPtr]::Zero
 $archivePath = Join-Path $gameRoot $archiveName
 $before = Get-Item -LiteralPath $archivePath
 if (-not [ClassArmorMpqRead]::SFileOpenArchive($archivePath, 0, 0, [ref]$archive)) { throw "Cannot open $archiveName" }
 try {
  $names = @('data\global\excel\Armor.txt', 'data\global\excel\ArmType.txt', 'data\global\excel\Composit.txt')
  foreach ($classCode in @('BA', 'NE')) {
   foreach ($weightCode in @('lit', 'med', 'hvy')) {
    $names += "data\global\chars\$classCode\TR\${classCode}TR${weightCode}NUHTH.dcc"
   }
  }
  foreach ($name in $names) {
   $file = [IntPtr]::Zero
   if (-not [ClassArmorMpqRead]::SFileOpenFileEx($archive, $name, 0, [ref]$file)) { continue }
   try {
    $size = [ClassArmorMpqRead]::SFileGetFileSize($file, [IntPtr]::Zero)
    if ($size -gt 5000000) { throw "Unexpected file size for $name" }
    $bytes = New-Object byte[] $size
    [uint32]$read = 0
    if (-not [ClassArmorMpqRead]::SFileReadFile($file, $bytes, $size, [ref]$read, [IntPtr]::Zero)) { throw "Read failed: $name" }
    if ($read -ne $size) { throw "Short read: $name" }
    $hasher = [Security.Cryptography.SHA256]::Create()
    try { $hash = ([BitConverter]::ToString($hasher.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant() }
    finally { $hasher.Dispose() }
    if ($name.EndsWith('.txt')) {
     $out = Join-Path $workRoot ($archiveName.Replace('.mpq', '') + '-' + [IO.Path]::GetFileName($name))
     [IO.File]::WriteAllBytes($out, $bytes)
    }
    $report += [ordered]@{ archive=$archiveName; file=$name; bytes=$size; sha256=$hash }
   } finally { [ClassArmorMpqRead]::SFileCloseFile($file) | Out-Null }
  }
 } finally { [ClassArmorMpqRead]::SFileCloseArchive($archive) | Out-Null }
 $after = Get-Item -LiteralPath $archivePath
 if ($before.Length -ne $after.Length -or $before.LastWriteTimeUtc -ne $after.LastWriteTimeUtc) { throw "Archive changed: $archiveName" }
}
$result = [ordered]@{ source=$gameRoot; read_only=$true; game_launched=$false; saves_accessed=$false; entries=$report }
$json = $result | ConvertTo-Json -Depth 6
[IO.File]::WriteAllText((Join-Path $workRoot 'inspection.json'), $json, (New-Object Text.UTF8Encoding($false)))
$report | ForEach-Object { '{0} {1} {2}' -f $_.archive, $_.file, $_.bytes }
