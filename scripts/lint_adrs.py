#!/usr/bin/env python3
"""Validate the phase-0 LifeChronicle architecture decision records.

The ADR README is the human-readable index, while this checker enforces the
stable phase-0 shape declared there.  It intentionally uses only the Python
standard library so the governance gate can run before project dependencies
are installed.
"""

from __future__ import annotations

import datetime as dt
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from urllib.parse import unquote, urlsplit


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
ADR_ROOT = REPOSITORY_ROOT / "docs" / "adr"
ADR_README = ADR_ROOT / "README.md"

EXPECTED_IDS = tuple(f"ADR-{number:03d}" for number in range(1, 14))
ADR_FILENAME_RE = re.compile(r"^(ADR-\d{3})-[a-z0-9-]+\.md$")
ADR_ID_RE = re.compile(r"^ADR-\d{3}$")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
VALID_STATUSES = frozenset({"proposed", "accepted", "deprecated", "superseded"})
STATUS_SECTION_LABELS = {
    "proposed": ("proposed", "提议"),
    "accepted": ("accepted", "已接受"),
    "deprecated": ("deprecated", "已弃用", "弃用"),
    "superseded": ("superseded", "已取代", "被取代"),
}

FRONT_MATTER_FIELDS = (
    "adr",
    "title",
    "status",
    "date",
    "owners",
    "reviewers",
    "supersedes",
    "superseded_by",
    "related",
)
LIST_FIELDS = frozenset({"owners", "reviewers", "supersedes", "related"})
REQUIRED_SECTIONS = (
    "状态",
    "上下文",
    "决策",
    "备选方案",
    "后果",
    "迁移",
    "回滚",
    "测试",
    "退出条件",
)

INDEX_ROW_RE = re.compile(
    r"^\|\s*\[(ADR-\d{3})\]\(([^)]+)\)\s*\|.*\|\s*([a-z]+)\s*\|\s*$"
)
TOP_LEVEL_FIELD_RE = re.compile(r"^([a-z][a-z0-9_]*):(?:\s*(.*))?$")
LIST_ITEM_RE = re.compile(r"^  -\s+(.+?)\s*$")
H1_RE = re.compile(r"^#\s+(.+?)\s*$")
H2_RE = re.compile(r"^##\s+(.+?)\s*$")
FENCE_RE = re.compile(r"^[ \t]{0,3}(`{3,}|~{3,})(.*)$")
INLINE_LINK_RE = re.compile(r"!?\[[^\]\n]*\]\(([^)\n]+)\)")
REFERENCE_LINK_RE = re.compile(r"^[ \t]{0,3}\[[^\]\n]+\]:\s*(\S+)", re.MULTILINE)
HTML_COMMENT_RE = re.compile(r"<!--.*?-->", re.DOTALL)


@dataclass(frozen=True)
class AdrDocument:
    path: Path
    adr_id: str
    title: str
    status: str
    supersedes: tuple[str, ...]
    superseded_by: str | None
    metadata: dict[str, object]


def display_path(path: Path) -> str:
    """Return a deterministic repository-relative path for diagnostics."""

    try:
        return path.resolve().relative_to(REPOSITORY_ROOT.resolve()).as_posix()
    except (OSError, ValueError):
        return str(path)


def add_error(errors: list[str], path: Path, message: str) -> None:
    errors.append(f"{display_path(path)}: {message}")


def read_text(path: Path, errors: list[str]) -> str | None:
    try:
        return path.read_text(encoding="utf-8-sig")
    except (OSError, UnicodeError) as exc:
        add_error(errors, path, f"cannot read UTF-8 text: {exc}")
        return None


def decode_scalar(raw_value: str) -> str:
    value = raw_value.strip()
    if len(value) >= 2 and value[0] == value[-1] == '"':
        try:
            decoded = json.loads(value)
        except json.JSONDecodeError:
            return value
        return decoded if isinstance(decoded, str) else value
    if len(value) >= 2 and value[0] == value[-1] == "'":
        return value[1:-1].replace("''", "'")
    return value


def parse_front_matter(
    path: Path, text: str, errors: list[str]
) -> tuple[dict[str, object], list[str]] | None:
    lines = text.splitlines()
    if not lines or lines[0] != "---":
        add_error(errors, path, "must start with YAML Front Matter delimiter '---'")
        return None

    try:
        closing_index = lines.index("---", 1)
    except ValueError:
        add_error(errors, path, "YAML Front Matter has no closing '---' delimiter")
        return None

    raw_metadata = lines[1:closing_index]
    metadata: dict[str, object] = {}
    field_order: list[str] = []
    index = 0

    while index < len(raw_metadata):
        line = raw_metadata[index]
        match = TOP_LEVEL_FIELD_RE.fullmatch(line)
        if match is None:
            add_error(
                errors,
                path,
                f"Front Matter line {index + 2} is not a supported top-level field",
            )
            index += 1
            continue

        key, raw_value = match.groups()
        if key in metadata:
            add_error(errors, path, f"Front Matter field '{key}' is duplicated")
        field_order.append(key)

        if raw_value == "[]":
            value: object = []
        elif raw_value in {"null", "~"}:
            value = None
        elif raw_value:
            value = decode_scalar(raw_value)
        else:
            items: list[str] = []
            lookahead = index + 1
            while lookahead < len(raw_metadata):
                item_match = LIST_ITEM_RE.fullmatch(raw_metadata[lookahead])
                if item_match is None:
                    break
                items.append(decode_scalar(item_match.group(1)))
                lookahead += 1
            value = items
            index = lookahead - 1

        metadata[key] = value
        index += 1

    if tuple(field_order) != FRONT_MATTER_FIELDS:
        add_error(
            errors,
            path,
            "Front Matter fields must appear exactly once in this order: "
            + ", ".join(FRONT_MATTER_FIELDS),
        )

    return metadata, lines[closing_index + 1 :]


def visible_headings(body_lines: list[str]) -> list[tuple[int, int, str]]:
    """Return Markdown headings outside fenced code blocks."""

    headings: list[tuple[int, int, str]] = []
    fence_character: str | None = None
    fence_length = 0

    for line_index, line in enumerate(body_lines):
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

        h2_match = H2_RE.fullmatch(line)
        if h2_match is not None:
            headings.append((2, line_index, h2_match.group(1)))
            continue

        h1_match = H1_RE.fullmatch(line)
        if h1_match is not None:
            headings.append((1, line_index, h1_match.group(1)))

    return headings


def section_has_content(section_text: str) -> bool:
    content = HTML_COMMENT_RE.sub("", section_text).strip()
    if not content:
        return False
    normalized = re.sub(r"[\s`*_.:/-]+", "", content).casefold()
    return normalized not in {"na", "none", "todo", "tbd", "无", "待定"}


def validate_metadata(
    path: Path,
    filename_id: str,
    metadata: dict[str, object],
    errors: list[str],
) -> None:
    adr_id = metadata.get("adr")
    if adr_id != filename_id:
        add_error(
            errors,
            path,
            f"Front Matter adr must be {filename_id!r}, found {adr_id!r}",
        )

    title = metadata.get("title")
    if not isinstance(title, str) or not title.strip():
        add_error(errors, path, "Front Matter title must be a non-empty string")

    status = metadata.get("status")
    if status not in VALID_STATUSES:
        add_error(
            errors,
            path,
            "Front Matter status must be one of: "
            + ", ".join(sorted(VALID_STATUSES)),
        )

    date_value = metadata.get("date")
    if not isinstance(date_value, str) or DATE_RE.fullmatch(date_value) is None:
        add_error(errors, path, "Front Matter date must use YYYY-MM-DD")
    else:
        try:
            dt.date.fromisoformat(date_value)
        except ValueError:
            add_error(errors, path, f"Front Matter date is invalid: {date_value!r}")

    for field in LIST_FIELDS:
        value = metadata.get(field)
        if not isinstance(value, list):
            add_error(errors, path, f"Front Matter {field} must be a YAML list")
            continue
        if any(not isinstance(item, str) or not item.strip() for item in value):
            add_error(errors, path, f"Front Matter {field} contains an empty item")
        if len(value) != len(set(value)):
            add_error(errors, path, f"Front Matter {field} contains duplicate items")

    for field in ("owners", "reviewers"):
        value = metadata.get(field)
        if isinstance(value, list) and not value:
            add_error(errors, path, f"Front Matter {field} must not be empty")

    supersedes = metadata.get("supersedes")
    if isinstance(supersedes, list):
        for target in supersedes:
            if isinstance(target, str) and ADR_ID_RE.fullmatch(target) is None:
                add_error(errors, path, f"invalid supersedes ADR id: {target!r}")

    superseded_by = metadata.get("superseded_by")
    if superseded_by is not None and (
        not isinstance(superseded_by, str)
        or ADR_ID_RE.fullmatch(superseded_by) is None
    ):
        add_error(
            errors,
            path,
            "Front Matter superseded_by must be an ADR-NNN id or null",
        )

    if superseded_by is not None and status != "superseded":
        add_error(
            errors,
            path,
            "an ADR with superseded_by set must have status 'superseded'",
        )
    if status == "superseded" and superseded_by is None:
        add_error(
            errors,
            path,
            "an ADR with status 'superseded' must set superseded_by",
        )

    related = metadata.get("related")
    if isinstance(related, list):
        for raw_target in related:
            if not isinstance(raw_target, str):
                continue
            parsed = urlsplit(raw_target)
            if parsed.scheme or parsed.netloc or raw_target.startswith(("/", "\\")):
                add_error(
                    errors,
                    path,
                    f"related entry must be a repository-relative path: {raw_target!r}",
                )
                continue
            target = (path.parent / unquote(parsed.path)).resolve()
            try:
                target.relative_to(REPOSITORY_ROOT.resolve())
            except ValueError:
                add_error(
                    errors,
                    path,
                    f"related entry escapes the repository: {raw_target!r}",
                )
                continue
            if not target.exists():
                add_error(
                    errors,
                    path,
                    f"related entry does not exist: {raw_target!r}",
                )


def validate_body(
    path: Path,
    metadata: dict[str, object],
    body_lines: list[str],
    errors: list[str],
) -> None:
    headings = visible_headings(body_lines)
    h1_headings = [(line, title) for level, line, title in headings if level == 1]
    expected_h1 = f"{metadata.get('adr')}：{metadata.get('title')}"
    if len(h1_headings) != 1:
        add_error(errors, path, "must contain exactly one level-1 ADR title")
    elif h1_headings[0][1] != expected_h1:
        add_error(
            errors,
            path,
            f"level-1 title must be '# {expected_h1}'",
        )

    h2_headings = [(line, title) for level, line, title in headings if level == 2]
    actual_sections = tuple(title for _, title in h2_headings)
    if actual_sections != REQUIRED_SECTIONS:
        add_error(
            errors,
            path,
            "level-2 sections must appear exactly once in this order: "
            + ", ".join(REQUIRED_SECTIONS),
        )

    for section_index, (line_index, title) in enumerate(h2_headings):
        if title not in REQUIRED_SECTIONS:
            continue
        next_line = (
            h2_headings[section_index + 1][0]
            if section_index + 1 < len(h2_headings)
            else len(body_lines)
        )
        section_text = "\n".join(body_lines[line_index + 1 : next_line])
        if not section_has_content(section_text):
            add_error(errors, path, f"section '## {title}' must not be empty")

        if title == "状态":
            status = metadata.get("status")
            if isinstance(status, str) and not any(
                label.casefold() in section_text.casefold()
                for label in STATUS_SECTION_LABELS.get(status, ())
            ):
                add_error(
                    errors,
                    path,
                    "section '## 状态' must state the Front Matter status "
                    f"{status!r}",
                )

        if title in {"测试", "退出条件"}:
            normalized = HTML_COMMENT_RE.sub("", section_text).strip()
            unverifiable = re.search(
                r"(?:人工|手工)(?:确认|检查|验证).{0,12}(?:正常|通过)"
                r"|manual(?:ly)?\s+(?:confirm|check|verify)",
                normalized,
                re.IGNORECASE,
            )
            if unverifiable and len(normalized) < 120:
                add_error(
                    errors,
                    path,
                    f"section '## {title}' substitutes manual confirmation for evidence",
                )


def parse_index(
    readme_text: str, errors: list[str]
) -> dict[str, tuple[str, str]]:
    lines = readme_text.splitlines()
    in_index = False
    rows: dict[str, tuple[str, str]] = {}

    for line in lines:
        if line.strip() == "## ADR 索引":
            in_index = True
            continue
        if in_index and line.startswith("## "):
            break
        if not in_index:
            continue
        match = INDEX_ROW_RE.fullmatch(line)
        if match is None:
            continue
        adr_id, link_target, status = match.groups()
        if adr_id in rows:
            add_error(errors, ADR_README, f"ADR index duplicates {adr_id}")
        rows[adr_id] = (link_target, status)

    if tuple(rows) != EXPECTED_IDS:
        add_error(
            errors,
            ADR_README,
            "ADR index must declare exactly ADR-001 through ADR-013 in order",
        )

    for adr_id, (_, status) in rows.items():
        if status not in VALID_STATUSES:
            add_error(
                errors,
                ADR_README,
                f"{adr_id} index status is invalid: {status!r}",
            )

    return rows


def validate_readme_contract(readme_text: str, errors: list[str]) -> None:
    start_marker = "## 稳定文件结构"
    end_marker = "## 生命周期与修改规则"
    start = readme_text.find(start_marker)
    end = readme_text.find(end_marker)
    if start < 0 or end < 0 or end <= start:
        add_error(
            errors,
            ADR_README,
            "must declare the stable file structure before lifecycle rules",
        )
        return

    stable_section = readme_text[start:end]
    if "ADR-[0-9]{3}-[a-z0-9-]+.md" not in stable_section:
        add_error(
            errors,
            ADR_README,
            "stable structure must declare the ADR filename pattern",
        )

    declared_lines = stable_section.splitlines()
    for field in FRONT_MATTER_FIELDS:
        if declared_lines.count(field) != 1:
            add_error(
                errors,
                ADR_README,
                f"stable structure must declare Front Matter field {field!r} once",
            )
    if declared_lines.count("# ADR-NNN：标题") != 1:
        add_error(
            errors,
            ADR_README,
            "stable structure must declare the '# ADR-NNN：标题' heading once",
        )
    for section in REQUIRED_SECTIONS:
        heading = f"## {section}"
        if declared_lines.count(heading) != 1:
            add_error(
                errors,
                ADR_README,
                f"stable structure must declare required heading {heading!r} once",
            )
    for status in VALID_STATUSES:
        if f"`{status}`" not in stable_section:
            add_error(
                errors,
                ADR_README,
                f"stable structure must declare status {status!r}",
            )


def link_destination(raw_destination: str) -> str:
    destination = raw_destination.strip()
    if destination.startswith("<"):
        closing = destination.find(">")
        return destination[1:closing] if closing >= 0 else destination[1:]
    return destination.split(maxsplit=1)[0]


def visible_markdown_text(text: str) -> str:
    output: list[str] = []
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
            output.append("")
            continue
        if fence_match is not None:
            marker = fence_match.group(1)
            fence_character = marker[0]
            fence_length = len(marker)
            output.append("")
            continue
        output.append(line)
    return "\n".join(output)


def validate_internal_links(
    path: Path, text: str, errors: list[str]
) -> None:
    visible_text = visible_markdown_text(text)
    raw_destinations = [
        match.group(1) for match in INLINE_LINK_RE.finditer(visible_text)
    ]
    raw_destinations.extend(
        match.group(1) for match in REFERENCE_LINK_RE.finditer(visible_text)
    )

    for raw_destination in raw_destinations:
        destination = link_destination(raw_destination)
        if not destination or destination.startswith("#"):
            continue
        parsed = urlsplit(destination)
        if parsed.scheme or parsed.netloc:
            continue
        raw_path = unquote(parsed.path)
        if not raw_path:
            continue
        target = (
            REPOSITORY_ROOT / raw_path.lstrip("/\\")
            if raw_path.startswith(("/", "\\"))
            else path.parent / raw_path
        ).resolve()
        try:
            target.relative_to(REPOSITORY_ROOT.resolve())
        except ValueError:
            add_error(errors, path, f"link escapes the repository: {destination!r}")
            continue
        if not target.exists():
            add_error(errors, path, f"broken repository link: {destination!r}")


def validate_supersession_links(
    documents: dict[str, AdrDocument], errors: list[str]
) -> None:
    for document in documents.values():
        if document.superseded_by is not None:
            successor = documents.get(document.superseded_by)
            if successor is None:
                add_error(
                    errors,
                    document.path,
                    f"superseded_by target does not exist: {document.superseded_by}",
                )
            elif document.adr_id not in successor.supersedes:
                add_error(
                    errors,
                    document.path,
                    f"{document.superseded_by} must list {document.adr_id} in supersedes",
                )

        for predecessor_id in document.supersedes:
            predecessor = documents.get(predecessor_id)
            if predecessor is None:
                add_error(
                    errors,
                    document.path,
                    f"supersedes target does not exist: {predecessor_id}",
                )
            elif predecessor.superseded_by != document.adr_id:
                add_error(
                    errors,
                    document.path,
                    f"{predecessor_id} must set superseded_by to {document.adr_id}",
                )


def main() -> int:
    errors: list[str] = []
    readme_text = read_text(ADR_README, errors)
    if readme_text is None:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1

    index = parse_index(readme_text, errors)
    validate_readme_contract(readme_text, errors)
    validate_internal_links(ADR_README, readme_text, errors)

    if not ADR_ROOT.is_dir():
        add_error(errors, ADR_ROOT, "ADR directory does not exist")
        adr_paths: list[Path] = []
    else:
        adr_paths = sorted(
            path
            for path in ADR_ROOT.glob("ADR-*.md")
            if path.name != ADR_README.name
        )

    filenames: dict[str, Path] = {}
    for path in adr_paths:
        match = ADR_FILENAME_RE.fullmatch(path.name)
        if match is None:
            add_error(
                errors,
                path,
                "filename must match ADR-[0-9]{3}-[a-z0-9-]+.md",
            )
            continue
        adr_id = match.group(1)
        if adr_id in filenames:
            add_error(errors, path, f"duplicate ADR number {adr_id}")
        filenames[adr_id] = path

    if tuple(sorted(filenames)) != EXPECTED_IDS or len(adr_paths) != 13:
        add_error(
            errors,
            ADR_ROOT,
            "must contain exactly one ADR file for each ADR-001 through ADR-013",
        )

    documents: dict[str, AdrDocument] = {}
    for adr_id in EXPECTED_IDS:
        path = filenames.get(adr_id)
        if path is None:
            continue
        text = read_text(path, errors)
        if text is None:
            continue
        parsed = parse_front_matter(path, text, errors)
        if parsed is None:
            continue
        metadata, body_lines = parsed
        validate_metadata(path, adr_id, metadata, errors)
        validate_body(path, metadata, body_lines, errors)
        validate_internal_links(path, text, errors)

        title = metadata.get("title")
        status = metadata.get("status")
        supersedes_value = metadata.get("supersedes")
        superseded_by_value = metadata.get("superseded_by")
        if (
            isinstance(title, str)
            and isinstance(status, str)
            and isinstance(supersedes_value, list)
            and all(isinstance(item, str) for item in supersedes_value)
            and (
                superseded_by_value is None
                or isinstance(superseded_by_value, str)
            )
        ):
            documents[adr_id] = AdrDocument(
                path=path,
                adr_id=adr_id,
                title=title,
                status=status,
                supersedes=tuple(supersedes_value),
                superseded_by=superseded_by_value,
                metadata=metadata,
            )

        index_entry = index.get(adr_id)
        if index_entry is not None:
            link_target, index_status = index_entry
            if Path(unquote(urlsplit(link_target).path)).name != path.name:
                add_error(
                    errors,
                    ADR_README,
                    f"{adr_id} index link must target {path.name}",
                )
            if isinstance(status, str) and index_status != status:
                add_error(
                    errors,
                    ADR_README,
                    f"{adr_id} index status {index_status!r} does not match "
                    f"Front Matter status {status!r}",
                )

    validate_supersession_links(documents, errors)

    if errors:
        for error in sorted(set(errors)):
            print(f"ERROR: {error}", file=sys.stderr)
        print(
            f"ADR lint failed with {len(set(errors))} issue(s).",
            file=sys.stderr,
        )
        return 1

    print(
        "ADR lint passed: README index and ADR-001 through ADR-013 are valid "
        f"({len(documents)} ADRs)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
