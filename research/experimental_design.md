# Experimental design (blocked; no Aster experiment registered or run)

**Status:** The novelty gate failed at paper level. This is a conditional design sketch only; it is not a preregistration and must not be presented as an experiment. No product code or research prototype was changed.

## Reopen criteria

Do not run this study unless a specific C→safe-Aster ownership algorithm survives comparison against Scylla, Crown, Laertes, &inator, CCured, C2SaferRust, and translation-validation/proof systems, and an Aster safe memory model is specified without an `unsafe` escape hatch.

## Conditional hypotheses

- **H1 (coverage):** The new analysis accepts a meaningful subset of ownership-sensitive C routines that existing safe-source approaches reject, while producing only statically verified Aster.
- **H2 (correctness):** All accepted test instances satisfy the stated source-to-target trace relation; a certificate checker rejects intentionally corrupted certificates/translations.
- **H3 (cost):** Any reduction in runtime checks/copies or generated code remains after comparison against optimized native C and the strongest available safe-translation baseline.
- **H4 (cost tradeoff):** The new solver’s accepted-program rate, analysis time, certificate size/check time, source edits, binary size, and runtime lie on a reported Pareto frontier; no single post-hoc weighted score hides regressions.

H1–H4 are hypotheses, not claims or expected results.

## Dataset and baselines (conditional)

- **Fixtures:** hand-checked C cases for unique ownership, shared immutable alias, mutable alias, interior-pointer/split-array patterns, loops, recursion, allocation/free, bounds edge cases, integer overflow in index calculations, and rejected UB/FFI/volatile/atomic cases. Include positive and negative cases; the test oracle must distinguish “defined source behavior” from undefined behavior.
- **Real code:** only after the toy semantics and checker are fixed, use a reproducible subset from public C-to-Rust benchmark suites (e.g., CRUST-Bench/Scylla case studies) with their licenses and exact commits recorded. Do not select only cases the prototype already handles.
- **Baselines:** Scylla on its supported fragment; C2Rust followed by Crown/Laertes or C2SaferRust where compatible; hand-written safe target; optimized Clang/GCC C baseline; existing Aster scalar migration only as an unrelated engineering baseline. &inator is a type/interface baseline, not a body translator.
- **Compiler controls:** pin Clang/LLVM/Rust/Aster versions, target triple, ABI, optimization/LTO flags, allocator, CPU, OS, and input data. Confirm generated memory-safety instrumentation by source/IR/binary inspection.

## Metrics

1. **Correctness/safety:** accepted cases, rejected cases by reason, exact counterexamples, differential traces/stdout/stderr/exit code, certificate accept/reject tests, fuzz/property tests. Testing is not proof.
2. **Coverage:** function/program acceptance, supported construct distribution, percentage needing source edits/annotations, and unsupported boundary causes.
3. **Translation effort:** semantic source edits, mechanical edits, annotations, proof obligations, manual repair time, and reviewer time reported separately.
4. **Resource cost:** static analysis time, peak memory, certificate generation and independent check time, binary size, allocations/copies/runtime guard count, and target runtime.
5. **Performance:** repeated runs after warmup, randomized/interleaved order, enough repetitions to estimate uncertainty, report median and confidence intervals, disclose outliers. Keep compiler configuration and baseline equivalence fixed.

## Ablations and falsification

- Disable inferred ownership choices; compare with a fixed conservative representation.
- Replace certificate checking with compilation/tests; report which assurance is lost.
- Compare a single pass/greedy pipeline with the alleged joint solver.
- Remove source refactoring and measure rejection/coverage.
- Compare static proof success with dynamic-check fallback—although Aster should reject rather than insert hidden memory-safety checks.
- Falsify the contribution if accepted cases are only scalar/trivial, the same decisions arise from standard analyses, correctness depends only on tests, runtime gains vanish under optimized equal baselines, or acceptance/compile time is not useful.

No values, scripts, benchmark outcomes, or performance claims are available today. See `benchmark_results.md` for the explicit empty-results record.
