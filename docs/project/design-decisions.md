---
title: Design decisions and research register
description: How Rukn records language decisions, proposals, alternatives, and evidence.
---

# Design decisions and research register

A programming language needs a decision record, not just a roadmap. This page tracks topics that can change syntax, portability, security, compatibility, and future compiler architecture.

**Status labels**
- **Implemented baseline:** visible in the current code and tests.
- **Open research:** alternatives are still being evaluated.
- **Deferred:** intentionally postponed until a prerequisite exists.
- **Accepted:** a decision has been reviewed and its consequences documented.
- **Superseded:** replaced by a newer decision record.

## Current register

| ID | Topic | Status | Evidence required to close |
| --- | --- | --- | --- |
| RUKN-DR-0001 | Unicode identifier profile and normalization | XID baseline implemented; normalization open | Pinned `unicode-ident`, Arabic-script conformance tests; normalization/confusable policy and migration analysis remain open |
| RUKN-DR-0002 | Confusables and bidirectional-control policy | Open research; rejected-character code points are shown | Threat model, strings/comments, terminal/editor examples, false-positive review |
| RUKN-DR-0003 | Tree-walk interpreter, bytecode VM, or native backend | Deferred | Stable semantics, common conformance suite, reproducible benchmarks |
| RUKN-DR-0004 | Bootstrap and self-hosting | Deferred | Stable parser, standard library, compiler tests, reproducible bootstrap |
| RUKN-DR-0005 | Arabic logical operators | Implemented baseline in draft 0.2; reserved words are pre-alpha compatibility choices | Boolean-only operands, left associativity, precedence and short-circuit regression tests |
| RUKN-DR-0006 | Numeric literal digit-family consistency | Implemented experimental default in draft 0.3; compatibility review remains open | Tests for all supported digit families, decimal separators, exact error locations and a migration note for mixed-family literals |
| RUKN-DR-0007 | Runtime string size and cumulative copy budget | Implemented guardrail in draft 0.4; conservative accounting, not a sandbox | Regression tests for repeated copies, concatenation and print output; review limits before 1.0 and for embedding |
| RUKN-DR-0008 | Minimum control-flow core and lexical scopes | Implemented experimental baseline in draft 0.5 | Conformance cases for scope/shadowing, assignment, boolean conditions, loop limits and malformed blocks; revisit language ergonomics before 1.0 |
| RUKN-DR-0009 | Named functions, return flow and recursion limits | Draft 0.7: top-level named functions can be referenced as values; closures remain deferred | Tests for exact arity, forward/mutual recursion, higher-order calls, return propagation, named-reference equality, no caller-local capture and call-depth limit |

These entries are research topics, not approvals to implement a specific option.

## Review process

1. Describe the problem and affected users.
2. List at least two realistic options, including keeping current behavior.
3. Document compatibility, security, implementation cost, tooling, and maintenance consequences.
4. Include small source examples and tests that distinguish the options.
5. Link external standards or research and identify assumptions that still need validation.
6. Open a reviewable GitHub issue or pull request; change an entry to **Accepted** only after review.
7. Add a migration note when a decision changes accepted source programs or observable behavior.

## Decision record template

```md
# RUKN-DR-NNNN — Short title

- Status: Open research | Accepted | Rejected | Superseded
- Date:
- Owners/reviewers:

## Context
What problem are we solving?

## Options
What credible alternatives were considered?

## Decision
What was selected, and what remains unresolved?

## Consequences
Compatibility, diagnostics, performance, security, maintenance.

## Evidence and tests
Standards, papers, reproducible benchmarks, and conformance cases.
```

## Priorities

The next high-value design review is the Unicode identifier profile described in [Unicode and identifier security](/reference/unicode-and-security). A compiler backend decision should wait until behavior can be compared against one conformance corpus. Self-hosting is a long-term research objective, not a shortcut around stable semantics, tests, and toolchain maintenance.
