![Rukn — ركن](docs/public/rukn-wordmark.svg)

# Rukn — ركن

**An experimental Arabic-first programming language, implemented in Rust.**

Rukn explores programming syntax and diagnostics designed to read naturally in Arabic while remaining interoperable with the wider software ecosystem. The repository contains the language implementation, draft specification, tests, research notes, and multilingual documentation.

- **Arabic-first:** Arabic keywords, Unicode identifiers, Arabic diagnostics and Arabic-Indic numerals.
- **Explicit semantics:** no implicit conversion between strings and numbers.
- **Rust implementation:** a small tree-walk interpreter with no unsafe Rust.
- **Evidence-led research:** language decisions distinguish implemented behavior from open questions and future proposals.
- **Open development:** behavior is documented and tested; research sources are linked from the [technical reading list](docs/reference/research-reading-list.md).

## Current status

**Experimental, pre-alpha. Not for production use.** The initial interpreter supports variable declarations, printing, numbers, strings, booleans, a nil-like value, arithmetic, comparisons and equality. It is not self-hosting.

Not implemented yet: functions, loops, modules, collections, a package manager, bytecode VM, native compiler, IDE integration, language-level file/network access or a stable 1.0 specification.

## Example

```rukn
دع الاسم = «العالم»؛
اطبع(«مرحبًا يا » + الاسم)؛
دع الناتج = ٦ * ٧؛
اطبع(الناتج)؛
اطبع(صحيح == خطأ)؛
```

Expected output:

```text
مرحبًا يا العالم
42
خطأ
```

## Build and test

Requirements: stable Rust with edition 2024 support.

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo run --locked -- examples/hello.rukn
```

The command-line runner accepts one UTF-8 source file. The VitePress site includes the language guide, draft specification, architecture, research register, Unicode security notes, testing strategy, roadmap and contribution rules.

- **Documentation website:** https://deku-foundation.github.io/deku-rukn/
- **Arabic overview:** [docs/ar/index.md](docs/ar/index.md)
- **English overview:** [docs/index.md](docs/index.md)
- **French overview:** [docs/fr/index.md](docs/fr/index.md)
- **Research reading list:** [docs/reference/research-reading-list.md](docs/reference/research-reading-list.md)
- **Visual identity guidelines:** [docs/project/visual-identity.md](docs/project/visual-identity.md)

## Project roadmap

1. Strengthen the language specification, diagnostics and conformance tests.
2. Define an explicit Unicode identifier profile with security and compatibility review.
3. Add reviewed control-flow and function semantics.
4. Build formatter and language-server tooling.
5. Compare interpreter, bytecode and native back ends using correctness suites and reproducible benchmarks.
6. Explore self-hosting only after a reliable compiler bootstrap process exists.

Self-hosting is a long-term research goal, not a current feature or a substitute for a stable specification.

## Branch policy

Keep only `main` and `dev` as persistent branches. Make routine changes on `dev`, run the relevant quality gates, and promote to `main` through a reviewed pull request after the checks pass. Avoid long-lived feature branches; the branch-hygiene workflow removes redundant branches after integration.

## Contributing and security

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. Security limitations and reporting guidance are in [SECURITY.md](SECURITY.md). Language changes should include tests and updates to the specification and translated documentation.

## Brand assets

The editable SVG assets in `docs/public/` use the Deku Foundation evergreen palette and are available under the repository's project licensing terms. See the [visual identity guidelines](docs/project/visual-identity.md) before changing colours, spacing or logo variants.
