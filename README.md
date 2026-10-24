<p align="center">
  <img src="docs/public/rukn-mark.svg" alt="Rukn — ركن logo" width="76" height="76">
</p>

<h1 align="center">ركن · Rukn</h1>

<p align="center"><strong>An experimental Arabic-first programming language, implemented in Rust.</strong></p>

<p align="center">
  <a href="https://deku-foundation.github.io/deku-rukn/">Documentation</a> ·
  <a href="https://deku-foundation.github.io/deku-rukn/ar/">التوثيق العربي</a> ·
  <a href="https://github.com/Deku-Foundation/deku-rukn/actions">CI workflows</a>
</p>

Rukn explores programming syntax and diagnostics designed to read naturally in Arabic while remaining interoperable with the wider software ecosystem. This repository contains the language implementation, draft specification, tests, research notes, and multilingual documentation.

## Project status

**Experimental · pre-alpha · not for production use.**

The initial Rust tree-walk interpreter supports variable declarations, printing, numbers, strings, booleans, a nil-like value, arithmetic, comparisons and equality. Rukn is not self-hosting.

Not implemented yet: functions, loops, modules, collections, a package manager, bytecode VM, native compiler, IDE integration, language-level file/network access or a stable 1.0 specification.

## Example

```rukn
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

## Design principles

- **Arabic-first syntax:** Arabic keywords, Unicode identifiers and Arabic-Indic numerals.
- **Explicit semantics:** no implicit conversion between strings and numbers.
- **Unicode-aware engineering:** identifier rules are tested against a documented profile; normalization and visual-confusable policy remain explicit research questions.
- **Evidence-led development:** language behavior is specified and tested, while proposed features are labelled as proposals.
- **Safe implementation:** Rust implementation with no unsafe Rust.

## Build and test

Requirements: stable Rust with edition 2024 support.

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo run --locked -- examples/hello.rukn
```

## Documentation

- **Website:** https://deku-foundation.github.io/deku-rukn/
- **Arabic overview:** [docs/ar/index.md](docs/ar/index.md)
- **English overview:** [docs/index.md](docs/index.md)
- **French overview:** [docs/fr/index.md](docs/fr/index.md)
- **Research reading list:** [docs/reference/research-reading-list.md](docs/reference/research-reading-list.md)
- **Accessibility and contrast policy:** [docs/reference/documentation-accessibility.md](docs/reference/documentation-accessibility.md)
- **Visual identity:** [docs/project/visual-identity.md](docs/project/visual-identity.md)

## Roadmap

1. Strengthen the language specification, diagnostics and conformance tests.
2. Define and test an explicit Unicode identifier and security profile.
3. Add reviewed control-flow and function semantics.
4. Build formatter and language-server tooling.
5. Compare interpreter, bytecode and native back ends with correctness suites and reproducible benchmarks.
6. Explore self-hosting only after a reliable compiler bootstrap process exists.

Self-hosting is a long-term research goal, not a current feature or a substitute for a stable specification.

## Branch policy

Keep only `main` and `dev` as persistent branches. Make routine changes on `dev`, run the relevant quality gates, and promote to `main` through a reviewed pull request after checks pass. Avoid long-lived feature branches.

## Contributing and security

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. Security limitations and reporting guidance are in [SECURITY.md](SECURITY.md). Language changes should include tests and updates to the specification and translated documentation.

## Brand assets

The icon and wordmark SVGs live in `docs/public/`. The README uses the compact mark rather than embedding Arabic typography in a wide SVG, avoiding SVG text-shaping inconsistencies across Markdown renderers. See the [visual identity guide](docs/project/visual-identity.md) before changing colors, spacing or logo variants.
