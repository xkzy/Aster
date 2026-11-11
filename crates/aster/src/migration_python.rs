use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PythonMigrationReport {
    pub verified: bool,
    pub python_version: String,
    pub safety_obligations: Vec<crate::safety::Obligation>,
}
#[derive(Debug)]
pub struct PythonMigrationError(pub String);
impl fmt::Display for PythonMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for PythonMigrationError {}

/// Migrate only a Python script importing sys and exiting with an i32 literal.
pub fn migrate_python_file(
    input: &Path,
    output: &Path,
) -> Result<PythonMigrationReport, PythonMigrationError> {
    if input.extension().and_then(|e| e.to_str()) != Some("py") {
        return Err(PythonMigrationError(
            "Python migration input must use `.py`".into(),
        ));
    }
    if output.extension().and_then(|e| e.to_str()) != Some("as") {
        return Err(PythonMigrationError("output path must end in `.as`".into()));
    }
    let source = fs::read_to_string(input)
        .map_err(|e| PythonMigrationError(format!("cannot read source: {e}")))?;
    let python = std::env::var_os("ASTER_PYTHON").unwrap_or_else(|| "python3".into());
    let parse = Command::new(&python)
        .args([
            "-c",
            "import ast,sys; ast.parse(sys.stdin.read(), filename='<migration>')",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| PythonMigrationError(format!("cannot start Python parser: {e}")))?;
    let parse_out = write_wait(parse, &source)?;
    if !parse_out.status.success() {
        return Err(PythonMigrationError(format!(
            "Python syntax validation failed: {}",
            String::from_utf8_lossy(&parse_out.stderr).trim()
        )));
    }
    let code = translate_script(&source)?;
    let ast = crate::parser::parse(&code).map_err(|e| {
        PythonMigrationError(format!("generated Aster parse failed: {}", e.message))
    })?;
    let safety = crate::safety::verify(&ast).map_err(|e| {
        PythonMigrationError(format!("generated Aster safety verification failed: {e}"))
    })?;
    let exe = temp_path("aster-bin");
    let mut cleanup = TempFiles(vec![exe.clone()]);
    crate::backend::build(&code, &exe)
        .map_err(|e| PythonMigrationError(format!("generated Aster did not compile: {e}")))?;
    let original = Command::new(&python)
        .arg(input)
        .output()
        .map_err(|e| PythonMigrationError(format!("cannot run original Python script: {e}")))?;
    let migrated = Command::new(&exe)
        .output()
        .map_err(|e| PythonMigrationError(format!("cannot run generated executable: {e}")))?;
    if !same_behavior(&original, &migrated) {
        return Err(PythonMigrationError(
            "original and migrated executions differ in exit status, stdout, or stderr".into(),
        ));
    }
    fs::write(output, &code)
        .map_err(|e| PythonMigrationError(format!("cannot write Aster output: {e}")))?;
    let version = Command::new(&python)
        .arg("--version")
        .output()
        .map_err(|e| PythonMigrationError(format!("cannot query Python version: {e}")))?;
    let version_text = String::from_utf8_lossy(&version.stdout).trim().to_owned();
    let obligations = safety
        .obligations
        .iter()
        .map(|o| format!("- {}: {:?} — {}", o.property, o.status, o.rationale))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(output.with_extension("migration.md"),format!("# Strict Python migration report\n\n- Frontend: `{version_text}` (parse/run only; not embedded in Aster)\n- Generated Aster compiled: yes\n- Static scalar-subset safety verification: passed\n- Runtime memory-safety checks: none\n- Original/migrated exit status and output matched: yes\n- General semantic equivalence proven: no\n\n## Safety obligations\n\n{obligations}\n\n## Limitations\n\nOnly `import sys` followed by `sys.exit(<signed i32 literal>)` is accepted. Dynamic types, reflection, functions, imports other than `sys`, I/O, mutable state, dispatch, and all other statements are rejected. Python runs only during migration validation; no Python runtime is linked into output.\n")).map_err(|e|PythonMigrationError(format!("Aster output was written but migration report failed: {e}")))?;
    cleanup.0.clear();
    Ok(PythonMigrationReport {
        verified: true,
        python_version: version_text,
        safety_obligations: safety.obligations,
    })
}
fn write_wait(
    mut child: std::process::Child,
    source: &str,
) -> Result<Output, PythonMigrationError> {
    child
        .stdin
        .take()
        .expect("piped parser stdin")
        .write_all(source.as_bytes())
        .map_err(|e| PythonMigrationError(format!("cannot send Python source to parser: {e}")))?;
    child
        .wait_with_output()
        .map_err(|e| PythonMigrationError(format!("cannot wait for Python parser: {e}")))
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Word(String),
    Integer(String),
    Dot,
    LParen,
    RParen,
    Minus,
    Semicolon,
    End,
}
fn translate_script(source: &str) -> Result<String, PythonMigrationError> {
    let tokens = tokenize(source)?;
    let expected = [
        Token::Word("import".into()),
        Token::Word("sys".into()),
        Token::Word("sys".into()),
        Token::Dot,
        Token::Word("exit".into()),
        Token::LParen,
    ];
    if tokens.get(..expected.len()) != Some(expected.as_slice()) {
        return Err(PythonMigrationError("unsupported Python source; expected `import sys` followed by `sys.exit(<signed i32 literal>)`".into()));
    }
    let mut i = expected.len();
    let negative = if tokens.get(i) == Some(&Token::Minus) {
        i += 1;
        true
    } else {
        false
    };
    let Some(Token::Integer(n)) = tokens.get(i) else {
        return Err(PythonMigrationError(
            "unsupported Python exit status; expected an integer literal".into(),
        ));
    };
    i += 1;
    if tokens.get(i) != Some(&Token::RParen) {
        return Err(PythonMigrationError(
            "unsupported Python sys.exit call arguments".into(),
        ));
    }
    i += 1;
    while tokens.get(i) == Some(&Token::Semicolon) {
        i += 1;
    }
    if tokens.get(i) != Some(&Token::End) {
        return Err(PythonMigrationError(
            "unsupported Python behavior after sys.exit".into(),
        ));
    }
    let mut value: i32 = n.parse().map_err(|_| {
        PythonMigrationError("Python exit value is outside the Aster i32 range".into())
    })?;
    if negative {
        value = value.checked_neg().ok_or_else(|| {
            PythonMigrationError("Python exit value is outside the Aster i32 range".into())
        })?;
    }
    Ok(format!("fn main() -> i32 {{\n    return {value};\n}}\n"))
}
fn tokenize(s: &str) -> Result<Vec<Token>, PythonMigrationError> {
    let b = s.as_bytes();
    let mut i = 0;
    let mut out = Vec::new();
    while i < b.len() {
        if b[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b[i] == b'#' {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        let st = i;
        if b[i].is_ascii_alphabetic() || b[i] == b'_' {
            i += 1;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            out.push(Token::Word(s[st..i].into()));
            continue;
        }
        if b[i].is_ascii_digit() {
            i += 1;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            out.push(Token::Integer(s[st..i].into()));
            continue;
        }
        let t = match b[i] {
            b'.' => Token::Dot,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'-' => Token::Minus,
            b';' => Token::Semicolon,
            _ => {
                return Err(PythonMigrationError(format!(
                    "unsupported Python syntax at byte {i}"
                )));
            }
        };
        out.push(t);
        i += 1;
    }
    out.push(Token::End);
    Ok(out)
}
fn same_behavior(a: &Output, b: &Output) -> bool {
    a.status.code() == b.status.code() && a.stdout == b.stdout && a.stderr == b.stderr
}
fn temp_path(ext: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "aster-python-{}-{}.{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed),
        ext
    ))
}
struct TempFiles(Vec<PathBuf>);
impl Drop for TempFiles {
    fn drop(&mut self) {
        for p in &self.0 {
            let _ = fs::remove_file(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::migrate_python_file;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    fn temp(ext: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "aster-py-{}-{}.{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
            ext
        ))
    }

    #[test]
    fn translates_builds_and_compares_literal_sys_exit_script() {
        let input = temp("py");
        let output = temp("as");
        fs::write(&input, "import sys\nsys.exit(19)\n").unwrap();
        let report = migrate_python_file(&input, &output).unwrap();
        assert!(report.verified);
        assert_eq!(
            fs::read_to_string(&output).unwrap(),
            "fn main() -> i32 {\n    return 19;\n}\n"
        );
        let report_text = fs::read_to_string(output.with_extension("migration.md")).unwrap();
        assert!(report_text.contains("Runtime memory-safety checks: none"));
        let _ = fs::remove_file(input);
        let _ = fs::remove_file(output.with_extension("migration.md"));
        let _ = fs::remove_file(output);
    }

    #[test]
    fn rejects_dynamic_python_without_generating_aster() {
        let input = temp("py");
        let output = temp("as");
        fs::write(&input, "import sys\nsys.exit(eval('3'))\n").unwrap();
        let error = migrate_python_file(&input, &output).unwrap_err();
        assert!(error.0.contains("unsupported"));
        assert!(!output.exists());
        let _ = fs::remove_file(input);
    }
}
