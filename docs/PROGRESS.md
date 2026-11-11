# Aster progress

## Status at 2026-06-04

- [x] Bootstrapped Rust workspace with lexer/parser, scalar type checker, checked native C-emitting backend, and fail-closed scalar safety gate.
- [x] Experimental strict migration front ends: Rust (tiny standalone subset), C/C++ (constant-return `main`), Python (`sys.exit` with signed i32 literal). Generated Aster is statically checked, built, and compared with the source execution for these accepted forms.
- [x] Added a Ghidra 12.1.4 extension script that exports versioned JSON evidence: program metadata, recovered signatures/types, high P-code varnodes and widths, CFG blocks/edges, instruction-level P-code, references, and decompiler text. With `ASTER_CLI` configured, its GUI path also displays a modal preview of supported generated Aster and the partial/rejected report status.
- [x] Added typed Rust IR parsing and a fail-closed lowerer. Only a P-code-proven constant-return `main` with debug-recovered signed i32 type is emitted. Other functions/operations are classified unresolved or unsafe-boundary and never replaced with placeholders.
- [x] Added a headless Ghidra smoke test. It exports a tiny compiled ELF, builds the generated Aster, and compares exit status/stdout/stderr with the original. The fixture reports partial recovery when standard runtime functions remain unresolved.
- [ ] Add broader Ghidra/P-code lowering, recovered-type confidence rules, and behavior-driven tests across architectures and optimization levels.
- [ ] Formalize core semantics and mechanically validate the compiler/backend safety argument.
- [ ] Implement borrow/ownership safety and evaluate CAOI independently of safety decisions.
- [ ] Continue primary-source research; CAOI novelty remains unclaimed.

## Current checkpoint

The Ghidra exporter and CLI pipeline are implemented for evidence extraction and one intentionally narrow lowering case. `aster ghidra lower <export.json> -o <output.as>` writes a source file only for a supported `main`, runs Aster safety verification/native compilation, and writes a `.ghidra-report.json`. An incomplete report produces a nonzero CLI status even if a supported subset source was emitted. Successful compilation is not behavioral equivalence; the smoke test separately compares one controlled run. Binaries without enough recovered type evidence fail closed.

The Aster language and static verifier remain scalar-only. C emission is a bootstrap backend, not direct LLVM lowering or a mechanized backend proof. Migration coverage is deliberately tiny, Cargo project migration is not implemented, and CAOI novelty/performance is not established.

## Next actions

1. Expand and adversarially test typed P-code lowering; preserve operation/address provenance and keep unknown constructs rejected.
2. Add differential fixtures for loops, calls, memory, ABI variations, stripped binaries, and multiple architectures; do not broaden accepted lowering until evidence supports it.
3. Continue direct literature review and update novelty assessment only with verified primary-source evidence.
