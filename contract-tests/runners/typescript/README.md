# TypeScript golden runner

The runner uses the generated Protobuf-ES bindings and the exact dependency
versions locked by `codegen/typescript/pnpm-lock.yaml`. It reads the flattened
golden vector and emits computed `key=value` results for the cross-language
orchestrator.

From the repository root:

```powershell
& contract-tests/runners/typescript/run.ps1
```
