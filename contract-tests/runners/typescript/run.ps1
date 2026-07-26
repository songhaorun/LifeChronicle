$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$vector = if ($args.Count -gt 0) {
    (Resolve-Path $args[0]).Path
} else {
    Join-Path $repoRoot "contract-tests\vectors\phase0-v1.properties"
}
$typescriptRoot = Join-Path $repoRoot "codegen\typescript"
$compiler = Join-Path $typescriptRoot "node_modules\typescript\bin\tsc"
$configuration = Join-Path $PSScriptRoot "tsconfig.json"
$entrypoint = Join-Path $PSScriptRoot (
    "build\contract-tests\runners\typescript\src\main.js"
)

if (-not (Test-Path $compiler)) {
    throw "TypeScript dependencies are missing; run scripts/generate_contracts.ps1."
}

& node $compiler --project $configuration
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$env:NODE_PATH = Join-Path $typescriptRoot "node_modules"
& node $entrypoint $vector
exit $LASTEXITCODE
