---
layout: home
title: Rukn — Arabic-first programming language
description: Explore Rukn, an experimental Arabic-first programming language implemented in Rust.
hero:
  name: Rukn · ركن
  text: Arabic-first programming
  tagline: A small, readable language experiment with Arabic keywords, explicit semantics, and an interpreter written in Rust.
  image:
    light: /rukn-hero.svg
    dark: /rukn-hero-dark.svg
    alt: Rukn's arch mark beside an Arabic-language example and its expected output.
  actions:
    - theme: brand
      text: Run your first program
      link: /guide/getting-started
    - theme: alt
      text: Read the specification
      link: /reference/specification
    - theme: alt
      text: اقرأ بالعربية
      link: /ar/
features:
  - title: Arabic-first syntax
    details: Arabic keywords, Unicode identifiers, and ASCII, Arabic-Indic, or Eastern Arabic-Indic digits.
    link: /guide/language
    linkText: Explore the language
  - title: Small, explicit core
    details: A Rust tree-walk interpreter with numbers, strings, booleans, immutable lists, lexical scopes, control flow, named functions and clear diagnostics.
    link: /reference/architecture
    linkText: Understand the interpreter
  - title: Research before promises
    details: Implemented behavior, open questions, and future proposals are documented separately and tested as the language evolves.
    link: /project/design-decisions
    linkText: Read design decisions
---

## A small program, end to end

The same example lives in `examples/hello.rukn` and is exercised by CI.

```rukn
# أول برنامج بلغة ركن
دع الاسم = «العالم»؛
اطبع(«مرحبًا يا » + الاسم)؛
دع الناتج = ٦ * ٧؛
اطبع(الناتج)؛
اطبع(صحيح == خطأ)؛
اطبع(صحيح أو خطأ)؛
```

Expected output:

```text
مرحبًا يا العالم
42
خطأ
صحيح
```

## Project status

**Experimental · pre-alpha · not for production use.**

The current interpreter supports assignment, lexical block scopes, boolean conditions, bounded `بينما` loops, named top-level functions, function references, higher-order calls, parameters, returns, recursion, immutable lists, zero-based indexing and the `طول` / `أضف` built-ins. Closures, nested function declarations, records/maps, modules, a package manager, bytecode VM, native compilation, IDE integration and self-hosting remain future work.

The first priorities are a precise and tested language specification, robust diagnostics, Unicode identifier policy, regression tests, and a reproducible documentation build.
