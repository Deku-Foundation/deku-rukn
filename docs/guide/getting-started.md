---
title: Install and run
description: Build the Rukn interpreter from source and run your first Arabic program.
---

# Install and run

Rukn currently runs from source with the Rust toolchain. There are no official binary releases yet.

## Requirements

- Rust stable toolchain with edition 2024 support.
- Git.
- A terminal with UTF-8 support and an Arabic-capable font.

## Build and test

    git clone https://github.com/Deku-Foundation/deku-rukn.git
    cd deku-rukn
    cargo test --locked
    cargo build --release --locked

Run the included example:

    cargo run --locked -- examples/hello.rukn

Run the named-functions and recursion example:

    cargo run --locked -- examples/functions.rukn

Expected output for `hello.rukn`:

    مرحبًا يا العالم
    42
    خطأ

## Write a program

Create a file named hello.rukn:

    # A comment begins with #
    دع الاسم = «العالم»؛
    اطبع(«مرحبًا يا » + الاسم)؛
    دع العدد = ٦ * ٧؛
    اطبع(العدد)؛

Then run cargo run -- hello.rukn.

## Troubleshooting

- If Rust is too old, install the current stable toolchain using the official Rust instructions.
- If Arabic appears disconnected or reversed, use a terminal and font with Arabic shaping support.
- Every statement currently ends with ؛ or ;. String literals must end on the same line.
- Errors include the source line and column.

The CLI accepts one source file. A REPL, closures, modules/imports, collections, language-level file I/O, package manager, bytecode VM and native compilation are not implemented yet.
