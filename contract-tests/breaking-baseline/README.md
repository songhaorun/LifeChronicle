# Protobuf v1 breaking baseline

`lifechronicle-v1.binpb` is the accepted Buf image for the first
LifeChronicle event contract baseline.

- SHA-256:
  `f160ec95b89b4b4e2c7236d9ba35e1cf016dbc81e938c0fb116ac91b8c6221f9`
- Generated with Buf `1.72.0` from `proto/`.
- Compatibility policy: `FILE`, as declared by `proto/buf.yaml`.

`scripts/run_proto_contract_tests.py` verifies the current source against this
image and also creates a temporary field deletion/renumbering to prove that
the breaking gate rejects it. Replacing the image is a reviewed contract
baseline change, not an automatic response to a breaking-test failure.
