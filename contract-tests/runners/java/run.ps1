[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string] $VectorPath
)

$ErrorActionPreference = "Stop"
$runnerDirectory = Split-Path -Parent $MyInvocation.MyCommand.Path
$gradleCommand = Get-Command gradle -ErrorAction SilentlyContinue
if ($null -eq $gradleCommand) {
    [Console]::Error.WriteLine(
        "Gradle 9.0.0 is required to run the Java contract runner."
    )
    exit 127
}

$gradleArguments = @("-q", "--no-daemon", "--console=plain", "run")
if ($VectorPath) {
    try {
        $resolvedVector = (Resolve-Path -LiteralPath $VectorPath -ErrorAction Stop).Path
    }
    catch {
        [Console]::Error.WriteLine("Vector file does not exist: $VectorPath")
        exit 2
    }
    $gradleArguments += "-PvectorPath=$resolvedVector"
}

$runnerExitCode = 1
Push-Location -LiteralPath $runnerDirectory
try {
    & $gradleCommand.Source @gradleArguments
    $runnerExitCode = $LASTEXITCODE
}
catch {
    [Console]::Error.WriteLine($_.Exception.Message)
    $runnerExitCode = 1
}
finally {
    Pop-Location
}
exit $runnerExitCode
