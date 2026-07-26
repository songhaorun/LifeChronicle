[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$toolsRoot = Join-Path $repoRoot ".tools"
$cacheRoot = Join-Path $toolsRoot "cache"
$lock = Get-Content -LiteralPath (Join-Path $repoRoot "toolchain.lock.json") -Raw |
    ConvertFrom-Json

New-Item -ItemType Directory -Force -Path $toolsRoot, $cacheRoot | Out-Null

function Assert-UnderTools {
    param([Parameter(Mandatory)][string]$Path)

    $resolved = [System.IO.Path]::GetFullPath($Path)
    $prefix = $toolsRoot.TrimEnd(
        [System.IO.Path]::DirectorySeparatorChar,
        [System.IO.Path]::AltDirectorySeparatorChar
    ) + [System.IO.Path]::DirectorySeparatorChar
    if (-not $resolved.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to modify a path outside .tools: $resolved"
    }
    return $resolved
}

function Get-VerifiedDownload {
    param(
        [Parameter(Mandatory)][uri]$Uri,
        [Parameter(Mandatory)][string]$Destination,
        [Parameter(Mandatory)][string]$ExpectedSha256
    )

    $destinationPath = Assert-UnderTools $Destination
    if (Test-Path -LiteralPath $destinationPath -PathType Leaf) {
        $existingHash = (
            Get-FileHash -LiteralPath $destinationPath -Algorithm SHA256
        ).Hash.ToLowerInvariant()
        if ($existingHash -eq $ExpectedSha256.ToLowerInvariant()) {
            return $destinationPath
        }
        Remove-Item -LiteralPath $destinationPath -Force
    }

    $partialPath = Assert-UnderTools "$destinationPath.partial"
    try {
        Invoke-WebRequest -Uri $Uri -OutFile $partialPath
        $actualHash = (
            Get-FileHash -LiteralPath $partialPath -Algorithm SHA256
        ).Hash.ToLowerInvariant()
        if ($actualHash -ne $ExpectedSha256.ToLowerInvariant()) {
            throw "SHA-256 mismatch for $Uri (expected $ExpectedSha256, got $actualHash)."
        }
        Move-Item -LiteralPath $partialPath -Destination $destinationPath -Force
    } finally {
        if (Test-Path -LiteralPath $partialPath -PathType Leaf) {
            Remove-Item -LiteralPath $partialPath -Force
        }
    }
    return $destinationPath
}

function Add-ToolPath {
    param([Parameter(Mandatory)][string]$Directory)

    $resolved = [System.IO.Path]::GetFullPath($Directory)
    $env:PATH = "$resolved$([System.IO.Path]::PathSeparator)$env:PATH"
    if ($env:GITHUB_PATH) {
        Add-Content -LiteralPath $env:GITHUB_PATH -Value $resolved -Encoding utf8
    }
}

function Set-CiEnvironment {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)][string]$Value
    )

    Set-Item -Path "Env:$Name" -Value $Value
    if ($env:GITHUB_ENV) {
        Add-Content -LiteralPath $env:GITHUB_ENV -Value "$Name=$Value" -Encoding utf8
    }
}

function Assert-Version {
    param(
        [Parameter(Mandatory)][string]$Label,
        [Parameter(Mandatory)][string]$Expected,
        [Parameter(Mandatory)][scriptblock]$Command
    )

    $output = (& $Command 2>&1 | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $output -notmatch [Regex]::Escape($Expected)) {
        throw "$Label version mismatch: expected '$Expected', got '$output'."
    }
    Write-Host "${Label}: $output"
}

$bufVersion = [string]$lock.tools.buf.version
$bufPath = Join-Path $toolsRoot "buf.exe"
$null = Get-VerifiedDownload `
    -Uri "https://github.com/bufbuild/buf/releases/download/v$bufVersion/buf-Windows-x86_64.exe" `
    -Destination $bufPath `
    -ExpectedSha256 ([string]$lock.tools.buf.windows_x86_64_sha256)
Add-ToolPath $toolsRoot
Assert-Version "Buf" $bufVersion { & $bufPath --version }

$protocVersion = [string]$lock.tools.protoc.version
$protocArchive = Join-Path $cacheRoot "protoc-$protocVersion-win64.zip"
$null = Get-VerifiedDownload `
    -Uri "https://github.com/protocolbuffers/protobuf/releases/download/v$protocVersion/protoc-$protocVersion-win64.zip" `
    -Destination $protocArchive `
    -ExpectedSha256 ([string]$lock.tools.protoc.windows_x86_64_zip_sha256)
$protocRoot = Assert-UnderTools (Join-Path $toolsRoot "protoc-$protocVersion")
$protocPath = Join-Path $protocRoot "bin/protoc.exe"
if (-not (Test-Path -LiteralPath $protocPath -PathType Leaf)) {
    if (Test-Path -LiteralPath $protocRoot) {
        Remove-Item -LiteralPath $protocRoot -Recurse -Force
    }
    Expand-Archive -LiteralPath $protocArchive -DestinationPath $protocRoot
}
Add-ToolPath (Join-Path $protocRoot "bin")
Assert-Version "protoc" $protocVersion { & $protocPath --version }

Assert-Version "Go" "go$($lock.tools.go.version)" { & go version }
$goBin = Assert-UnderTools (Join-Path $toolsRoot "gobin")
New-Item -ItemType Directory -Force -Path $goBin | Out-Null
Set-CiEnvironment "GOBIN" $goBin
Set-CiEnvironment "GOTOOLCHAIN" "local"
& go install ([string]$lock.tools.'protoc-gen-go'.module)
if ($LASTEXITCODE -ne 0) {
    throw "go install for protoc-gen-go failed with exit code $LASTEXITCODE."
}
Add-ToolPath $goBin
$goPlugin = Join-Path $goBin "protoc-gen-go.exe"
Assert-Version "protoc-gen-go" ([string]$lock.tools.'protoc-gen-go'.version) {
    & $goPlugin --version
}

Assert-Version "Node.js" "v$($lock.tools.node.version)" { & node --version }
$corepackHome = Assert-UnderTools (Join-Path $toolsRoot "corepack")
New-Item -ItemType Directory -Force -Path $corepackHome | Out-Null
Set-CiEnvironment "COREPACK_HOME" $corepackHome
& corepack prepare "pnpm@$($lock.tools.pnpm.version)" --activate
if ($LASTEXITCODE -ne 0) {
    throw "Corepack failed to prepare the pinned pnpm release."
}
Assert-Version "pnpm" ([string]$lock.tools.pnpm.version) {
    & corepack pnpm --version
}
& corepack pnpm --dir (Join-Path $repoRoot "codegen/typescript") install --frozen-lockfile
if ($LASTEXITCODE -ne 0) {
    throw "Pinned TypeScript codegen dependencies failed to install."
}

if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    throw "rustup is required on the Windows CI image."
}
$rustVersion = [string]$lock.tools.rust.rustc_version
& rustup toolchain install $rustVersion --profile minimal --no-self-update
if ($LASTEXITCODE -ne 0) {
    throw "rustup failed to install Rust $rustVersion."
}
Set-CiEnvironment "RUSTUP_TOOLCHAIN" $rustVersion
Assert-Version "rustc" $rustVersion { & rustc --version }
Assert-Version "cargo" ([string]$lock.tools.rust.cargo_version) { & cargo --version }

Assert-Version "Java" ([string]$lock.tools.java.jdk_version) { & java -version }

$gradleVersion = [string]$lock.tools.gradle.version
$gradleArchive = Join-Path $cacheRoot "gradle-$gradleVersion-bin.zip"
$gradleRoot = Assert-UnderTools (Join-Path $toolsRoot "gradle-$gradleVersion")
$gradlePath = Join-Path $gradleRoot "bin/gradle.bat"
$null = Get-VerifiedDownload `
    -Uri ([uri]$lock.tools.gradle.source) `
    -Destination $gradleArchive `
    -ExpectedSha256 ([string]$lock.tools.gradle.bin_zip_sha256)
if (-not (Test-Path -LiteralPath $gradlePath -PathType Leaf)) {
    if (Test-Path -LiteralPath $gradleRoot) {
        Remove-Item -LiteralPath $gradleRoot -Recurse -Force
    }
    Expand-Archive -LiteralPath $gradleArchive -DestinationPath $toolsRoot
}
Add-ToolPath (Join-Path $gradleRoot "bin")
Assert-Version "Gradle" $gradleVersion { & $gradlePath --version }

Write-Host "Pinned Phase 0 contract tools are installed and verified."
