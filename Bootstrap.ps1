$rustupExecutable = "rustup"
$rustupPath = Get-Command $rustupExecutable -ErrorAction Silent
if (-not $rustupPath) {
    Write-Information "installing rustup..."
    Install-Rustup
} else {
    Write-Information "rustup is already installed installed at $($rustupPath.Path)"
}

$justExecutable = "just.exe"
$justPath = Get-Command $justExecutable -ErrorAction Silent
if (-not $justPath) {
    Write-Information "installing just..."
    WinGet install --id Casey.Just --exact
} else {
    Write-Information "just is already installed"
}

function Install-Rustup {
    param (
        [string]$url = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe",
        [string]$output = "C:\Users\Public\Downloads\rustup-init.exe"
    )

    Invoke-WebRequest -Uri $url -OutFile $output
    Start-Process -FilePath $output -Wait
    Remove-Item -Path $output -Force
}
