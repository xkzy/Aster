# Rust migration prior art (current summary)

**Updated:** 2026-11-12. The expanded source comparison is in [`research/novelty_matrix.md`](../../research/novelty_matrix.md), with primary claim citations in [`research/claim_ledger.md`](../../research/claim_ledger.md).

C-to-Rust migration is a mature and active research area. C2Rust translates source but commonly emits raw pointers/`unsafe`; Laertes uses compiler-guided rewrites; Crown infers ownership for some C2Rust output; Scylla translates a structured C subset directly to safe Rust (but states it has no formal translation-correctness guarantee); C2SaferRust uses static decomposition, LLM rewrites, compiler/test feedback and reports partial reductions in unsafe code; Cpp2Rust translates a C++ subset using runtime wrappers and optimization.

These distinctions matter: “safe/safer Rust,” “no `unsafe` in Aster,” “no inserted runtime safety checks,” and “semantic equivalence” are different properties. Aster has no `unsafe` construct, but its current scalar-only subset cannot exercise ownership inference over heap memory. No broad C/C++ migration novelty claim is supported, and no Aster memory-safety or runtime-performance result is implied.
