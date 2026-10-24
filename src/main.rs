use std::{env, fs::File, io::Read, path::Path, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let Some(path) = args.next() else {
        eprintln!("الاستخدام: {} <ملف.ركن>", program.to_string_lossy());
        eprintln!("مثال: {} examples/hello.rukn", program.to_string_lossy());
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("خطأ: يُسمح بملف مصدر واحد في هذه النسخة.");
        return ExitCode::from(2);
    }

    let source = match read_source(&path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    match rukn::run(&source) {
        Ok(output) => {
            for line in output {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Read at most one byte beyond the language source limit before rejecting a file.
/// This keeps the CLI's memory bound effective before the interpreter receives the source.
fn read_source(path: &std::ffi::OsStr) -> Result<String, String> {
    let display_path = Path::new(path).to_string_lossy();
    let file = File::open(path)
        .map_err(|error| format!("تعذر قراءة الملف {}: {}", display_path, error))?;
    let limit = rukn::MAX_SOURCE_BYTES as u64 + 1;
    let mut reader = file.take(limit);
    let mut bytes = Vec::with_capacity(rukn::MAX_SOURCE_BYTES.min(64 * 1024));

    reader
        .read_to_end(&mut bytes)
        .map_err(|error| format!("تعذر قراءة الملف {}: {}", display_path, error))?;

    if bytes.len() > rukn::MAX_SOURCE_BYTES {
        return Err(format!(
            "تجاوز حجم المصدر الحد الأقصى ({} بايت): {}",
            rukn::MAX_SOURCE_BYTES,
            display_path
        ));
    }

    String::from_utf8(bytes).map_err(|error| {
        format!(
            "ملف المصدر ليس UTF-8 صالحًا: {}: {}",
            display_path, error
        )
    })
}

#[cfg(test)]
mod tests {
    use super::read_source;
    use std::{fs, io::Write, path::PathBuf};

    fn temporary_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rukn-cli-{}-{}.rukn",
            std::process::id(),
            label
        ))
    }

    #[test]
    fn reads_valid_utf8_source_without_changing_it() {
        let path = temporary_path("valid-utf8");
        fs::write(&path, "اطبع(١)؛").unwrap();

        let source = read_source(path.as_os_str()).unwrap();

        assert_eq!(source, "اطبع(١)؛");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejects_oversized_source_files() {
        let path = temporary_path("oversized");
        let mut file = fs::File::create(&path).unwrap();
        let block = [b' '; 64 * 1024];
        let mut remaining = rukn::MAX_SOURCE_BYTES + 1;

        while remaining > 0 {
            let amount = remaining.min(block.len());
            file.write_all(&block[..amount]).unwrap();
            remaining -= amount;
        }
        drop(file);

        let error = read_source(path.as_os_str()).unwrap_err();

        assert!(error.contains("الحد الأقصى"), "{error}");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejects_non_utf8_source_files_with_a_clear_error() {
        let path = temporary_path("invalid-utf8");
        fs::write(&path, [0xFF, 0xFE]).unwrap();

        let error = read_source(path.as_os_str()).unwrap_err();

        assert!(error.contains("UTF-8 صالح"), "{error}");
        fs::remove_file(path).unwrap();
    }
}
