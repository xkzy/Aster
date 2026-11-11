# Rust subset prior art (current summary)

**Updated:** 2026-11-12. This complements [`rust-migration-prior-art.md`](rust-migration-prior-art.md); full citations and evidence status are in [`research/novelty_matrix.md`](../../research/novelty_matrix.md).

The current experimental Rust migration supports a tiny, explicit source subset. This is an engineering boundary, not a contribution claim. C2Rust/Laertes/Crown/Scylla/C2SaferRust/Cpp2Rust show that source migration, compiler-guided safety refinement, ownership inference, and test-driven repairs already have substantial prior art. Oxide and Polonius formalize/check Rust ownership/loans; Morphic performs automatic borrow/lifetime inference in another language.

Do not infer that compiling or passing a fixture proves semantic equivalence for general Rust. The migration remains subject to documented restrictions on drop order, traits, generics, panic behavior, modules, macros, unsafe input, build scripts, and project-level semantics. Aster itself has no `unsafe` syntax; unsupported source constructs must be rejected, not copied into an unsafe target mode.
