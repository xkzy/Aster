use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dialect {
    C,
    Cpp,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CMigrationReport {
    pub verified: bool,
    pub compiler: String,
    pub safety_obligations: Vec<crate::safety::Obligation>,
}
#[derive(Debug)]
pub struct CMigrationError(pub String);
impl fmt::Display for CMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for CMigrationError {}

/// Migrate the strict constant-return C/C++ executable subset. Unsupported constructs fail closed.
pub fn migrate_c_file(
    input: &Path,
    output: &Path,
    dialect: Dialect,
) -> Result<CMigrationReport, CMigrationError> {
    let extension = input.extension().and_then(|e| e.to_str()).unwrap_or("");
    let extension_ok = match dialect {
        Dialect::C => extension == "c",
        Dialect::Cpp => matches!(extension, "cpp" | "cc" | "cxx"),
    };
    if !extension_ok {
        return Err(CMigrationError(match dialect {
            Dialect::C => "C migration input must use `.c`".into(),
            Dialect::Cpp => "C++ migration input must use `.cpp`, `.cc`, or `.cxx`".into(),
        }));
    }
    if output.extension().and_then(|e| e.to_str()) != Some("as") {
        return Err(CMigrationError("output path must end in `.as`".into()));
    }
    let source = fs::read_to_string(input)
        .map_err(|e| CMigrationError(format!("cannot read source: {e}")))?;
    let compiler = match dialect {
        Dialect::C => std::env::var_os("ASTER_CC").unwrap_or_else(|| "clang".into()),
        Dialect::Cpp => std::env::var_os("ASTER_CXX").unwrap_or_else(|| "clang++".into()),
    };
    let compiler_name = compiler.to_string_lossy().into_owned();
    let translated = translate_main(&source)?;
    let temp_bin = temp_path("native");
    let mut cleanup = TempFiles(vec![temp_bin.clone()]);
    let syntax = Command::new(&compiler)
        .args([
            if dialect == Dialect::C {
                "-std=c11"
            } else {
                "-std=c++20"
            },
            "-Werror",
            "-fsyntax-only",
        ])
        .arg(input)
        .output()
        .map_err(|e| CMigrationError(format!("failed to run {compiler_name}: {e}")))?;
    if !syntax.status.success() {
        return Err(CMigrationError(format!(
            "original source does not compile: {}",
            String::from_utf8_lossy(&syntax.stderr).trim()
        )));
    }
    check_int_width(&compiler)?;
    let ast = crate::parser::parse(&translated)
        .map_err(|e| CMigrationError(format!("generated Aster parse failed: {}", e.message)))?;
    let safety = crate::safety::verify(&ast)
        .map_err(|e| CMigrationError(format!("generated Aster safety verification failed: {e}")))?;
    crate::backend::build(&translated, &temp_bin)
        .map_err(|e| CMigrationError(format!("generated Aster did not compile: {e}")))?;
    let original_bin = temp_bin.with_extension("original");
    cleanup.0.push(original_bin.clone());
    let original = Command::new(&compiler)
        .args([
            if dialect == Dialect::C {
                "-std=c11"
            } else {
                "-std=c++20"
            },
            "-O2",
        ])
        .arg(input)
        .arg("-o")
        .arg(&original_bin)
        .output()
        .map_err(|e| CMigrationError(format!("failed to compile original: {e}")))?;
    if !original.status.success() {
        return Err(CMigrationError(format!(
            "original native build failed: {}",
            String::from_utf8_lossy(&original.stderr).trim()
        )));
    }
    let run_original = Command::new(&original_bin)
        .output()
        .map_err(|e| CMigrationError(format!("could not run original program: {e}")))?;
    let run_aster = Command::new(&temp_bin)
        .output()
        .map_err(|e| CMigrationError(format!("could not run generated program: {e}")))?;
    if !same_behavior(&run_original, &run_aster) {
        return Err(CMigrationError(
            "original and migrated executions differ in exit status, stdout, or stderr".into(),
        ));
    }
    fs::write(output, &translated)
        .map_err(|e| CMigrationError(format!("cannot write Aster output: {e}")))?;
    let version = Command::new(&compiler)
        .arg("--version")
        .output()
        .map_err(|e| CMigrationError(format!("cannot query compiler version: {e}")))?;
    let version_text = String::from_utf8_lossy(&version.stdout)
        .lines()
        .next()
        .unwrap_or("unknown compiler")
        .to_owned();
    let obligations = safety
        .obligations
        .iter()
        .map(|o| format!("- {}: {:?} — {}", o.property, o.status, o.rationale))
        .collect::<Vec<_>>()
        .join("\n");
    let report = format!(
        "# Strict C/C++ migration report\n\n- Frontend compiler: `{version_text}`\n- Generated Aster compiled: yes\n- Static scalar-subset safety verification: passed\n- Runtime memory-safety checks: none\n- Original/migrated exit status and output matched: yes\n- General semantic equivalence proven: no\n\n## Discharged or inapplicable obligations\n\n{obligations}\n\n## Limitations\n\nOnly `int main(void) {{ return <constant i32 expression>; }}` (C++ may use `int main()`) is supported. Pointers, allocations, objects, destructors, external ABI, includes, functions, and all other constructs are rejected.\n"
    );
    fs::write(output.with_extension("migration.md"), report)
        .map_err(|e| CMigrationError(format!("Aster output was written but report failed: {e}")))?;
    Ok(CMigrationReport {
        verified: true,
        compiler: version_text,
        safety_obligations: safety.obligations,
    })
}

fn check_int_width(compiler: &std::ffi::OsStr) -> Result<(), CMigrationError> {
    let mut child = Command::new(compiler)
        .args(["-dM", "-E", "-x", "c", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| CMigrationError(format!("cannot inspect target int width: {e}")))?;
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(b"\n")
        .map_err(|e| CMigrationError(format!("cannot query target: {e}")))?;
    let output = child
        .wait_with_output()
        .map_err(|e| CMigrationError(format!("cannot query target: {e}")))?;
    if !output.status.success()
        || !String::from_utf8_lossy(&output.stdout)
            .lines()
            .any(|l| l.trim() == "#define __SIZEOF_INT__ 4")
    {
        return Err(CMigrationError(
            "strict migration requires a compiler target with 32-bit int".into(),
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tok {
    Ident(String),
    Number(String),
    Sym(char),
    End,
}
fn translate_main(source: &str) -> Result<String, CMigrationError> {
    let tokens = tokenize(source)?;
    let mut p = CParser { tokens, pos: 0 };
    p.expect_ident("int")?;
    p.expect_ident("main")?;
    p.expect_sym('(')?;
    if p.peek() == &Tok::Ident("void".into()) {
        p.bump();
    }
    p.expect_sym(')')?;
    p.expect_sym('{')?;
    p.expect_ident("return")?;
    let value = p.expr(0)?;
    p.expect_sym(';')?;
    p.expect_sym('}')?;
    if p.peek() != &Tok::End {
        return Err(CMigrationError(
            "unsupported C/C++ construct after main; only one return statement is accepted".into(),
        ));
    }
    Ok(format!("fn main() -> i32 {{\n    return {value};\n}}\n"))
}
fn tokenize(source: &str) -> Result<Vec<Tok>, CMigrationError> {
    let b = source.as_bytes();
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < b.len() {
        if b[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b.get(i..i + 2) == Some(b"//") {
            i += 2;
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b.get(i..i + 2) == Some(b"/*") {
            i += 2;
            while i + 1 < b.len() && b.get(i..i + 2) != Some(b"*/") {
                i += 1;
            }
            if i + 1 >= b.len() {
                return Err(CMigrationError("unterminated C comment".into()));
            }
            i += 2;
            continue;
        }
        let start = i;
        if b[i].is_ascii_alphabetic() || b[i] == b'_' {
            i += 1;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            tokens.push(Tok::Ident(source[start..i].into()));
            continue;
        }
        if b[i].is_ascii_digit() {
            i += 1;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            tokens.push(Tok::Number(source[start..i].into()));
            continue;
        }
        if b"(){};+-*/%".contains(&b[i]) {
            tokens.push(Tok::Sym(b[i] as char));
            i += 1;
            continue;
        }
        return Err(CMigrationError(format!(
            "unsupported C/C++ token at byte {i}; pointers, directives, declarations, and non-scalar constructs are not migrated"
        )));
    }
    tokens.push(Tok::End);
    Ok(tokens)
}
struct CParser {
    tokens: Vec<Tok>,
    pos: usize,
}
impl CParser {
    fn peek(&self) -> &Tok {
        &self.tokens[self.pos]
    }
    fn bump(&mut self) -> Tok {
        let t = self.tokens[self.pos].clone();
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        t
    }
    fn expect_sym(&mut self, c: char) -> Result<(), CMigrationError> {
        if self.bump() == Tok::Sym(c) {
            Ok(())
        } else {
            Err(CMigrationError(format!(
                "unsupported C/C++ syntax: expected `{c}`"
            )))
        }
    }
    fn expect_ident(&mut self, s: &str) -> Result<(), CMigrationError> {
        if self.bump() == Tok::Ident(s.into()) {
            Ok(())
        } else {
            Err(CMigrationError(format!(
                "unsupported C/C++ syntax: expected `{s}`"
            )))
        }
    }
    fn expr(&mut self, min: u8) -> Result<i32, CMigrationError> {
        let mut lhs = match self.bump() {
            Tok::Number(n) => n.parse::<i32>().map_err(|_| {
                CMigrationError("only signed 32-bit decimal constants are migratable".into())
            })?,
            Tok::Sym('-') => {
                if let Tok::Number(n) = self.bump() {
                    let v = n
                        .parse::<u64>()
                        .map_err(|_| CMigrationError("invalid integer".into()))?;
                    if v == i32::MAX as u64 + 1 {
                        i32::MIN
                    } else {
                        i32::try_from(v)
                            .ok()
                            .and_then(i32::checked_neg)
                            .ok_or_else(|| CMigrationError("constant is out of i32 range".into()))?
                    }
                } else {
                    return Err(CMigrationError("unsupported unary expression".into()));
                }
            }
            Tok::Sym('(') => {
                let v = self.expr(0)?;
                self.expect_sym(')')?;
                v
            }
            _ => {
                return Err(CMigrationError(
                    "unsupported return expression; only constant arithmetic is migratable".into(),
                ));
            }
        };
        loop {
            let (op, bp) = match self.peek() {
                Tok::Sym('+') => ('+', 1),
                Tok::Sym('-') => ('-', 1),
                Tok::Sym('*') => ('*', 2),
                Tok::Sym('/') => ('/', 2),
                Tok::Sym('%') => ('%', 2),
                _ => break,
            };
            if bp < min {
                break;
            }
            self.bump();
            let rhs = self.expr(bp + 1)?;
            lhs = match op {
                '+' => lhs.checked_add(rhs),
                '-' => lhs.checked_sub(rhs),
                '*' => lhs.checked_mul(rhs),
                '/' if rhs != 0 => lhs.checked_div(rhs),
                '%' if rhs != 0 => lhs.checked_rem(rhs),
                _ => None,
            }
            .ok_or_else(|| {
                CMigrationError(
                    "C/C++ arithmetic is undefined or outside i32; strict migration refuses it"
                        .into(),
                )
            })?;
        }
        Ok(lhs)
    }
}
fn same_behavior(a: &Output, b: &Output) -> bool {
    a.status.code() == b.status.code() && a.stdout == b.stdout && a.stderr == b.stderr
}
fn temp_path(ext: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "aster-c-{}-{}.{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed),
        ext
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

#[cfg(test)]
mod tests {
    use super::{Dialect, migrate_c_file};
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static NEXT: AtomicUsize = AtomicUsize::new(0);
    fn temp(ext: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "aster-c-migrate-{}-{}.{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
            ext
        ))
    }

    #[test]
    fn migrates_compiles_and_compares_constant_c_main() {
        let input = temp("c");
        let output = temp("as");
        fs::write(&input, "int main(void) { return 17; }").unwrap();
        let report = migrate_c_file(&input, &output, Dialect::C).unwrap();
        assert!(report.verified);
        assert_eq!(
            fs::read_to_string(&output).unwrap(),
            "fn main() -> i32 {\n    return 17;\n}\n"
        );
        let _ = fs::remove_file(input);
        let _ = fs::remove_file(output.with_extension("migration.md"));
        let _ = fs::remove_file(output);
    }

    #[test]
    fn migrates_constant_cpp_main() {
        let input = temp("cpp");
        let output = temp("as");
        fs::write(&input, "int main() { return 23; }").unwrap();
        let report = migrate_c_file(&input, &output, Dialect::Cpp).unwrap();
        assert!(report.verified);
        let _ = fs::remove_file(input);
        let _ = fs::remove_file(output.with_extension("migration.md"));
        let _ = fs::remove_file(output);
    }

    #[test]
    fn rejects_pointer_and_heap_operations_in_c_and_cpp() {
        for (dialect, ext, source) in [
            (Dialect::C, "c", "int main(void) { int *p = 0; return *p; }"),
            (
                Dialect::Cpp,
                "cpp",
                "int main() { int *p = new int(2); return *p; }",
            ),
        ] {
            let input = temp(ext);
            let output = temp("as");
            fs::write(&input, source).unwrap();
            let error = migrate_c_file(&input, &output, dialect).unwrap_err();
            assert!(error.0.contains("unsupported"));
            assert!(!output.exists());
            let _ = fs::remove_file(input);
        }
    }
}
