---
title: Research reading list
description: Primary standards and engineering references for developing Rukn as a robust Arabic-first programming language.
---

# Research reading list

**Reviewed: 25 October 2026.** This is a curated starting point, not a claim that Rukn conforms to every linked standard. Prefer primary standards and official project documentation over summaries. Record the version and access date when a technical decision depends on a source.

## 1. Arabic language, identifiers and Unicode security

| Source | What it informs | Status in Rukn |
| --- | --- | --- |
| [UAX #31 — Unicode Identifiers and Syntax](https://www.unicode.org/reports/tr31/) | Identifier start/continue properties, profiles and normalization guidance | Normative reference; Rukn uses pinned `unicode-ident 1.0.27` for XID_Start/XID_Continue and pinned `unicode-normalization 0.1.25` for NFC identifier tokens |
| [UAX #15 — Unicode Normalization Forms](https://www.unicode.org/reports/tr15/) | NFC/NFKC and equivalence rules | NFC identifier normalization is implemented and tested; literal strings/comments remain unchanged; NFKC is not used |
| [UTS #39 — Unicode Security Mechanisms](https://www.unicode.org/reports/tr39/) | Confusable detection, mixed-script review and security profiles | Open design question; no dedicated confusable checker exists |
| [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/) | Correct display of mixed right-to-left and left-to-right text | Relevant to source display, diagnostics, documentation and future editor tooling |
| [UTS #55 — Unicode Source Code Handling](https://www.unicode.org/reports/tr55/) | Unicode usability and security recommendations specifically for source code | Recommended design reference before stabilizing identifiers and editor diagnostics |

The Unicode Consortium’s current Unicode 18.0 publications include UAX #31 revision 45 (1 September 2026), UAX #15 revision 58 (12 August 2026), UTS #39 revision 34 (27 August 2026), and UAX #9 revision 52 (1 September 2026). Use the “Latest Version” links above rather than copying tables from older articles. These version details were checked on 23 October 2026.

Rukn uses pinned [unicode-ident 1.0.27](https://docs.rs/unicode-ident/1.0.27/unicode_ident/) for XID classification and [unicode-normalization 0.1.25](https://docs.rs/crate/unicode-normalization/0.1.25) for NFC identifier normalization. Both are locked in `Cargo.lock`. NFC applies before keyword recognition/name lookup; string-literal contents and comments are preserved. Updates to either crate can affect language behavior and require conformance tests and compatibility review.

### Questions to resolve before a stable identifier specification

1. Which precise identifier properties are accepted, and which Unicode version is normative?
2. Are canonically equivalent spellings the same variable name? If so, where does normalization occur?
3. How should Arabic combining marks, Persian character variants, join controls and mixed Latin/Arabic identifiers behave?
4. Should suspicious confusables produce a warning, an error, or an opt-in linter result?
5. How are invisible bidirectional controls displayed in editors, terminal diagnostics and code review?

Use valid/invalid program pairs and a versioned conformance corpus. Do not silently normalise user strings as a side effect of normalising identifiers.

The numeric-literal change follows Unicode's documented Arabic decimal punctuation and CLDR's model of locale-sensitive decimal symbols. Rukn accepts U+066B only as a numeric-literal separator; this does not imply locale-sensitive output formatting or acceptance of grouping separators. See [Unicode CLDR, Part 3: Numbers](https://unicode.org/reports/tr35/tr35-numbers.html) and [Unicode Technical Report #25, Unicode Support for Mathematics](https://unicode-org.github.io/unicode-reports/tr25/tr25.html).

### Arabic programming languages and localization research

- [ARABLAN: Design of an Arabic Programming Language (1995)](https://www.sciencedirect.com/science/article/pii/0096055195000062) — historical Arabic-language design, primarily educational.
- [Designing and Implementing an Arabic Programming Language for Teaching Pupils (2019)](https://doi.org/10.35741/issn.0258-2724.54.3.11) — describes lexical, syntactic and semantic analysis and translation of Arabic source to C++; useful prior art, not evidence of a self-hosted native toolchain.
- [Arabic Programming Language Evolution—A Survey (2018)](https://www.academia.edu/39953345/Arabic_Programming_Language_Evolution-A_Survey) — secondary survey; consult its primary references for claims about individual languages.
- [Learning to Code in Localized Programming Languages (ACM Learning @ Scale 2017)](https://doi.org/10.1145/3051457.3051464) — a study of over 15,000 Scratch users in five countries supporting localization in a learning context; it does not show that Arabic syntax alone improves compiler or generated-code performance.
- [PWCT2 dual-language self-hosted visual language (2025 preprint)](https://arxiv.org/abs/2509.20426) — a modern case study, not an independently replicated performance benchmark.

The implementation implications and limits of these studies are documented in the [self-hosting design](/project/self-hosting-design).

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

**Compiler bootstrap and assurance:** the [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/) documents staged compiler builds and architecture; the [LLVM Kaleidoscope tutorial](https://llvm.org/docs/tutorial/) is a worked path from a small front end to IR and native output; [CompCert](https://compcert.org/doc/) is a research reference for formally verified compilation. Ken Thompson's [Reflections on Trusting Trust](https://doi.org/10.1145/358198.358210) explains why trusting the compiler binary matters as well as reviewing source. These references are synthesized into Rukn's [compiler bootstrap strategy](/project/compiler-bootstrap); self-hosting is a target, not an existing capability.

### Executable semantics and mechanized reasoning

Before adding a bytecode VM or a native backend, define a compact semantic model for the language features that already exist. Two useful primary resources are [PLT Redex documentation](https://docs.racket-lang.org/redex/), which supports executable language semantics and reduction relations, and [Software Foundations: Programming Language Foundations](https://softwarefoundations.cis.upenn.edu/plf-current/index.html), which develops operational semantics and type-system properties with machine-checked proofs.

These tools need not become Rukn runtime dependencies. Use small models to pin down observable rules—scope lookup, function return propagation, list immutability, evaluation order and error cases—then convert each agreed rule into regression/conformance tests for the Rust implementation. Mechanized proofs only establish properties of the model and assumptions actually formalized; they do not automatically prove the implementation correct.

## 3. Testing, fuzzing and performance

- [Rust Compiler Development Guide: Fuzzing](https://rustc-dev-guide.rust-lang.org/fuzzing.html) — practical guidance on finding and minimizing compiler failures.
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and the [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) — a route to fuzzing Rust parsing and evaluation boundaries.

As checked on 24 October 2026, the latest tagged cargo-fuzz release is [0.13.2](https://github.com/rust-fuzz/cargo-fuzz/releases/tag/0.13.2), published 9 June 2026. Rukn pins that version in its optional smoke workflow; re-check upstream before upgrading.
- [Criterion.rs](https://github.com/criterion-rs/criterion.rs) — statistical microbenchmarking for Rust.
- [Rust testing documentation](https://doc.rust-lang.org/book/ch11-00-testing.html) — unit, integration and documentation test foundations.
- [Selective Feature-Sensitive Coverage for Conformance Testing of Programming Languages (2026)](https://doi.org/10.1145/3808231) — recent peer-reviewed research on designing conformance tests around language features and execution paths.

Suggested sequence:

1. Preserve unit tests for normal behaviour and invalid input.
2. Maintain the machine-readable CLI conformance corpus with input, expected stdout, diagnostic fragment, and exit status; it currently covers 42 cases including scopes, assignment, conditions, bounded loops, named functions, function values, higher-order calls, immutable lists, zero-based indexing, Unicode scalar indexing and error paths. Extend it whenever semantics change.
3. Fuzz lexer/parser/run boundaries and convert each minimized issue into a regression test.
4. Benchmark only with correctness checked against that corpus.
5. Record source workloads, hardware, compiler version, commands and variance for every published comparison.

The bounded end-to-end fuzz smoke target and a 42-case CLI conformance corpus are active in CI. Parser-specific fuzzing, longer campaigns and reproducible benchmark gates remain future work.

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
