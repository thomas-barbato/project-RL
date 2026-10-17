# Run with Windows PowerShell 32-bit; uses only archive read APIs.
$ErrorActionPreference = 'Stop'
if ([IntPtr]::Size -ne 4) { throw 'Run this helper with Windows PowerShell 32-bit (SysWOW64).' }
$base = 'D:\JEUX\Diablo 2 old\Diablo II'
$destination = 'C:\Users\User\Desktop\project-RL\artifacts\diablo2-study-2026-10-17\tables'
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class MpqStudyRead {
  [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", CharSet=CharSet.Ansi, SetLastError=true)]
  public static extern bool SFileOpenArchive(string path, uint priority, uint flags, out IntPtr handle);
  [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", CharSet=CharSet.Ansi, SetLastError=true)]
  public static extern bool SFileOpenFileEx(IntPtr archive, string name, uint scope, out IntPtr handle);
  [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", SetLastError=true)]
  public static extern uint SFileGetFileSize(IntPtr file, IntPtr high);
  [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", SetLastError=true)]
  public static extern bool SFileReadFile(IntPtr file, byte[] bytes, uint count, out uint read, IntPtr overlapped);
  [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
  public static extern bool SFileCloseFile(IntPtr file);
  [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
  public static extern bool SFileCloseArchive(IntPtr archive);
}
'@
$archives = @(
  @('PD2', (Join-Path $base 'ProjectD2\pd2data.mpq')),
  @('LoD', (Join-Path $base 'patch_d2.mpq'))
)
foreach ($pair in $archives) {
  $archive = [IntPtr]::Zero
  if (-not [MpqStudyRead]::SFileOpenArchive($pair[1], 0, 0, [ref]$archive)) { throw "Open archive failed $($pair[0])" }
  try {
    $out = Join-Path $destination $pair[0]
    [IO.Directory]::CreateDirectory($out) | Out-Null
    foreach ($name in @('Levels','MonStats','MonStats2','MonPreset','SuperUniques','LvlPrest','LvlMaze','LvlTypes','LvlSub','MonUMod','Levels.bin','MonStats.bin')) {
      $leaf = if ($name.Contains('.')) { $name } else { "$name.txt" }
      $file = [IntPtr]::Zero
      if (-not [MpqStudyRead]::SFileOpenFileEx($archive, "data\global\excel\$leaf", 0, [ref]$file)) { Write-Output "Missing $($pair[0]) $name"; continue }
      try {
        $size = [MpqStudyRead]::SFileGetFileSize($file, [IntPtr]::Zero)
        if ($size -gt 5000000) { throw 'Unexpected table size' }
        $bytes = New-Object byte[] $size
        [uint32]$read = 0
        if (-not [MpqStudyRead]::SFileReadFile($file, $bytes, $size, [ref]$read, [IntPtr]::Zero)) { throw "Read failed $name" }
        if ($read -ne $size) { throw "Short read $name" }
        [IO.File]::WriteAllBytes((Join-Path $out "$leaf"), $bytes)
        Write-Output "$($pair[0]) $name $size"
      } finally { [MpqStudyRead]::SFileCloseFile($file) | Out-Null }
    }
  } finally { [MpqStudyRead]::SFileCloseArchive($archive) | Out-Null }
}
