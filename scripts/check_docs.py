#!/usr/bin/env python3
"""Run dependency-free structural checks over repository Markdown documents."""

from __future__ import annotations

import re
import sys
import unicodedata
from dataclasses import dataclass
from pathlib import Path
from urllib.parse import unquote, urlsplit


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
STATUS_DOCUMENTS = (
    REPOSITORY_ROOT / "README.md",
    REPOSITORY_ROOT / "docs" / "contract" / "project-contract.md",
)

EXCLUDED_DIRECTORIES = frozenset(
    {
        ".git",
        ".tools",
        ".venv",
        "node_modules",
        "vendor",
        "dist",
        "build",
        "target",
    }
)

FENCE_RE = re.compile(r"^[ \t]{0,3}(`{3,}|~{3,})(.*)$")
TRAILING_WHITESPACE_RE = re.compile(r"[ \t]+$")
INLINE_LINK_RE = re.compile(r"!?\[[^\]\n]*\]\(([^)\n]+)\)")
REFERENCE_DEFINITION_RE = re.compile(
    r"^[ \t]{0,3}\[([^\]\n]+)\]:\s*(\S.*)$"
)
INLINE_CODE_RE = re.compile(r"(`+)(.+?)\1")
HEADING_RE = re.compile(r"^[ \t]{0,3}(#{1,6})[ \t]+(.+?)[ \t]*#*[ \t]*$")
EXPLICIT_ANCHOR_RE = re.compile(
    r"<a\s+(?:[^>]*?\s)?(?:id|name)=[\"']([^\"']+)[\"'][^>]*>",
    re.IGNORECASE,
)

CURRENT_MARKER_RE = re.compile(
    r"(?:当前|目前|现阶段|本候选稿(?:快照)?|当前仓库|仓库当前|仓库)",
    re.IGNORECASE,
)
ABSENCE_MARKER_RE = re.compile(
    r"(?:尚无|尚未|还没有|不存在|缺少|缺失|"
    r"未(?:创建|建立|提交|包含|提供|选择|登记|实现))",
    re.IGNORECASE,
)
LICENSE_UNRESOLVED_RE = re.compile(
    r"(?:仓库)?许可证(?:仍然?|尚)?未决"
    r"|尚未选择并提交.{0,80}(?:`?LICENSE`?|许可证)"
    r"|本契约不预选.{0,100}(?:MIT|许可证)",
    re.IGNORECASE | re.DOTALL,
)


@dataclass(frozen=True)
class Link:
    line_number: int
    destination: str


def display_path(path: Path) -> str:
    try:
        return path.resolve().relative_to(REPOSITORY_ROOT.resolve()).as_posix()
    except (OSError, ValueError):
        return str(path)


def add_error(
    errors: list[str],
    path: Path,
    message: str,
    line_number: int | None = None,
) -> None:
    location = display_path(path)
    if line_number is not None:
        location = f"{location}:{line_number}"
    errors.append(f"{location}: {message}")


def markdown_files() -> list[Path]:
    files: list[Path] = []
    for path in REPOSITORY_ROOT.rglob("*.md"):
        try:
            relative_parts = path.relative_to(REPOSITORY_ROOT).parts
        except ValueError:
            continue
        if any(part in EXCLUDED_DIRECTORIES for part in relative_parts[:-1]):
            continue
        if path.is_file():
            files.append(path)
    return sorted(files, key=lambda item: item.as_posix())


def read_utf8(path: Path, errors: list[str]) -> str | None:
    try:
        return path.read_text(encoding="utf-8-sig")
    except (OSError, UnicodeError) as exc:
        add_error(errors, path, f"cannot read UTF-8 text: {exc}")
        return None


def validate_trailing_whitespace(
    path: Path, text: str, errors: list[str]
) -> None:
    for line_number, line in enumerate(text.splitlines(), start=1):
        if TRAILING_WHITESPACE_RE.search(line):
            add_error(errors, path, "trailing whitespace", line_number)


def validate_fences(path: Path, text: str, errors: list[str]) -> None:
    fence_character: str | None = None
    fence_length = 0
    opening_line = 0

    for line_number, line in enumerate(text.splitlines(), start=1):
        match = FENCE_RE.match(line)
        if match is None:
            continue
        marker, remainder = match.groups()

        if fence_character is None:
            fence_character = marker[0]
            fence_length = len(marker)
            opening_line = line_number
            continue

        if (
            marker[0] == fence_character
            and len(marker) >= fence_length
            and not remainder.strip()
        ):
            fence_character = None
            fence_length = 0
            opening_line = 0

    if fence_character is not None:
        add_error(
            errors,
            path,
            f"unclosed {fence_character * fence_length} code fence",
            opening_line,
        )


def links_outside_fences(text: str) -> list[Link]:
    links: list[Link] = []
    fence_character: str | None = None
    fence_length = 0

    for line_number, line in enumerate(text.splitlines(), start=1):
        fence_match = FENCE_RE.match(line)
        if fence_character is not None:
            if (
                fence_match is not None
                and fence_match.group(1)[0] == fence_character
                and len(fence_match.group(1)) >= fence_length
                and not fence_match.group(2).strip()
            ):
                fence_character = None
                fence_length = 0
            continue

        if fence_match is not None:
            marker = fence_match.group(1)
            fence_character = marker[0]
            fence_length = len(marker)
            continue

        visible_line = INLINE_CODE_RE.sub("", line)
        for match in INLINE_LINK_RE.finditer(visible_line):
            links.append(Link(line_number, match.group(1)))

        reference_match = REFERENCE_DEFINITION_RE.match(visible_line)
        if reference_match is not None and not reference_match.group(1).startswith("^"):
            links.append(Link(line_number, reference_match.group(2)))

    return links


def extract_destination(raw_destination: str) -> str:
    destination = raw_destination.strip()
    if destination.startswith("<"):
        closing = destination.find(">")
        return destination[1:closing] if closing >= 0 else destination[1:]
    return destination.split(maxsplit=1)[0]


def exact_case_exists(path: Path) -> bool:
    """Check path existence using repository casing even on Windows."""

    try:
        relative = path.relative_to(REPOSITORY_ROOT.resolve())
    except ValueError:
        return False

    current = REPOSITORY_ROOT.resolve()
    for component in relative.parts:
        try:
            names = {child.name for child in current.iterdir()}
        except OSError:
            return False
        if component not in names:
            return False
        current = current / component
    return current.exists()


def normalize_heading_text(raw_heading: str) -> str:
    text = re.sub(r"!\[([^\]]*)\]\([^)]*\)", r"\1", raw_heading)
    text = re.sub(r"\[([^\]]+)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"<[^>]+>", "", text)
    text = text.replace("`", "").replace("*", "").replace("_", "")
    return unicodedata.normalize("NFKC", text).strip().casefold()


def github_slug(raw_heading: str) -> str:
    normalized = normalize_heading_text(raw_heading)
    output: list[str] = []
    for character in normalized:
        category = unicodedata.category(character)
        if character in {"-", "_"}:
            output.append(character)
        elif character.isspace():
            output.append("-")
        elif category[0] in {"L", "M", "N"}:
            output.append(character)
    return "".join(output)


def document_anchors(path: Path, cache: dict[Path, set[str]]) -> set[str]:
    resolved = path.resolve()
    cached = cache.get(resolved)
    if cached is not None:
        return cached

    try:
        text = resolved.read_text(encoding="utf-8-sig")
    except (OSError, UnicodeError):
        cache[resolved] = set()
        return set()

    anchors: set[str] = set()
    slug_counts: dict[str, int] = {}
    fence_character: str | None = None
    fence_length = 0

    for line in text.splitlines():
        fence_match = FENCE_RE.match(line)
        if fence_character is not None:
            if (
                fence_match is not None
                and fence_match.group(1)[0] == fence_character
                and len(fence_match.group(1)) >= fence_length
                and not fence_match.group(2).strip()
            ):
                fence_character = None
                fence_length = 0
            continue
        if fence_match is not None:
            marker = fence_match.group(1)
            fence_character = marker[0]
            fence_length = len(marker)
            continue

        for explicit_match in EXPLICIT_ANCHOR_RE.finditer(line):
            anchors.add(unquote(explicit_match.group(1)).casefold())

        heading_match = HEADING_RE.match(line)
        if heading_match is None:
            continue
        base_slug = github_slug(heading_match.group(2))
        if not base_slug:
            continue
        occurrence = slug_counts.get(base_slug, 0)
        slug_counts[base_slug] = occurrence + 1
        slug = base_slug if occurrence == 0 else f"{base_slug}-{occurrence}"
        anchors.add(slug)

    cache[resolved] = anchors
    return anchors


def validate_links(
    path: Path,
    text: str,
    errors: list[str],
    anchor_cache: dict[Path, set[str]],
) -> None:
    for link in links_outside_fences(text):
        destination = extract_destination(link.destination)
        if not destination:
            add_error(errors, path, "empty Markdown link destination", link.line_number)
            continue

        parsed = urlsplit(destination)
        if parsed.scheme or parsed.netloc:
            continue

        raw_target = unquote(parsed.path)
        if raw_target:
            candidate = (
                REPOSITORY_ROOT / raw_target.lstrip("/\\")
                if raw_target.startswith(("/", "\\"))
                else path.parent / raw_target
            ).resolve()
        else:
            candidate = path.resolve()

        try:
            candidate.relative_to(REPOSITORY_ROOT.resolve())
        except ValueError:
            add_error(
                errors,
                path,
                f"relative link escapes the repository: {destination!r}",
                link.line_number,
            )
            continue

        if not candidate.exists():
            add_error(
                errors,
                path,
                f"broken relative link: {destination!r}",
                link.line_number,
            )
            continue
        if not exact_case_exists(candidate):
            add_error(
                errors,
                path,
                f"relative link has incorrect path casing: {destination!r}",
                link.line_number,
            )
            continue

        if parsed.fragment and candidate.suffix.casefold() in {".md", ".markdown"}:
            fragment = unquote(parsed.fragment).casefold()
            if fragment not in document_anchors(candidate, anchor_cache):
                add_error(
                    errors,
                    path,
                    f"relative link has unknown heading anchor: {destination!r}",
                    link.line_number,
                )


def paragraphs(text: str) -> list[tuple[int, str]]:
    result: list[tuple[int, str]] = []
    current: list[str] = []
    starting_line = 1
    block_start_re = re.compile(
        r"^[ \t]*(?:#{1,6}[ \t]+|[-+*][ \t]+|\d+[.)][ \t]+|\|)"
    )

    for line_number, line in enumerate(text.splitlines(), start=1):
        if line.strip():
            if current and block_start_re.match(line):
                result.append((starting_line, " ".join(current)))
                current = []
            if not current:
                starting_line = line_number
            current.append(line.strip())
            continue
        if current:
            result.append((starting_line, " ".join(current)))
            current = []
    if current:
        result.append((starting_line, " ".join(current)))
    return result


def artifact_presence() -> dict[str, tuple[bool, tuple[str, ...]]]:
    adr_files = list((REPOSITORY_ROOT / "docs" / "adr").glob("ADR-*.md"))
    proto_files = list((REPOSITORY_ROOT / "proto").rglob("*.proto"))
    registry_files = [
        *list((REPOSITORY_ROOT / "registry" / "streams").rglob("*.yaml")),
        *list((REPOSITORY_ROOT / "registry" / "metrics").rglob("*.yaml")),
    ]
    return {
        "LICENSE": (
            (REPOSITORY_ROOT / "LICENSE").is_file(),
            ("`license`", "license", "许可证"),
        ),
        "ADR": (
            len(adr_files) >= 12
            and (REPOSITORY_ROOT / "docs" / "adr" / "README.md").is_file(),
            ("正式 adr 集", "adr", "架构决策记录"),
        ),
        "Proto": (
            bool(proto_files),
            ("proto", "protobuf"),
        ),
        "Registry": (
            bool(registry_files),
            ("registry", "注册表"),
        ),
    }


def validate_current_state_claims(
    errors: list[str], loaded_documents: dict[Path, str]
) -> None:
    artifacts = artifact_presence()
    for path in STATUS_DOCUMENTS:
        text = loaded_documents.get(path)
        if text is None:
            if not path.is_file():
                add_error(errors, path, "status document is missing")
            continue

        for line_number, paragraph in paragraphs(text):
            folded = paragraph.casefold()
            for artifact_name, (is_present, aliases) in artifacts.items():
                if not is_present:
                    continue
                mentions_artifact = any(alias in folded for alias in aliases)
                absence_mentions_artifact = any(
                    re.search(
                        rf"{ABSENCE_MARKER_RE.pattern}.{{0,120}}{re.escape(alias)}"
                        rf"|{re.escape(alias)}.{{0,30}}{ABSENCE_MARKER_RE.pattern}",
                        folded,
                        re.IGNORECASE,
                    )
                    is not None
                    for alias in aliases
                )
                current_absence = (
                    CURRENT_MARKER_RE.search(paragraph) is not None
                    and absence_mentions_artifact
                )
                special_license_claim = (
                    artifact_name == "LICENSE"
                    and LICENSE_UNRESOLVED_RE.search(paragraph) is not None
                )
                if mentions_artifact and (current_absence or special_license_claim):
                    excerpt = re.sub(r"\s+", " ", paragraph)
                    if len(excerpt) > 180:
                        excerpt = excerpt[:177] + "..."
                    add_error(
                        errors,
                        path,
                        f"stale current-state claim says {artifact_name} is absent "
                        f"even though its artifact exists: {excerpt!r}",
                        line_number,
                    )


def main() -> int:
    errors: list[str] = []
    files = markdown_files()
    loaded_documents: dict[Path, str] = {}
    anchor_cache: dict[Path, set[str]] = {}

    for path in files:
        text = read_utf8(path, errors)
        if text is None:
            continue
        loaded_documents[path] = text
        validate_trailing_whitespace(path, text, errors)
        validate_fences(path, text, errors)
        validate_links(path, text, errors, anchor_cache)

    for path in STATUS_DOCUMENTS:
        if path not in loaded_documents and path.is_file():
            text = read_utf8(path, errors)
            if text is not None:
                loaded_documents[path] = text

    validate_current_state_claims(errors, loaded_documents)

    if errors:
        unique_errors = sorted(set(errors))
        for error in unique_errors:
            print(f"ERROR: {error}", file=sys.stderr)
        print(
            f"Documentation check failed with {len(unique_errors)} issue(s) "
            f"across {len(files)} Markdown file(s).",
            file=sys.stderr,
        )
        return 1

    print(
        f"Documentation check passed for {len(files)} Markdown file(s): "
        "relative links, anchors, code fences, whitespace, and status claims."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
