# Contributing to Rukn

Rukn is experimental language research. Please open an issue before large changes to syntax, semantics, compiler architecture or package format.

## Development checks

    cargo fmt --all -- --check
    cargo test --locked
    cargo clippy --all-targets --all-features --locked -- -D warnings
    cargo run --locked -- examples/hello.rukn

For documentation, use the checked-in lockfile so CI and local validation install the same dependency graph:

    cd docs
    npm ci --no-audit --no-fund
    npm run build

## Change expectations

- Add tests for successful and failing programs.
- Update the language specification when syntax or semantics change.
- Keep Arabic, French and English documentation aligned for user-visible changes.
- Do not claim a feature is implemented until code, tests and documentation agree.
- Keep pull requests focused and explain compatibility impact.
