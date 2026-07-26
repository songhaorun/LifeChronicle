param(
    [Parameter(Position = 0)]
    [string] $VectorPath
)

$ErrorActionPreference = 'Stop'
$runnerRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repositoryRoot = (Resolve-Path -LiteralPath (Join-Path $runnerRoot '..\..\..')).Path

if ([string]::IsNullOrWhiteSpace($VectorPath)) {
    $resolvedVector = Join-Path $repositoryRoot 'contract-tests\vectors\phase0-v1.properties'
}
else {
    $resolvedVector = (Resolve-Path -LiteralPath $VectorPath).Path
}

$localGo = Join-Path $repositoryRoot '.tools\go-sdk\go\bin\go.exe'
if (Test-Path -LiteralPath $localGo) {
    $go = $localGo
}
else {
    $go = (Get-Command go -ErrorAction Stop).Source
}

$cacheRoot = Join-Path $runnerRoot '.cache'
$env:GOPATH = Join-Path $cacheRoot 'gopath'
$env:GOMODCACHE = Join-Path $env:GOPATH 'pkg\mod'
$env:GOCACHE = Join-Path $cacheRoot 'build'

Push-Location -LiteralPath $runnerRoot
try {
    & $go run -mod=readonly . $resolvedVector
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}
finally {
    Pop-Location
}
