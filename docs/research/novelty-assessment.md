# Novelty assessment (provisional)

## Claim status

**No novelty claim.** CAOI and CAOI-P remain names for hypotheses, not established contributions.

## Known overlap

The requested ingredients have substantial prior art: Rust/Polonius and Oxide cover loan/lifetime checking; Morphic appears to target fully automatic borrow/lifetime inference (publication status must be verified); Hylo addresses native mutable value semantics; Perceus provides uniqueness-aware reuse under reference counting; MLKit infers regions; LLVM and conventional compilers already combine escape, alias, copy, inlining and loop analyses. A weighted cost objective does not by itself make an algorithm novel.

## Potentially defensible question (not yet a contribution)

Can a practical, deterministic, incremental, independently checkable optimizer jointly choose among static borrow, logical copy, ownership transfer, and storage reuse in a CFG with checked arithmetic/value semantics, while preserving a small safety kernel and improving a defined cost vector over a strong sequence-of-passes baseline? A defensible contribution would require exact constraints and deterministic tie-breaking, sound evidence checking, module summaries, complexity bounds, and ablations showing a benefit unavailable from composing established passes. No such algorithm or evidence exists yet.

The extension CAOI-P is broader still; inlining, representation choices, allocation placement, PGO and loop transforms are established optimization families. Their coupling to ownership may be useful engineering, but must be compared against LLVM/Clang/Rust optimized pipelines with equivalent flags and annotations. A combined objective is not sufficient novelty.

## Gate conditions before revising this assessment

1. Verify and study Morphic's actual publication/manuscript, restrictions, algorithm and implementation.
2. Study Perceus and Hylo details and compare exact reuse/copy conditions.
3. Specify the Aster core and candidate analysis precisely.
4. State a falsifiable difference from prior art.
5. Implement independently checkable transformations and an unoptimized baseline.
6. Run reproducible differential correctness tests and ablations against established pass sequences.
7. Report negative results and distinguish improved defaults/inference from genuinely new optimization.
