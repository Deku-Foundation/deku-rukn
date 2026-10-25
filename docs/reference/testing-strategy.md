---
title: Testing and conformance strategy
description: A practical validation plan for the Rukn lexer, parser, evaluator, CLI, and documentation.
---

# Testing and conformance strategy

**Status: engineering baseline with a bounded fuzz smoke target and a CLI conformance corpus.** Unit tests and the standard CI gates remain the primary correctness checks. The dependency-free integration runner executes the checked-in 37-case corpus against the real CLI, including stdout, exit status, and diagnostic-fragment checks. The corpus covers lexical shadowing, assignment, if/else, while-loop behavior, named function calls, function-value assignment/return, higher-order calls, function-reference equality, forward references, mutual recursion, arity/duplicate-definition errors, non-callable shadowing, undefined functions, malformed blocks, and resource limits. A separate workflow runs a 60-second libFuzzer campaign for pull requests that touch source or fuzz files, weekly, and by manual dispatch. Parser-focused fuzzing, longer campaigns, structured diagnostic assertions, and benchmark gates remain future work.

## Test layers

| Layer | What to verify | Representative cases |
| --- | --- | --- |
| Lexer and string decoding | Token kinds, numeral conversion, source locations, invalid characters, escape decoding | Arabic and ASCII semicolons, three digit families, unterminated strings, escaped delimiters, unknown escapes |
| Parser | Precedence, associativity, block syntax, required terminators, bounded nesting | `2 + 3 * 4`, missing `)`, missing `}`, deeply nested blocks and unary expressions |
| Evaluator | Value semantics and failure behavior | Mixed-type addition, undefined names/assignments, lexical shadowing, boolean conditions, bounded loops, named calls, return propagation, recursion and arity limits, division by zero, non-finite arithmetic, bounded runtime string copies |
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
- Source, token, parser/block-nesting, single-string, cumulative runtime string-allocation, and while-condition-evaluation limits reject excessive inputs predictably.
- Assignment updates the nearest existing lexical binding; block-local names do not leak into the enclosing scope.
- If and while conditions require booleans and never use implicit truthiness.
- Function arguments are evaluated left to right, arity is exact, recursive calls respect the active-call budget, and functions do not capture caller-block locals.
- Return flow must propagate through blocks and conditionals, and a top-level return must produce a diagnostic.
- The CLI reads no more than one byte beyond the source-size limit before rejecting an oversized file; it must not buffer an unbounded file before calling the interpreter.
- All code examples in the documentation are either copied from a real example file or explicitly labelled as illustrative.

The current corpus is the first executable compatibility contract for the CLI. Extend it whenever a language feature, diagnostic category, or runtime bound changes; keep the data format independent of the current interpreter so future VM and compiler implementations can consume the same expected behavior.

## Fuzzing plan

The [cargo-fuzz project](https://github.com/rust-fuzz/cargo-fuzz) and [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) document libFuzzer-based testing for Rust. Rukn now has an end-to-end target in `fuzz/fuzz_targets/run_source.rs`: it maps arbitrary bytes to valid UTF-8 with replacement for invalid sequences, then calls the public `rukn::run` API. A language diagnostic is a valid outcome; a panic or sanitizer finding is a failure. This target exercises the interpreter API, not the CLI's raw invalid-UTF-8 file path, which has separate unit tests.

Suggested separation:

1. Lexer target: arbitrary bytes and Unicode boundary cases.
2. Parser target: token streams or generated source with deep nesting and missing delimiters.
3. End-to-end target: call the public interpreter and treat structured diagnostics as valid outcomes.
4. Regression corpus: every minimized crash or incorrect result becomes a permanent test.

The dedicated `Fuzz smoke` workflow pins `cargo-fuzz 0.13.2` and runs two bounded targets on source/fuzz pull requests, weekly, and by manual dispatch: `run_source` mutates arbitrary UTF-8-decoded bytes, while `generated_program` selects from grammar-aware templates for arithmetic, branches, bounded loops, functions, higher-order calls, NFC identifiers, lists and Unicode indexing. The seed corpora include canonical-equivalent Arabic identifiers, a bidirectional control in executable source, list/higher-order programs, Unicode string indexing and the nested-list depth boundary. This improves feature-directed exploration but is not a full grammar generator or a security proof; longer campaigns and a dedicated parser target remain future work. Keep every minimized crash or incorrect result as a permanent regression test.

Recent relevant research: [Bootstrapping Fuzzers for Compilers of Low-Resource Language Dialects Using Language Models (2025)](https://arxiv.org/abs/2512.05887) studies grammar-aware fuzzer generation across MLIR dialects. It is a research direction, not a guarantee that model-generated tests are valid or that its coverage results transfer directly to Rukn. The immediate Rukn approach uses bounded hand-reviewed templates first, with conformance cases as the semantic oracle.

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
