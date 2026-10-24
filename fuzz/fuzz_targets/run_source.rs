#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    // The public interpreter accepts &str. Lossy decoding keeps arbitrary byte
    // sequences useful to the lexer while ensuring the target always supplies
    // valid UTF-8. CLI invalid-UTF-8 handling is covered by unit tests.
    let source = String::from_utf8_lossy(bytes);
    let _ = rukn::run(&source);
});
