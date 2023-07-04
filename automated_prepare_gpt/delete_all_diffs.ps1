$files = Get-ChildItem -Path .\diffs
foreach ($file in $files) {
    Remove-Item -Path $file.FullName -Force
}
