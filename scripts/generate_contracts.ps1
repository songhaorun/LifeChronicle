[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$lockPath = Join-Path $repoRoot "toolchain.lock.json"
$templatePath = Join-Path $repoRoot "buf.gen.yaml"
$lock = Get-Content -LiteralPath $lockPath -Raw | ConvertFrom-Json

function Resolve-PinnedTool {
    param(
        [Parameter(Mandatory)]
        [string]$Name,
        [Parameter(Mandatory)]
        [string]$ExpectedVersion,
        [Parameter(Mandatory)]
        [string[]]$VersionArguments,
        [Parameter(Mandatory)]
        [string[]]$LiteralCandidates,
        [Parameter(Mandatory)]
        [string]$InstallHint
    )

    $candidates = [System.Collections.Generic.List[string]]::new()
    $command = Get-Command $Name -CommandType Application, ExternalScript -ErrorAction SilentlyContinue |
        Select-Object -First 1
    if ($null -ne $command) {
        $candidates.Add($command.Source)
    }
    foreach ($candidate in $LiteralCandidates) {
        $absolute = if ([System.IO.Path]::IsPathRooted($candidate)) {
            $candidate
        } else {
            Join-Path $repoRoot $candidate
        }
        if (Test-Path -LiteralPath $absolute -PathType Leaf) {
            $candidates.Add([System.IO.Path]::GetFullPath($absolute))
        }
    }

    $diagnostics = [System.Collections.Generic.List[string]]::new()
    foreach ($candidate in $candidates | Select-Object -Unique) {
        try {
            $output = (& $candidate @VersionArguments 2>&1 | Out-String).Trim()
            $exitCode = $LASTEXITCODE
            if ($exitCode -eq 0 -and $output -match [Regex]::Escape($ExpectedVersion)) {
                return [pscustomobject]@{
                    Name = $Name
                    Path = $candidate
                    VersionOutput = $output
                }
            }
            $diagnostics.Add("'$candidate' reported '$output' (exit $exitCode)")
        } catch {
            $diagnostics.Add("'$candidate' failed: $($_.Exception.Message)")
        }
    }

    $details = if ($diagnostics.Count -gt 0) {
        " Candidates checked: " + ($diagnostics -join "; ")
    } else {
        ""
    }
    throw "Pinned tool '$Name' $ExpectedVersion was not found on PATH or in .tools.$details $InstallHint"
}

$template = Get-Content -LiteralPath $templatePath -Raw
if ($template -match "(?m)^\s*-\s*remote:" -or $template -match "(?m)^\s*remote:") {
    throw "Remote Buf plugins are forbidden: generation must not upload the Proto schema."
}

$buf = Resolve-PinnedTool `
    -Name "buf" `
    -ExpectedVersion $lock.tools.buf.version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(".tools/buf.exe", ".tools/buf") `
    -InstallHint "Install the pinned Buf release or place it at .tools/buf.exe."

$protoc = Resolve-PinnedTool `
    -Name "protoc" `
    -ExpectedVersion $lock.tools.protoc.version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(
        ".tools/protoc-$($lock.tools.protoc.version)/bin/protoc.exe",
        ".tools/protoc/bin/protoc.exe",
        ".tools/protoc/bin/protoc"
    ) `
    -InstallHint "Install protoc $($lock.tools.protoc.version) or extract it under .tools/protoc-$($lock.tools.protoc.version)."

$goPlugin = Resolve-PinnedTool `
    -Name "protoc-gen-go" `
    -ExpectedVersion $lock.tools.'protoc-gen-go'.version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(
        ".tools/gobin/protoc-gen-go.exe",
        ".tools/gobin/protoc-gen-go"
    ) `
    -InstallHint "Run: go install $($lock.tools.'protoc-gen-go'.module) with GOBIN set to .tools/gobin."

$node = Resolve-PinnedTool `
    -Name "node" `
    -ExpectedVersion "v$($lock.tools.node.version)" `
    -VersionArguments @("--version") `
    -LiteralCandidates @(".tools/node/node.exe", ".tools/node/bin/node") `
    -InstallHint "Install Node.js $($lock.tools.node.version) in PATH or .tools/node."

$esPlugin = Resolve-PinnedTool `
    -Name "protoc-gen-es" `
    -ExpectedVersion $lock.tools.'protoc-gen-es'.version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(
        ".tools/protoc-gen-es.exe",
        ".tools/protoc-gen-es.CMD",
        "codegen/typescript/node_modules/.bin/protoc-gen-es.CMD",
        "codegen/typescript/node_modules/.bin/protoc-gen-es"
    ) `
    -InstallHint "Run: pnpm --dir codegen/typescript install --frozen-lockfile."

$pathSeparator = [System.IO.Path]::PathSeparator
$toolDirectories = @(
    (Split-Path -Parent $protoc.Path),
    (Split-Path -Parent $goPlugin.Path),
    (Split-Path -Parent $esPlugin.Path),
    (Split-Path -Parent $node.Path)
) | Select-Object -Unique
$env:PATH = (($toolDirectories + ($env:PATH -split [Regex]::Escape([string]$pathSeparator))) |
    Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
    Select-Object -Unique) -join $pathSeparator

$bufCache = Join-Path ([System.IO.Path]::GetTempPath()) "lifechronicle-buf-cache-v1-$PID"
New-Item -ItemType Directory -Force -Path $bufCache | Out-Null
$env:BUF_CACHE_DIR = $bufCache

Push-Location $repoRoot
try {
    & $buf.Path lint proto
    if ($LASTEXITCODE -ne 0) {
        throw "Buf lint failed with exit code $LASTEXITCODE."
    }

    & $buf.Path generate --template $templatePath
    if ($LASTEXITCODE -ne 0) {
        throw "Local five-language generation failed with exit code $LASTEXITCODE."
    }

    # The Go module boundary must contain the generated package so downstream
    # runners can replace the published module with generated/go directly.
    $generatedGoRoot = Join-Path $repoRoot "generated/go"
    Copy-Item -LiteralPath (Join-Path $repoRoot "codegen/go/go.mod"), `
        (Join-Path $repoRoot "codegen/go/go.sum") `
        -Destination $generatedGoRoot -Force
} finally {
    Pop-Location
}

Write-Host "Generated Go, Rust, Kotlin, Java, and TypeScript contracts locally."
