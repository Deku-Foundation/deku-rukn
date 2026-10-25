---
title: Compiler bootstrap and self-hosting strategy
description: An evidence-based path from the current Rust interpreter to a self-hosted Rukn compiler.
---

# Compiler bootstrap and self-hosting strategy

**Status: engineering strategy, not a claim that self-hosting is implemented.** This plan defines what “independent language” means for Rukn, what must be built first, and how to prove that each stage works.

## The technical decision

Rukn's long-term goal is a compiler and core toolchain whose primary implementation language is Rukn itself. That goal is achievable through bootstrapping; it is not achieved by pretending the first compiler can exist without an implementation substrate.

A compiler must initially be executed by an existing machine-code runtime, assembler, compiler, or a very small trusted seed. Even compilers for self-hosting languages need a stage-zero executable to begin the chain. The appropriate target is therefore:

- **Self-hosted implementation:** the maintained compiler and core language tools are written in Rukn.
- **Documented bootstrap root:** a small stage-zero compiler/interpreter can build the first Rukn compiler from source.
- **Reducible trust base:** the stage-zero implementation and external build tools are minimized, versioned, tested, and auditable over time.
- **Reproducible stages:** each compiler stage passes the same conformance suite; binary reproducibility is pursued where the backend and build environment make it practical.

Rewriting the project in Rukn immediately would not remove dependencies. Rukn now has blocks, lexical scopes, assignment, conditions, bounded while loops, named top-level functions, parameters, returns and recursion. It still lacks first-class function values, closures, modules, collections and the library facilities needed to express a compiler cleanly. The next task is to stabilize these semantics, then define module boundaries and the minimum standard library.

## Bootstrap model

    Stage 0: Rust reference interpreter and CLI
                |
                | run programs in the conformance corpus
                v
    Stage 1: Rukn compiler prototype written in Rust
                |
                | compile the Rukn compiler source
                v
    Stage 2: compiler binary produced by Stage 1
                |
                | compile the same compiler source again
                v
    Stage 3: compiler binary produced by Stage 2
                |
                | compare tests, outputs, and reproducible artifacts
                v
    Self-hosted Rukn compiler with a documented seed bootstrap

This is a target architecture. Stage 1 and later self-hosted stages do not exist yet.

## Engineering stages and acceptance criteria

### B0 — Make behavior measurable

- Maintain a written language specification and an executable, machine-readable CLI conformance corpus.
- Test successful output, diagnostic categories, exit status, malformed source, Unicode numeral families, and runtime limits.
- Run formatting, tests, linting, example checks, documentation builds, and bounded fuzzing in CI.
- Convert every minimized fuzz failure or semantic bug into a permanent regression case.

**Exit criterion:** a change to the language cannot silently change behavior without changing an expected conformance result or an explicitly reviewed specification.

### B1 — Define a compiler-capable language core

Before implementing the compiler in Rukn, decide and specify:

- expression and statement semantics, blocks and lexical scopes (implemented experimentally; stabilize through conformance tests);
- first-class functions, closures, module boundaries and a defined import model;
- a type model, numeric widths and overflow rules, mutability, and type inference policy;
- collections, records or algebraic data types, strings, and explicit error propagation;
- modules, imports, source locations, and a stable diagnostic format;
- a minimal standard library for files, command-line arguments, paths, and process exit status.

Do not add all these as unrelated syntax patches. Each feature needs a specification, valid and invalid examples, compatibility notes, conformance tests, and matching Arabic, English, and French documentation. Keep the initial core small enough to implement and audit.

### B2 — Introduce a compiler front end and explicit intermediate representation

Separate the implementation into independently testable phases:

1. source decoding and lexing;
2. parsing to a syntax tree;
3. name resolution and scope checking;
4. type checking and semantic validation;
5. lowering to a typed intermediate representation (IR);
6. execution or code generation.

Keep the IR independent of Arabic surface syntax. It should express the semantics of the language rather than duplicate its spelling. Add structural dumps and tests for each phase before optimization.

### B3 — Add a bytecode VM, then evaluate native code generation

A bytecode VM is a practical intermediate milestone: it can execute compiler output on multiple platforms and gives the self-hosting compiler a runtime target. Compare it with the reference tree-walk interpreter using the same conformance corpus.

Native output is a separate milestone. Evaluate LLVM or Cranelift as engineering accelerators only after the IR and correctness suite exist. These back ends can help Rukn produce useful executables earlier, but they are dependencies, not proof that the complete toolchain is independent. A Rukn-written native back end and object/linking strategy can follow once target requirements, ABI, and correctness tests are defined.

### B4 — Build the first compiler in Rust

Keep a small, understandable Rust implementation as the reference compiler while language semantics are changing quickly. Add:

- a versioned IR and documented lowering rules;
- diagnostics that preserve source locations through transformations;
- differential tests comparing interpreter, VM, and compiler output;
- fuzz targets for lexing, parsing, semantic analysis, and IR lowering;
- benchmarks only after equivalent implementations pass the same correctness suite.

Do not optimize or select a backend based on reputation alone. Publish reproducible workloads, commands, toolchain versions, target details, and correctness results.

### B5 — Port the compiler to Rukn and prove the bootstrap chain

Once Rukn can express the compiler and required standard-library code:

1. Write the compiler source in Rukn.
2. Use Stage 0 to run or compile the first Rukn compiler.
3. Use that compiler to compile its own source and produce the next stage.
4. Run the full conformance corpus, compiler tests, and real project builds with each stage.
5. Investigate byte-for-byte reproducibility; normalize timestamps, paths, ordering, and other nondeterministic inputs before treating binary comparisons as meaningful.
6. Document the exact bootstrap commands, compiler versions, source revisions, and trusted tools.

Matching binaries are valuable evidence when builds are deterministic, but not a complete security proof. Ken Thompson's “Reflections on Trusting Trust” explains why checking compiler source alone cannot exclude every compiler-stage compromise. Diverse double-compiling is a possible later assurance experiment, not a substitute for reproducible builds, code review, and a transparent toolchain.

### B6 — Reduce the trusted seed over time

After self-hosting is stable, evaluate whether the Stage 0 Rust implementation can be reduced to a small bootstrap compiler or interpreter. Keep the bootstrap reproducible and maintained; do not delete the seed merely to claim purity. A toolchain that cannot be rebuilt from documented sources is less independent in practice, not more.

## Research and standards informing this plan

- The [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/) documents a mature staged compiler build process.
- [LLVM's Kaleidoscope tutorial](https://llvm.org/docs/tutorial/) develops a language incrementally through lexing, parsing, IR generation, optimization, and object-code output. Its own tutorial warns that it prioritizes teaching over all production engineering practices.
- [CompCert](https://compcert.org/doc/) is a research reference for formally verified compiler correctness. Its proof-oriented approach is a long-term direction to study, not a realistic prerequisite for the first working Rukn compiler.
- The 2026 study [Selective Feature-Sensitive Coverage for Conformance Testing of Programming Languages](https://doi.org/10.1145/3808231) motivates designing conformance tests around language features and execution paths rather than counting tests alone.
- [Ken Thompson, “Reflections on Trusting Trust” (1984)](https://doi.org/10.1145/358198.358210) motivates treating the compiler and build chain as part of the trusted computing base.
- [NIST SP 800-218, Secure Software Development Framework](https://csrc.nist.gov/pubs/sp/800/218/final) recommends disciplined, auditable software-development and toolchain practices. It is guidance, not a language specification.

For Arabic-first source, the current normative Unicode references are [UAX #31, revision 45](https://www.unicode.org/reports/tr31/), [UAX #15, revision 58](https://www.unicode.org/reports/tr15/), and [UTS #39, revision 34](https://www.unicode.org/reports/tr39/) in the Unicode 18.0 release family. These inform identifier profiles, normalization, and security review; they do not mean Rukn already conforms to every recommendation.

Arabic computational-linguistics research can inform language-facing choices such as terminology, morphology, and Arabic text processing, but it is not a substitute for programming-language semantics, compiler testing, or bootstrap engineering. The language must define formal rules that tools can implement consistently.

## Near-term work

1. Keep the new CLI conformance corpus small, explicit, and reviewed as observable behavior changes.
2. Separate parser/fuzz coverage from the end-to-end interpreter target.
3. Stabilize the semantics of the existing control flow; specify types and numeric behavior before adding more language features.
4. Named functions, parameters, returns and recursion now have regression tests; next define and test module boundaries and the minimum standard library before attempting to port the compiler.
5. Choose a bytecode or native backend only with conformance evidence and a written target/ABI plan.

See the [roadmap](/project/roadmap), [testing strategy](/reference/testing-strategy), and [design-decision register](/project/design-decisions).