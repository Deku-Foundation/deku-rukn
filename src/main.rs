use std::{env, fs, process::ExitCode};

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

    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("تعذر قراءة الملف {}: {error}", path.to_string_lossy());
            return ExitCode::FAILURE;
        }
    };

    match rukn::run(&source) {
        Ok(output) => {
            for line in output { println!("{line}"); }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
