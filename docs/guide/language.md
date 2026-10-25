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

Numeric literals accept either the ASCII dot or the Arabic decimal separator ٫ (U+066B). Both 12.5 and ١٢٫٥ represent the same value; the separator must be followed by at least one digit. Use one digit family throughout a number: ASCII, Arabic-Indic (٠١٢٣), or Extended Arabic-Indic (Persian; ۰۱۲۳). Mixing families inside one numeric literal is an error.

A binding can be reassigned by declaring the same name again in the same scope; lexical scopes and immutable bindings are not implemented yet.

### Print a value

    اطبع(«مرحبًا»)؛
    اطبع(٢ + ٣)؛
    اطبع صحيح؛

Parentheses around the printed expression are optional.

## Assignment and lexical scopes

Use `=` as a statement to update an existing variable. Assigning to an undefined name is an error. The assignment target is the nearest binding with that name in the current scope or an outer scope.

Curly-braced blocks create lexical scopes. A variable declared inside a block is not visible after the block ends. Declaring the same name in a nested block shadows the outer binding; declaring it again in the same scope replaces that scope's current binding.

```rukn
دع الاسم = «خارج»؛
{
    دع الاسم = «داخل»؛
    الاسم = «معدّل»؛
    اطبع(الاسم)؛
}
اطبع(الاسم)؛
```

Expected output:

```text
معدّل
خارج
```

## Conditions and loops

`إذا` selects a block when its condition is `صحيح`; optional `وإلا` selects the alternative block. `بينما` repeats its block while the condition evaluates to `صحيح`. Conditions must evaluate to booleans; numbers and strings are not implicitly converted.

```rukn
دع العد = ٠؛
بينما (العد < ٣) {
    اطبع(العد)؛
    العد = العد + ١؛
}
إذا (العد == ٣) {
    اطبع(«انتهى»)؛
} وإلا {
    اطبع(«لم ينتهِ»)؛
}
```

Expected output:

```text
0
1
2
انتهى
```

The interpreter stops a run after 1,000,000 total `بينما` condition evaluations. This is an implementation resource limit, not a timing guarantee or a full sandbox.

## Named functions, arguments and returns

Function declarations use the Arabic keyword `دالة`, a name, a comma-separated parameter list, and a braced body. Call a named function with parentheses; the number of arguments must match the declared parameter count.

```rukn
دالة جمع(أ, ب) {
    أرجع أ + ب؛
}

دالة عاملي(ن) {
    إذا (ن <= ١) {
        أرجع ١؛
    }
    أرجع ن * عاملي(ن - ١)؛
}

اطبع(جمع(٢٠, ٢٢))؛
اطبع(عاملي(٥))؛
```

Expected output:

```text
42
120
```

`أرجع expression؛` returns a value. `أرجع؛` returns `لاشيء`, and reaching the end of a function without an explicit return also returns `لاشيء`. A return statement outside a function is an error. Functions may be declared only at top level in this version and support recursion up to 128 active calls. At most 64 parameters or arguments are allowed.

A named function can also be used as a value: assign it to a variable, pass it to another function, or return it from a function. Calls still use identifier syntax, so store a returned function in a variable before calling it. Function references do not capture caller-local variables; closures, nested declarations, modules and a static type checker remain future work.

```rukn
دالة جمع(أ, ب) { أرجع أ + ب؛ }
دالة تطبيق(عملية, أ, ب) { أرجع عملية(أ, ب)؛ }
دع العملية = جمع؛
اطبع(العملية(٢, ٣))؛
اطبع(تطبيق(جمع, ٤, ٥))؛
```

Expected output: `5` and `9`.

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

Closures, anonymous functions, arbitrary call-expression chaining, collections, user-defined types, modules, explicit error-handling syntax, a static type checker and a stable ABI are not implemented yet. Named function values, higher-order calls, parameters, returns, recursion, conditionals, blocks, lexical scopes, assignment and bounded while loops are implemented. See the specification and roadmap.


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
