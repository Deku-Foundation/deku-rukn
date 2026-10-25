---
title: Interpreter architecture
description: The current Rukn pipeline, resource limits, safety boundaries, and open implementation decisions.
---

# Interpreter architecture

Rukn is currently a small tree-walk interpreter written in safe Rust. The implementation deliberately avoids native code generation and external runtime dependencies.

```text
UTF-8 source
    ↓
Lexer — tokens with line and column locations
    ↓
Recursive-descent parser — statement and expression trees
    ↓
Evaluator — environment and values
    ↓
Collected output or a structured language error
```

## Components

- **Lexer:** recognizes punctuation, identifiers, numeric literals, strings, comments and Arabic keywords.
- **Parser:** implements operator precedence, assignment, braced blocks, conditionals and while loops with recursive-descent functions; block nesting is bounded.
- **Evaluator:** evaluates statements in source order with nested lexical environments. Blocks push and pop scopes; assignment updates the nearest existing binding. `إذا` and `بينما` conditions must be booleans, and while-condition evaluations have a per-run budget. Invalid type combinations and non-finite arithmetic produce diagnostics.
- **CLI:** reads one UTF-8 source file, prints successful output to stdout, and reports errors to stderr with a non-zero exit code.
- **Tests:** cover Arabic tokens, numeral normalization, precedence, type errors, unknown variables, malformed source and resource-limit regressions.

## Implemented resource limits

The current implementation defines explicit limits for source size, token count, parser/block nesting, individual runtime string values, cumulative runtime string allocations/copies, and while-condition evaluations. It rejects oversized source, excess tokens, deeply nested syntax, oversized concatenation results, runaway loops, and programs that exceed the per-run string budget with language errors. The string budget accounts for string clones, concatenation results and captured print output. The loop budget allows at most 1,000,000 condition evaluations across all while loops in one run. Numeric operations also reject non-finite results instead of silently allowing infinity or NaN to flow through the program.

These limits are practical guardrails, not a formal sandbox or proof against every denial-of-service scenario. They should remain visible in the specification and be covered by regression tests whenever adjusted.

## Safety boundaries

The language has no language-level file, network or process APIs and does not load native plugins. The CLI reads the explicitly supplied source file. This reduces the available attack surface but is not a formal isolation guarantee.

Do not embed arbitrary untrusted programs in a privileged multi-tenant service without a separate threat model, process-level isolation, resource monitoring and a review of the embedding boundary.

## Why Rust first?

Rust provides enums for tokens and values, explicit error handling, memory-safety defaults, and a strong build and test ecosystem. Rust is the implementation language; it does not mean the future language must expose Rust syntax or semantics.

## Open design questions

- Unicode normalization, Arabic combining marks and visually confusable identifiers.
- Which keywords are reserved and how identifiers interact with future syntax.
- Type rules, mutation and lexical scopes.
- Functions, modules and error propagation.
- Stable package/module format and standard-library stability.
- Tree-walk execution versus bytecode VM or native back ends.
- Foreign-function interface and ABI policy.
- Compiler bootstrap and self-hosting milestones.

Self-hosting should be evaluated only after the grammar, standard library, diagnostics, conformance suite and bootstrap process are stable. A self-hosting compiler is not proof of safety, performance or language quality by itself.
