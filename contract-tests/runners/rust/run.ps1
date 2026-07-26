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

$localCargo = Join-Path $repositoryRoot '.tools\cargo\bin\cargo.exe'
if (Test-Path -LiteralPath $localCargo) {
    $cargo = $localCargo
    $env:CARGO_HOME = Join-Path $repositoryRoot '.tools\cargo'
    $env:RUSTUP_HOME = Join-Path $repositoryRoot '.tools\rustup'
}
else {
    $cargo = (Get-Command cargo -ErrorAction Stop).Source
}
$env:CARGO_TARGET_DIR = Join-Path $runnerRoot '.cache\target'

Push-Location -LiteralPath $runnerRoot
try {
    & $cargo run --locked --quiet -- $resolvedVector
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}
finally {
    Pop-Location
}
