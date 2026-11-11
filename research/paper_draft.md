# Aster/CAOI-P research report — draft outline, not a paper

**Status:** preliminary evidence report; not suitable for submission. No novelty claim, theorem, prototype, or Aster benchmark is available.

## Working title

**Aster Ownership Inference: Prior Art, Feasibility, and an Interim No-Go**

## Draft abstract

We reviewed primary sources on ownership inference, C/C++ migration to memory-safe languages, static memory safety, cost-aware representation selection, and formal translation/verification. The closest work includes Scylla and Cpp2Rust for source migration; Crown, Laertes, C2SaferRust, and Morphic for ownership/safety refinement; and CCured and &inator for cost-constrained choices. We found no evidence supporting broad novelty claims for automatic ownership inference, safe-subset translation, or cost-aware optimization. Aster currently has no `unsafe` construct and only a scalar subset without heap operations, so its memory-safety result does not yet test ownership inference. A proof-producing C-to-safe-Aster translator remains a possible research question, but no distinct algorithmic delta or viable target model has been established. We therefore do not implement a research prototype or claim novelty at this stage.

**Abstract status:** evidence summary only. The statements are provisional and bounded by the review coverage in `prior_art.md`.

## Intended report structure

1. Research question, cutoff date, source-selection/evidence protocol.
2. Aster baseline and exact no-`unsafe` language boundary.
3. Three closest directions and detailed comparison matrix.
4. Separate answers for cross-language inference, static safety/no runtime checks, and cost-aware optimization.
5. Six candidate algorithms, ranking, closest prior art, and falsification criteria.
6. Novelty/feasibility gate and no-go decision.
7. Formalization/proof obligations required to reopen the lead (not proofs).
8. Conditional experiment design; explicit statement that no prototype/results exist.
9. Limitations, pending primary-source review, and conclusions.

## Claims this draft must not make

- “First,” “novel,” “publishable,” or “semantics-preserving” for Aster CAOI/CAOI-P.
- A mechanized safety theorem or backend soundness proof.
- That Aster currently handles pointers, references, arrays, allocation, or ownership inference.
- That no `unsafe` language construct alone implies safety for arbitrary translated source.
- That no added memory-safety checks imply zero runtime overhead.
- Any Aster benchmark, proof result, or cross-paper performance ranking.

## Evidence and status

- Full/interim literature and citations: `prior_art.md`, `novelty_matrix.md`, `claim_ledger.md`.
- Candidate ranking: `candidate_algorithms.md`.
- Gate decision: `novelty_assessment.md`.
- Open proof and semantics requirements: `formal_model.md`, `proof_obligations.md`, `feasibility_limits.md`.
- No Aster prototype/results: `experimental_design.md`, `benchmark_results.md`.

## Why it is not yet a conference paper

The related-work chain is incomplete; the best candidate has no specific new algorithm; Aster’s memory model is not defined; no proof artifacts exist; and no research experiments have run. Filling these gaps requires first finding a differentiating technical contribution, not merely implementing the already-known ideas in Aster.
