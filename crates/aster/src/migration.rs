use std::{
    fmt, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
use syn::{Expr, Item, ReturnType, Stmt, Type};

pub const PINNED_RUSTC: &str = "1.99.1";
static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationReport {
    pub rustc_version: String,
    pub verified: bool,
    pub safety_obligations: Vec<crate::safety::Obligation>,
    pub notes: Vec<String>,
}

#[derive(Debug)]
pub struct MigrationError(pub String);
impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for MigrationError {}

/// Migrate the deliberately limited, standalone Rust executable subset.
/// Success means pinned-rustc validation, Aster native compilation, and matching
/// original/migrated entrypoint exit status and output; it is not a formal proof.
pub fn migrate_rust_file(input: &Path, output: &Path) -> Result<MigrationReport, MigrationError> {
    if input.extension().and_then(|e| e.to_str()) != Some("rs") {
        return Err(MigrationError(
            "Rust migration input must have a `.rs` extension".into(),
        ));
    }
    if input
        .ancestors()
        .any(|path| path.join("Cargo.toml").is_file())
    {
        return Err(MigrationError("Cargo project migration is not implemented; features, dependency resolution, build scripts, proc macros, targets, and tests must be handled before migrating a member source".into()));
    }
    if output.extension().and_then(|e| e.to_str()) != Some("as") {
        return Err(MigrationError(
            "Rust migration output must have a `.as` extension".into(),
        ));
    }
    let source = fs::read_to_string(input)
        .map_err(|e| MigrationError(format!("cannot read Rust source: {e}")))?;
    let compiler_version = check_pinned_rustc()?;
    let rustc_mir = temp_path("mir");
    let rustc_exe = temp_path("rust-bin");
    let aster_exe = temp_path("aster-bin");
    let mut cleanup = TempFiles(vec![
        rustc_mir.clone(),
        rustc_exe.clone(),
        aster_exe.clone(),
    ]);

    let analyzed = rustc(&[
        "--edition=2024",
        "--crate-type",
        "bin",
        "--emit=mir",
        "-o",
        rustc_mir
            .to_str()
            .ok_or_else(|| MigrationError("invalid temporary path".into()))?,
        input
            .to_str()
            .ok_or_else(|| MigrationError("input path is not UTF-8".into()))?,
    ])?;
    if !analyzed.status.success() {
        return Err(MigrationError(format!(
            "pinned rustc rejected source:\n{}",
            String::from_utf8_lossy(&analyzed.stderr)
        )));
    }
    let translated = translate_subset(&source)?;
    let aster_ast = crate::parser::parse(&translated)
        .map_err(|e| MigrationError(format!("generated Aster parse failed: {}", e.message)))?;
    let safety = crate::safety::verify(&aster_ast)
        .map_err(|e| MigrationError(format!("generated Aster safety verification failed: {e}")))?;
    crate::backend::build(&translated, &aster_exe)
        .map_err(|e| MigrationError(format!("generated Aster did not compile: {e}")))?;
    let built_original = rustc(&[
        "--edition=2024",
        "--crate-type",
        "bin",
        "-o",
        rustc_exe
            .to_str()
            .ok_or_else(|| MigrationError("invalid temporary path".into()))?,
        input
            .to_str()
            .ok_or_else(|| MigrationError("input path is not UTF-8".into()))?,
    ])?;
    if !built_original.status.success() {
        return Err(MigrationError(format!(
            "pinned rustc could not build original executable:\n{}",
            String::from_utf8_lossy(&built_original.stderr)
        )));
    }
    let original_run = Command::new(&rustc_exe)
        .output()
        .map_err(|e| MigrationError(format!("could not run original Rust executable: {e}")))?;
    let migrated_run = Command::new(&aster_exe)
        .output()
        .map_err(|e| MigrationError(format!("could not run migrated executable: {e}")))?;
    if !same_behavior(&original_run, &migrated_run) {
        return Err(MigrationError(
            "original and migrated entrypoints differ in exit status, stdout, or stderr".into(),
        ));
    }

    fs::write(output, &translated)
        .map_err(|e| MigrationError(format!("cannot write Aster output: {e}")))?;
    let report = MigrationReport {
        rustc_version: compiler_version,
        verified: true,
        safety_obligations: safety.obligations,
        notes: vec![
            "Original and migrated entrypoints were built and run; exit status, stdout, and stderr matched for this run.".into(),
            "Only private, zero-argument i32 functions and a simple main entrypoint are supported.".into(),
            "Rust comments are not preserved; Cargo projects, macros, generics, traits, ownership-bearing values, unsafe code, panic paths, and dependencies are rejected or unsupported.".into(),
            "Rust compiler MIR was emitted as a semantic validation check; Aster does not parse rustc's unstable textual MIR format or use rustc internal APIs.".into(),
        ],
    };
    let report_path = output.with_extension("migration.md");
    fs::write(&report_path, render_report(&report)).map_err(|e| {
        MigrationError(format!(
            "Aster output was written but migration report could not be written: {e}"
        ))
    })?;
    cleanup.0.clear();
    Ok(report)
}

fn check_pinned_rustc() -> Result<String, MigrationError> {
    let version = Command::new("rustup")
        .args(["run", PINNED_RUSTC, "rustc", "--version"])
        .output()
        .map_err(|e| {
            MigrationError(format!(
                "rustup is required to invoke pinned rustc {PINNED_RUSTC}: {e}"
            ))
        })?;
    if !version.status.success() {
        return Err(MigrationError(format!(
            "pinned rustc {PINNED_RUSTC} is unavailable; install it with rustup toolchain install {PINNED_RUSTC}"
        )));
    }
    let text = String::from_utf8_lossy(&version.stdout).trim().to_owned();
    if !text.starts_with(&format!("rustc {PINNED_RUSTC} ")) {
        return Err(MigrationError(format!(
            "Rust frontend requires rustc {PINNED_RUSTC}, got `{text}`"
        )));
    }
    Ok(text)
}

fn rustc(args: &[&str]) -> Result<Output, MigrationError> {
    Command::new("rustup")
        .arg("run")
        .arg(PINNED_RUSTC)
        .arg("rustc")
        .args(args)
        .output()
        .map_err(|e| MigrationError(format!("failed to run pinned rustc: {e}")))
}

fn translate_subset(source: &str) -> Result<String, MigrationError> {
    let file =
        syn::parse_file(source).map_err(|e| MigrationError(format!("Rust parse failed: {e}")))?;
    if file.items.is_empty() {
        return Err(MigrationError(
            "unsupported Rust source: empty crate".into(),
        ));
    }
    let mut functions = Vec::new();
    for item in file.items {
        let Item::Fn(function) = item else {
            return Err(MigrationError(
                "unsupported Rust item; only free functions are migratable".into(),
            ));
        };
        if !function.attrs.is_empty()
            || !matches!(function.vis, syn::Visibility::Inherited)
            || function.sig.asyncness.is_some()
            || function.sig.unsafety.is_some()
            || function.sig.abi.is_some()
            || !function.sig.generics.params.is_empty()
        {
            return Err(MigrationError(format!(
                "unsupported modifiers or attributes on `{}`",
                function.sig.ident
            )));
        }
        if !function.sig.inputs.is_empty() {
            return Err(MigrationError(format!(
                "unsupported parameters on `{}`; current Rust migration requires zero-argument functions",
                function.sig.ident
            )));
        }
        let name = function.sig.ident.to_string();
        if name == "main" {
            if !matches!(function.sig.output, ReturnType::Default) {
                return Err(MigrationError(
                    "Rust main must return unit in the supported subset".into(),
                ));
            }
            let body = match function.block.stmts.as_slice() {
                [] => "0".to_owned(),
                [Stmt::Expr(Expr::Call(call), Some(_))] if is_process_exit(&call.func) && call.args.len() == 1 => {
                    translate_expr(call.args.first().expect("one argument"))?
                }
                _ => return Err(MigrationError("unsupported main body; only empty main or `std::process::exit(constant_or_zero_arg_call)` is supported".into())),
            };
            functions.push(format!("fn main() -> i32 {{\n    return {body};\n}}"));
            continue;
        }
        if !is_i32_return(&function.sig.output)
            || !function
                .block
                .stmts
                .iter()
                .all(|stmt| matches!(stmt, Stmt::Expr(_, None)))
            || function.block.stmts.len() != 1
        {
            return Err(MigrationError(format!(
                "unsupported function `{name}`; expected a zero-argument `-> i32` function with one tail expression"
            )));
        }
        let Stmt::Expr(expr, None) = &function.block.stmts[0] else {
            unreachable!()
        };
        let result = translate_expr(expr)?;
        functions.push(format!("fn {name}() -> i32 {{\n    return {result};\n}}"));
    }
    if !functions.iter().any(|f| f.starts_with("fn main()")) {
        return Err(MigrationError(
            "unsupported Rust crate: missing `fn main()`".into(),
        ));
    }
    Ok(format!("{}\n", functions.join("\n\n")))
}

fn is_i32_return(output: &ReturnType) -> bool {
    match output {
        ReturnType::Type(_, ty) => {
            matches!(ty.as_ref(), Type::Path(path) if path.qself.is_none() && path.path.is_ident("i32"))
        }
        ReturnType::Default => false,
    }
}
fn is_process_exit(expr: &Expr) -> bool {
    let Expr::Path(path) = expr else {
        return false;
    };
    path.qself.is_none()
        && path
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            == ["std", "process", "exit"]
}
fn translate_expr(expr: &Expr) -> Result<String, MigrationError> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(value) => {
                let value: i32 = value.base10_parse().map_err(|_| MigrationError("unsupported integer literal: only i32-range decimal constants are supported".into()))?;
                Ok(value.to_string())
            }
            _ => Err(MigrationError("unsupported literal; only integer constants are supported".into())),
        },
        Expr::Call(call) if call.args.is_empty() => {
            let Expr::Path(path) = call.func.as_ref() else { return Err(MigrationError("unsupported indirect function call".into())); };
            if path.qself.is_some() || path.path.segments.len() != 1 { return Err(MigrationError("unsupported qualified function call".into())); }
            Ok(format!("{}()", path.path.segments[0].ident))
        }
        _ => Err(MigrationError("unsupported expression; only an i32 literal or direct zero-argument function call is supported".into())),
    }
}

fn same_behavior(a: &Output, b: &Output) -> bool {
    a.status.code() == b.status.code() && a.stdout == b.stdout && a.stderr == b.stderr
}
fn temp_path(extension: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "aster-rust-{}-{}.{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed),
        extension
    ))
}
struct TempFiles(Vec<PathBuf>);
impl Drop for TempFiles {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = fs::remove_file(path);
        }
    }
}
fn render_report(report: &MigrationReport) -> String {
    let obligations = report
        .safety_obligations
        .iter()
        .map(|o| {
            format!(
                "- `{}` / {}: {:?} — {}",
                o.function, o.property, o.status, o.rationale
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# Rust migration compatibility report\n\n- Rust compiler: `{}` (pinned)\n- Generated `.as` compiled with Aster: yes\n- Static safety verifier: passed for the scalar-only subset\n- Runtime memory-safety machinery: none in the accepted subset\n- Original/migrated entrypoint run matched: yes\n- Semantic equivalence proven: no\n\n## Safety obligations\n\n{}\n\n## Scope and limitations\n\n{}\n",
        report.rustc_version,
        obligations,
        report
            .notes
            .iter()
            .map(|n| format!("- {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[cfg(test)]
mod tests {
    use super::migrate_rust_file;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static NEXT: AtomicUsize = AtomicUsize::new(0);
    fn temp(ext: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "aster-migrate-{}-{}.{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
            ext
        ))
    }

    #[test]
    fn migrates_compiles_and_runtime_checks_simple_rust_main() {
        let rust = temp("rs");
        let aster = temp("as");
        fs::write(&rust, "fn main() { std::process::exit(7); }").unwrap();
        let report = migrate_rust_file(&rust, &aster).unwrap();
        assert!(report.verified);
        assert_eq!(
            fs::read_to_string(&aster).unwrap(),
            "fn main() -> i32 {\n    return 7;\n}\n"
        );
        let report_text = fs::read_to_string(aster.with_extension("migration.md")).unwrap();
        assert!(report_text.contains("Safety obligations"));
        assert!(report_text.contains("Runtime memory-safety machinery: none"));
        let _ = fs::remove_file(rust);
        let _ = fs::remove_file(aster.with_extension("migration.md"));
        let _ = fs::remove_file(aster);
    }

    #[test]
    fn migrates_called_zero_arg_helper_and_compares_its_result() {
        let rust = temp("rs");
        let aster = temp("as");
        fs::write(
            &rust,
            "fn answer() -> i32 { 42 } fn main() { std::process::exit(answer()); }",
        )
        .unwrap();
        let report = migrate_rust_file(&rust, &aster).unwrap();
        assert!(report.verified);
        let generated = fs::read_to_string(&aster).unwrap();
        assert!(generated.contains("fn answer() -> i32"));
        assert!(generated.contains("return answer();"));
        let _ = fs::remove_file(rust);
        let _ = fs::remove_file(aster.with_extension("migration.md"));
        let _ = fs::remove_file(aster);
    }

    #[test]
    fn refuses_to_treat_a_cargo_member_as_a_standalone_crate() {
        let dir = temp("dir");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("Cargo.toml"),
            "[package]\\nname='probe'\\nversion='0.1.0'\\nedition='2024'\\n",
        )
        .unwrap();
        let rust = dir.join("main.rs");
        let aster = dir.join("main.as");
        fs::write(&rust, "fn main() {}").unwrap();
        let error = migrate_rust_file(&rust, &aster).unwrap_err();
        assert!(
            error
                .0
                .contains("Cargo project migration is not implemented")
        );
        assert!(!aster.exists());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn rejects_unsupported_rust_without_emitting_a_migration() {
        let rust = temp("rs");
        let aster = temp("as");
        fs::write(&rust, "fn main() { println!(\"hello\"); }").unwrap();
        let error = migrate_rust_file(&rust, &aster).unwrap_err();
        assert!(error.0.contains("unsupported"));
        assert!(!aster.exists());
        let _ = fs::remove_file(rust);
    }
}
