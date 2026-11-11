use crate::{parser::Function, typecheck};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObligationStatus {
    VerifiedBySubset,
    NotApplicable,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obligation {
    pub function: String,
    pub property: &'static str,
    pub status: ObligationStatus,
    pub rationale: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafetyReport {
    pub obligations: Vec<Obligation>,
    pub runtime_safety_checks: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafetyError(pub String);
impl fmt::Display for SafetyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for SafetyError {}

/// Verify the current scalar-only safe subset. All memory-bearing constructs are absent from its AST.
pub fn verify(functions: &[Function]) -> Result<SafetyReport, SafetyError> {
    typecheck::check(functions)
        .map_err(|e| SafetyError(format!("type/initialization obligation failed: {}", e.0)))?;
    let mut obligations = Vec::new();
    for function in functions {
        obligations.push(obligation(
            function,
            "initialized scalar reads",
            ObligationStatus::VerifiedBySubset,
            "bindings require an initializer and name resolution rejects unknown names",
        ));
        obligations.push(obligation(
            function,
            "use-after-free / double-free",
            ObligationStatus::NotApplicable,
            "the safe AST has no allocation, resource, reference, or destruction operation",
        ));
        obligations.push(obligation(
            function, "borrow lifetime / mutable alias exclusion", ObligationStatus::NotApplicable,
            "the safe AST has no references or borrowed locations; scalar arguments are copied by value",
        ));
        obligations.push(obligation(
            function,
            "pointer arithmetic / bounds safety",
            ObligationStatus::NotApplicable,
            "the safe AST has no pointers, arrays, indexing, or pointer arithmetic",
        ));
        obligations.push(obligation(
            function,
            "allocation and deallocation pairing",
            ObligationStatus::NotApplicable,
            "the safe AST has no dynamic allocation or deallocation",
        ));
    }
    if obligations
        .iter()
        .any(|o| o.status == ObligationStatus::Unknown)
    {
        return Err(SafetyError(
            "unknown safety obligation; refusing strict-mode compilation".into(),
        ));
    }
    Ok(SafetyReport {
        obligations,
        runtime_safety_checks: Vec::new(),
    })
}

fn obligation(
    function: &Function,
    property: &'static str,
    status: ObligationStatus,
    rationale: &'static str,
) -> Obligation {
    Obligation {
        function: function.name.clone(),
        property,
        status,
        rationale,
    }
}

#[cfg(test)]
mod tests {
    use super::{ObligationStatus, verify};
    use crate::parser::parse;

    #[test]
    fn scalar_program_has_only_statically_discharged_or_inapplicable_obligations() {
        let program = parse("fn main() -> i32 { let x = 21; return x * 2; }").unwrap();
        let report = verify(&program).unwrap();
        assert!(!report.obligations.is_empty());
        assert!(
            report
                .obligations
                .iter()
                .all(|o| o.status != ObligationStatus::Unknown)
        );
        assert!(report.runtime_safety_checks.is_empty());
    }

    #[test]
    fn pointer_and_index_syntax_is_rejected_before_verification() {
        assert!(parse("fn main() -> i32 { let x = 1; return *x; }").is_err());
        assert!(parse("fn main() -> i32 { return xs[0]; }").is_err());
    }
}
