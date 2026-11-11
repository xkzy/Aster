use crate::parser::{BinOp, Expr, Function, Stmt, Type, UnOp};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeError(pub String);

#[derive(Clone, Copy)]
struct Binding {
    ty: Type,
    mutable: bool,
}

pub fn check(functions: &[Function]) -> Result<(), TypeError> {
    let mut signatures = HashMap::new();
    for function in functions {
        let signature = (
            function
                .params
                .iter()
                .map(|(_, ty)| *ty)
                .collect::<Vec<_>>(),
            function.result,
        );
        if signatures
            .insert(function.name.as_str(), signature)
            .is_some()
        {
            return Err(TypeError(format!("duplicate function `{}`", function.name)));
        }
    }
    for function in functions {
        let mut bindings = HashMap::new();
        for (name, ty) in &function.params {
            if bindings
                .insert(
                    name.as_str(),
                    Binding {
                        ty: *ty,
                        mutable: false,
                    },
                )
                .is_some()
            {
                return Err(TypeError(format!("duplicate parameter `{name}`")));
            }
        }
        let mut returned = false;
        for (index, stmt) in function.body.iter().enumerate() {
            if returned {
                return Err(TypeError("unreachable statement after return".into()));
            }
            match stmt {
                Stmt::Return(expr) => {
                    let actual = expr_type(expr, &bindings, &signatures)?;
                    if actual != function.result {
                        return Err(TypeError("return type mismatch".into()));
                    }
                    returned = true;
                    if index + 1 != function.body.len() {
                        return Err(TypeError("unreachable statement after return".into()));
                    }
                }
                Stmt::Bind {
                    mutable,
                    name,
                    ty,
                    value,
                } => {
                    if bindings.contains_key(name.as_str()) {
                        return Err(TypeError(format!("duplicate binding `{name}`")));
                    }
                    let inferred = expr_type(value, &bindings, &signatures)?;
                    let ty = ty.unwrap_or(inferred);
                    if ty != inferred {
                        return Err(TypeError(format!("initializer type mismatch for `{name}`")));
                    }
                    bindings.insert(
                        name,
                        Binding {
                            ty,
                            mutable: *mutable,
                        },
                    );
                }
                Stmt::Assign { name, value } => {
                    let binding = bindings
                        .get(name.as_str())
                        .ok_or_else(|| TypeError(format!("unknown binding `{name}`")))?;
                    if !binding.mutable {
                        return Err(TypeError(format!(
                            "cannot assign to immutable binding `{name}`"
                        )));
                    }
                    if expr_type(value, &bindings, &signatures)? != binding.ty {
                        return Err(TypeError("assignment type mismatch".into()));
                    }
                }
                Stmt::Expr(expr) => {
                    expr_type(expr, &bindings, &signatures)?;
                }
            }
        }
        if !returned {
            return Err(TypeError(format!(
                "function `{}` may exit without returning",
                function.name
            )));
        }
    }
    Ok(())
}

fn expr_type(
    expr: &Expr,
    bindings: &HashMap<&str, Binding>,
    signatures: &HashMap<&str, (Vec<Type>, Type)>,
) -> Result<Type, TypeError> {
    match expr {
        Expr::Integer(text) => {
            let value: u64 = text
                .parse()
                .map_err(|_| TypeError("integer literal is too large".into()))?;
            if value > i32::MAX as u64 {
                return Err(TypeError("positive integer literal exceeds i32".into()));
            }
            Ok(Type::I32)
        }
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Name(name) => bindings
            .get(name.as_str())
            .map(|b| b.ty)
            .ok_or_else(|| TypeError(format!("unknown name `{name}`"))),
        Expr::Unary(UnOp::Negate, inner) => {
            if let Expr::Integer(text) = inner.as_ref()
                && text.parse::<u64>().ok() == Some(i32::MAX as u64 + 1)
            {
                return Ok(Type::I32);
            }
            require(
                expr_type(inner, bindings, signatures)?,
                Type::I32,
                "unary `-` requires i32",
            )?;
            Ok(Type::I32)
        }
        Expr::Unary(UnOp::Not, inner) => {
            require(
                expr_type(inner, bindings, signatures)?,
                Type::Bool,
                "`!` requires bool",
            )?;
            Ok(Type::Bool)
        }
        Expr::Binary(lhs, op, rhs) => {
            let left = expr_type(lhs, bindings, signatures)?;
            let right = expr_type(rhs, bindings, signatures)?;
            if left != right {
                return Err(TypeError("binary operands have different types".into()));
            }
            match op {
                BinOp::Add
                | BinOp::Subtract
                | BinOp::Multiply
                | BinOp::Divide
                | BinOp::Remainder => {
                    require(left, Type::I32, "arithmetic requires i32")?;
                    Ok(Type::I32)
                }
                BinOp::Equal => Ok(Type::Bool),
                BinOp::Less | BinOp::LessEqual => {
                    require(left, Type::I32, "ordering requires i32")?;
                    Ok(Type::Bool)
                }
            }
        }
        Expr::Call(name, args) => {
            let (params, result) = signatures
                .get(name.as_str())
                .ok_or_else(|| TypeError(format!("unknown function `{name}`")))?;
            if args.len() != params.len() {
                return Err(TypeError(format!("wrong argument count for `{name}`")));
            }
            for (arg, expected) in args.iter().zip(params) {
                require(
                    expr_type(arg, bindings, signatures)?,
                    *expected,
                    "argument type mismatch",
                )?;
            }
            Ok(*result)
        }
    }
}

fn require(actual: Type, expected: Type, message: &'static str) -> Result<(), TypeError> {
    if actual == expected {
        Ok(())
    } else {
        Err(TypeError(message.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::parser::parse;

    #[test]
    fn accepts_consistent_calls_and_inferred_local_types() {
        let program = parse("fn inc(x: i32) -> i32 { let y = x + 1; return y; } fn main() -> i32 { return inc(4); }").unwrap();
        assert!(check(&program).is_ok());
    }

    #[test]
    fn rejects_type_mismatch_unknown_names_and_immutable_assignment() {
        for source in [
            "fn f() -> i32 { return true; }",
            "fn f() -> i32 { return missing; }",
            "fn f() -> i32 { let x = 1; x = 2; return x; }",
        ] {
            let program = parse(source).unwrap();
            assert!(check(&program).is_err(), "accepted: {source}");
        }
    }
}
