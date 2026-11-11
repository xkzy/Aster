# Novelty assessment (interim, no claim)

**Date:** 2026-11-12. **Status:** no candidate currently passes the novelty/feasibility gate.

Aster’s no-`unsafe` design is a real constraint: the language offers no unsafe escape hatch. However, its current scalar-only subset has no pointer, reference, array, allocation, deallocation, or FFI operations, so memory-safety obligations are inapplicable rather than solved by a novel ownership algorithm. The backend preservation argument is not mechanized.

Substantial related work covers restricted C/C++→safe/safer Rust translation (Scylla, Cpp2Rust), ownership-guided retyping and compiler-directed refactoring (Crown, Laertes), LLM/compiler-assisted safety refinement (C2SaferRust), automatic borrow inference (Morphic), cost-minimizing representation/interface choices (CCured, &inator), best-typing objectives for ownership systems, uniqueness-aware reuse (Perceus), and formally verified compilation/translation checking (CompCert, RefinedC, Oxide, translation validation, and translational verifier foundations).

A proof-producing C-to-safe-Aster translator is the strongest research lead, but the current description has no precise new inference rule or certificate beyond composition of known techniques. It also requires a nontrivial safe Aster memory model that does not exist. A prototype now would risk demonstrating only a tiny, vacuous subset or an engineering port.

**Decision:** continue primary-source coverage and narrow the question. Do not implement the research prototype or claim publication novelty yet. A negative/no-go result is valid. See [`research/candidate_algorithms.md`](../../research/candidate_algorithms.md), [`research/novelty_matrix.md`](../../research/novelty_matrix.md), and [`research/feasibility_limits.md`](../../research/feasibility_limits.md) when available.
