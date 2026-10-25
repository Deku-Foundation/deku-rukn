---
title: Rust Independence and Self-Hosting Design
description: Research-grounded architecture, Arabic/Unicode profile, bootstrap stages, and measurable acceptance gates for Rukn.
---

# Rust Independence and Self-Hosting Design

**Status: architecture proposal for the experimental language, not a claim that self-hosting is implemented.**

## 1. Define independence precisely

Rukn has three different independence goals. They must be tracked separately.

1. **Language independence:** the language specification defines observable behavior without referring to Rust data types, Rust evaluation rules, or a particular compiler framework.
2. **Implementation independence:** a conforming interpreter, VM, compiler, formatter, and test runner can be implemented without requiring Rust. The current interpreter does not meet this goal.
3. **Bootstrap independence:** a clean machine can rebuild the toolchain from a documented, auditable seed without an unavailable or uninspectable compiler binary. This is harder than self-hosting and requires a deliberate stage-0 strategy.

A program that compiles itself is self-hosting; that fact alone does not prove the compiler correct, make the bootstrap seed disappear, or demonstrate better performance than C, Rust, or Python.

## 2. Repository baseline and immediate implications

The current implementation is an experimental Rust tree-walk interpreter. It supports Arabic keywords, Unicode identifiers via unicode-ident, numbers and strings, lexical scopes, conditionals, bounded loops, named functions and function references, immutable lists, indexing, and the built-ins طول and أضف. The current CLI conformance corpus and CI are intended to preserve observable behavior.

Important capabilities are still missing for a compiler that can build itself comfortably: a module/import system, a standard file API, a stable language-level error/result model, records or maps, a typed intermediate representation, bytecode, a VM, a native code generator, a package format, and stable FFI/ABI rules. Do not describe a native compiler, a VM, or self-hosting as implemented until executable artifacts and tests exist.

The first structural task after restoring green CI is to split the implementation into independently testable modules (lexer, tokens/diagnostics, parser/AST, values, evaluator, built-ins, CLI) without changing language semantics. This is a refactor, not a language feature; the conformance corpus must remain identical before and after it.

## 3. Research findings that affect the design

### Arabic language design: more than translating keywords

The 1995 ARABLAN paper proposed an Arabic programming language primarily for teaching. The 2019 Alf..Eih paper describes lexical, syntactic and semantic analysis followed by translation of Arabic source to C++. The 2018 Arabic Programming Language Evolution—A Survey catalogs earlier attempts and their limits. These are useful historical precedents, but they do not establish a modern, high-performance, self-hosting Arabic toolchain.

A 2017 ACM Learning @ Scale study examined longitudinal data from more than 15,000 Scratch users in five countries and reported that novice users with localized keywords and environments learned new programming concepts faster than peers using English localization. That is meaningful evidence for localization in an educational setting; it is not proof that Arabic keywords alone improve compiler throughput, professional developer productivity, or generated-code performance.

A 2025 preprint describes PWCT2 as a dual Arabic/English self-hosted visual language with a textual implementation language. Treat it as a useful modern case study, not as an independently replicated performance benchmark.

**Design consequence:** Rukn should be Arabic-first in syntax, diagnostics, documentation and tooling, while exposing the same well-defined semantics to editors, build tools, debuggers and foreign-function interfaces. Arabic terminology should be reviewed by programmers and language specialists; literal text must remain exact, and the language must not depend on machine translation at compile time.

### Unicode and right-to-left source code are correctness concerns

The current scanner uses Unicode XID properties, which is a sensible starting point but not a complete Unicode source-language policy.

- [UAX #31 — Unicode Identifiers and Syntax](https://unicode.org/reports/tr31/) defines recommended identifier profiles and normalization behavior. For a case-sensitive language, NFC-equivalent identifiers are the normal reference choice.
- [UTS #55 — Unicode Source Code Handling](https://www.unicode.org/reports/tr55/) discusses bidirectional reordering, misleading source presentation, identifier confusables, and tooling diagnostics. It recommends syntax-aware presentation rather than treating source files as ordinary RTL paragraphs.
- [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/) defines the algorithm used to display mixed-direction text; the current published annex is Unicode 18.0.0, dated September 2026.

**Required profile before 1.0:** pin the Unicode data version; define NFC equivalence for identifiers without normalizing string contents; test canonical-equivalent Arabic identifiers and collisions; document Arabic combining marks, tatweel, Arabic/Persian letters, Arabic-Indic and Persian digits; define the allowed numeric separator and digit-family rules; reject or diagnose bidi formatting controls in executable tokens; and provide syntax-aware source rendering and confusable/mixed-script warnings. The exact behavior should be written as conformance tests before changing the lexer. Do not implement incomplete hand-written normalization tables.

### IR and native-code infrastructure should be optional backends

The [LLVM language tutorial](https://llvm.org/docs/tutorial/) demonstrates a progression from lexer and parser to LLVM IR, control flow, optimization, object code and debug information. [MLIR](https://mlir.llvm.org/docs/LangRef/) supports multiple abstraction levels and verifiable transformations. They are valuable implementation options, but neither should define Rukn's language semantics or become a mandatory dependency of its portable specification.

**Proposed layering:** keep the language specification and conformance tests backend-neutral; define a versioned, verified Rukn bytecode format first; build a reference bytecode verifier and VM; then add native backends. LLVM/MLIR can be an optional optimization/code-generation backend, while a portable bytecode target gives CI and cross-platform builds a stable baseline. A backend may only be advertised as supported after it passes the same conformance corpus and its own ABI/target tests.

### Bootstrap is a chain of evidence, not a single self-compile

[GNU Mes](https://www.gnu.org/software/mes/manual/mes.html) documents stage-0 and source-bootstrap approaches. [Diverse Double-Compiling (DDC)](https://arxiv.org/abs/1004.5534) explains how a diverse compiler can help check whether a compiler binary corresponds to its claimed source, under explicit assumptions. Reproducible builds are a necessary aid, but bit-for-bit reproducibility alone does not prove the source is trustworthy.

The bootstrap strategy must therefore preserve and document the initial seed, its provenance, exact build instructions, binary hashes, target architecture and limitations. A minimal seed is a practical necessity: a self-hosted compiler still has to be launched by some already-executable program. The goal is to reduce, document, reproduce and eventually replace the Rust bootstrap dependency—not to pretend that the initial seed can be avoided.

## 4. Recommended architecture

1. **Language contract:** reviewed specification, normalized token/AST representation, stable error categories, resource-limit semantics and a machine-readable conformance corpus.
2. **Reference implementation:** the current Rust interpreter stays a temporary differential oracle while language behavior is being frozen.
3. **Module and standard-library layer:** deterministic imports, file reading/writing, path handling, command-line arguments, byte sequences, maps/records, and explicit error values; all I/O goes through a small documented host interface.
4. **Portable Rukn bytecode (RBC):** versioned header, target-independent opcodes, constant pool, source-map table, verifier, bounded decoder and deterministic serialization. Do not serialize Rust memory layouts or AST implementation structs.
5. **VM:** execute verified bytecode with defined stack, call, allocation, error and resource-limit semantics. A Rust VM is acceptable as a temporary stage; the bytecode contract must not depend on Rust.
6. **Self-hosted compiler:** implement lexer/parser, semantic checks and RBC generation in Rukn. Initially execute that compiler with the reference interpreter; compare diagnostics and output against the existing implementation.
7. **Native backend:** only after the bytecode path and conformance contract stabilize, lower a verified intermediate representation to LLVM IR or a separately maintained native backend. Preserve debug locations, deterministic symbol names and explicit target/ABI descriptions.
8. **Tooling:** formatter, language server, syntax-aware RTL display, package tool and debugger should consume the same language metadata and source maps instead of reimplementing token rules independently.

## 5. Bootstrap milestones and exit gates

| Stage | Deliverable | Exit gate |
|---|---|---|
| B0 | Stabilize current source and conformance suite | Green format, unit, integration, Clippy, documentation and bounded fuzz jobs; each feature has positive and negative tests |
| B1 | Modular reference interpreter and frozen language/Unicode profile | No unreviewed semantic drift; normalization/confusable decisions have tests and a version policy |
| B2 | Module system and minimum standard library | Compile and test multiple source files; deterministic imports; bounded and explicit I/O errors |
| B3 | RBC v1, verifier and reference VM | Malformed bytecode is rejected safely; verifier and VM pass the same conformance corpus |
| B4 | Compiler implementation written in Rukn | It compiles representative Rukn programs and itself through the reference interpreter; output and diagnostics are checked |
| B5 | Self-hosted rebuild | Compiler source compiled by version N and version N+1 yields equivalent behavior and, where the format is deterministic, identical artifacts |
| B6 | Rust-free ordinary release path | Build/release CI runs with Rust removed from required build dependencies; a documented reproducible seed and rollback path remain available |
| B7 | Native code and bootstrap assurance | Target/ABI tests, differential execution, deterministic builds, and a diverse bootstrap comparison are documented and automated |

Do not delete the Rust reference implementation just because B5 passes. First make it non-required for the ordinary release path (B6), then preserve a small, auditable bootstrap/recovery path and independently verify the new toolchain. Fully removing Rust source from the repository is a later maintenance decision, not the definition of self-hosting.

## 6. Performance claims require benchmark evidence

There is no defensible general claim that Rukn is faster than C, Rust or Python at this stage. Interpreter performance, compilation speed, generated program speed, memory use and developer productivity are different measurements.

Create a versioned benchmark suite before claiming superiority. It should cover lexer/parser throughput, numeric loops, recursive calls, list operations (including immutable append), string processing, module loading and compiler self-build time. Compare equivalent algorithms, datasets, input/output, optimization levels and warm-up policies. Record CPU, OS, compiler/runtime versions, flags, allocation/peak-memory figures and the exact source revision. Run multiple repetitions and report median plus dispersion; do not turn noisy microbenchmarks into a CI gate. Correctness is the first gate; performance is measured after semantic parity is established.

## 7. Research references

- ARABLAN, *Design of an Arabic Programming Language* (1995): [ScienceDirect record](https://www.sciencedirect.com/science/article/pii/0096055195000062).
- Razaq et al., *Designing and Implementing an Arabic Programming Language for Teaching Pupils* (2019): [DOI 10.35741/issn.0258-2724.54.3.11](https://doi.org/10.35741/issn.0258-2724.54.3.11). The described implementation translates Arabic source to C++.
- Al-Motwakel et al., *Arabic Programming Language Evolution—A Survey* (2018): [article copy](https://www.academia.edu/39953345/Arabic_Programming_Language_Evolution-A_Survey). Treat this as a secondary survey and check primary references when relying on individual claims.
- Dasgupta and Hill, *Learning to Code in Localized Programming Languages* (ACM Learning @ Scale 2017): [DOI 10.1145/3051457.3051464](https://doi.org/10.1145/3051457.3051464).
- Fayed, *Dual-Language General-Purpose Self-Hosted Visual Language and new Textual Programming Language for Applications* (2025 preprint): [arXiv:2509.20426](https://arxiv.org/abs/2509.20426).
- Unicode: [UAX #31](https://unicode.org/reports/tr31/), [UTS #55](https://www.unicode.org/reports/tr55/), [UAX #9](https://www.unicode.org/reports/tr9/).
- Compiler infrastructure: [LLVM language tutorial](https://llvm.org/docs/tutorial/), [MLIR language reference](https://mlir.llvm.org/docs/LangRef/), [CompCert](https://compcert.org/doc/).
- Bootstrapping and trust: [GNU Mes manual](https://www.gnu.org/software/mes/manual/mes.html), [Diverse Double-Compiling dissertation](https://arxiv.org/abs/1004.5534).

See also the [compiler bootstrap strategy](/project/compiler-bootstrap), [language specification](/reference/specification), [testing strategy](/reference/testing-strategy) and [roadmap](/project/roadmap).
