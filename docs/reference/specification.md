---
title: Language specification (draft)
description: The current grammar, values and semantics of Rukn.
---

# Rukn language specification

**Status: draft, version 0.1.** This document specifies the current interpreter, not a future 1.0 language. Any behavior not listed here should be treated as unsupported.

## Lexical rules

- Source files are UTF-8.
- Identifiers begin with an underscore or a Unicode alphabetic character; later characters may also be Unicode numeric characters.
- Reserved words: دع, اطبع, صحيح, خطأ, لاشيء.
- Number literals accept ASCII, Arabic-Indic and Eastern Arabic-Indic digits. The decimal point is ASCII dot.
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
    expression   = equality ;
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
- A variable lookup for an unknown name is an error.
- Division by positive or negative zero is an error.
- The interpreter evaluates statements in source order and collects print output.

## Known gaps

The grammar is not yet generated from a formal parser definition. There is no Unicode normalization, confusable-identifier policy, block scope, immutable binding model, static type checker, resource budget, bytecode, or specification compatibility suite. These are open design topics before a stable language release.
