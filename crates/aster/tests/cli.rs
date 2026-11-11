use std::{
    fs,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

fn source_file(contents: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "aster-{}-{}.ast",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn check_command_accepts_well_typed_source() {
    let path = source_file("fn main() -> i32 { return 6 * 7; }");
    let output = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args(["check", path.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = fs::remove_file(path);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("statically memory-safe"));
}

#[test]
fn build_command_produces_native_executable() {
    let source = source_file("fn main() -> i32 { return 42; }");
    let output = source.with_extension("native");
    let result = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args([
            "build",
            source.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let run = Command::new(&output).status().unwrap();
    let _ = fs::remove_file(source);
    let _ = fs::remove_file(output);
    assert_eq!(run.code(), Some(42));
}

#[test]
fn migrate_rust_command_emits_compiled_verified_aster() {
    let source = source_file("fn main() { std::process::exit(9); }").with_extension("rs");
    fs::write(&source, "fn main() { std::process::exit(9); }").unwrap();
    let output = source.with_extension("as");
    let result = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args([
            "migrate",
            "rust",
            source.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("verified"));
    let executable = output.with_extension("native");
    let build = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args([
            "build",
            output.to_str().unwrap(),
            "-o",
            executable.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let run = Command::new(&executable).status().unwrap();
    assert_eq!(run.code(), Some(9));
    let _ = fs::remove_file(source);
    let _ = fs::remove_file(output.with_extension("migration.md"));
    let _ = fs::remove_file(executable);
    let _ = fs::remove_file(output);
}

#[test]
fn c_migration_command_emits_safety_verified_aster() {
    let input = source_file("int main(void) { return 12; }").with_extension("c");
    fs::write(&input, "int main(void) { return 12; }").unwrap();
    let output = input.with_extension("as");
    let result = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args([
            "migrate",
            "c",
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report = fs::read_to_string(output.with_extension("migration.md")).unwrap();
    assert!(report.contains("Runtime memory-safety checks: none"));
    let _ = fs::remove_file(input);
    let _ = fs::remove_file(output.with_extension("migration.md"));
    let _ = fs::remove_file(output);
}

#[test]
fn python_migration_cli_verifies_supported_subset_and_rejects_dynamic_code() {
    let input = source_file("import sys\nsys.exit(11)\n").with_extension("py");
    fs::write(&input, "import sys\nsys.exit(11)\n").unwrap();
    let output = input.with_extension("as");
    let result = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args([
            "migrate",
            "python",
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        fs::read_to_string(output.with_extension("migration.md"))
            .unwrap()
            .contains("Runtime memory-safety checks: none")
    );
    let _ = fs::remove_file(input);
    let _ = fs::remove_file(output.with_extension("migration.md"));
    let _ = fs::remove_file(&output);

    let dynamic = source_file("import sys\nsys.exit(eval('3'))\n").with_extension("py");
    fs::write(&dynamic, "import sys\nsys.exit(eval('3'))\n").unwrap();
    let rejected = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args([
            "migrate",
            "python",
            dynamic.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("unsupported"));
    assert!(!output.exists());
    let _ = fs::remove_file(dynamic);
}

#[test]
fn check_command_rejects_type_errors() {
    let path = source_file("fn main() -> i32 { return true; }");
    let output = Command::new(env!("CARGO_BIN_EXE_aster"))
        .args(["check", path.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = fs::remove_file(path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("return type mismatch"));
}
