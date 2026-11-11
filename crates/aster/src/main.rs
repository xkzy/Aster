use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        eprintln!(
            "usage: aster check <source.ast> | aster build <source.ast> -o <executable> | aster migrate <language> <source> -o <output.as> | aster ghidra lower <export.json> -o <output.as>"
        );
        return ExitCode::from(2);
    };
    if command == "ghidra" {
        if args.next().as_deref() != Some("lower") {
            eprintln!("usage: aster ghidra lower <export.json> -o <output.as>");
            return ExitCode::from(2);
        }
        let Some(input) = args.next() else {
            eprintln!("aster: missing Ghidra export JSON path");
            return ExitCode::from(2);
        };
        if args.next().as_deref() != Some("-o") {
            eprintln!("usage: aster ghidra lower <export.json> -o <output.as>");
            return ExitCode::from(2);
        }
        let Some(output) = args.next() else {
            eprintln!("aster: missing Aster output path");
            return ExitCode::from(2);
        };
        if args.next().is_some() {
            eprintln!("aster: unexpected extra arguments");
            return ExitCode::from(2);
        }
        return match aster::ghidra::lower_file(input.as_ref(), output.as_ref()) {
            Ok(report) if report.complete => {
                println!(
                    "Ghidra P-code lowering compiled; complete for the supported subset: `{output}`"
                );
                ExitCode::SUCCESS
            }
            Ok(report) => {
                let report_path =
                    std::path::Path::new(&output).with_extension("ghidra-report.json");
                eprintln!(
                    "Ghidra lowering is partial; see `{}`; unsupported functions were not emitted",
                    report_path.display()
                );
                if report.generated_source_compiles {
                    println!("safe supported subset written to `{output}`");
                }
                ExitCode::FAILURE
            }
            Err(error) => {
                eprintln!("Ghidra lowering failed: {error}");
                ExitCode::FAILURE
            }
        };
    }
    if command == "migrate" {
        let Some(language) = args.next() else {
            eprintln!("usage: aster migrate <c|cpp|rust|python> <source> -o <output.as>");
            return ExitCode::from(2);
        };
        if language == "c" || language == "cpp" {
            let Some(input) = args.next() else {
                eprintln!("aster: missing C/C++ source path");
                return ExitCode::from(2);
            };
            if args.next().as_deref() != Some("-o") {
                eprintln!("usage: aster migrate <c|cpp> <source> -o <output.as>");
                return ExitCode::from(2);
            }
            let Some(output) = args.next() else {
                eprintln!("aster: missing Aster output path");
                return ExitCode::from(2);
            };
            if args.next().is_some() {
                eprintln!("aster: unexpected extra arguments");
                return ExitCode::from(2);
            }
            let dialect = if language == "c" {
                aster::migration_c::Dialect::C
            } else {
                aster::migration_c::Dialect::Cpp
            };
            return match aster::migration_c::migrate_c_file(
                input.as_ref(),
                output.as_ref(),
                dialect,
            ) {
                Ok(_) => {
                    println!(
                        "{language} migration verified; static safety obligations passed: `{output}`"
                    );
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("{language} migration failed: {error}");
                    ExitCode::FAILURE
                }
            };
        }
        if language == "python" {
            let Some(input) = args.next() else {
                eprintln!("aster: missing Python source path");
                return ExitCode::from(2);
            };
            if args.next().as_deref() != Some("-o") {
                eprintln!("usage: aster migrate python <source.py> -o <output.as>");
                return ExitCode::from(2);
            }
            let Some(output) = args.next() else {
                eprintln!("aster: missing Aster output path");
                return ExitCode::from(2);
            };
            if args.next().is_some() {
                eprintln!("aster: unexpected extra arguments");
                return ExitCode::from(2);
            }
            return match aster::migration_python::migrate_python_file(
                input.as_ref(),
                output.as_ref(),
            ) {
                Ok(report) => {
                    println!(
                        "Python migration verified with {}: `{output}`",
                        report.python_version
                    );
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("Python migration failed: {error}");
                    ExitCode::FAILURE
                }
            };
        }
        if language != "rust" {
            eprintln!(
                "strict migration refused for `{language}`: no verified frontend is implemented; no Aster output was generated"
            );
            return ExitCode::FAILURE;
        }
        let Some(input) = args.next() else {
            eprintln!("aster: missing Rust source path");
            return ExitCode::from(2);
        };
        if args.next().as_deref() != Some("-o") {
            eprintln!("usage: aster migrate rust <source.rs> -o <output.as>");
            return ExitCode::from(2);
        }
        let Some(output) = args.next() else {
            eprintln!("aster: missing Aster output path");
            return ExitCode::from(2);
        };
        if args.next().is_some() {
            eprintln!("aster: unexpected extra arguments");
            return ExitCode::from(2);
        }
        return match aster::migration::migrate_rust_file(input.as_ref(), output.as_ref()) {
            Ok(report) => {
                println!(
                    "Rust migration verified with {}: `{output}`",
                    report.rustc_version
                );
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("Rust migration failed: {error}");
                ExitCode::FAILURE
            }
        };
    }
    let Some(path) = args.next() else {
        eprintln!("aster: missing source path");
        return ExitCode::from(2);
    };
    let output = match command.as_str() {
        "check" => {
            if args.next().is_some() {
                eprintln!("aster: unexpected extra arguments");
                return ExitCode::from(2);
            }
            None
        }
        "build" => {
            if args.next().as_deref() != Some("-o") {
                eprintln!("usage: aster build <source.ast> -o <executable>");
                return ExitCode::from(2);
            }
            let Some(output) = args.next() else {
                eprintln!("aster: missing output path");
                return ExitCode::from(2);
            };
            if args.next().is_some() {
                eprintln!("aster: unexpected extra arguments");
                return ExitCode::from(2);
            }
            Some(output)
        }
        _ => {
            eprintln!(
                "aster: unsupported command `{command}`; expected `check`, `build`, `migrate`, or `ghidra lower`"
            );
            return ExitCode::from(2);
        }
    };
    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("aster: cannot read `{path}`: {error}");
            return ExitCode::FAILURE;
        }
    };
    if let Some(output) = output {
        return match aster::backend::build(&source, output.as_ref()) {
            Ok(()) => {
                println!("built native executable `{output}`");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("{path}: {error}");
                ExitCode::FAILURE
            }
        };
    }
    let program = match aster::parser::parse(&source) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{path}:{}: parse error: {}", error.offset, error.message);
            return ExitCode::FAILURE;
        }
    };
    match aster::safety::verify(&program) {
        Ok(report) => {
            println!(
                "{path}: statically memory-safe for the scalar subset ({} proof obligations; no runtime safety checks)",
                report.obligations.len()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{path}: static safety verification failed: {error}");
            ExitCode::FAILURE
        }
    }
}
