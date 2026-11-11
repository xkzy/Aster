use crate::parser::{BinOp, Expr, Function, Stmt, Type, UnOp};
use std::{
    fmt,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct BuildError(pub String);
impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for BuildError {}

/// Build the checked subset to a native executable using the configured C compiler.
pub fn build(source: &str, output: &Path) -> Result<(), BuildError> {
    let program = crate::parser::parse(source)
        .map_err(|e| BuildError(format!("parse error at byte {}: {}", e.offset, e.message)))?;
    crate::safety::verify(&program)
        .map_err(|e| BuildError(format!("static safety verification failed: {e}")))?;
    if !program
        .iter()
        .any(|f| f.name == "main" && f.params.is_empty() && f.result == Type::I32)
    {
        return Err(BuildError(
            "native executable requires `fn main() -> i32`".into(),
        ));
    }
    let c_source = emit_c(&program);
    let compiler = std::env::var_os("ASTER_CC").unwrap_or_else(|| "clang".into());
    let mut child = Command::new(compiler)
        .args(["-std=c11", "-O2", "-x", "c", "-", "-o"])
        .arg(output)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| BuildError(format!("failed to start C compiler: {e}")))?;
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(c_source.as_bytes())
        .map_err(|e| {
            BuildError(format!(
                "failed to send generated source to C compiler: {e}"
            ))
        })?;
    let result = child
        .wait_with_output()
        .map_err(|e| BuildError(format!("failed waiting for C compiler: {e}")))?;
    if !result.status.success() {
        return Err(BuildError(format!(
            "native compilation failed: {}",
            String::from_utf8_lossy(&result.stderr).trim()
        )));
    }
    Ok(())
}

fn emit_c(functions: &[Function]) -> String {
    let mut out = String::from(
        "#include <stdint.h>\n#include <stdbool.h>\n#include <limits.h>\n#include <stdlib.h>\n\
static int32_t aster_add(int32_t a,int32_t b){int32_t r;if(__builtin_add_overflow(a,b,&r))abort();return r;}\n\
static int32_t aster_sub(int32_t a,int32_t b){int32_t r;if(__builtin_sub_overflow(a,b,&r))abort();return r;}\n\
static int32_t aster_mul(int32_t a,int32_t b){int32_t r;if(__builtin_mul_overflow(a,b,&r))abort();return r;}\n\
static int32_t aster_neg(int32_t a){return aster_sub(0,a);}\n\
static int32_t aster_div(int32_t a,int32_t b){if(b==0||(a==INT32_MIN&&b==-1))abort();return a/b;}\n\
static int32_t aster_rem(int32_t a,int32_t b){if(b==0||(a==INT32_MIN&&b==-1))abort();return a%b;}\n",
    );
    for function in functions {
        out.push_str(&function_prototype(function));
        out.push_str(";\n");
    }
    for function in functions {
        out.push_str(&function_prototype(function));
        out.push_str(" {\n");
        for statement in &function.body {
            match statement {
                Stmt::Return(expr) => {
                    out.push_str("return ");
                    out.push_str(&expr_c(expr));
                    out.push_str(";\n");
                }
                Stmt::Bind { name, value, .. } => {
                    out.push_str("int32_t aster_v_");
                    out.push_str(name);
                    out.push_str(" = ");
                    out.push_str(&expr_c(value));
                    out.push_str(";\n");
                }
                Stmt::Assign { name, value } => {
                    out.push_str("aster_v_");
                    out.push_str(name);
                    out.push_str(" = ");
                    out.push_str(&expr_c(value));
                    out.push_str(";\n");
                }
                Stmt::Expr(expr) => {
                    out.push_str("(void)(");
                    out.push_str(&expr_c(expr));
                    out.push_str(");\n");
                }
            }
        }
        out.push_str("}\n");
    }
    out.push_str("int main(void){return (int)aster_fn_main();}\n");
    out
}

fn function_prototype(function: &Function) -> String {
    let result = c_type(function.result);
    let mut text = format!("{result} aster_fn_{}(", function.name);
    if function.params.is_empty() {
        text.push_str("void");
    }
    for (index, (name, ty)) in function.params.iter().enumerate() {
        if index > 0 {
            text.push(',');
        }
        text.push_str(c_type(*ty));
        text.push_str(" aster_v_");
        text.push_str(name);
    }
    text.push(')');
    text
}
fn c_type(ty: Type) -> &'static str {
    match ty {
        Type::I32 => "int32_t",
        Type::Bool => "bool",
    }
}

fn expr_c(expr: &Expr) -> String {
    match expr {
        Expr::Integer(value) => {
            let decimal = value.trim_start_matches('0');
            if decimal.is_empty() {
                "0".into()
            } else {
                decimal.into()
            }
        }
        Expr::Bool(value) => {
            if *value {
                "true".into()
            } else {
                "false".into()
            }
        }
        Expr::Name(name) => format!("aster_v_{name}"),
        Expr::Unary(UnOp::Negate, inner) => {
            if matches!(inner.as_ref(), Expr::Integer(n) if n == "2147483648") {
                "INT32_MIN".into()
            } else {
                format!("aster_neg({})", expr_c(inner))
            }
        }
        Expr::Unary(UnOp::Not, inner) => format!("(!{})", expr_c(inner)),
        Expr::Binary(lhs, op, rhs) => {
            let a = expr_c(lhs);
            let b = expr_c(rhs);
            match op {
                BinOp::Add => format!("aster_add({a},{b})"),
                BinOp::Subtract => format!("aster_sub({a},{b})"),
                BinOp::Multiply => format!("aster_mul({a},{b})"),
                BinOp::Divide => format!("aster_div({a},{b})"),
                BinOp::Remainder => format!("aster_rem({a},{b})"),
                BinOp::Equal => format!("(({a})==({b}))"),
                BinOp::Less => format!("(({a})<({b}))"),
                BinOp::LessEqual => format!("(({a})<=({b}))"),
            }
        }
        Expr::Call(name, args) => format!(
            "aster_fn_{}({})",
            name,
            args.iter().map(expr_c).collect::<Vec<_>>().join(",")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::build;
    use std::{
        fs,
        path::PathBuf,
        process::Command,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn output_path() -> PathBuf {
        std::env::temp_dir().join(format!(
            "aster-native-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }

    #[test]
    fn compiles_and_runs_a_native_program() {
        let path = output_path();
        build("fn main() -> i32 { return 40 + 2; }", &path).unwrap();
        let result = Command::new(&path).status().unwrap();
        let _ = fs::remove_file(&path);
        assert_eq!(result.code(), Some(42));
    }

    #[test]
    fn decimal_leading_zero_is_not_translated_as_c_octal() {
        let path = output_path();
        build("fn main() -> i32 { return 0008; }", &path).unwrap();
        let result = Command::new(&path).status().unwrap();
        let _ = fs::remove_file(&path);
        assert_eq!(result.code(), Some(8));
    }

    #[test]
    fn division_by_zero_traps_in_generated_native_program() {
        let path = output_path();
        build("fn main() -> i32 { return 5 / 0; }", &path).unwrap();
        let result = Command::new(&path).status().unwrap();
        let _ = fs::remove_file(&path);
        assert!(!result.success());
    }

    #[test]
    fn checked_overflow_traps_in_generated_native_program() {
        let path = output_path();
        build("fn main() -> i32 { return 2147483647 + 1; }", &path).unwrap();
        let result = Command::new(&path).status().unwrap();
        let _ = fs::remove_file(&path);
        assert!(!result.success());
    }
}
