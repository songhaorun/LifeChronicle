PYTHON ?= python
PWSH ?= pwsh

.RECIPEPREFIX := >

.PHONY: governance-test adr-lint docs-test repository-scan proto-test registry-test
.PHONY: codegen-test contract-test phase-0-gate

governance-test:
>$(PYTHON) scripts/check_governance.py

adr-lint:
>$(PYTHON) scripts/lint_adrs.py

docs-test:
>$(PYTHON) scripts/check_docs.py

repository-scan:
>$(PYTHON) scripts/scan_repository.py

proto-test:
>$(PYTHON) scripts/run_proto_contract_tests.py

registry-test:
>$(PYTHON) scripts/validate_registry.py
>$(PYTHON) scripts/run_semantic_contract_tests.py

codegen-test:
>$(PWSH) -NoLogo -NoProfile -File scripts/verify_codegen.ps1

contract-test:
>$(PYTHON) scripts/run_proto_contract_tests.py
>$(PYTHON) scripts/validate_registry.py
>$(PYTHON) scripts/run_semantic_contract_tests.py
>$(PYTHON) contract-tests/generate_vectors.py --check
>$(PYTHON) scripts/run_cross_language_contract_tests.py

phase-0-gate:
>$(PYTHON) scripts/phase0_gate.py
