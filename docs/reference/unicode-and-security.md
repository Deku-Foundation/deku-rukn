---
title: Unicode and identifier security
description: Research-backed design questions for Arabic identifiers, normalization, confusables, and bidirectional controls in Rukn.
---

# Unicode and identifier security

**Status: implementation baseline plus open security questions. This page does not claim complete UAX #31 conformance.**

Arabic-first programming makes Unicode behavior part of the language design, not a cosmetic feature. Rukn uses pinned `unicode-ident 1.0.27` for `XID_Start` / `XID_Continue` and pinned `unicode-normalization 0.1.25` to normalize identifier tokens to NFC before keyword recognition and name lookup. This is a deliberate, tested identifier policy, not complete UAX #31 or UTS #55 conformance.

## Current implementation

- Source is UTF-8.
- Identifier start is underscore or Unicode `XID_Start`; continuation is underscore or `XID_Continue`, provided by the pinned `unicode-ident 1.0.27` dependency.
- ASCII digits, Arabic-Indic digits (٠–٩), and Eastern Arabic-Indic digits (۰–۹) are accepted in numeric literals.
- Digits in numeric literals are normalized to ASCII before parsing.
- XID continuation allows supported combining marks after the first character, so Arabic diacritics may appear inside identifiers.
- The regression suite also exercises a supplementary-plane `XID_Start` character (`U+10400`) followed by Arabic identifier characters. This guards code-point handling without claiming that every Unicode character is a valid identifier.
- Identifier tokens are normalized to NFC before keyword recognition and name lookup. Canonically equivalent spellings resolve to one binding; token line/column positions continue to refer to the original source. String-literal contents and comments are not normalized.
- NFC normalization does not detect visually confusable names or mixed scripts. Those warnings, and syntax-aware source presentation / dedicated bidi diagnostics, remain open.
- If the lexer rejects a character outside strings and comments, the diagnostic includes its `U+XXXX` code point and escaped form. This does not inspect bidirectional controls inside comments or string values.
- Accepted XID characters follow the data bundled with the pinned dependency; update that dependency only with identifier conformance tests and a compatibility review.

This is a more consistent lexical baseline, but not a complete Unicode security profile. Confusable detection, mixed-script policy, invisible-control handling, and syntax-aware right-to-left source presentation remain open.

## Research baseline

### 1. Define a deliberate identifier profile

[Unicode Standard Annex #31 (UAX #31)](https://www.unicode.org/reports/tr31/) describes recommended identifier properties and tailoring. Rukn has adopted `XID_Start` / `XID_Continue` as the current lexical baseline, with a documented underscore tailoring and a pinned implementation. The next step is to maintain a conformance corpus and explicitly review future Unicode table updates.

The review must specifically test Arabic combining marks, diacritics, Persian characters, Arabic and Persian variants of Yeh/Kaf, join controls, supplementary-plane characters, and malformed or unsupported code points. The accepted set should be listed by Unicode properties and version—not as an open-ended claim that “all Unicode works.”

### 2. Choose normalization semantics explicitly

[Unicode Standard Annex #15 (UAX #15)](https://www.unicode.org/reports/tr15/) defines normalization forms. Rukn now applies NFC before keyword matching and name resolution using the pinned `unicode-normalization 0.1.25` dependency. The conformance tests prove that Arabic alef plus combining hamza above resolves as the precomposed alef-with-hamza identifier, and that string-literal content remains unchanged. The package version and lockfile are pinned; a dependency update requires compatibility review.

Do not normalize string-literal contents merely because identifier normalization is selected. Identifiers and user text have different semantics. Since NFC can make previously distinct source spellings equivalent, the language specification must keep this collision behavior explicit. Unicode normalization stability does not eliminate the need to review XID-property updates separately.

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
- NFC identifier normalization and string-preservation behavior are tested and documented.
- Confusable/mixed-script policy and right-to-left source presentation receive a separate reviewed design.
- Invisible-control handling is reviewed for editors, terminals, and diagnostics.
- Changes to the identifier profile have a compatibility and migration note.

## Sources

- [UAX #31 — Unicode Identifiers and Syntax](https://www.unicode.org/reports/tr31/)
- [UAX #15 — Unicode Normalization Forms](https://www.unicode.org/reports/tr15/)
- [UTS #39 — Unicode Security Mechanisms](https://www.unicode.org/reports/tr39/)
- [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/)

These standards inform the research; they do not imply that Rukn currently conforms to them.
