---
title: Roadmap
description: Observable milestones for the Rukn language and toolchain.
---

# Roadmap

Milestones are evidence-based. Dates are not promises, and a feature is not complete until tests and documentation cover it.

## M0 — Repository foundation

- [x] Rust crate, CLI entry point and initial test suite.
- [x] Arabic keywords and Unicode identifiers.
- [x] Initial grammar and documentation in Arabic, French and English.
- [ ] CI passes formatting, lint, unit tests and example output checks.
- [ ] Publish a versioned design decision process.

## M1 — Language core

- [ ] Add source-size, token-count and nesting limits.
- [ ] Add parser fuzzing and a corpus of invalid programs.
- [ ] Define Unicode normalization and identifier-confusable handling.
- [ ] Add إذا / وإلا, blocks and lexical scopes after semantics are reviewed.
- [ ] Define integer versus floating-point semantics and numeric overflow policy.
- [ ] Stabilize structured diagnostics and machine-readable error codes.

## M2 — Programming abstractions

- [ ] Functions and return values.
- [ ] Collections and iteration.
- [ ] Modules, imports and a dependency model.
- [ ] Explicit error handling and resource limits.
- [ ] A standard library with documented stability levels.

## M3 — Tooling

- [ ] Formatter and syntax-aware diagnostics.
- [ ] Language Server Protocol implementation.
- [ ] Editor grammar and syntax highlighting.
- [ ] Package manifest and reproducible dependency resolution.
- [ ] Benchmark suite and regression corpus.

## M4 — Compiler and bootstrap research

- [ ] Specify a stable intermediate representation.
- [ ] Compare tree-walk, bytecode VM and native back ends using measured workloads.
- [ ] Define an ABI and foreign-function interface before promising C interoperability.
- [ ] Build a compiler in Rust that emits a documented target.
- [ ] Establish reproducible bootstrap tests before considering a compiler written in Rukn.

## Release policy

A 0.1 release requires passing CI and a reviewed language specification. A 1.0 release additionally requires a compatibility policy, stable module semantics, migration policy, security review, documented limits, and a release/maintenance process. Self-hosting is not a 1.0 requirement unless the project later decides otherwise.
