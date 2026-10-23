# Rukn — ركن

**An experimental Arabic-first programming language, implemented in Rust.**

Rukn explores programming syntax and diagnostics designed to read naturally in Arabic while remaining interoperable with the wider software ecosystem. The repository contains the language implementation, draft specification, tests, and multilingual documentation in one place.

- **Arabic-first:** Arabic keywords, Unicode identifiers, Arabic diagnostics and Arabic-Indic numerals.
- **Explicit semantics:** no implicit conversion between strings and numbers.
- **Rust implementation:** a small tree-walk interpreter with no unsafe Rust.
- **Open development:** language behavior is documented and tested; future ideas are marked as proposals.

## Current status

**Experimental, pre-alpha. Not for production use.** The initial interpreter supports variable declarations, printing, numbers, strings, booleans, a nil-like value, arithmetic, comparisons and equality. It is not self-hosting.

Not implemented yet: functions, loops, modules, collections, a package manager, bytecode VM, native compiler, IDE integration, language-level file/network access or a stable 1.0 specification.

## Example

    دع الاسم = «العالم»؛
    اطبع(«مرحبًا يا » + الاسم)؛
    دع الناتج = ٦ * ٧؛
    اطبع(الناتج)؛

Output:

    مرحبًا يا العالم
    42

## Build and test

Requirements: current stable Rust with edition 2024 support.

    cargo fmt --all
    cargo test --locked
    cargo clippy --all-targets --all-features --locked -- -D warnings
    cargo run --locked -- examples/hello.rukn

The command-line runner accepts one UTF-8 source file. See the [documentation site](https://deku-foundation.github.io/deku-rukn/) for the getting-started guide, grammar, architecture, roadmap and contributing rules.

## Documentation languages

- Arabic: [نظرة عامة](docs/ar/index.md)
- English: [Overview](docs/index.md)
- French: [Présentation](docs/fr/index.md)

## Roadmap

1. Harden parser limits, numeric behavior, and Unicode identifier handling.
2. Stabilize grammar, diagnostics and a machine-readable conformance test suite.
3. Add reviewed control-flow and function semantics.
4. Build formatter and language-server tooling.
5. Evaluate bytecode or native compilation using benchmarks and a stable intermediate representation.
6. Explore self-hosting only after a reliable bootstrap process exists.

Self-hosting is a long-term research goal, not a current feature or a substitute for a stable specification.

## Contributing and security

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. Security limitations and reporting guidance are in [SECURITY.md](SECURITY.md). Language changes should include tests and updates to the specification and translated documentation.
