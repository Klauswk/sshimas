cargo build --release;

$location = './chocolatey/tools/bin'

If((Test-Path $location) -eq $False) {
    Write-Output "Creating bin folder"
    New-Item -Path $location -ItemType Directory
} else {
    Write-Output "Folder already exists"
}

$sshimasLocation = "./target/release/sshimas.exe"
$sshimasDestination = "./chocolatey/tools/sshimas.exe"

$puttyLocation = "./bin/putty.exe"
$puttyDestination = "./chocolatey/tools/bin/putty.exe"

If((Test-Path $sshimasDestination) -eq $true) {
    Write-Output "Removing older version of sshimas"
    Remove-Item -Path $sshimasDestination
} 

Write-Output "Pushing sshimas to folder"
Copy-Item -Path $sshimasLocation -Destination $sshimasDestination


If((Test-Path $puttyDestination) -eq $true) {
    Write-Output "Removing older version of putty"
    Remove-Item -Path $puttyDestination
} 

Write-Output "Pushing putty to folder"
Copy-Item -Path $puttyLocation -Destination $puttyDestination

Set-Location -Path "./chocolatey"

choco pack;

choco push sshimas.0.1.0.nupkg --source https://push.chocolatey.org/