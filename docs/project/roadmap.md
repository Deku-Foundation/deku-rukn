---
title: Roadmap
description: Observable milestones for the Rukn language and toolchain.
---

# Roadmap

Milestones are evidence-based, not time promises. A feature is complete only when implementation, tests and documentation agree.

## M0 — Repository foundation

- [x] Rust crate, CLI entry point and initial test suite.
- [x] Arabic keywords, Unicode identifiers and Arabic diagnostics.
- [x] Initial grammar and documentation in Arabic, French and English.
- [x] CI passes formatting, lint, unit tests and example-output checks.
- [x] Publish a reviewed design-decision process.

## M1 — Language core

- [x] Bound source size, token count and parser nesting; reject non-finite numeric results.
- [x] Add a bounded, scheduled end-to-end fuzz smoke target for the public interpreter API.
- [ ] Add parser-focused fuzzing.
- [x] Add a machine-readable CLI conformance corpus for valid and invalid programs, expected output/error fragment, and exit status.
- [x] Adopt a pinned Unicode XID_Start / XID_Continue lexical baseline with underscore tailoring and Arabic/Persian regression tests.
- [ ] Decide identifier normalization, mixed-script/confusable warnings and bidirectional-control diagnostics, with a compatibility corpus.
- [x] Add boolean conditionals, blocks, lexical scopes and assignment with regression tests.
- [x] Add a bounded while loop with explicit boolean conditions and a per-run condition-evaluation budget.
- [ ] Specify number semantics and error codes for tooling.
- [ ] Stabilize structured diagnostics for editor integration.

## M2 — Programming abstractions

- [ ] Functions, parameters and return values.
- [ ] Collections and iteration.
- [ ] Modules, imports and a dependency model.
- [ ] Explicit error propagation and semantics for additional loop forms.
- [ ] A standard library with documented stability levels.

## M3 — Tooling

- [ ] Formatter and syntax-aware diagnostics.
- [ ] Language Server Protocol implementation.
- [ ] Editor grammar and syntax highlighting.
- [ ] Package manifest and reproducible dependency resolution.
- [ ] Benchmark suite and regression corpus.

## M4 — Compiler and bootstrap research

See the [compiler bootstrap strategy](/project/compiler-bootstrap) for staged milestones and acceptance criteria.

- [x] Document the self-hosting goal, stage-zero constraint, and staged bootstrap validation plan.

- [ ] Specify a stable intermediate representation.
- [ ] Compare tree-walk, bytecode VM and native back ends with a shared correctness suite and reproducible benchmarks.
- [ ] Define an ABI and foreign-function interface before promising C interoperability.
- [ ] Build a Rust compiler targeting a documented platform.
- [ ] Establish reproducible bootstrap tests before considering a compiler in Rukn.

## Release policy

A 0.1 release requires passing CI and a reviewed language specification. A 1.0 release additionally requires stable module semantics, compatibility and migration policies, a security review, documented resource limits, and a maintenance process. Self-hosting is not a 1.0 requirement unless a reviewed decision makes it one.
