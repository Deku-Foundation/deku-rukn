---
title: Contributing
description: How to propose and review changes to Rukn.
---

# Contributing

Rukn is early-stage research. Small, testable changes are more useful than broad feature branches.

## Before coding

1. Search existing issues and pull requests.
2. For language syntax or semantics, open an issue or design proposal before implementation.
3. State the user-facing behavior and failure cases.
4. Avoid changing unrelated parser, documentation and tooling concerns in one patch.

## Every behavior change should include

- Tests for normal and invalid inputs.
- Updated Arabic documentation; add corresponding English/French updates when the affected page is translated.
- A short rationale for changes to syntax or semantics.
- No unsafe Rust unless a separately reviewed design proves it necessary (the current crate forbids unsafe code).
- CI passing on the submitted commit.

Use concrete examples that run against the current interpreter. Mark future ideas as proposals. Avoid claims such as production-ready, fully compatible, secure by design or self-hosting unless there is verifiable evidence and a release decision supporting them.

## Code review

Review correctness, language consistency, error behavior, tests, accessibility of documentation, and compatibility impact. A passing build is necessary, not sufficient.
