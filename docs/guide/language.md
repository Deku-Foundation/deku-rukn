---
title: Language guide
description: A practical guide to the current Rukn syntax and semantics.
---

# Language guide

This page documents behavior implemented by the current interpreter. It is intentionally narrower than the long-term language vision.

## Statements

Every statement ends with the Arabic semicolon ؛ or the ASCII semicolon ;.

### Bind a variable

    دع الاسم = «ليلى»؛
    دع السعر = 12.5؛

The numeric lexer currently recognizes the ASCII dot as a decimal separator. Write 12.5 or ١٢.٥. The Arabic decimal separator ٫ is not yet supported.

A binding can be reassigned by declaring the same name again in the same scope; lexical scopes and immutable bindings are not implemented yet.

### Print a value

    اطبع(«مرحبًا»)؛
    اطبع(٢ + ٣)؛
    اطبع صحيح؛

Parentheses around the printed expression are optional.

## Values

| Kind | Examples | Notes |
| --- | --- | --- |
| Number | 42, ٣.٥ | IEEE-754 floating-point number |
| String | "hello", «أهلًا» | Single-line text |
| Boolean | صحيح, خطأ | Displayed using Arabic words |
| Nil-like | لاشيء | Represents an absent value |

Arabic-Indic digits (٠١٢٣٤٥٦٧٨٩), Eastern Arabic-Indic digits (۰۱۲۳۴۵۶۷۸۹) and ASCII digits are accepted in number literals.

## Operators

| Operator | Meaning | Operand types |
| --- | --- | --- |
| - (unary) | Negation | Number |
| ! (unary) | Boolean negation | Boolean |
| *, / | Multiplication, division | Two numbers |
| + | Addition or string concatenation | Two numbers or two strings |
| - | Subtraction | Two numbers |
| <, <=, >, >= | Numeric comparison | Two numbers |
| ==, != | Equality / inequality | Any two values |

Precedence from high to low: unary, multiplication/division, addition/subtraction, comparison, equality. Parentheses override precedence.

## Comments

A hash begins a comment that continues to the end of the line:

    # This is a comment
    دع الإجابة = ٤٢؛ # an inline comment

## Error behavior

Undefined variables, malformed strings, unsupported characters, division by zero and invalid type combinations return a diagnostic with a line and column. A failed program exits with a non-zero process status.

## Not part of the language yet

Control flow, functions, closures, collections, user-defined types, modules, error handling syntax, a formal module system and a stable ABI are not implemented. See the specification and roadmap.
