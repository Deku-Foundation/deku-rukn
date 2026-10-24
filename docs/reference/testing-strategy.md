---
title: Testing and conformance strategy
description: A practical validation plan for the Rukn lexer, parser, evaluator, CLI, and documentation.
---

# Testing and conformance strategy

**Status: engineering baseline with a bounded fuzz smoke target.** Unit tests and the standard CI gates remain the primary correctness checks. A separate workflow runs a 60-second libFuzzer campaign for pull requests that touch source or fuzz files, weekly, and by manual dispatch. A machine-readable conformance corpus, longer fuzz campaigns, and benchmark gates remain future work.

## Test layers

| Layer | What to verify | Representative cases |
| --- | --- | --- |
| Lexer and string decoding | Token kinds, numeral conversion, source locations, invalid characters, escape decoding | Arabic and ASCII semicolons, three digit families, unterminated strings, escaped delimiters, unknown escapes |
| Parser | Precedence, associativity, required terminators, bounded nesting | `2 + 3 * 4`, missing `)`, deeply nested unary expressions |
| Evaluator | Value semantics and failure behavior | Mixed-type addition, undefined names, division by zero, non-finite arithmetic |
| CLI | Bounded file input, UTF-8 validation, stdout/stderr and exit codes | Missing path, multiple arguments, invalid UTF-8, over-limit source files |
| Documentation | Pages resolve and snippets remain accurate | Build all three locales; compare the featured example to CI output |
| Release | Repeatable checks and compatibility notes | Locked dependencies, draft spec review, release checklist |

String delimiter and escape behavior is documented in [String literal behavior](./string-literals.md). The implementation and documentation should change together when the escape grammar changes.

## Invariants worth testing

- For every bounded UTF-8 source, the public interpreter API returns either a result or a language diagnostic; it must not panic on malformed source.
- Lexical and syntax errors report stable, useful source locations.
- Numeric literal normalization does not change the value represented by an accepted numeral.
- Operator precedence is consistent with the specification and tested for mixed operators.
- Invalid arithmetic produces a diagnostic instead of silently returning infinity or NaN.
- Source, token, and parser-nesting limits reject oversized input predictably.
- The CLI reads no more than one byte beyond the source-size limit before rejecting an oversized file; it must not buffer an unbounded file before calling the interpreter.
- All code examples in the documentation are either copied from a real example file or explicitly labelled as illustrative.

A useful next step is a machine-readable corpus of source files paired with expected stdout, stderr category, and process exit code. This can be consumed by tests and later by alternate interpreter/compiler implementations.

## Fuzzing plan

The [cargo-fuzz project](https://github.com/rust-fuzz/cargo-fuzz) and [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) document libFuzzer-based testing for Rust. Rukn now has an end-to-end target in `fuzz/fuzz_targets/run_source.rs`: it maps arbitrary bytes to valid UTF-8 with replacement for invalid sequences, then calls the public `rukn::run` API. A language diagnostic is a valid outcome; a panic or sanitizer finding is a failure. This target exercises the interpreter API, not the CLI's raw invalid-UTF-8 file path, which has separate unit tests.

Suggested separation:

1. Lexer target: arbitrary bytes and Unicode boundary cases.
2. Parser target: token streams or generated source with deep nesting and missing delimiters.
3. End-to-end target: call the public interpreter and treat structured diagnostics as valid outcomes.
4. Regression corpus: every minimized crash or incorrect result becomes a permanent test.

The dedicated `Fuzz smoke` workflow pins `cargo-fuzz 0.13.2` and runs `cargo +nightly fuzz run run_source -- -max_total_time=60 -timeout=5 -max_len=16384`. It runs on source/fuzz pull requests, weekly, and by manual dispatch. The bounded run keeps CI cost predictable; it is not a security proof and does not replace longer campaigns. Keep every minimized crash or incorrect result as a permanent regression test, and expand toward a separate parser target and a machine-readable conformance corpus.

## Benchmarks and compiler research

Performance decisions should wait until the language has stable semantics and representative workloads. Benchmark lexing, parsing, execution, startup cost, memory use, and error-heavy inputs separately. Compare the tree-walk interpreter with a bytecode VM or native back end only after an equivalent conformance corpus exists; otherwise a faster implementation may simply do less work.

Do not adopt a backend on reputation alone. Publish the benchmark source, environment, toolchain version, correctness gate, and reproducible commands with each comparison.

## Checks used by the current repository

Run from the repository root:

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo run --locked -- examples/hello.rukn
```

Run the VitePress build from `docs/`:

```sh
npm ci
npm run build
```

## Further reading

- [Rust Compiler Development Guide: Fuzzing](https://rustc-dev-guide.rust-lang.org/fuzzing.html)
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)
- [VitePress deployment and build guide](https://vitepress.dev/guide/deploy)

The presence of a testing plan is not proof that every proposed test or gate already exists.
