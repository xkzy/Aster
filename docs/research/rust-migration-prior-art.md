# Rust migration frontend and prior-art notes

**Checked:** 2026-06-04. This is an initial scope note, not a comprehensive survey of Rust translation tools.

## Rust compiler frontend choice

- The Rust Compiler Development Guide documents external `rustc_driver`/`rustc_interface` use: https://rustc-dev-guide.rust-lang.org/rustc-driver/intro.html. These APIs are compiler internals, not a stable external API; they require toolchain coordination and are version-sensitive.
- Rust's `rustc --emit=mir` CLI can emit MIR, but the compiler labels this output human-facing and subject to change. The prototype pins `rustc 1.99.1`, runs semantic validation, and does **not** parse the textual MIR.
- `syn` parses Rust source to a syntax tree (`parse_file`) but is not a type checker: https://docs.rs/syn/2.0.119/syn/. Aster pairs the AST parser with pinned rustc validation. The current converter consumes only an AST subset; it does not extract typed HIR/MIR ownership facts.
- Rust `rustc_private` and compiler driver details: https://doc.rust-lang.org/beta/unstable-book/language-features/rustc-private.html.

## Existing migration tools

- C2Rust migrates C to Rust, not Rust to Aster. Its own repository describes a source translation workflow and warns that generated code may be unsafe/non-idiomatic: https://github.com/immunant/c2rust and https://c2rust.com/manual/.
- A search surfaced Rust-to-C++ translators including `rusty-cpp`, but no primary-paper or comprehensive semantics guarantee was established in this initial pass. Do not treat a source-to-source tool's existence as evidence of semantic equivalence.

## Current prototype compared with requested target

The prototype's pinned compiler check, source AST translation, Aster native compilation, and one-run output/exit comparison is a verifiable vertical slice. It does not preserve general Rust ownership, clones, destructor/drop order, enums/patterns, trait dispatch, generics, panic behavior, unsafe boundaries, atomics, modules, Cargo features, build scripts, proc macros, test suites, comments, or source maps. Cargo-project input is rejected explicitly. No cross-language CAOI-P information is extracted. These are outstanding work, not supported claims.
