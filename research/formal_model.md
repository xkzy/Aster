# Formal model status

**Status:** no research candidate passed the novelty gate; therefore no candidate formalization is claimed. This file records the minimum formal definitions needed before the leading hypothesis could be reopened. It is a requirements map, not a calculus, proof, or mechanized model.

## Current Aster fragment

Use [`docs/spec/aster-0.1.md`](../docs/spec/aster-0.1.md) and [`docs/spec/aster-safety.md`](../docs/spec/aster-safety.md) as the implementation contract: scalar `i32`/`bool` programs, no `unsafe` construct, no pointers/references/arrays/heap/FFI/concurrency. The current source-level memory-safety argument is by inexpressibility of memory operations; backend preservation remains an open trusted-code boundary. This fragment has no ownership-inference problem to formalize.

## Candidate 1 model still required (not defined here)

Before a proof-producing C→Aster ownership translator can be evaluated, its authors would need to define:

1. **Source domain:** a C syntax/typing subset, object lifetime and storage duration, pointer/provenance rules, integer conversions/overflow, allocation/deallocation/failure, external effects, and the exact defined-behavior domain. `undefined behavior` must not be silently assigned an Aster meaning.
2. **Target domain:** a safe Aster heap/object/array model, ownership and borrow types, allocation/initialization/destruction behavior, failure semantics, and target observation model. No `unsafe` construct or hidden unverified escape hatch may exist.
3. **Operational semantics:** small-step or big-step semantics for both source and target, configurations, observable traces, errors, allocation failures, and nondeterminism.
4. **Translation relation:** source-to-target term relation, memory relation, object identity relation, and treatment of source-defined executions. Decide whether the intended theorem is trace equality or trace refinement; do not conflate the two.
5. **Typing/ownership judgments:** unique owner, immutable/mutable borrow exclusivity, lifetime termination, bounds facts, initializedness, alias invalidation, and function-call summaries.
6. **Cost semantics:** count target operations and code/data size under a fixed abstract machine; keep compile-time cost, source edits, proof burden, and runtime cost as separate dimensions unless a justified scalarization is specified.

## Candidate theorem shapes (unproved)

Any later work would need to state and prove (for exactly the modeled subset):

- translation preserves the selected source behaviors on the defined-input domain;
- every generated Aster program is accepted by the static checker and has no memory-safety failure in its semantics;
- certificate checking implies the preceding properties independently of the synthesis procedure;
- backend compilation preserves Aster observations, or else explicitly leaves the backend trusted;
- any ownership-driven optimization preserves the chosen semantics and does not rely on unproved cost assumptions.

No theorem is asserted, no formalization is provided, and no proof assistant artifact exists for these candidate claims.
