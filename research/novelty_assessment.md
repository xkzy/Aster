# Novelty assessment

**Research cutoff:** 2026-11-12. **Decision:** no publishable algorithmic contribution is established by the current evidence; no research prototype is justified at this stage. This is an interim no-go, not a universal claim that no possible Aster contribution exists.

## Executive finding

Aster’s lack of an `unsafe` construct is an important language constraint. Its present scalar-only implementation, however, has no pointers/references/arrays/allocation/FFI, so its current memory-safety boundary is by exclusion and cannot demonstrate ownership inference. Extending the memory model safely would itself be a substantial language/compiler project.

The broad CAOI/CAOI-P ideas overlap with established work: ownership/borrow inference (Crown, Laertes, C borrow-checking work, Morphic, ECOOP 2012, OwnKit, Clouseau), C/C++→safe/safer Rust migration (Scylla, Cpp2Rust, C2SaferRust), cost-sensitive representation and check selection (CCured, &inator), and ownership-driven reuse/region/copy control (Perceus, Hylo, MLKit). Verification/translation-validation methods are also mature (CompCert, VST-Floyd, RefinedC, RustBelt, Aeneas, symbolic Rust borrow checking, translational verifier foundations, and formal translation validators).

## Strongest lead and why it is not selected

The strongest lead is a machine-checkable, ownership-guided C-to-safe-Aster translator for a nontrivial memory-bearing C subset. Current evidence leaves a *guarantee gap*—Scylla explicitly lacks a formal translation-correctness guarantee; Crown’s theorem is local to ownership transfer/unique-owner preservation; &inator handles interface types rather than bodies. But no new ownership decision procedure or certificate calculus has been identified. Combining these methods with an established translation validator is not, by itself, a novel algorithm. Aster also lacks the memory-bearing target constructs required for a non-vacuous experiment.

## Gate outcome

- Unified ownership inference for C/C++/Rust/Python: **not established**, but absence is unproven; semantics differ and the review is not exhaustive.
- Static memory safety without inserted runtime checks: **established design point for restricted accepted languages**, but Aster’s current case is vacuous for heap safety because memory is unexpressible.
- Cost-aware ownership optimization: **not novel as a broad claim**; CCured explicitly minimizes runtime-check cost under safety constraints, &inator minimizes interface type cost, and other systems optimize ownership-related reuse/RC/regions.
- Best candidate: **fails novelty/feasibility gate pending a precise algorithmic delta and a safe Aster memory model**.
- Prototype, Aster performance experiment, semantic-equivalence theorem, and publication claim: **not produced**.

## Reopen criteria

Reopen only after a primary-source review identifies a decision/certificate not covered by nearest work; define source C UB and target Aster semantics; specify all safety constraints with no `unsafe` escape hatch; establish a feasible proof boundary; and preregister a controlled experiment against strong comparable baselines. See `candidate_algorithms.md`, `claim_ledger.md`, `formal_model.md`, `proof_obligations.md`, and `experimental_design.md`.
