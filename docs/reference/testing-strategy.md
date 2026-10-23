---
title: Testing and conformance strategy
description: A practical validation plan for the Rukn lexer, parser, evaluator, CLI, and documentation.
---

# Testing and conformance strategy

**Status: engineering plan.** The existing unit tests and CI checks are the baseline; fuzzing, a formal conformance corpus, and benchmark gates are future work until they are implemented and run in CI.

## Test layers

| Layer | What to verify | Representative cases |
| --- | --- | --- |
| Lexer | Token kinds, numeral conversion, source locations, invalid characters | Arabic and ASCII semicolons, three digit families, unterminated strings |
| Parser | Precedence, associativity, required terminators, bounded nesting | `2 + 3 * 4`, missing `)`, deeply nested unary expressions |
| Evaluator | Value semantics and failure behavior | Mixed-type addition, undefined names, division by zero, non-finite arithmetic |
| CLI | File input, stdout/stderr, exit codes | Missing path, multiple arguments, invalid UTF-8 file contents |
| Documentation | Pages resolve and snippets remain accurate | Build all three locales; compare the featured example to CI output |
| Release | Repeatable checks and compatibility notes | Locked dependencies, draft spec review, release checklist |

## Invariants worth testing

- For every bounded UTF-8 source, the public interpreter API returns either a result or a language diagnostic; it must not panic on malformed source.
- Lexical and syntax errors report stable, useful source locations.
- Numeric literal normalization does not change the value represented by an accepted numeral.
- Operator precedence is consistent with the specification and tested for mixed operators.
- Invalid arithmetic produces a diagnostic instead of silently returning infinity or NaN.
- Source, token, and parser-nesting limits reject oversized input predictably.
- All code examples in the documentation are either copied from a real example file or explicitly labelled as illustrative.

A useful next step is a machine-readable corpus of source files paired with expected stdout, stderr category, and process exit code. This can be consumed by tests and later by alternate interpreter/compiler implementations.

## Fuzzing plan

The Rust ecosystem's [cargo-fuzz project](https://github.com/rust-fuzz/cargo-fuzz) and [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) provide a route to fuzzing Rust code with libFuzzer. A proposed Rukn fuzz target should call the public parse/run boundary using arbitrary UTF-8 input and check for panics, hangs within a controlled fuzzing run, and resource-limit bypasses.

Suggested separation:

1. Lexer target: arbitrary bytes and Unicode boundary cases.
2. Parser target: token streams or generated source with deep nesting and missing delimiters.
3. End-to-end target: call the public interpreter and treat structured diagnostics as valid outcomes.
4. Regression corpus: every minimized crash or incorrect result becomes a permanent test.

Fuzzing is not yet listed as a completed capability. It needs a separately reviewed harness, reproducible seeds, and an automated schedule before it is described as active CI coverage. `cargo-fuzz` requires nightly Rust and supported sanitizer tooling; it can initially run as a dedicated optional workflow.

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
