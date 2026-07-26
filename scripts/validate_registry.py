#!/usr/bin/env python3
"""Validate the LifeChronicle Stream/Metric Registry without third-party code.

Registry YAML files intentionally use YAML 1.2's JSON-compatible profile.  The
validator implements the deliberately small Draft 2020-12 JSON Schema subset
used by registry/schemas, then applies cross-field and cross-file invariants.
"""

from __future__ import annotations

import argparse
import copy
import json
import re
import sys
from pathlib import Path
from typing import Any, Iterable


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
REGISTRY_ROOT = REPOSITORY_ROOT / "registry"
SCHEMA_ROOT = REGISTRY_ROOT / "schemas"

STREAM_API = "lifechronicle.io/stream/v1"
METRIC_API = "lifechronicle.io/metric/v1"
FIXTURE_API = "lifechronicle.io/registry-fixture/v1"

SCHEMA_FILES = {
    STREAM_API: SCHEMA_ROOT / "stream-definition.schema.json",
    METRIC_API: SCHEMA_ROOT / "metric-definition.schema.json",
    FIXTURE_API: SCHEMA_ROOT / "fixture-case.schema.json",
}

CORE_STREAM_PAYLOADS = {
    "app.foreground": "lifechronicle.events.v1.AppForeground",
    "device.idle.state": "lifechronicle.events.v1.IdleState",
    "device.screen.state": "lifechronicle.events.v1.ScreenState",
}

DURATION_MULTIPLIERS_NS = {
    "ns": 1,
    "us": 1_000,
    "ms": 1_000_000,
    "s": 1_000_000_000,
    "m": 60 * 1_000_000_000,
    "h": 60 * 60 * 1_000_000_000,
    "d": 24 * 60 * 60 * 1_000_000_000,
}


class RegistryError(Exception):
    """Raised for an unreadable registry artifact."""


def load_json_profile(path: Path) -> Any:
    """Load a registry YAML/JSON document using the deterministic JSON profile."""

    try:
        with path.open("r", encoding="utf-8") as handle:
            return json.load(handle)
    except OSError as exc:
        raise RegistryError(f"{path}: cannot read file: {exc}") from exc
    except json.JSONDecodeError as exc:
        raise RegistryError(
            f"{path}:{exc.lineno}:{exc.colno}: not valid YAML 1.2 JSON profile: "
            f"{exc.msg}"
        ) from exc


def resolve_ref(root_schema: dict[str, Any], reference: str) -> dict[str, Any]:
    if not reference.startswith("#/"):
        raise RegistryError(f"unsupported non-local JSON Schema reference: {reference}")
    current: Any = root_schema
    for raw_part in reference[2:].split("/"):
        part = raw_part.replace("~1", "/").replace("~0", "~")
        if not isinstance(current, dict) or part not in current:
            raise RegistryError(f"unresolvable JSON Schema reference: {reference}")
        current = current[part]
    if not isinstance(current, dict):
        raise RegistryError(f"JSON Schema reference is not an object: {reference}")
    return current


def type_matches(instance: Any, expected: str) -> bool:
    if expected == "null":
        return instance is None
    if expected == "boolean":
        return isinstance(instance, bool)
    if expected == "integer":
        return isinstance(instance, int) and not isinstance(instance, bool)
    if expected == "number":
        return isinstance(instance, (int, float)) and not isinstance(instance, bool)
    if expected == "string":
        return isinstance(instance, str)
    if expected == "array":
        return isinstance(instance, list)
    if expected == "object":
        return isinstance(instance, dict)
    raise RegistryError(f"unsupported JSON Schema type: {expected}")


def json_identity(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def validate_schema_instance(
    instance: Any,
    schema: dict[str, Any],
    root_schema: dict[str, Any],
    path: str = "$",
) -> list[str]:
    """Validate against the JSON Schema subset used by the registry."""

    errors: list[str] = []

    if "$ref" in schema:
        referenced = resolve_ref(root_schema, schema["$ref"])
        errors.extend(validate_schema_instance(instance, referenced, root_schema, path))
        remaining = {key: value for key, value in schema.items() if key != "$ref"}
        if remaining:
            errors.extend(validate_schema_instance(instance, remaining, root_schema, path))
        return errors

    if "allOf" in schema:
        for branch in schema["allOf"]:
            errors.extend(validate_schema_instance(instance, branch, root_schema, path))

    if "anyOf" in schema:
        branches = [
            validate_schema_instance(instance, branch, root_schema, path)
            for branch in schema["anyOf"]
        ]
        if not any(not branch_errors for branch_errors in branches):
            errors.append(f"{path}: does not satisfy any allowed schema")
            return errors

    if "oneOf" in schema:
        matches = sum(
            not validate_schema_instance(instance, branch, root_schema, path)
            for branch in schema["oneOf"]
        )
        if matches != 1:
            errors.append(f"{path}: must satisfy exactly one allowed schema")
            return errors

    if "const" in schema and instance != schema["const"]:
        errors.append(f"{path}: must equal {schema['const']!r}")

    if "enum" in schema and instance not in schema["enum"]:
        allowed = ", ".join(repr(value) for value in schema["enum"])
        errors.append(f"{path}: must be one of {allowed}")

    expected_type = schema.get("type")
    if expected_type is not None:
        expected_types = (
            expected_type if isinstance(expected_type, list) else [expected_type]
        )
        if not any(type_matches(instance, item) for item in expected_types):
            errors.append(
                f"{path}: expected {' or '.join(expected_types)}, "
                f"got {type(instance).__name__}"
            )
            return errors

    if isinstance(instance, str):
        if len(instance) < schema.get("minLength", 0):
            errors.append(f"{path}: string is shorter than {schema['minLength']}")
        if "maxLength" in schema and len(instance) > schema["maxLength"]:
            errors.append(f"{path}: string is longer than {schema['maxLength']}")
        if "pattern" in schema and re.search(schema["pattern"], instance) is None:
            errors.append(f"{path}: value does not match {schema['pattern']!r}")

    if isinstance(instance, (int, float)) and not isinstance(instance, bool):
        if "minimum" in schema and instance < schema["minimum"]:
            errors.append(f"{path}: must be >= {schema['minimum']}")
        if "maximum" in schema and instance > schema["maximum"]:
            errors.append(f"{path}: must be <= {schema['maximum']}")

    if isinstance(instance, list):
        if len(instance) < schema.get("minItems", 0):
            errors.append(f"{path}: must contain at least {schema['minItems']} item(s)")
        if "maxItems" in schema and len(instance) > schema["maxItems"]:
            errors.append(f"{path}: must contain at most {schema['maxItems']} item(s)")
        if schema.get("uniqueItems"):
            identities = [json_identity(value) for value in instance]
            if len(set(identities)) != len(identities):
                errors.append(f"{path}: items must be unique")
        if "items" in schema:
            for index, value in enumerate(instance):
                errors.extend(
                    validate_schema_instance(
                        value, schema["items"], root_schema, f"{path}[{index}]"
                    )
                )

    if isinstance(instance, dict):
        required = schema.get("required", [])
        for field in required:
            if field not in instance:
                errors.append(f"{path}: missing required field {field!r}")

        properties = schema.get("properties", {})
        if schema.get("additionalProperties") is False:
            for field in sorted(set(instance) - set(properties)):
                errors.append(f"{path}: unknown field {field!r}")

        for field, value in instance.items():
            property_schema = properties.get(field)
            if property_schema is not None:
                errors.extend(
                    validate_schema_instance(
                        value, property_schema, root_schema, f"{path}.{field}"
                    )
                )

    return errors


def audit_strict_objects(node: Any, path: str = "$") -> list[str]:
    """Ensure every object subschema rejects unknown fields."""

    errors: list[str] = []
    if isinstance(node, dict):
        if node.get("type") == "object" and node.get("additionalProperties") is not False:
            errors.append(f"{path}: object schema must set additionalProperties to false")
        for key, value in node.items():
            errors.extend(audit_strict_objects(value, f"{path}.{key}"))
    elif isinstance(node, list):
        for index, value in enumerate(node):
            errors.extend(audit_strict_objects(value, f"{path}[{index}]"))
    return errors


def duration_ns(value: str) -> int | None:
    match = re.fullmatch(r"(0|[1-9][0-9]*)(ns|us|ms|s|m|h|d)", value)
    if match is None:
        return None
    return int(match.group(1)) * DURATION_MULTIPLIERS_NS[match.group(2)]


def lifecycle_errors(document: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    lifecycle = document["lifecycle"]
    status = document["status"]
    introduced = lifecycle["introduced_registry_version"]
    deprecated = lifecycle["deprecated_registry_version"]
    retired = lifecycle["retired_registry_version"]

    if deprecated is not None and deprecated < introduced:
        errors.append(
            "$.lifecycle.deprecated_registry_version: cannot precede introduction"
        )
    lower_retirement_bound = deprecated if deprecated is not None else introduced
    if retired is not None and retired < lower_retirement_bound:
        errors.append("$.lifecycle.retired_registry_version: lifecycle is not monotonic")

    if status in {"draft", "active"} and (deprecated is not None or retired is not None):
        errors.append(f"$.lifecycle: {status} definitions cannot have retirement markers")
    if status == "deprecated" and (deprecated is None or retired is not None):
        errors.append(
            "$.lifecycle: deprecated definitions require only a deprecation version"
        )
    if status == "retired" and (deprecated is None or retired is None):
        errors.append(
            "$.lifecycle: retired definitions require deprecation and retirement versions"
        )
    if lifecycle["replacement"] is not None and status not in {"deprecated", "retired"}:
        errors.append(
            "$.lifecycle.replacement: replacement is only valid after deprecation"
        )
    return errors


def registry_name_errors(name: str) -> list[str]:
    if name.startswith("plugin.") and len(name.split(".")) != 4:
        return [
            "$.name: third-party names must be "
            "plugin.<publisher>.<plugin>.<metric>"
        ]
    return []


def definition_path_errors(
    document: dict[str, Any], source_path: Path | None
) -> list[str]:
    if source_path is None:
        return []
    try:
        relative = source_path.resolve().relative_to(REGISTRY_ROOT.resolve())
    except ValueError:
        return []
    if not relative.parts or relative.parts[0] not in {"streams", "metrics"}:
        return []
    if len(relative.parts) != 3:
        return [f"$: definition path must be <kind>/<name>/v<version>.yaml"]
    kind, directory_name, filename = relative.parts
    expected_kind = "streams" if document["api_version"] == STREAM_API else "metrics"
    errors: list[str] = []
    if kind != expected_kind:
        errors.append(f"$: {document['api_version']} must live under {expected_kind}/")
    if directory_name != document["name"]:
        errors.append(
            f"$.name: {document['name']!r} does not match directory {directory_name!r}"
        )
    expected_filename = f"v{document['schema']['version']}.yaml"
    if filename != expected_filename:
        errors.append(f"$.schema.version: expected file name {expected_filename!r}")
    return errors


def validate_stream_semantics(
    document: dict[str, Any], source_path: Path | None
) -> list[str]:
    errors = lifecycle_errors(document)
    errors.extend(registry_name_errors(document["name"]))
    errors.extend(definition_path_errors(document, source_path))

    name = document["name"]
    payload_type = document["schema"]["payload_type"]
    expected_payload = CORE_STREAM_PAYLOADS.get(name)
    if expected_payload is not None and payload_type != expected_payload:
        errors.append(
            f"$.schema.payload_type: {name} requires payload {expected_payload!r}"
        )
    if not document["schema"]["event_type"].startswith(f"{name}."):
        errors.append("$.schema.event_type: must be namespaced beneath the stream name")

    if document["defaults"]["privacy_class"] != "PRIVATE":
        errors.append("$.defaults.privacy_class: registry v1 defaults must be PRIVATE")
    if document["privacy"]["public_projection"] != "none":
        errors.append(
            "$.privacy.public_projection: v1 registry definitions must set "
            "public_projection to none"
        )
    if (
        document["defaults"]["retention_class"]
        not in document["accepted_retention_classes"]
    ):
        errors.append(
            "$.accepted_retention_classes: must include defaults.retention_class"
        )

    version = document["schema"]["version"]
    processor_ids: set[str] = set()
    for index, processor in enumerate(document["processors"]):
        if processor["id"] in processor_ids:
            errors.append(f"$.processors[{index}].id: processor ids must be unique")
        processor_ids.add(processor["id"])
        if version not in processor["accepted_schema_versions"]:
            errors.append(
                f"$.processors[{index}].accepted_schema_versions: "
                f"must include current schema version {version}"
            )

    event_time = document["event_time"]
    max_out_of_order = duration_ns(event_time["max_out_of_order"])
    allowed_lateness = duration_ns(event_time["realtime_allowed_lateness"])
    if (
        max_out_of_order is not None
        and allowed_lateness is not None
        and allowed_lateness < max_out_of_order
    ):
        errors.append(
            "$.event_time.realtime_allowed_lateness: cannot be less than "
            "max_out_of_order"
        )

    if document["source_policy"]["strategy"] == "preserve_all" and document[
        "source_policy"
    ]["priorities"]:
        errors.append(
            "$.source_policy.priorities: preserve_all cannot declare source priority"
        )

    if document["record_kind"] == "SERIES" and document["series"] is None:
        errors.append("$.series: SERIES records require a series byte contract")
    if document["record_kind"] != "SERIES" and document["series"] is not None:
        errors.append("$.series: non-Series records must set series to null")
    return errors


def validate_metric_semantics(
    document: dict[str, Any],
    source_path: Path | None,
    stream_catalog: dict[str, set[int]],
) -> list[str]:
    errors = lifecycle_errors(document)
    errors.extend(registry_name_errors(document["name"]))
    errors.extend(definition_path_errors(document, source_path))

    if document["defaults"]["privacy_class"] != "PRIVATE":
        errors.append("$.defaults.privacy_class: registry v1 defaults must be PRIVATE")
    if document["privacy"]["public_projection"] != "none":
        errors.append(
            "$.privacy.public_projection: v1 registry definitions must set "
            "public_projection to none"
        )
    if (
        document["defaults"]["retention_class"]
        not in document["accepted_retention_classes"]
    ):
        errors.append(
            "$.accepted_retention_classes: must include defaults.retention_class"
        )

    version = document["schema"]["version"]
    processor_ids: set[str] = set()
    for index, processor in enumerate(document["processors"]):
        if processor["id"] in processor_ids:
            errors.append(f"$.processors[{index}].id: processor ids must be unique")
        processor_ids.add(processor["id"])
        if version not in processor["accepted_metric_versions"]:
            errors.append(
                f"$.processors[{index}].accepted_metric_versions: "
                f"must include current metric version {version}"
            )

    for index, source in enumerate(document["source_streams"]):
        available_versions = stream_catalog.get(source["stream"])
        if available_versions is None:
            errors.append(
                f"$.source_streams[{index}].stream: unknown stream {source['stream']!r}"
            )
        elif not set(source["schema_versions"]).issubset(available_versions):
            missing = sorted(set(source["schema_versions"]) - available_versions)
            errors.append(
                f"$.source_streams[{index}].schema_versions: unavailable versions "
                f"{missing}"
            )

    max_out_of_order = duration_ns(document["event_time"]["max_out_of_order"])
    allowed_lateness = duration_ns(
        document["event_time"]["realtime_allowed_lateness"]
    )
    if (
        max_out_of_order is not None
        and allowed_lateness is not None
        and allowed_lateness < max_out_of_order
    ):
        errors.append(
            "$.event_time.realtime_allowed_lateness: cannot be less than "
            "max_out_of_order"
        )

    if document["window"]["type"] == "SESSION" and document["window"]["size"] is not None:
        errors.append("$.window.size: SESSION metrics must use a null fixed size")
    if document["window"]["type"] != "SESSION" and document["window"]["size"] is None:
        errors.append("$.window.size: non-SESSION metrics require an explicit size")

    if document["name"] == "app.duration":
        if document["unit"] != "ns":
            errors.append("$.unit: app.duration must use canonical nanoseconds (ns)")
        if document["schema"]["value_type"] != "INT64":
            errors.append("$.schema.value_type: app.duration must use INT64")
        source_names = {item["stream"] for item in document["source_streams"]}
        if source_names != {"app.foreground"}:
            errors.append(
                "$.source_streams: app.duration must derive only from app.foreground"
            )
        if (
            document["aggregation"]["operation"]
            != "DURATION_BETWEEN_TRANSITIONS"
        ):
            errors.append(
                "$.aggregation.operation: app.duration must use ordered transitions"
            )
    return errors


def validate_document(
    document: Any,
    schemas: dict[str, dict[str, Any]],
    stream_catalog: dict[str, set[int]],
    source_path: Path | None = None,
) -> list[str]:
    if not isinstance(document, dict):
        return ["$: registry document must be an object"]
    api_version = document.get("api_version")
    schema = schemas.get(api_version)
    if schema is None or api_version == FIXTURE_API:
        return [f"$.api_version: unsupported registry API {api_version!r}"]
    errors = validate_schema_instance(document, schema, schema)
    if errors:
        return errors
    if api_version == STREAM_API:
        return validate_stream_semantics(document, source_path)
    return validate_metric_semantics(document, source_path, stream_catalog)


def set_mutation(document: dict[str, Any], path: str, value: Any) -> None:
    segments = path.split(".")
    parent: Any = document
    for segment in segments[:-1]:
        if not isinstance(parent, dict) or segment not in parent:
            raise RegistryError(f"fixture mutation parent does not exist: {path}")
        parent = parent[segment]
    if not isinstance(parent, dict):
        raise RegistryError(f"fixture mutation parent is not an object: {path}")
    parent[segments[-1]] = value


def delete_mutation(document: dict[str, Any], path: str) -> None:
    segments = path.split(".")
    parent: Any = document
    for segment in segments[:-1]:
        if not isinstance(parent, dict) or segment not in parent:
            raise RegistryError(f"fixture mutation parent does not exist: {path}")
        parent = parent[segment]
    if not isinstance(parent, dict) or segments[-1] not in parent:
        raise RegistryError(f"fixture delete target does not exist: {path}")
    del parent[segments[-1]]


def materialize_fixture(
    fixture: dict[str, Any],
    schemas: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    fixture_schema = schemas[FIXTURE_API]
    errors = validate_schema_instance(fixture, fixture_schema, fixture_schema)
    if errors:
        raise RegistryError("invalid fixture descriptor: " + "; ".join(errors))

    base_path = (REGISTRY_ROOT / fixture["base"]).resolve()
    try:
        base_path.relative_to(REGISTRY_ROOT.resolve())
    except ValueError as exc:
        raise RegistryError("fixture base escapes registry directory") from exc
    document = copy.deepcopy(load_json_profile(base_path))
    for mutation in fixture["mutations"]:
        if mutation["op"] == "set":
            set_mutation(document, mutation["path"], mutation["value"])
        else:
            delete_mutation(document, mutation["path"])
    return document


def definition_paths() -> list[Path]:
    paths = list((REGISTRY_ROOT / "streams").glob("*/*.yaml"))
    paths.extend((REGISTRY_ROOT / "metrics").glob("*/*.yaml"))
    return sorted(paths)


def template_paths() -> list[Path]:
    return sorted((REGISTRY_ROOT / "templates").glob("*.yaml"))


def fixture_paths() -> list[Path]:
    return sorted((REGISTRY_ROOT / "fixtures").glob("*/*.json"))


def build_stream_catalog(paths: Iterable[Path]) -> dict[str, set[int]]:
    catalog: dict[str, set[int]] = {}
    for path in paths:
        document = load_json_profile(path)
        if (
            isinstance(document, dict)
            and document.get("api_version") == STREAM_API
            and isinstance(document.get("schema"), dict)
            and isinstance(document["schema"].get("version"), int)
            and isinstance(document.get("name"), str)
        ):
            catalog.setdefault(document["name"], set()).add(
                document["schema"]["version"]
            )
    return catalog


def load_schemas() -> dict[str, dict[str, Any]]:
    schemas: dict[str, dict[str, Any]] = {}
    for api_version, path in SCHEMA_FILES.items():
        schema = load_json_profile(path)
        if not isinstance(schema, dict):
            raise RegistryError(f"{path}: schema must be an object")
        if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
            raise RegistryError(f"{path}: expected JSON Schema Draft 2020-12")
        strict_errors = audit_strict_objects(schema)
        if strict_errors:
            raise RegistryError(f"{path}: " + "; ".join(strict_errors))
        schemas[api_version] = schema
    return schemas


def display_errors(path: Path, errors: list[str]) -> None:
    for error in errors:
        print(f"ERROR {path}: {error}", file=sys.stderr)


def validate_explicit_paths(paths: list[Path], quiet: bool) -> int:
    schemas = load_schemas()
    catalog = build_stream_catalog(definition_paths())
    failed = False
    for path in paths:
        document = load_json_profile(path)
        if isinstance(document, dict) and document.get("api_version") == FIXTURE_API:
            document = materialize_fixture(document, schemas)
            errors = validate_document(document, schemas, catalog)
        else:
            errors = validate_document(document, schemas, catalog, path)
        if errors:
            failed = True
            display_errors(path, errors)
        elif not quiet:
            print(f"PASS  {path}")
    return 1 if failed else 0


def validate_suite(quiet: bool) -> int:
    schemas = load_schemas()
    definitions = definition_paths()
    templates = template_paths()
    fixtures = fixture_paths()
    catalog = build_stream_catalog(definitions)
    failures: list[str] = []

    identities: set[tuple[str, str, int]] = set()
    for path in definitions + templates:
        document = load_json_profile(path)
        errors = validate_document(document, schemas, catalog, path)
        if errors:
            failures.extend(f"{path}: {error}" for error in errors)
            continue
        if path in definitions:
            identity = (
                document["api_version"],
                document["name"],
                document["schema"]["version"],
            )
            if identity in identities:
                failures.append(f"{path}: duplicate definition {identity}")
            identities.add(identity)
        if not quiet:
            print(f"PASS  definition {path.relative_to(REPOSITORY_ROOT)}")

    valid_fixture_count = 0
    invalid_fixture_count = 0
    for path in fixtures:
        fixture = load_json_profile(path)
        try:
            materialized = materialize_fixture(fixture, schemas)
        except RegistryError as exc:
            failures.append(f"{path}: {exc}")
            continue
        errors = validate_document(materialized, schemas, catalog)
        expected = fixture["expected"]
        if expected["valid"]:
            valid_fixture_count += 1
            if errors:
                failures.append(
                    f"{path}: expected valid, received: {'; '.join(errors)}"
                )
            elif not quiet:
                print(f"PASS  valid fixture {path.relative_to(REPOSITORY_ROOT)}")
        else:
            invalid_fixture_count += 1
            needle = expected["error_contains"]
            if not errors:
                failures.append(f"{path}: invalid fixture was accepted")
            elif needle is not None and not any(needle in error for error in errors):
                failures.append(
                    f"{path}: expected error containing {needle!r}, received: "
                    f"{'; '.join(errors)}"
                )
            elif not quiet:
                print(f"PASS  rejected fixture {path.relative_to(REPOSITORY_ROOT)}")

    if valid_fixture_count == 0:
        failures.append("registry/fixtures/valid: at least one valid fixture is required")
    if invalid_fixture_count == 0:
        failures.append(
            "registry/fixtures/invalid: at least one invalid fixture is required"
        )

    if failures:
        for failure in failures:
            print(f"ERROR {failure}", file=sys.stderr)
        print(
            f"Registry validation failed with {len(failures)} error(s).",
            file=sys.stderr,
        )
        return 1

    print(
        "Registry validation passed: "
        f"{len(definitions)} definitions, {len(templates)} templates, "
        f"{valid_fixture_count} valid fixtures, "
        f"{invalid_fixture_count} negative fixtures."
    )
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate LifeChronicle Stream/Metric Registry v1."
    )
    parser.add_argument(
        "paths",
        nargs="*",
        type=Path,
        help="Optional registry definition or fixture paths.",
    )
    parser.add_argument(
        "--quiet",
        action="store_true",
        help="Only print the suite summary and errors.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if args.paths:
            return validate_explicit_paths(
                [path.resolve() for path in args.paths], args.quiet
            )
        return validate_suite(args.quiet)
    except RegistryError as exc:
        print(f"ERROR {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
