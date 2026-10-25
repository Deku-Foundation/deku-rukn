---
title: Language specification (draft)
description: The current grammar, values and semantics of Rukn.
---

# Rukn language specification

**Status: draft, version 0.9.** This document specifies the current interpreter, not a future 1.0 language. Any behavior not listed here should be treated as unsupported.

## Lexical rules

- Source files are UTF-8.
- Identifiers begin with underscore or a Unicode XID_Start character; later characters may be underscore or Unicode XID_Continue characters, using the pinned `unicode-ident 1.0.27` dependency (UAX #31). This permits supported combining marks after the first character.
- Identifier tokens are normalized to NFC using the pinned `unicode-normalization 0.1.25` dependency before keyword recognition and name lookup. Canonically equivalent spellings refer to the same binding; original source line/column positions and the original spelling for diagnostics are retained. Unchanged identifiers do not store a duplicate raw spelling. String literal contents and comments are not normalized.
- NFC normalization does not detect visual confusables or mixed-script identifiers. Dedicated confusable warnings, mixed-script policy and bidirectional-source presentation/diagnostics are not implemented yet.
- Scientific notation is not part of the current numeric-literal grammar.
- Reserved words: دع, اطبع, دالة, أرجع, إذا, وإلا, بينما, صحيح, خطأ, لاشيء, و, أو.
- Number literals accept ASCII, Arabic-Indic and Extended Arabic-Indic (Persian) digits. The decimal separator may be the ASCII full stop (`.`) or Arabic decimal separator (`٫`, U+066B); both have the same numeric meaning. A separator must be followed by at least one digit. Repeated separators are invalid. Every digit in one numeric literal must come from the same digit family; mixing families is rejected at the first mismatching digit.
- String literals use paired ASCII double quotes or Arabic guillemets: "text" or «نص». Physical LF and CR line breaks inside a string are rejected, including after a backslash.
- Comments begin with # and end at the line break.
- Statements end with ؛ or ;.

## Grammar (EBNF-like)

    program      = { statement } ;
    statement    = function_declaration | return_statement | declaration
                 | assignment | print_statement | if_statement | while_statement
                 | block | expression_statement ;
    function_declaration = "دالة" identifier "(" [ parameter_list ] ")" block ;
    parameter_list = identifier { "," identifier } ;
    return_statement = "أرجع" [ expression ] terminator ;
    declaration  = "دع" identifier "=" expression terminator ;
    assignment   = identifier "=" expression terminator ;
    print_statement = "اطبع" ( "(" expression ")" | expression ) terminator ;
    if_statement = "إذا" "(" expression ")" block [ "وإلا" block ] ;
    while_statement = "بينما" "(" expression ")" block ;
    block        = "{" { statement } "}" ;
    expression_statement = expression terminator ;
    terminator   = "؛" | ";" ;
    expression   = logical_or ;
    logical_or   = logical_and { "أو" logical_and } ;
    logical_and  = equality { "و" equality } ;
    equality     = comparison { ("==" | "!=") comparison } ;
    comparison   = term { (">" | ">=" | "<" | "<=") term } ;
    term         = factor { ("+" | "-") factor } ;
    factor       = unary { ("*" | "/") unary } ;
    unary        = ("!" | "-") unary | postfix ;
    postfix      = primary { "[" expression "]" } ;
    primary      = number | string | list_literal | "صحيح" | "خطأ" | "لاشيء"
                 | call_expression | identifier | "(" expression ")" ;
    list_literal  = "[" [ argument_list ] "]" ;
    call_expression = identifier "(" [ argument_list ] ")" ;
    argument_list = expression { "," expression } ;

The print parentheses are optional in the implementation, but if an opening parenthesis is present, the closing parenthesis is required.

## Evaluation

- Numeric operations use Rust f64 values.
- Plus accepts two numbers or two strings; it does not coerce types.
- Comparison operators accept numbers only.
- Equality compares values and their variants.
- The operators و and أو are left-associative and short-circuit. The left operand must evaluate to a boolean. The right operand is evaluated and must also be boolean only when needed; a skipped right operand is neither evaluated nor runtime-type-checked. No truthiness conversion is performed.
- A declaration using دع defines or replaces a binding in the current lexical scope.
- Braced blocks create lexical scopes. Name lookup and assignment search from the innermost scope outwards; a block-local binding does not leak after the block ends.
- Assignment updates the nearest existing binding. Assigning to an undefined name is an error; assignment is a statement, not an expression.
- Conditions for إذا and بينما must evaluate to a boolean. There is no truthiness conversion.
- while conditions are checked in source order, and the interpreter rejects a run after 1,000,000 total while-condition evaluations to bound runaway loops.
- A variable lookup for an unknown name is an error.
- Division by positive or negative zero is an error.
- Non-finite numeric results are rejected with a language diagnostic.
- Function declarations use `دالة name(parameter, ...)` and are permitted only at top level in this version. All top-level function names are registered before statement execution, so forward calls and mutual recursion work. Duplicate function names are rejected during parsing. A function name evaluates to a named function reference when no lexical variable shadows it; references can be assigned to variables, passed as arguments, and returned from functions. References with the same function name compare equal. Calls use identifier syntax (`name(...)`), so an alias can be called but arbitrary call results cannot yet be chained directly. Function references do not capture an environment: closures and nested function declarations are unsupported.
- List literals use square brackets and comma-separated expressions, for example `[1, «نص», صحيح]`. Lists are immutable and can contain mixed value types. Indexing is zero-based and requires a finite, non-negative integer. Indexing a list returns an element; indexing a string returns one Unicode scalar value as a string. Out-of-range indexes are errors.
- The built-in `طول(value)` accepts a string or list and returns a number. String length and string indexes count Unicode scalar values, not grapheme clusters. The built-in `أضف(list, value)` returns a new list with the appended value; it does not mutate the original list. Both built-ins can be passed as function references. User-defined top-level functions may not reuse the built-in names.
- Arguments are evaluated left to right in the caller's environment. Arity must match exactly; at most 64 parameters/arguments are accepted.
- A function body runs with the global environment and a fresh parameter/local environment. It cannot capture caller-block locals; closures and nested function declarations are unsupported. Assignments to global bindings persist after the call.
- `أرجع expression؛` returns a value; `أرجع؛` returns `لاشيء`. Falling off the end of a function also returns `لاشيء`. A return outside a function is an error.
- Recursive calls are supported, with a maximum active call depth of 32 in the current tree-walk interpreter; a managed VM stack is required before raising this limit. Expression nesting is bounded independently within each function frame.
- The interpreter evaluates statements in source order and collects print output.

## Implemented resource limits

The current implementation applies these guardrails:

| Resource | Current limit |
| --- | ---: |
| Source size | 1,048,576 bytes (1 MiB) |
| Lexer tokens | 16,384 |
| Parser nesting | 64 levels |
| Single runtime string | 1,048,576 UTF-8 bytes (1 MiB) |
| Cumulative runtime string allocation/copy budget per run | 16,777,216 bytes (16 MiB) |
| while-condition evaluations per run | 1,000,000 |
| Active function-call depth (tree-walk interpreter) | 32 |
| Function parameters / call arguments | 64 |
| Elements in one list | 16,384 |
| Cumulative list elements allocated/copied per run | 1,000,000 |
| Nested list depth | 64 |

The runtime string budget conservatively counts string-value clones, named function-reference name copies, concatenation results, captured print output, and string/function-name copies made by list append during one run. The collection budget counts list elements created or copied, including each immutable append; it is cumulative accounting rather than exact live-heap measurement. Inputs exceeding these bounds are rejected with a language error. These are implementation limits and may change before a stable language release; they are not a formal sandbox and do not guarantee isolation against every denial-of-service scenario.

## Known gaps

The grammar is not yet generated from a formal parser definition. A confusable-identifier policy, mixed-script diagnostics, immutable bindings, a static type checker, closures, records/maps, modules, a typed intermediate representation, bytecode, and a conformance corpus shared across future interpreter/VM/compiler implementations remain open work before a stable language release.
