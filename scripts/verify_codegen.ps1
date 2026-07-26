[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$lock = Get-Content -LiteralPath (Join-Path $repoRoot "toolchain.lock.json") -Raw |
    ConvertFrom-Json
$generateScript = Join-Path $PSScriptRoot "generate_contracts.ps1"

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

function Get-GeneratedFingerprint {
    $generatedRoot = Join-Path $repoRoot "generated"
    if (-not (Test-Path -LiteralPath $generatedRoot -PathType Container)) {
        throw "Generated output directory is missing: $generatedRoot"
    }

    $entries = Get-ChildItem -LiteralPath $generatedRoot -Recurse -File |
        Sort-Object FullName |
        ForEach-Object {
            $relative = [System.IO.Path]::GetRelativePath($generatedRoot, $_.FullName).
                Replace("\", "/")
            $hash = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
            "$relative`t$hash"
        }
    if ($entries.Count -eq 0) {
        throw "Generation produced no files."
    }
    return $entries -join "`n"
}

function Copy-DirectoryContents {
    param(
        [Parameter(Mandatory)]
        [string]$Source,
        [Parameter(Mandatory)]
        [string]$Destination
    )
    New-Item -ItemType Directory -Force -Path $Destination | Out-Null
    Get-ChildItem -LiteralPath $Source -Force | ForEach-Object {
        Copy-Item -LiteralPath $_.FullName -Destination $Destination -Recurse -Force
    }
}

function Invoke-Checked {
    param(
        [Parameter(Mandatory)]
        [string]$Label,
        [Parameter(Mandatory)]
        [string]$Executable,
        [Parameter(Mandatory)]
        [string[]]$Arguments,
        [string]$WorkingDirectory = $repoRoot
    )

    Push-Location $WorkingDirectory
    try {
        Write-Host "[$Label] $Executable $($Arguments -join ' ')"
        & $Executable @Arguments
        if ($LASTEXITCODE -ne 0) {
            throw "$Label failed with exit code $LASTEXITCODE."
        }
    } finally {
        Pop-Location
    }
}

& $generateScript
$firstFingerprint = Get-GeneratedFingerprint

$repositoryCargoHome = Join-Path $repoRoot ".tools/cargo"
$repositoryRustupHome = Join-Path $repoRoot ".tools/rustup"
if (Test-Path -LiteralPath $repositoryCargoHome -PathType Container) {
    $env:CARGO_HOME = $repositoryCargoHome
}
if (Test-Path -LiteralPath $repositoryRustupHome -PathType Container) {
    $env:RUSTUP_HOME = $repositoryRustupHome
}

$go = Resolve-PinnedTool `
    -Name "go" `
    -ExpectedVersion "go$($lock.tools.go.version)" `
    -VersionArguments @("version") `
    -LiteralCandidates @(
        ".tools/go-sdk/go/bin/go.exe",
        ".tools/go/bin/go.exe",
        ".tools/go-sdk/go/bin/go"
    ) `
    -InstallHint "Install Go $($lock.tools.go.version) or extract it under .tools/go-sdk."

$cargo = Resolve-PinnedTool `
    -Name "cargo" `
    -ExpectedVersion $lock.tools.rust.cargo_version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(
        ".tools/cargo/bin/cargo.exe",
        ".tools/cargo/bin/cargo",
        ".tools/rust/bin/cargo.exe"
    ) `
    -InstallHint "Install the pinned Rust toolchain with cargo in PATH or .tools/cargo/bin."

$rustc = Resolve-PinnedTool `
    -Name "rustc" `
    -ExpectedVersion $lock.tools.rust.rustc_version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(
        ".tools/cargo/bin/rustc.exe",
        ".tools/cargo/bin/rustc",
        ".tools/rust/bin/rustc.exe"
    ) `
    -InstallHint "Install the pinned Rust toolchain with rustc in PATH or .tools/cargo/bin."

$java = Resolve-PinnedTool `
    -Name "java" `
    -ExpectedVersion $lock.tools.java.jdk_version `
    -VersionArguments @("-version") `
    -LiteralCandidates @(".tools/jdk/bin/java.exe", ".tools/jdk/bin/java") `
    -InstallHint "Install JDK $($lock.tools.java.jdk_version) in PATH or .tools/jdk."

$gradle = Resolve-PinnedTool `
    -Name "gradle" `
    -ExpectedVersion $lock.tools.gradle.version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(
        ".tools/gradle-$($lock.tools.gradle.version)/bin/gradle.bat",
        ".tools/gradle/bin/gradle",
        ".tools/gradle/bin/gradle.bat"
    ) `
    -InstallHint "Install Gradle $($lock.tools.gradle.version) in PATH or .tools."

$node = Resolve-PinnedTool `
    -Name "node" `
    -ExpectedVersion "v$($lock.tools.node.version)" `
    -VersionArguments @("--version") `
    -LiteralCandidates @(".tools/node/node.exe", ".tools/node/bin/node") `
    -InstallHint "Install Node.js $($lock.tools.node.version) in PATH or .tools/node."

$repositoryCorepackHome = Join-Path $repoRoot ".tools/corepack"
if (Test-Path -LiteralPath $repositoryCorepackHome -PathType Container) {
    $env:COREPACK_HOME = $repositoryCorepackHome
}
$pnpm = Resolve-PinnedTool `
    -Name "corepack" `
    -ExpectedVersion $lock.tools.pnpm.version `
    -VersionArguments @("pnpm", "--version") `
    -LiteralCandidates @(".tools/corepack.cmd", ".tools/corepack") `
    -InstallHint "Use Corepack to prepare pnpm $($lock.tools.pnpm.version) under .tools/corepack."

$tsc = Resolve-PinnedTool `
    -Name "tsc" `
    -ExpectedVersion $lock.tools.typescript.version `
    -VersionArguments @("--version") `
    -LiteralCandidates @(
        "codegen/typescript/node_modules/.bin/tsc.CMD",
        "codegen/typescript/node_modules/.bin/tsc",
        ".tools/tsc.CMD"
    ) `
    -InstallHint "Run: pnpm --dir codegen/typescript install --frozen-lockfile."

$null = $rustc
$null = $java
$null = $node
$null = $pnpm

$temporaryBase = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
$temporaryRoot = Join-Path $temporaryBase ("lifechronicle-codegen-" + [Guid]::NewGuid().ToString("N"))
$cacheRoot = Join-Path $temporaryBase "lifechronicle-codegen-cache-v1"
New-Item -ItemType Directory -Force -Path $temporaryRoot, $cacheRoot | Out-Null

try {
    $goStage = Join-Path $temporaryRoot "go"
    Copy-DirectoryContents (Join-Path $repoRoot "generated/go") $goStage
    $env:GOPATH = Join-Path $cacheRoot "go-path"
    $env:GOMODCACHE = Join-Path $cacheRoot "go-mod"
    $env:GOCACHE = Join-Path $cacheRoot "go-build"
    $env:GOTOOLCHAIN = "local"
    Invoke-Checked "Go compile" $go.Path @("test", "-mod=readonly", "./...") $goStage

    $rustProject = Join-Path $temporaryRoot "codegen/rust"
    $rustGenerated = Join-Path $temporaryRoot "generated/rust"
    Copy-DirectoryContents (Join-Path $repoRoot "codegen/rust") $rustProject
    Copy-DirectoryContents (Join-Path $repoRoot "generated/rust") $rustGenerated
    Invoke-Checked "Rust compile" $cargo.Path @(
        "check",
        "--locked",
        "--manifest-path",
        (Join-Path $rustProject "Cargo.toml"),
        "--target-dir",
        (Join-Path $temporaryRoot "cargo-target")
    )

    foreach ($language in @("java", "kotlin")) {
        $project = Join-Path $temporaryRoot "codegen/$language"
        Copy-DirectoryContents (Join-Path $repoRoot "codegen/$language") $project
    }
    Copy-DirectoryContents (Join-Path $repoRoot "generated/java") `
        (Join-Path $temporaryRoot "generated/java")
    Copy-DirectoryContents (Join-Path $repoRoot "generated/kotlin") `
        (Join-Path $temporaryRoot "generated/kotlin")
    $gradleHome = Join-Path $repoRoot ".tools/gradle-cache"
    New-Item -ItemType Directory -Force -Path $gradleHome | Out-Null
    Invoke-Checked "Java compile" $gradle.Path @(
        "--no-daemon",
        "--console=plain",
        "--gradle-user-home",
        $gradleHome,
        "-p",
        (Join-Path $temporaryRoot "codegen/java"),
        "compileJava"
    )
    Invoke-Checked "Kotlin compile" $gradle.Path @(
        "--no-daemon",
        "--console=plain",
        "--gradle-user-home",
        $gradleHome,
        "-p",
        (Join-Path $temporaryRoot "codegen/kotlin"),
        "compileKotlin"
    )

    Invoke-Checked "TypeScript compile" $tsc.Path @(
        "--project",
        (Join-Path $repoRoot "codegen/typescript/tsconfig.json"),
        "--noEmit"
    )

    & $generateScript
    $secondFingerprint = Get-GeneratedFingerprint
    if ($firstFingerprint -cne $secondFingerprint) {
        throw "Repeated generation changed file names or SHA-256 values."
    }
} finally {
    $resolvedTemporaryRoot = [System.IO.Path]::GetFullPath($temporaryRoot)
    if (-not $resolvedTemporaryRoot.StartsWith(
        $temporaryBase,
        [System.StringComparison]::OrdinalIgnoreCase
    )) {
        throw "Refusing to delete a temporary path outside the system temp directory: $resolvedTemporaryRoot"
    }
    if (Test-Path -LiteralPath $resolvedTemporaryRoot) {
        Remove-Item -LiteralPath $resolvedTemporaryRoot -Recurse -Force
    }
}

Write-Host "Five-language generated code compiles and repeated generation is byte-for-byte stable."
