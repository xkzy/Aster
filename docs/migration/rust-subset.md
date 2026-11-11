# Rust migration subset (experimental)

**Status:** Working prototype with severe limits; not a general Rust transpiler.

## Command

```sh
aster migrate rust input.rs -o output.as
```

The prototype accepts standalone `.rs` files outside any ancestor containing `Cargo.toml`. It invokes the exact pinned `rustc 1.99.1` using `rustup run`, emits MIR to force compiler parsing/type/borrow validation, and performs a source-preserving-where-possible translation with pinned `syn = 2.0.119`. It does not use `rustc_private` or parse rustc's textual MIR (rustc explicitly labels that output human-facing and subject to change).

The supported source grammar is intentionally narrow:

- Private, zero-argument free functions.
- Helpers explicitly returning `i32` and containing one integer literal or direct zero-argument function-call tail expression.
- `fn main() {}` or `fn main() { std::process::exit(<supported expression>); }`.
- Decimal constants in `i32` range and direct local helper calls.
- Only source whose `rustc`-checked behavior can be preserved under Aster's checked arithmetic and scalar-only subset.

Every generated program is checked and built with Aster; then original Rust and generated native executables run once and must have matching exit status, stdout, and stderr. On success, the tool writes the `.as` file and a `.migration.md` compatibility report. `verified` means this limited check passed, not a theorem, exhaustive test, memory-safety proof, or equivalence for all inputs.

Unsupported syntax/features fail explicitly. Current limitations include Cargo projects and dependencies, module trees, features, build scripts, proc macros, test targets, non-`i32` types, parameters, structs/enums, patterns, traits/generics, ownership-bearing values, borrow/lifetime translation, `Drop`, panic behavior, unsafe code, concurrency/atomics, macros, comments, and source maps. Comments and source locations are not preserved by this subset.

## Why rustc + syn

`syn` provides a full source AST for readable translation but does not type-check. The pinned external rustc invocation validates the standalone source through the compiler and emits MIR; the translator currently consumes compiler success/failure, not the MIR representation itself. Thus no claim is made that typed MIR facts have been extracted or that the prototype handles ownership transformations. Using rustc internals would require a pinned nightly/compiler build and unstable `rustc_driver`/`rustc_interface` APIs; the prototype deliberately avoids that maintenance cost until its semantic subset justifies it.

## Migration success criteria

A successful command currently demonstrates that a tiny translated program compiles natively with Aster and matches the original executable's single observed run. It does not meet the full project's required differential test matrix. Cargo migration, broad ownership/drop preservation, source mappings, and randomized/differential test suites remain unimplemented.
