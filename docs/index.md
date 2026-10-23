---
title: Rukn — an Arabic-first programming language
description: Learn the experimental Arabic-first language and its Rust interpreter.
---

# Programming should read naturally in Arabic.

Rukn (ركن) is an experimental Arabic-first programming language. The first implementation is a small interpreter written in Rust. The initial goal is modest: make the language syntax explicit, make errors understandable, and make every implemented feature testable.

<div class="vp-doc">

[Run your first program](/guide/getting-started){.VPButton .medium .alt}
[Read the language specification](/reference/specification){.VPButton .medium}

</div>

## What works today

- Arabic keywords and Unicode identifiers.
- Arabic-Indic, Eastern Arabic-Indic and ASCII digits in numeric literals.
- Numbers, strings, booleans and a null-like value.
- Variables, arithmetic, comparisons and equality.
- Parentheses, semicolon-terminated statements and Arabic diagnostics.
- A Rust library API, command-line runner and unit tests.

## What does not work yet

There are no functions, loops, user-defined types, modules, package manager, bytecode VM, native compiler, IDE integration or self-hosting compiler yet. Those are design and implementation milestones, not current features.

## A first program

```rukn
# examples/hello.rukn
دع الاسم = «العالم»؛
اطبع(«مرحبًا يا » + الاسم)؛
دع الناتج = ٦ * ٧؛
اطبع(الناتج)؛
```

Expected output:

```text
مرحبًا يا العالم
42
```

## Design principles

1. **Arabic-first, not Arabic-only.** Arabic keywords and diagnostics are first-class; ASCII digits and conventional operators remain available for interoperability.
2. **A small grammar before a large feature list.** The specification and tests should lead implementation.
3. **Useful errors.** Diagnostics should include source locations and explain the failed operation.
4. **Explicit semantics.** No implicit type coercion in the initial language.
5. **Build in public.** The repository should distinguish implemented behavior from proposals and record decisions in reviewable changes.

## Project status

**Experimental, pre-alpha.** Rukn is not suitable for production use. Syntax and semantics may change before a 1.0 language specification.
