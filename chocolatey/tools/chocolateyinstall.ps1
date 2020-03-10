$toolsDir   = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$fileLocation = Join-Path $toolsDir 'sshimas.exe'

$packageName = 'sshimas'

$packageArgs = @{
  packageName   = $packageName
  fileType      = 'EXE'
  softwareName  = 'sshimas*'
  file          = $fileLocation

  validExitCodes= @(0)
}

Install-ChocolateyPackage @packageArgs