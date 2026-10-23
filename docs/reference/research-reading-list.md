---
title: Research reading list
description: Primary standards and engineering references for developing Rukn as a robust Arabic-first programming language.
---

# Research reading list

**Reviewed: 23 October 2026.** This is a curated starting point, not a claim that Rukn conforms to every linked standard. Prefer primary standards and official project documentation over summaries. Record the version and access date when a technical decision depends on a source.

## 1. Arabic text, identifiers and Unicode security

| Source | What it informs | Status in Rukn |
| --- | --- | --- |
| [UAX #31 — Unicode Identifiers and Syntax](https://www.unicode.org/reports/tr31/) | Identifier start/continue properties, profiles and normalization guidance | Normative reference; Rukn uses pinned `unicode-ident 1.0.27` for XID_Start/XID_Continue, with underscore tailoring |
| [UAX #15 — Unicode Normalization Forms](https://www.unicode.org/reports/tr15/) | NFC/NFKC and equivalence rules | Open design question; identifier normalization is not implemented |
| [UTS #39 — Unicode Security Mechanisms](https://www.unicode.org/reports/tr39/) | Confusable detection, mixed-script review and security profiles | Open design question; no dedicated confusable checker exists |
| [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/) | Correct display of mixed right-to-left and left-to-right text | Relevant to source display, diagnostics, documentation and future editor tooling |
| [UTS #55 — Unicode Source Code Handling](https://www.unicode.org/reports/tr55/) | Unicode usability and security recommendations specifically for source code | Recommended design reference before stabilizing identifiers and editor diagnostics |

The Unicode Consortium’s current Unicode 18.0 publications include UAX #31 revision 45 (1 September 2026), UAX #15 revision 58 (12 August 2026), UTS #39 revision 34 (27 August 2026), and UAX #9 revision 52 (1 September 2026). Use the “Latest Version” links above rather than copying tables from older articles. These version details were checked on 23 October 2026.

Rukn uses [unicode-ident 1.0.27](https://docs.rs/unicode-ident/1.0.27/unicode_ident/) for XID classification. The crate is intentionally pinned in `Cargo.lock`; changing it may alter which source identifiers are accepted, so treat dependency updates as language-specification changes and run the conformance tests. NFC/NFKC normalization is not implemented.

### Questions to resolve before a stable identifier specification

1. Which precise identifier properties are accepted, and which Unicode version is normative?
2. Are canonically equivalent spellings the same variable name? If so, where does normalization occur?
3. How should Arabic combining marks, Persian character variants, join controls and mixed Latin/Arabic identifiers behave?
4. Should suspicious confusables produce a warning, an error, or an opt-in linter result?
5. How are invisible bidirectional controls displayed in editors, terminal diagnostics and code review?

Use valid/invalid program pairs and a versioned conformance corpus. Do not silently normalise user strings as a side effect of normalising identifiers.

## 2. Language semantics and Rust implementation

- [The Rust Reference](https://doc.rust-lang.org/reference/) — authoritative reference for Rust syntax and language behaviour used by the implementation.
- [Rust 1.99.1 release announcement](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) — current stable release snapshot as of 23 October 2026; the CI workflow deliberately tracks stable and checks each commit.
- [The Rust Book](https://doc.rust-lang.org/book/) — accessible foundation for ownership, error handling, testing and project structure.
- [The Cargo Book](https://doc.rust-lang.org/cargo/) — dependency management, workspaces, lockfiles and reproducible builds.
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/) — compiler architecture, diagnostics, testing and development practices.
- [LLVM Documentation](https://llvm.org/docs/) — primary reference when evaluating an LLVM-based compiler backend.
- [Cranelift project](https://cranelift.dev/) — alternative code-generation ecosystem to evaluate if native code becomes justified.

Rukn currently uses a hand-written lexer, recursive-descent parser and tree-walk evaluator. Do not select LLVM, Cranelift or a bytecode VM before the language semantics and correctness corpus are mature enough to compare implementations fairly.

**Parser-library review (not adopted as a dependency):** [winnow 1.0.4](https://docs.rs/winnow/latest/winnow/) is a maintained parser-combinator toolbox with imperative and declarative styles. [Chumsky 0.13.0](https://docs.rs/chumsky/latest/chumsky/) offers error-recovery and parser-combinator capabilities; its GitHub repository was archived after the project moved to Codeberg ([repository notice](https://github.com/zesterer/chumsky)). For Rukn's small grammar, the existing recursive-descent parser remains easier to audit. Revisit a parser-library migration when grammar complexity, error recovery or measured maintenance costs justify it—not merely to increase the dependency count.

## 3. Testing, fuzzing and performance

- [Rust Compiler Development Guide: Fuzzing](https://rustc-dev-guide.rust-lang.org/fuzzing.html) — practical guidance on finding and minimizing compiler failures.
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and the [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) — a route to fuzzing Rust parsing and evaluation boundaries.
- [Criterion.rs](https://github.com/criterion-rs/criterion.rs) — statistical microbenchmarking for Rust.
- [Rust testing documentation](https://doc.rust-lang.org/book/ch11-00-testing.html) — unit, integration and documentation test foundations.

Suggested sequence:

1. Preserve unit tests for normal behaviour and invalid input.
2. Add a machine-readable conformance corpus with input, expected output/error class, and exit status.
3. Fuzz lexer/parser/run boundaries and convert each minimized issue into a regression test.
4. Benchmark only with correctness checked against that corpus.
5. Record source workloads, hardware, compiler version, commands and variance for every published comparison.

Fuzzing and benchmarks are not current completed Rukn features unless the repository later adds and continuously runs them.

## 4. Editor and tooling interoperability

- [Language Server Protocol specification](https://microsoft.github.io/language-server-protocol/) — editor/compiler communication contract for diagnostics, completion, navigation and related features.
- [TextMate grammar documentation](https://macromates.com/manual/en/language_grammars) — a simple syntax-highlighting starting point for editors.
- [Unicode Source Code Handling (UTS #55)](https://www.unicode.org/reports/tr55/) — practical considerations for source editors and programming environments.

Define stable diagnostic codes and source ranges before building a language server. A formatter and editor grammar can be useful earlier, but they must share the same token and string rules as the specification.

## 5. Documentation and deployment

- [VitePress documentation](https://vitepress.dev/) and [deployment guide](https://vitepress.dev/guide/deploy) — build, base paths, routing, search and GitHub Pages configuration.
- [WCAG 2.2](https://www.w3.org/TR/WCAG22/) — accessibility requirements for contrast, keyboard operation and user-interface behaviour.
- [SVG 2](https://www.w3.org/TR/SVG2/) and [SVG Accessibility API Mappings](https://www.w3.org/TR/svg-aam-1.0/) — scalable vector assets and accessible SVG naming.

The documentation site currently stays on the VitePress 1.6.4 stable line. The upstream release page lists VitePress 2.0.0-alpha releases rather than a stable 2.0 release as of this review, so moving to the alpha line is not justified for a production-facing documentation workflow. Reassess this only after a stable release and a tested migration path are available.

## Decision and source hygiene

Every research-driven language change should record: the source and version, the problem being addressed, alternatives considered, compatibility risks, test cases, and the decision’s status. A link is evidence to consult—not proof that the implementation follows the linked recommendation.

Suggested status vocabulary:

- **Implemented:** present in code and covered by tests.
- **Proposed:** a specific design under review.
- **Open research:** evidence or user-facing trade-offs remain unresolved.
- **Deferred:** a prerequisite is missing.
- **Accepted:** reviewers agreed to a documented decision.
- **Rejected / superseded:** the rationale and replacement are recorded.

For active questions, use the [design-decision register](/project/design-decisions), [Unicode and identifier security](/reference/unicode-and-security), and [testing strategy](/reference/testing-strategy).
