# Benchmark results

**Aster research prototype:** none. **Aster ownership inference benchmarks run:** none. **Aster memory-safety translation results:** none. **Date:** 2026-11-12.

The current Aster compiler’s unit tests and controlled Ghidra smoke test are engineering checks for the implemented scalar/Ghidra subset; they do not measure CAOI, memory-bearing translation, general semantic equivalence, or performance. There is no Aster research analysis, generated ownership certificate, benchmark harness, or reproducible runtime dataset. No Aster proof or performance improvement is claimed.

## Related-paper measurements (not Aster results; not comparable across papers)

| Work | Reported measurement | Scope and caveat | Primary source |
|---|---|---|---|
| Cpp2Rust (2026) | Abstract reports ~2% performance penalty on WOFF2 compression and up to 6× slower on Brunsli; detailed tables separate compression/decompression and optimization variants | Two C++ programs totaling ~13k LOC; runtime wrappers and source optimizations; do not generalize to Aster or other suites | [PLDI paper, DOI 10.1145/3808266](https://doi.org/10.1145/3808266), §§6.1–6.3 |
| C2SaferRust (2026) | Reports up to 38% fewer raw pointers and up to 28% less unsafe code; benchmark outputs pass its available test suites | Seven Coreutils programs plus a selected subset of Laertes benchmarks; reductions are not elimination, and tests are not a proof | [IEEE TSE DOI 10.1109/TSE.2025.3641486](https://doi.org/10.1109/TSE.2025.3641486), [author manuscript](https://arxiv.org/abs/2501.14257v2) |
| Morphic (2026) | Author abstract reports 75–100% fewer reference-count increments on benchmarks with baseline increments and 1.48× geometric-mean speedup over Perceus | Pure functional Morphic language and its benchmark suite; abstract-level result in current review, full result-table context still to be checked | [OOPSLA DOI 10.1145/3798221](https://doi.org/10.1145/3798221), [author publication page](https://www.cs.princeton.edu/~mpmilano/publication/fully-automatic-type-inference/) |
| Checked C (2018) | Preliminary suite: 8.6% average runtime overhead with about 90% of code in checked regions; more than half the benchmarks under 1% | Runtime bounds-checking extension; benchmark-specific, not directly comparable with other rows | [SecDev paper, DOI 10.1109/SecDev.2018.00015](https://doi.org/10.1109/SecDev.2018.00015) |
| &inator (2026) | Constraint solving takes several hours for some programs of a few thousand lines of C | Compile-time interface inference, not generated-program runtime | [PLDI DOI 10.1145/3808270](https://doi.org/10.1145/3808270), limitations/scalability discussion |

These rows are literature data, not an aggregate benchmark comparison. Toolchain versions, targets, inputs, flags, and metrics differ. Do not infer a ranking or combine the numbers.

## Required before any Aster performance claim

A future Aster study must provide benchmark sources/commits, licenses, scripts, environment, pinned compiler versions and flags, exact input data, independent optimized baselines, generated-code/instrumentation audit, repeated-run uncertainty, and all failures/rejections. Design sketch is in `experimental_design.md`; it is blocked until novelty/feasibility passes.
