---
title: Language specification (draft)
description: The current grammar, values and semantics of Rukn.
---

# Rukn language specification

**Status: draft, version 0.3.** This document specifies the current interpreter, not a future 1.0 language. Any behavior not listed here should be treated as unsupported.

## Lexical rules

- Source files are UTF-8.
- Identifiers begin with underscore or a Unicode XID_Start character; later characters may be underscore or Unicode XID_Continue characters, using the pinned `unicode-ident 1.0.27` dependency (UAX #31). This permits supported combining marks after the first character.
- Identifiers are not normalized; visually confusable detection and bidirectional-control diagnostics are not implemented.
- Scientific notation is not part of the current numeric-literal grammar.
- Reserved words: دع, اطبع, صحيح, خطأ, لاشيء, و, أو.
- Number literals accept ASCII, Arabic-Indic and Eastern Arabic-Indic digits. The decimal separator may be the ASCII full stop (`.`) or Arabic decimal separator (`٫`, U+066B); both have the same numeric meaning. A separator must be followed by at least one digit. Repeated separators are invalid.
- String literals use paired ASCII double quotes or Arabic guillemets: "text" or «نص». Newlines inside a string are rejected.
- Comments begin with # and end at the line break.
- Statements end with ؛ or ;.

## Grammar (EBNF-like)

    program      = { statement } ;
    statement    = declaration | print_statement | expression_statement ;
    declaration  = "دع" identifier "=" expression terminator ;
    print_statement = "اطبع" [ "(" ] expression [ ")" ] terminator ;
    expression_statement = expression terminator ;
    terminator   = "؛" | ";" ;
    expression   = logical_or ;
    logical_or   = logical_and { "أو" logical_and } ;
    logical_and  = equality { "و" equality } ;
    equality     = comparison { ("==" | "!=") comparison } ;
    comparison   = term { (">" | ">=" | "<" | "<=") term } ;
    term         = factor { ("+" | "-") factor } ;
    factor       = unary { ("*" | "/") unary } ;
    unary        = ("!" | "-") unary | primary ;
    primary      = number | string | "صحيح" | "خطأ" | "لاشيء"
                 | identifier | "(" expression ")" ;

The print parentheses are optional in the implementation, but if an opening parenthesis is present, the closing parenthesis is required.

## Evaluation

- Numeric operations use Rust f64 values.
- Plus accepts two numbers or two strings; it does not coerce types.
- Comparison operators accept numbers only.
- Equality compares values and their variants.
- The operators و and أو are left-associative and short-circuit. The left operand must evaluate to a boolean. The right operand is evaluated and must also be boolean only when needed; a skipped right operand is neither evaluated nor runtime-type-checked. No truthiness conversion is performed.
- A variable lookup for an unknown name is an error.
- Division by positive or negative zero is an error.
- Non-finite numeric results are rejected with a language diagnostic.
- The interpreter evaluates statements in source order and collects print output.

## Implemented resource limits

The current implementation applies these guardrails:

| Resource | Current limit |
| --- | ---: |
| Source size | 1,048,576 bytes (1 MiB) |
| Lexer tokens | 16,384 |
| Parser nesting | 64 levels |

Inputs exceeding these bounds are rejected with a language error. These values are implementation limits and may change before a stable language release. They are not a formal sandbox and do not guarantee isolation against every denial-of-service scenario.

## Known gaps

The grammar is not yet generated from a formal parser definition. Unicode normalization, a confusable-identifier policy, lexical block scopes, immutable bindings, a static type checker, bytecode, and a machine-readable language conformance suite remain open work before a stable language release.
