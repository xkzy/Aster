# Aster Language Specification 0.1 (bootstrap target)

**Status:** Draft proposal; not implemented. Anything marked in this document is a design target, not a claim that the compiler supports it. No conforming implementation exists yet.

## 1. Scope

Aster 0.1 is intended as a small, statically typed native language subset. Initial implementation target is integer/bool expressions, function declarations in both requested styles, lexical local bindings, return statements, and checked integer arithmetic. This draft does **not** define structs, heap allocation, references, arrays, slices, generics, concurrency, exceptions, a C ABI, or a complete ownership/borrow system. Those features must not be advertised as supported until separately specified and implemented.

## 2. Source forms

The two function declaration spellings are intended to denote the same typed function:

```c
int add(int a, int b) { return a + b; }
fn add(a: i32, b: i32) -> i32 { return a + b; }
```

In the initial subset, legacy `int` maps to signed 32-bit `i32`. Declarations may not mix styles within one function. No implicit conversion is defined.

## 3. Lexical and syntactic subset (EBNF)

```ebnf
program       = { function } ;
function      = c_function | modern_function ;
c_function    = type identifier "(" [ parameters ] ")" block ;
modern_function = "fn" identifier "(" [ modern_parameters ] ")" "->" type block ;
parameters    = parameter { "," parameter } ;
parameter     = type identifier ;
modern_parameters = modern_parameter { "," modern_parameter } ;
modern_parameter = identifier ":" type ;
type          = "int" | "i32" | "bool" ;
block         = "{" { statement } "}" ;
statement     = "return" expression ";"
              | ("let" | "var") identifier [ ":" type ] "=" expression ";"
              | identifier "=" expression ";"
              | expression ";" ;
expression    = comparison ;
comparison    = additive [ ("==" | "<" | "<=") additive ] ;
additive      = multiplicative { ("+" | "-") multiplicative } ;
multiplicative = unary { ("*" | "/" | "%") unary } ;
unary         = [ "-" | "!" ] primary ;
primary       = integer | "true" | "false" | identifier
              | identifier "(" [ arguments ] ")" | "(" expression ")" ;
arguments     = expression { "," expression } ;
```

The lexer must reject unknown characters and malformed integer literals with source locations. Comments and strings are not in this subset. Precedence is as shown; binary operators are left-associative. Evaluation is left-to-right. These productions are a target grammar, not proof of parser support.

## 4. Types, bindings, and values

Types are `i32` and `bool`. `int` is a spelling alias for `i32`; there are no implicit conversions. Local type inference, if implemented, derives a binding's type from its initializer; otherwise a local annotation may be required. `let` is immutable after initialization; `var` is mutable. Reads of uninitialized bindings and assignments to immutable bindings are rejected. Each function must return a value matching its declared return type on every path. The expression language must reject type mismatches.

Scalar `i32` and `bool` assignments have logical copy semantics. Integer values are plain scalars, not unique resources. Resource ownership, move behavior, borrow modes, lifetimes and destructor semantics are unspecified in 0.1 because the subset has no resource/reference types.

## 5. Arithmetic and observable behavior

Signed `i32` addition, subtraction, multiplication, negation, division, and remainder are checked. Overflow and division by zero cause a defined runtime trap (exact process-level trap ABI remains to be specified); they must never become LLVM poison or C-style undefined behavior. `i32::MIN / -1` is overflow and traps. Comparisons return `bool`. No floating-point operations exist, so no floating-point reassociation is permitted by this subset.

## 6. Memory and safety boundary

The 0.1 subset has no pointer operations, allocation, references, FFI, unsafe block, or concurrency. Safe source must not lower through undefined-behavior assumptions that are not proven. All unsupported syntax is a compile-time error. A future C interoperability design must expose an explicit unsafe boundary and separately specify ABI, ownership transfer, aliasing, and lifetime obligations.

## 7. Native lowering requirements

The implementation goal is native machine code. Both declaration styles must produce equivalent typed IR. Checked arithmetic must lower to explicit overflow checks or equivalent defined traps. LLVM attributes such as `noalias`, `readonly`, and `nocapture` are forbidden unless each is proven according to the LLVM Language Reference. No LLVM backend is implemented yet.

## 8. Conformance status (2026-06-04)

A lexer/parser, scalar type/initialization checker, and structural strict-memory-safety verifier exist for part of this grammar. Both `aster check` and `aster build` require verification. The accepted subset has no pointers, references, arrays/indexing, allocation, or destruction; corresponding safety obligations are therefore not applicable. Unknown obligations fail closed. There are no runtime memory-safety guards in this subset. Checked arithmetic can still trap to preserve arithmetic semantics; this is separate from memory safety.

`aster build <file> -o <executable>` emits restricted C and invokes Clang; it does not lower directly to LLVM IR. The source-level safety argument is documented in `aster-safety.md` but is not machine-checked and does not prove correctness of C/backend lowering. No ownership/borrow/lifetime analysis, structs, arrays/references, FFI, concurrency, or proof-assistant model exists. The parser/code generator do not implement every production in this draft. This remains a draft specification, not full conformance.
