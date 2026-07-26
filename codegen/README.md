# Local contract code generation

All five language bindings are generated locally from `proto/`. The checked-in
`buf.gen.yaml` contains no BSR remote plugin, so generation never uploads the
schema.

The exact compiler, generator, and runtime versions are recorded in
`toolchain.lock.json`. A tool may be available on `PATH` or in the documented
repository-local `.tools` location, but its reported version must match the lock.

## Required local tools

- Buf `1.72.0`
- protoc `35.0`
- Go `1.26.5` and `protoc-gen-go` `1.36.11`
- Rust `rustc` and Cargo `1.97.1`
- JDK `24.0.2` and Gradle `9.0.0`
- Node.js `22.12.0`, pnpm `9.15.4`, and the dependencies in
  `codegen/typescript/package.json`

PowerShell bootstrap examples for the two external generators:

```powershell
$env:GOBIN = (Resolve-Path '.tools').Path + '\gobin'
& .\.tools\go-sdk\go\bin\go.exe install google.golang.org/protobuf/cmd/protoc-gen-go@v1.36.11

pnpm --dir codegen/typescript install --frozen-lockfile
```

Run generation and the complete compile/reproducibility gate from the repository
root:

```powershell
pwsh -File scripts/generate_contracts.ps1
pwsh -File scripts/verify_codegen.ps1
```

Outputs are written only to:

```text
generated/go
generated/rust
generated/kotlin
generated/java
generated/typescript
```

`codegen/go/go.mod` and `go.sum` are immutable templates. Generation copies
them into `generated/go`, making that directory the module root for
`github.com/lifechronicle/lifechronicle/gen/go`. Downstream local builds should
use:

```text
replace github.com/lifechronicle/lifechronicle/gen/go => ../../generated/go
```

The official protoc Rust generator is still marked experimental. It requires the
upb kernel, a crate mapping for well-known types, and an exactly matching
`protobuf` runtime. Those settings are part of the checked-in template and must
not be removed independently.
