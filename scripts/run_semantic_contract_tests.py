#!/usr/bin/env python3
"""Run and completeness-check phase-0 semantic contract tests."""

from __future__ import annotations

import re
import sys
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
TEST_ROOT = REPOSITORY_ROOT / "contract-tests" / "python"
EXPECTED_IDS = {f"ES_C{number:03d}" for number in range(5, 17)}


def iter_tests(suite: unittest.TestSuite):
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            yield from iter_tests(item)
        else:
            yield item


def main() -> int:
    sys.dont_write_bytecode = True
    sys.path.insert(0, str(TEST_ROOT))
    loader = unittest.TestLoader()
    suite = loader.discover(
        str(TEST_ROOT),
        pattern="test_es_c005_c016.py",
        top_level_dir=str(TEST_ROOT),
    )

    test_names = [test.id() for test in iter_tests(suite)]
    observed_ids = {
        match.group(0)
        for name in test_names
        for match in [re.search(r"ES_C[0-9]{3}", name)]
        if match is not None
    }
    missing = sorted(EXPECTED_IDS - observed_ids)
    if missing:
        print(
            "Semantic contract suite is incomplete; missing: "
            + ", ".join(identifier.replace("_", "-") for identifier in missing),
            file=sys.stderr,
        )
        return 2

    result = unittest.TextTestRunner(verbosity=2).run(suite)
    if not result.wasSuccessful():
        return 1
    print(
        "Semantic contract coverage: "
        + ", ".join(
            identifier.replace("_", "-") for identifier in sorted(observed_ids)
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
