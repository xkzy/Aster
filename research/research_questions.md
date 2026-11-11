# Research questions and current answers

**Cutoff:** 2026-11-12. **Status:** provisional answers based on the sources listed in `prior_art.md` and `novelty_matrix.md`; the review is still incomplete. “Not found” is not proof of absence.

## 1. Is there a unified ownership-inference algorithm for C, C++, Rust, and Python?

**Current answer: no such unified algorithm is established by this review; this is not a universal absence claim.** The closest work is largely pairwise (C/C++→Rust) or source-language-specific (Rust borrows, Morphic’s pure functional language, region systems). The pipeline names often cover different tasks: interface inference, source rewriting, safe-target generation, verifier typing, compiler checking, or runtime management.

A common IR is plausible, but cannot erase source semantics. Any sound design needs per-language adapters for (at least) C’s object lifetime, pointer arithmetic/provenance and undefined behavior; C++ constructors/destructors, exceptions, aliasing and object identity; Rust move/drop/lifetime rules and unsafe blocks; and Python’s dynamic identity, aliasing, dispatch, exceptions, finalization and integer/object semantics. Reviewed Python systems such as Cython, mypyc, Numba, Nuitka, PyPy, and Codon address extensions, JIT/AOT specialization, or execution/runtime behavior; they are not evidence of cross-language ownership migration. The intersection must be defined explicitly, and inputs outside it must be rejected or reported as unresolved. A common IR alone is engineering, not an algorithmic contribution.

**Aster-specific consequence:** Aster has no `unsafe` construct or escape hatch. Its present scalar-only language cannot express ownership-bearing heap operations, so it cannot yet demonstrate a general ownership-inference algorithm. A future useful memory subset would need safe primitives and a static checker for every admitted operation; the current “no pointers” boundary is not a solution to inferred ownership for pointer-rich code.

## 2. Can static memory safety be achieved without added runtime memory-safety checks?

**Yes, conditionally, for a restricted accepted subset—but that fact is not novel by itself.** Safe Rust borrow/type checking, static ownership analyses, and safe-source translation show the general design point. Scylla’s source translation uses Rust’s static checks and rejects/reshapes unsupported cases; Crown statically retypes some pointers under its assumptions; Oxide models a safe ownership core. Conversely, Checked C, CCured and Cyclone use runtime checks or metadata where static evidence is insufficient. These are distinct safety models and not directly comparable overhead measurements.

**Aster today:** the parser rejects pointers, references, arrays, allocation/deallocation and FFI. For the scalar subset, memory-safety obligations are inapplicable because those operations are unexpressible. This is not a complete proof of backend preservation or machine-code safety; `docs/spec/aster-safety.md` records those limits. There is no Aster `unsafe` opt-out. The Ghidra report’s `UnsafeBoundary` is an unsupported-input status, not syntax in the language.

**Important distinction:** “no added runtime checks for memory safety” does not imply “zero runtime overhead” or “as fast as an unchecked native program.” Allocation, destruction, normal bounds checks, arithmetic semantics, ABI/runtime behavior and generated-code quality may still cost. Checked C’s preliminary benchmark reports an 8.6% mean runtime overhead with about 90% of code checked; CCured explicitly selects pointer qualifiers to minimize runtime overhead while preserving its model’s guarantees; Cyclone reports costs from dynamic checks and fat-pointer representations. Cpp2Rust uses runtime wrappers and optimization. These results come from different systems and suites; do not compare them as a shared benchmark.

A meaningful Aster claim must state (1) accepted source subset, (2) exact safety property and trust boundary, (3) which checks/instrumentation are absent, (4) other runtime operations still present, and (5) measured executable overhead against optimized baselines.

## 3. Is cost-aware ownership/representation optimization novel?

**Not at the level presently proposed.** Direct overlap includes:

- **CCured:** whole-program linear-time pointer-kind inference explicitly chooses qualifiers to minimize runtime overhead while preserving its memory-safety guarantees.
- **&inator:** SMT-based, semantics-aware C-to-Rust interface inference with a least-cost precision objective.
- **Perceus:** ownership/uniqueness-driven reference-count insertion and in-place reuse, with a formal resource-calculus account.
- **Morphic:** inferred borrows and whole-program mutability, using RC as fallback and reporting benchmark-specific count/runtime benefits.
- **Conventional compilers:** established alias, escape, region, copy, inlining, vectorization, and allocation-placement analyses; translation/optimization validation is established too.

Therefore a weighted objective over copies, ownership forms, or runtime checks is not itself a novelty claim. Aster would need a specific new choice space, sound algorithm, objective with a justified semantic interpretation, and an ablation showing a benefit beyond a strong sequence/composition of existing analyses. Cost dimensions such as compile time, memory footprint, source edit effort, proof burden, and machine time should not be casually merged into one arbitrary weighted sum.

## 4. Three closest research directions

1. **C/C++ migration to memory-safe Rust:** Scylla, Cpp2Rust, Crown, and Laertes cover restricted direct translation, runtime-wrapper translation/optimization, and static safety refinement. This is closest to Aster’s migration goal.
2. **Static ownership/borrow/lifetime inference:** Crown/Laertes and C borrow-checking work for migration; Morphic, Oxide and Polonius cover inference or formalization in different language settings. This is closest to CAOI’s ownership-analysis goal.
3. **Semantics- and cost-aware representation choice plus proof:** &inator and CCured already optimize representation/type decisions against correctness/safety constraints; Perceus/Morphic optimize reuse and reference-count behavior; CompCert, RefinedC, and translational verifier foundations establish mature proof techniques. This is closest to CAOI-P’s combined cost/proof goal.

These are research families, not a claim that any one paper already implements Aster’s entire proposed pipeline. Their overlap is sufficient to rule out broad claims such as “first automatic ownership inference,” “first static safe source translation,” “first cost-aware representation selection,” or “first proof-guided optimization.”

## 5. Does the strongest candidate pass the novelty gate?

**Not yet. No candidate currently passes.** The most promising hypothesis is a proof-producing, end-to-end translator for a nontrivial C memory subset into safe Aster, with no `unsafe` escape hatch and with statically justified ownership choices. It is not yet a contribution because:

- Scylla already translates a restricted C subset to safe Rust and analyzes pointer arithmetic/borrowing, albeit without an end-to-end correctness guarantee.
- &inator already formalizes correctness and least-cost choices for C-to-Rust interfaces.
- CCured already jointly chooses pointer representations and minimizes runtime checks under safety constraints.
- CompCert, verified translation validators, RefinedC, and translational separation-logic verifiers supply strong neighboring proof methods.
- Aster currently has no memory-bearing types or heap operations. A nontrivial experiment would require a substantial language/type-system/backend extension; translating only scalars would be uninformative.
- The present hypothesis is a combination of established translation, ownership inference, and certificate validation. The unique algorithmic delta has not been specified.

This is a **research go/no-go failure for implementation at this point**, not evidence that the broad field contains no possible publishable gap. Continue primary-source coverage and narrow the research question before reconsidering.

## 6. What evidence would change the decision?

A candidate must identify one precise analysis/translation decision not covered by the nearest work; state source and target semantics including C undefined behavior; emit Aster with no unsafe feature or unchecked boundary; prove or independently check a nontrivial safety/trace property; and beat a strong baseline on a prespecified dataset without hiding rejection rates or runtime checks. The literature review must first finish the primary-source chain around verified C translation, translation validation, source semantics, and proof-producing optimization. A prototype is warranted only after that comparison yields a defensible technical delta.
