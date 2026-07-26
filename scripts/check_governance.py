#!/usr/bin/env python3
"""Check the phase-0 repository and open-source governance baseline.

This is a deliberately dependency-free structural gate.  It does not attempt
to give legal advice or replace a license scanner; it prevents the repository
from silently losing the governance artifacts and owner wiring required by
R0-01.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]

LICENSE_PATH = REPOSITORY_ROOT / "LICENSE"
CONTRIBUTING_PATH = REPOSITORY_ROOT / "CONTRIBUTING.md"
CODE_OF_CONDUCT_PATH = REPOSITORY_ROOT / "CODE_OF_CONDUCT.md"
SECURITY_PATH = REPOSITORY_ROOT / "SECURITY.md"
GOVERNANCE_PATH = REPOSITORY_ROOT / "GOVERNANCE.md"
THIRD_PARTY_PATH = REPOSITORY_ROOT / "THIRD_PARTY_SOURCES.md"
CODEOWNERS_PATH = REPOSITORY_ROOT / ".github" / "CODEOWNERS"
PR_TEMPLATE_PATH = REPOSITORY_ROOT / ".github" / "PULL_REQUEST_TEMPLATE.md"
ISSUE_TEMPLATE_ROOT = REPOSITORY_ROOT / ".github" / "ISSUE_TEMPLATE"
COMMIT_CONVENTION_PATH = (
    REPOSITORY_ROOT / "docs" / "governance" / "commit-convention.md"
)
RESEARCH_TEMPLATE_PATH = (
    REPOSITORY_ROOT
    / "docs"
    / "governance"
    / "templates"
    / "reference-research-record.md"
)
SOURCE_TEMPLATE_PATH = (
    REPOSITORY_ROOT
    / "docs"
    / "governance"
    / "templates"
    / "third-party-source-record.md"
)

REQUIRED_FILES = (
    LICENSE_PATH,
    CONTRIBUTING_PATH,
    CODE_OF_CONDUCT_PATH,
    SECURITY_PATH,
    GOVERNANCE_PATH,
    THIRD_PARTY_PATH,
    CODEOWNERS_PATH,
    PR_TEMPLATE_PATH,
    ISSUE_TEMPLATE_ROOT / "config.yml",
    ISSUE_TEMPLATE_ROOT / "bug_report.yml",
    ISSUE_TEMPLATE_ROOT / "feature_request.yml",
    ISSUE_TEMPLATE_ROOT / "work_package.yml",
    COMMIT_CONVENTION_PATH,
    RESEARCH_TEMPLATE_PATH,
    SOURCE_TEMPLATE_PATH,
)

EMAIL_RE = re.compile(
    r"(?<![\w.+-])[\w.!#$%&'*+/=?^`{|}~-]+"
    r"@[\w](?:[\w.-]{0,251}[\w])?(?![\w.-])"
)
HANDLE_RE = re.compile(r"(?<![\w@])@[A-Za-z0-9](?:[A-Za-z0-9-]{0,38})")
BOOTSTRAP_ROW_RE = re.compile(
    r"^\|\s*Bootstrap maintainer\s*\|\s*(.*?)\s*\|",
    re.IGNORECASE | re.MULTILINE,
)
PLACEHOLDER_RE = re.compile(
    r"(?:\b(?:todo|tbd|placeholder)\b|待定|示例|example\.(?:com|org)|<[^>]+>)",
    re.IGNORECASE,
)

PROTECTED_CODEOWNER_PATTERNS = frozenset(
    {
        "/LICENSE",
        "/CONTRIBUTING.md",
        "/CODE_OF_CONDUCT.md",
        "/SECURITY.md",
        "/GOVERNANCE.md",
        "/THIRD_PARTY_SOURCES.md",
        "/.github/",
        "/docs/contract/",
        "/docs/protocol/",
        "/docs/architecture/",
        "/docs/operations/",
        "/proto/",
        "/registry/",
    }
)


def display_path(path: Path) -> str:
    try:
        return path.resolve().relative_to(REPOSITORY_ROOT.resolve()).as_posix()
    except (OSError, ValueError):
        return str(path)


def add_error(errors: list[str], path: Path, message: str) -> None:
    errors.append(f"{display_path(path)}: {message}")


def read_required_files(errors: list[str]) -> dict[Path, str]:
    contents: dict[Path, str] = {}
    for path in REQUIRED_FILES:
        if not path.is_file():
            add_error(errors, path, "required governance artifact is missing")
            continue
        try:
            text = path.read_text(encoding="utf-8-sig")
        except (OSError, UnicodeError) as exc:
            add_error(errors, path, f"cannot read UTF-8 text: {exc}")
            continue
        if not text.strip():
            add_error(errors, path, "required governance artifact is empty")
            continue
        contents[path] = text
    return contents


def require_signals(
    errors: list[str],
    path: Path,
    text: str,
    requirements: tuple[tuple[str, tuple[str, ...]], ...],
) -> None:
    folded = text.casefold()
    for description, alternatives in requirements:
        if not any(alternative.casefold() in folded for alternative in alternatives):
            add_error(errors, path, f"missing required {description}")


def validate_mit_license(errors: list[str], contents: dict[Path, str]) -> None:
    text = contents.get(LICENSE_PATH)
    if text is None:
        return
    require_signals(
        errors,
        LICENSE_PATH,
        text,
        (
            ("MIT title", ("MIT License",)),
            (
                "MIT permission grant",
                ("Permission is hereby granted, free of charge",),
            ),
            (
                "copyright and permission notice condition",
                (
                    "The above copyright notice and this permission notice "
                    "shall be included",
                ),
            ),
            (
                "MIT warranty disclaimer",
                ('THE SOFTWARE IS PROVIDED "AS IS"',),
            ),
            ("copyright holder and year", ("Copyright (c)",)),
        ),
    )


def extract_owner_identifiers(owner_cell: str) -> set[str]:
    plain_cell = owner_cell.replace("`", "")
    identifiers = set(EMAIL_RE.findall(plain_cell))
    identifiers.update(HANDLE_RE.findall(plain_cell))
    return identifiers


def parse_codeowners(
    text: str, errors: list[str]
) -> dict[str, tuple[str, ...]]:
    entries: dict[str, tuple[str, ...]] = {}
    for line_number, raw_line in enumerate(text.splitlines(), start=1):
        stripped = raw_line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        parts = stripped.split()
        if len(parts) < 2:
            add_error(
                errors,
                CODEOWNERS_PATH,
                f"line {line_number} has no owner",
            )
            continue
        pattern, *owners = parts
        if pattern in entries:
            add_error(
                errors,
                CODEOWNERS_PATH,
                f"pattern {pattern!r} is duplicated",
            )
        for owner in owners:
            if (
                EMAIL_RE.fullmatch(owner) is None
                and HANDLE_RE.fullmatch(owner) is None
            ):
                add_error(
                    errors,
                    CODEOWNERS_PATH,
                    f"line {line_number} has invalid owner token {owner!r}",
                )
            if PLACEHOLDER_RE.search(owner):
                add_error(
                    errors,
                    CODEOWNERS_PATH,
                    f"line {line_number} uses placeholder owner {owner!r}",
                )
        entries[pattern] = tuple(owners)
    return entries


def validate_bootstrap_owner(
    errors: list[str], contents: dict[Path, str]
) -> None:
    governance = contents.get(GOVERNANCE_PATH)
    codeowners_text = contents.get(CODEOWNERS_PATH)
    security = contents.get(SECURITY_PATH)
    if governance is None:
        return

    match = BOOTSTRAP_ROW_RE.search(governance)
    if match is None:
        add_error(
            errors,
            GOVERNANCE_PATH,
            "current-role table must register a Bootstrap maintainer",
        )
        bootstrap_identifiers: set[str] = set()
    else:
        owner_cell = match.group(1).strip()
        bootstrap_identifiers = extract_owner_identifiers(owner_cell)
        if PLACEHOLDER_RE.search(owner_cell):
            add_error(
                errors,
                GOVERNANCE_PATH,
                "Bootstrap maintainer must not be a placeholder",
            )
        if not bootstrap_identifiers:
            add_error(
                errors,
                GOVERNANCE_PATH,
                "Bootstrap maintainer must include an email or @account "
                "usable by CODEOWNERS",
            )

    require_signals(
        errors,
        GOVERNANCE_PATH,
        governance,
        (
            ("bootstrap mode declaration", ("当前模式：bootstrap", "current mode: bootstrap")),
            ("MIT project-license declaration", ("项目许可证：MIT", "project license: mit")),
            ("copyright-holder role", ("Copyright holder",)),
            ("release-owner role", ("Release owner",)),
            ("security-contact role", ("Security contact",)),
            ("multi-maintainer transition", ("转入多人治理", "multi-maintainer")),
            ("third-party governance", ("第三方与研究治理", "third-party")),
            ("high-risk restriction", ("高风险", "high-risk")),
        ),
    )

    entries: dict[str, tuple[str, ...]] = {}
    if codeowners_text is not None:
        entries = parse_codeowners(codeowners_text, errors)
        default_owners = set(entries.get("*", ()))
        if not default_owners:
            add_error(
                errors,
                CODEOWNERS_PATH,
                "must contain a repository-wide '*' owner rule",
            )
        elif bootstrap_identifiers and default_owners.isdisjoint(
            bootstrap_identifiers
        ):
            add_error(
                errors,
                CODEOWNERS_PATH,
                "repository-wide owners must include the registered "
                "Bootstrap maintainer",
            )

        missing_patterns = sorted(PROTECTED_CODEOWNER_PATTERNS - entries.keys())
        if missing_patterns:
            add_error(
                errors,
                CODEOWNERS_PATH,
                "missing explicit ownership for protected paths: "
                + ", ".join(missing_patterns),
            )

        for pattern in PROTECTED_CODEOWNER_PATTERNS:
            owners = set(entries.get(pattern, ()))
            if owners and bootstrap_identifiers and owners.isdisjoint(
                bootstrap_identifiers
            ):
                add_error(
                    errors,
                    CODEOWNERS_PATH,
                    f"{pattern} owners do not include the registered "
                    "Bootstrap maintainer",
                )

    if security is not None and bootstrap_identifiers:
        if not any(identifier in security for identifier in bootstrap_identifiers):
            add_error(
                errors,
                SECURITY_PATH,
                "private reporting channel must identify the registered "
                "Bootstrap maintainer contact",
            )


def validate_policy_documents(
    errors: list[str], contents: dict[Path, str]
) -> None:
    policies: dict[Path, tuple[tuple[str, tuple[str, ...]], ...]] = {
        CONTRIBUTING_PATH: (
            ("MIT contribution-license relationship", ("MIT License",)),
            ("code-of-conduct link", ("CODE_OF_CONDUCT.md",)),
            ("security-policy link", ("SECURITY.md",)),
            ("third-party source register link", ("THIRD_PARTY_SOURCES.md",)),
            ("pull-request rules", ("Pull Request",)),
            ("verification evidence rules", ("检查与证据", "verification evidence")),
            ("bootstrap review boundary", ("bootstrap",)),
        ),
        CODE_OF_CONDUCT_PATH: (
            ("community commitment", ("我们的承诺", "our pledge")),
            ("expected behavior", ("期望行为", "expected behavior")),
            ("unacceptable behavior", ("不可接受行为", "unacceptable behavior")),
            ("scope", ("适用范围", "scope")),
            ("private report process", ("报告与处理", "reporting")),
            ("enforcement process", ("执行原则", "enforcement")),
            ("anti-retaliation rule", ("反报复", "anti-retaliation")),
        ),
        SECURITY_PATH: (
            ("supported-version policy", ("支持范围", "supported versions")),
            ("private vulnerability reporting", ("私下报告漏洞", "reporting a vulnerability")),
            ("response target", ("响应目标", "response target")),
            ("safe-research boundary", ("安全研究边界", "safe harbor")),
            ("fix and disclosure process", ("修复与披露", "disclosure")),
        ),
        THIRD_PARTY_PATH: (
            ("MIT first-party declaration", ("自有内容许可证：MIT",)),
            ("research-only status", ("RESEARCH_ONLY",)),
            ("candidate status", ("CANDIDATE",)),
            ("approved-dependency status", ("APPROVED_DEPENDENCY",)),
            ("vendored status", ("VENDORED",)),
            ("rejected status", ("REJECTED",)),
            ("removed status", ("REMOVED",)),
            ("version or commit provenance", ("精确版本", "commit")),
            ("license obligations", ("许可证", "license")),
            ("clean-room rules", ("Clean-room",)),
            ("replacement or exit review", ("退出", "exit")),
            ("versioned research records", ("RES-001",)),
        ),
        COMMIT_CONVENTION_PATH: (
            ("commit title format", ("type(scope): summary",)),
            ("breaking-change notation", ("BREAKING CHANGE:",)),
            ("branch convention", ("分支", "branch")),
            ("verification evidence", ("验证", "verification")),
        ),
    }

    for path, requirements in policies.items():
        text = contents.get(path)
        if text is not None:
            require_signals(errors, path, text, requirements)

    security = contents.get(SECURITY_PATH)
    if security is not None and EMAIL_RE.search(security) is None:
        add_error(
            errors,
            SECURITY_PATH,
            "private vulnerability reporting must provide an email contact",
        )


def validate_templates(
    errors: list[str], contents: dict[Path, str]
) -> None:
    template_requirements: dict[
        Path, tuple[tuple[str, tuple[str, ...]], ...]
    ] = {
        PR_TEMPLATE_PATH: (
            ("work-package association", ("Issue / 工作包 ID",)),
            ("result and scope", ("结果与范围",)),
            ("contract and ADR review", ("契约与架构",)),
            ("security and privacy review", ("数据、安全与隐私",)),
            ("third-party review", ("第三方与来源",)),
            ("migration and rollback", ("兼容、迁移与回滚",)),
            ("verification evidence", ("验证证据",)),
            ("remaining risk", ("已知限制与剩余风险",)),
            ("bootstrap self-review", ("Bootstrap 自审",)),
        ),
        ISSUE_TEMPLATE_ROOT / "bug_report.yml": (
            ("form name", ("name:",)),
            ("form body", ("body:",)),
            ("security redirect", ("SECURITY.md",)),
            ("reproduction field", ("id: reproduce",)),
            ("impact field", ("id: impact",)),
            ("rollback field", ("回滚", "rollback")),
            ("required validation", ("required: true",)),
        ),
        ISSUE_TEMPLATE_ROOT / "feature_request.yml": (
            ("form name", ("name:",)),
            ("form body", ("body:",)),
            ("scope field", ("id: scope",)),
            ("contract field", ("id: contracts",)),
            ("privacy field", ("id: privacy",)),
            ("third-party field", ("id: third_party",)),
            ("acceptance field", ("id: acceptance",)),
            ("required validation", ("required: true",)),
        ),
        ISSUE_TEMPLATE_ROOT / "work_package.yml": (
            ("form name", ("name:",)),
            ("form body", ("body:",)),
            ("owner field", ("id: owner",)),
            ("authority field", ("id: authority",)),
            ("privacy field", ("id: privacy",)),
            ("dependency field", ("id: dependencies",)),
            ("migration field", ("id: migration",)),
            ("verification field", ("id: verification",)),
            ("ready checklist", ("id: ready",)),
            ("required validation", ("required: true",)),
        ),
        ISSUE_TEMPLATE_ROOT / "config.yml": (
            ("disabled blank issues", ("blank_issues_enabled: false",)),
            ("contact link declaration", ("contact_links:",)),
        ),
        RESEARCH_TEMPLATE_PATH: (
            ("research scope", ("研究问题与范围",)),
            ("official sources", ("官方来源",)),
            ("exact version anchor", ("可复现版本锚点",)),
            ("license evidence", ("许可证与来源",)),
            ("adopt boundary", ("可借鉴",)),
            ("reject boundary", ("不借鉴",)),
            ("clean-room boundary", ("Clean-room 边界",)),
            ("risk register", ("风险与未决项",)),
            ("affected module", ("LifeChronicle 相关模块",)),
        ),
        SOURCE_TEMPLATE_PATH: (
            ("source identity", ("身份与来源",)),
            ("exact version", ("精确 tag/commit/version",)),
            ("artifact hash", ("artifact hash",)),
            ("license evidence", ("许可证证据",)),
            ("obligations", ("义务与兼容性",)),
            ("clean-room boundary", ("Clean-room",)),
            ("security and maintenance", ("安全与维护",)),
            ("exit plan", ("退出计划",)),
            ("decision", ("决策",)),
        ),
    }

    for path, requirements in template_requirements.items():
        text = contents.get(path)
        if text is not None:
            require_signals(errors, path, text, requirements)


def main() -> int:
    errors: list[str] = []
    contents = read_required_files(errors)

    validate_mit_license(errors, contents)
    validate_bootstrap_owner(errors, contents)
    validate_policy_documents(errors, contents)
    validate_templates(errors, contents)

    if errors:
        unique_errors = sorted(set(errors))
        for error in unique_errors:
            print(f"ERROR: {error}", file=sys.stderr)
        print(
            f"Governance check failed with {len(unique_errors)} issue(s).",
            file=sys.stderr,
        )
        return 1

    print(
        "Governance check passed: MIT license, policies, owners, templates, "
        "and third-party provenance baseline are present."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
