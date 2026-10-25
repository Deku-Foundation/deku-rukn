use std::fs;
use std::process::Command;

const CORPUS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/conformance/cases.tsv");

fn unescape(field: &str, line_number: usize, field_name: &str) -> String {
    let mut output = String::with_capacity(field.len());
    let mut chars = field.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some('t') => output.push('\t'),
            Some('\\') => output.push('\\'),
            Some(other) => panic!(
                "{}:{}: unsupported escape \\{} in {}",
                CORPUS_PATH, line_number, other, field_name
            ),
            None => panic!(
                "{}:{}: incomplete escape in {}",
                CORPUS_PATH, line_number, field_name
            ),
        }
    }

    output
}

#[test]
fn cli_matches_machine_readable_conformance_corpus() {
    let corpus = fs::read_to_string(CORPUS_PATH).expect("read conformance corpus");
    let mut case_count = 0usize;

    for (zero_based_line, row) in corpus.lines().enumerate() {
        let line_number = zero_based_line + 1;
        if row.trim().is_empty() || row.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> = row.split('\t').collect();
        assert_eq!(
            fields.len(),
            5,
            "{}:{}: expected 5 tab-separated fields, found {}",
            CORPUS_PATH,
            line_number,
            fields.len()
        );

        let name = fields[0];
        assert!(
            !name.is_empty()
                && name
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_'),
            "{}:{}: invalid case name {:?}",
            CORPUS_PATH,
            line_number,
            name
        );
        let expected_exit: i32 = fields[1].parse().unwrap_or_else(|_| {
            panic!(
                "{}:{}: exit status must be an integer",
                CORPUS_PATH, line_number
            )
        });
        let expected_stdout = unescape(fields[2], line_number, "stdout");
        let expected_stderr_fragment = unescape(fields[3], line_number, "stderr fragment");
        let source = unescape(fields[4], line_number, "source");

        let source_path = std::env::temp_dir().join(format!(
            "rukn-conformance-{}-{name}.rukn",
            std::process::id()
        ));
        fs::write(&source_path, source.as_bytes()).unwrap_or_else(|error| {
            panic!(
                "{}:{}: could not write temporary source: {}",
                CORPUS_PATH, line_number, error
            )
        });

        let result = Command::new(env!("CARGO_BIN_EXE_rukn"))
            .arg(&source_path)
            .output()
            .unwrap_or_else(|error| {
                panic!(
                    "{}:{} [{}]: failed to execute CLI: {}",
                    CORPUS_PATH, line_number, name, error
                )
            });
        let _ = fs::remove_file(&source_path);

        let stdout = String::from_utf8(result.stdout).expect("CLI stdout must be UTF-8");
        let stderr = String::from_utf8(result.stderr).expect("CLI stderr must be UTF-8");
        assert_eq!(
            result.status.code(),
            Some(expected_exit),
            "{}:{} [{}]: unexpected exit code; stderr: {:?}",
            CORPUS_PATH,
            line_number,
            name,
            stderr
        );
        assert_eq!(
            stdout, expected_stdout,
            "{}:{} [{}]: stdout mismatch",
            CORPUS_PATH, line_number, name
        );

        if expected_stderr_fragment.is_empty() {
            assert!(
                stderr.is_empty(),
                "{}:{} [{}]: expected empty stderr, got {:?}",
                CORPUS_PATH,
                line_number,
                name,
                stderr
            );
        } else {
            assert!(
                stderr.contains(&expected_stderr_fragment),
                "{}:{} [{}]: stderr {:?} does not contain {:?}",
                CORPUS_PATH,
                line_number,
                name,
                stderr,
                expected_stderr_fragment
            );
        }

        case_count += 1;
    }

    assert!(
        case_count >= 10,
        "conformance corpus unexpectedly small: {case_count} cases"
    );
}
