---
title: Unicode and identifier security
description: Research-backed design questions for Arabic identifiers, normalization, confusables, and bidirectional controls in Rukn.
---

# Unicode and identifier security

**Status: research note. No new identifier policy is adopted by this page.**

Arabic-first programming makes Unicode behavior part of the language design, not a cosmetic feature. The current lexer accepts Unicode alphabetic characters at the start of an identifier, and alphabetic or numeric characters after it; underscore is also accepted. The current implementation does not define normalization, visual-confusable handling, or a dedicated policy for bidirectional formatting controls.

## Current implementation

- Source is UTF-8.
- Identifier start is underscore or Rust's `char::is_alphabetic`; continuation additionally accepts `char::is_numeric`.
- ASCII digits, Arabic-Indic digits (٠–٩), and Eastern Arabic-Indic digits (۰–۹) are accepted in numeric literals.
- Digits in numeric literals are normalized to ASCII before parsing.
- The lexer does not currently use Unicode's `XID_Start` / `XID_Continue` profile and does not normalize identifier names.

This behavior is a useful prototype, not a complete Unicode identifier specification.

## Research baseline

### 1. Define a deliberate identifier profile

[Unicode Standard Annex #31 (UAX #31)](https://www.unicode.org/reports/tr31/) describes recommended identifier properties and tailoring. Rukn should evaluate a profile based on `XID_Start` and `XID_Continue`, with a documented underscore rule, rather than treating all alphabetic/numeric characters as an indefinite policy.

The review must specifically test Arabic combining marks, diacritics, Persian characters, Arabic and Persian variants of Yeh/Kaf, join controls, supplementary-plane characters, and malformed or unsupported code points. The accepted set should be listed by Unicode properties and version—not as an open-ended claim that “all Unicode works.”

### 2. Choose normalization semantics explicitly

[Unicode Standard Annex #15 (UAX #15)](https://www.unicode.org/reports/tr15/) defines normalization forms. A language decision must state whether canonically equivalent identifier spellings resolve to the same binding, whether normalization occurs before lookup, and what spelling diagnostics preserve.

Do not normalize string-literal contents merely because identifier normalization is selected. Identifiers and user text have different semantics. If normalization is introduced, pin the Unicode version, add compatibility tests, and document any change that can make previously distinct identifiers collide.

### 3. Treat confusables and mixed scripts as a tooling concern

[Unicode Technical Standard #39 (UTS #39)](https://www.unicode.org/reports/tr39/) describes mechanisms for detecting visually confusable text and suspicious mixed-script identifiers. A future linter could warn about identifiers that are visually ambiguous or mix scripts unexpectedly.

A blanket ban on Latin characters would make interoperation and common technical identifiers awkward. Prefer a documented identifier profile and actionable warnings over silently rewriting names or rejecting every mixed-script identifier. The exact rule remains open.

### 4. Make bidirectional controls visible

[Unicode Standard Annex #9 (UAX #9)](https://www.unicode.org/reports/tr9/) specifies the Unicode Bidirectional Algorithm. Rukn tooling should decide how to detect or display invisible directional controls in source, comments, strings, and diagnostics. Diagnostics and code review views should make suspicious invisible characters inspectable without corrupting legitimate Arabic text.

## Recommended decision process

1. Write a short design proposal with examples of valid, invalid, and ambiguous identifiers.
2. Compare a UAX #31-based profile, a narrower Arabic-oriented profile, and the current Rust-character-property behavior.
3. Add tests for combining marks, normalization pairs, mixed scripts, controls, numeral families, and display columns.
4. Review compatibility and migration consequences before changing name lookup.
5. Record the outcome as an accepted or rejected design decision and version the chosen Unicode data.

## Acceptance criteria before a stable language release

- A normative identifier grammar and Unicode version are specified.
- The implementation and specification share a conformance corpus.
- Normalization and confusable behavior are tested and documented.
- Invisible-control handling is reviewed for editors, terminals, and diagnostics.
- Changes to the identifier profile have a compatibility and migration note.

## Sources

- [UAX #31 — Unicode Identifiers and Syntax](https://www.unicode.org/reports/tr31/)
- [UAX #15 — Unicode Normalization Forms](https://www.unicode.org/reports/tr15/)
- [UTS #39 — Unicode Security Mechanisms](https://www.unicode.org/reports/tr39/)
- [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/)

These standards inform the research; they do not imply that Rukn currently conforms to them.
