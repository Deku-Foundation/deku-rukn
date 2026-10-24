---
title: String literal behavior
description: Implemented string delimiters, escapes, and error behavior in the Rukn interpreter.
---

# String literal behavior

**Status: implemented behavior for the current experimental interpreter.** This page documents the behavior exercised by regression tests; it is not a promise of a stable 1.0 language specification.

## Delimiters

Rukn accepts either ASCII double quotes or Arabic guillemets:

```rukn
اطبع("مرحبًا")؛
اطبع(«أهلًا»)؛
```

A string literal must close with the matching delimiter. Physical LF (\n) and CR (\r) line breaks inside a string literal are rejected, even after a backslash; use the supported `\n` escape to put a line-feed character in the decoded value. Unterminated strings are reported at end-of-input, and a physical line break is reported at its own position.

## Escape sequences

| Source sequence | Decoded character |
| --- | --- |
| `\\n` | Line feed |
| `\\r` | Carriage return |
| `\\t` | Horizontal tab |
| `\\\\` | Backslash |
| `\\"` | ASCII double quote |
| `\\«` | Opening Arabic guillemet |
| `\\»` | Closing Arabic guillemet |

Unknown escape sequences are errors rather than being silently retained. This helps catch typos and keeps the grammar explicit. Diagnostics currently report the string token's starting location; more precise escape spans are a future diagnostic improvement.

String contents are not Unicode-normalized. Identifier normalization is a separate language-design question and must not change user text implicitly.

## Regression coverage

The interpreter tests cover both delimiter styles, escaped closing Arabic guillemets, decoded control characters, backslashes, and rejection of an unknown escape. Run:

```sh
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
```

## Design constraints

- Keep lexing (finding the end of the token) separate from decoding its contents.
- Reject unsupported escapes with a language diagnostic, not a panic.
- Preserve UTF-8 text exactly except for explicitly documented escape decoding.
- Extend the escape grammar only with tests and a documentation update.
