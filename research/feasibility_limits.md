# Feasibility limits and research boundaries

**Status:** interim analysis, 2026-11-12. The strongest candidate is not selected, and no prototype is justified yet.

## Aster-specific blocker

Aster has **no `unsafe` construct or escape hatch**. The current language contains only scalar `i32`/`bool` values and has no pointers, references, arrays, allocation/deallocation, FFI, or concurrency. Its current memory-safety result follows from those constructs being unavailable; it cannot infer ownership for them. A memory-bearing C→Aster study would first require a safe target memory model, type rules, verifier, backend representation, and semantics. That is a language/compiler design project before it is a translator experiment.

A scalar-only C subset could be translated today, but its lack of ownership-bearing memory makes it unable to test CAOI. Adding a single `unsafe` escape hatch would violate the Aster premise and invalidate the proposed safety contribution.

## Source-semantics limits

- **C:** pointer arithmetic, object lifetime, effective type, provenance, integer conversions, undefined/unspecified/implementation-defined behavior, allocation failure, volatile/atomic operations, and external calls must be modeled or rejected. Equivalence claims must be scoped to well-defined executions under a named ABI/compiler model.
- **C++:** constructors/destructors, exceptions, object identity, moves/copies, virtual dispatch, templates, allocator behavior, and dynamic object lifetimes compound the C boundary. C++2Rust’s runtime wrappers show one existing tradeoff; static no-check claims cannot assume those semantics away.
- **Rust:** ownership annotations and drop glue are already source semantics; `unsafe` blocks/libraries require contracts. Aster has no corresponding feature and should reject unsupported Rust input.
- **Python:** arbitrary aliasing/identity, dynamic dispatch, reflection, exceptions/finalizers, arbitrary-precision integers and runtime protocols prevent a general static ownership mapping. Any accepted subset must be named and tested.

No single source-independent ownership graph can erase these differences. Frontends need language-specific semantic adapters. If the common fragment is so restricted that no ownership-sensitive operation remains, it is not an informative demonstration.

## Analysis and proof limits

- Whole-program points-to/alias/lifetime questions can be undecidable in expressive languages; practical analyzers trade soundness, precision and cost. Aster must return `unknown/reject` on unresolved cases.
- Constraint solving for finite ownership choices may be NP-hard or worse after arithmetic/heap theories; &inator already reports superlinear scaling and hours on programs of a few thousand LOC. No solver complexity or Aster scaling result is known.
- A source-to-source theorem does not prove the parser, backend, native compiler, ABI, runtime, external libraries, or hardware unless they are inside the proof boundary.
- A certificate can validate only the formalized property and model. Test differential equality, ASan, Miri, fuzzing, and sanitizers are valuable bug-finding evidence, not a substitute for a proof.

## Runtime-cost limits

Static borrow checks can avoid dynamic borrow enforcement while allocations, drops, bounds checks, arithmetic traps, region bookkeeping, and ABI costs remain. Checked C, CCured, Cyclone, and Cpp2Rust measure different safety mechanisms and workloads. “No extra runtime memory-safety checks,” “zero overhead,” and “same speed as optimized C/C++” are separate propositions. Any experiment must audit generated code and report each metric separately.

## Prototype feasibility

Candidate 1 (proof-producing C→safe-Aster translation) is presently **low feasibility**: it requires new source parsing, source semantics, memory-bearing Aster features with no unsafe mode, ownership/bounds proof, code generation, and backend reasoning. A small scalar prototype would be vacuous; a memory prototype without a proved/checkable target model would make the central claim unsupported. A proof certificate added to a toy translator is not enough to pass novelty review.

## Scope that can be pursued without product-code changes

Continue verifying primary sources and narrowing the claim. A literature-backed no-go report is valid. Reopen implementation only if a precise algorithmic delta and a useful safe target fragment emerge; otherwise focus on Aster engineering rather than a novelty claim.
