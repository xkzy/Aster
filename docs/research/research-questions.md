# Research questions

1. After direct comparison with Morphic, what nontrivial automatic ownership decision remains unsolved under Aster's subset and value semantics?
2. Can an explicit weighted objective select among borrow/copy/move/reuse in a tractable way, or are legal choices better represented as ordered greedy refinements with proof-checkable local certificates?
3. Does joint selection measurably beat sequential borrow checking + copy elimination + escape/reuse analysis, after equal LLVM optimization and target flags?
4. Which ownership facts are strong enough to justify LLVM alias/effect attributes under the exact LLVM LangRef semantics, including pointer derivation, capture, and writes?
5. What is the smallest Aster core that can demonstrate observable value semantics, checked arithmetic and static safe borrowing without an unchecked backend gap?
6. Can module summaries carry effect, escape, alias and reusable-storage facts incrementally while preserving soundness under separate compilation?
7. Under what conditions can allocation placement move heap storage to stack/regions without changing failure behavior, stack limits, or externally observable allocation behavior?
8. Which candidate ownership-guided optimizations beat equally configured C (`restrict` when valid), Rust, and LLVM's existing passes on representative workloads?
9. Can proof certificates for copy elimination and reuse be independently checked at lower cost than re-running the optimization?
10. What are the true formal boundaries around FFI, concurrency, target ABI, backend semantics and allocation failure?

## Falsifiable early hypothesis

The high-level formulation “cost-aware joint inference” alone is not novel. A possible engineering hypothesis—not a novelty claim—is that a small, sound ownership-fact interface can allow LLVM to optimize more reliably without programmer alias annotations. This is disproven if equivalent C `restrict` annotations or Rust/LLVM passes yield the same code, if Aster cannot prove additional valid cases, or if compilation/runtime costs outweigh measured benefits.
