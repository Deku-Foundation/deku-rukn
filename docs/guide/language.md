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

Numeric literals accept either the ASCII dot or the Arabic decimal separator ٫ (U+066B). Both 12.5 and ١٢٫٥ represent the same value; the separator must be followed by at least one digit.

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
| و | Logical AND | Two booleans; skips the right operand when the left is خطأ |
| أو | Logical OR | Two booleans; skips the right operand when the left is صحيح |

Precedence from high to low: unary, multiplication/division, addition/subtraction, comparison, equality, و, then أو. Both logical operators are left-associative and short-circuit. The left operand must evaluate to a boolean. The right operand is evaluated and must also be boolean only when needed; a skipped right operand is not evaluated or runtime-type-checked. There is no truthiness conversion. Parentheses override precedence.

    اطبع(صحيح أو خطأ و خطأ)؛ # صحيح: و has higher precedence than أو
    اطبع(خطأ و (١ / ٠ == ٠))؛ # خطأ; division is skipped

## Comments

A hash begins a comment that continues to the end of the line:

    # This is a comment
    دع الإجابة = ٤٢؛ # an inline comment

## Error behavior

Undefined variables, malformed strings, unsupported characters, division by zero and invalid type combinations return a diagnostic with a line and column. A failed program exits with a non-zero process status.

## Not part of the language yet

Control flow, functions, closures, collections, user-defined types, modules, error handling syntax, a formal module system and a stable ABI are not implemented. See the specification and roadmap.


## Unicode identifiers

Identifiers use the Unicode `XID_Start` and `XID_Continue` properties from the pinned `unicode-ident 1.0.27` crate, with underscore (`_`) deliberately allowed at either position. This is the lexical baseline described by [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/).

Arabic and Persian letters are supported, as are combining marks when the XID profile permits them after the first character:

```rukn
دع عَلم = ٧؛
اطبع(عَلم)؛
دع پ = ۳؛
اطبع(پ)؛
```

Identifier text is **not normalized**. Canonically or visually similar spellings are not guaranteed to resolve to the same name. The normalization, confusable-warning, and invisible bidirectional-control policies remain open research; see [Unicode and identifier security](/reference/unicode-and-security). Updating the Unicode identifier data is a language-behavior change and must include conformance tests.
