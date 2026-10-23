---
title: Interpreter architecture
description: The current Rukn front end, evaluator, boundaries and next design decisions.
---

# Interpreter architecture

The first version is intentionally a small tree-walk interpreter with no unsafe Rust and no native code generation.

    UTF-8 source
        ↓
    Lexer — tokens with line and column locations
        ↓
    Recursive-descent parser — statement and expression trees
        ↓
    Evaluator — environment and values
        ↓
    Collected output or structured language error

## Components

- **Lexer:** recognizes punctuation, identifiers, numeric literals, strings, comments and Arabic keywords.
- **Parser:** implements precedence using recursive-descent functions. Parse failures stop execution.
- **Evaluator:** evaluates the parsed syntax in source order. Values are numbers, strings, booleans and a nil-like value.
- **CLI:** reads one UTF-8 source file, prints successful output to stdout, and reports errors to stderr with a non-zero exit code.
- **Tests:** cover Arabic tokens, numeral normalization, operator precedence, type errors, undefined variables and malformed source.

## Safety boundaries

The language currently has no file, network or process APIs and does not load native plugins. The CLI only reads the explicitly supplied source file. This is a useful small surface, not a formal sandbox guarantee.

The interpreter still needs limits for source size, token count, nesting depth and execution work before it can be embedded in untrusted multi-tenant services. Recursive parsing and evaluation can exhaust the stack on deeply nested input. Do not run arbitrary Rukn source in a privileged service.

## Why Rust first?

Rust provides strong memory-safety defaults, useful enums for tokens, syntax trees and value variants, and a well-established test/build ecosystem. Rust is an implementation language, not a requirement that the future language expose Rust syntax or semantics.

## Design decisions still open

- Canonical Unicode normalization and identifier confusables.
- Whether keywords are reserved or contextual.
- Type system and mutation rules.
- Functions, modules and error propagation.
- Stable package/module format.
- Bytecode VM versus native code generation.
- Foreign-function interface and C ABI policy.
- Compiler bootstrap and self-hosting milestones.

Self-hosting should begin only after the grammar, standard library, diagnostics, test corpus and bootstrap process are stable. A self-hosting compiler is not proof of safety or quality by itself.
