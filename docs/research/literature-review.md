# Literature review (status: active)

**Last updated:** 2026-11-12. This replaces the preliminary 2026-06-04 notes. It is an interim, primary-source-led review; it is not exhaustive, and no novelty/absence claim is made.

## Closest research families

1. **C/C++ migration to safe or safer Rust:** C2Rust is a compatibility-oriented translator that commonly emits raw pointers and `unsafe`; Laertes, Crown, C2SaferRust, Scylla, and Cpp2Rust tackle different stages/design points. Scylla translates a structured C subset to safe Rust but states that it lacks a formal translation-correctness guarantee. Cpp2Rust’s safe output uses runtime pointer/ownership wrappers and optimization; C2SaferRust combines C2Rust with LLM-guided rewrites and tests but reports reductions, not elimination, of unsafe code. See [`research/prior_art.md`](../../research/prior_art.md), [`research/novelty_matrix.md`](../../research/novelty_matrix.md), and [`research/claim_ledger.md`](../../research/claim_ledger.md).
2. **Ownership/borrow inference and checking:** Crown/Laertes and C borrow-checker work are close to migration; Oxide and Polonius model/check Rust ownership and loans; Morphic infers borrows in a pure functional language. Earlier Java/C/C++ ownership inference also predates these migration systems (e.g., ECOOP 2012 best-typing inference, OwnKit, and Clouseau). These works differ in language, safety guarantee, source generation, and cost model.
3. **Cost-sensitive choices and formal validation:** CCured explicitly chooses pointer qualifiers to minimize instrumentation overhead subject to safety; &inator seeks least-cost correct C/Rust interfaces; Perceus/Morphic optimize ownership-related reference-counting/reuse. CompCert, RefinedC, translation validation, and translational separation-logic verification provide substantial neighboring proof techniques.

## Aster-specific finding

Aster has **no `unsafe` construct or escape hatch**. Its current accepted subset has only scalar values and scalar operations; pointers, references, arrays, allocation, deallocation, and FFI are absent and rejected. Thus the existing memory-safety boundary is a restriction on what can be expressed, not evidence that Aster has solved ownership inference for heap-manipulating programs. Backend semantic preservation remains unproved. See [`docs/spec/aster-safety.md`](../spec/aster-safety.md).

## Current conclusion

The broad claims “first automatic ownership inference,” “first safe source translator,” “first cost-aware representation choice,” and “first proof-guided compiler optimization” are not supportable. No narrow candidate has yet passed the novelty/feasibility gate. Root `research/` documents are the canonical working review; they explicitly label what has and has not been verified from full primary text. Continue literature coverage before revisiting the gate; no prototype or Aster performance result is claimed.
