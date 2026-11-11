# Aster ownership/migration research

**Cutoff:** 2026-11-12. This directory contains an active, evidence-led prior-art review. It is not a final exhaustive survey or a publishable paper. No Aster ownership prototype or benchmark exists, and no novelty claim is made.

## Start here

- [`novelty_assessment.md`](novelty_assessment.md) — interim decision and no-go rationale.
- [`research_questions.md`](research_questions.md) — separate answers on cross-language inference, static safety/no added runtime checks, and cost-aware optimization.
- [`prior_art.md`](prior_art.md) — narrative closest-work review and sources.
- [`novelty_matrix.md`](novelty_matrix.md) — detailed work-by-work comparison and review status.
- [`claim_ledger.md`](claim_ledger.md) — claims, exact primary-source locators, caveats, and evidence status.
- [`candidate_algorithms.md`](candidate_algorithms.md) — six candidates, rankings, experiments, falsification, gate decisions.

## Formal/experimental status

- [`formal_model.md`](formal_model.md) and [`proof_obligations.md`](proof_obligations.md) are requirements/checklists only; no candidate passed the gate and no proof exists.
- [`feasibility_limits.md`](feasibility_limits.md) records current Aster/source-semantic limitations.
- [`experimental_design.md`](experimental_design.md) is conditional and not a preregistration.
- [`benchmark_results.md`](benchmark_results.md) explicitly records that no Aster research experiment ran; literature metrics are labeled as such.
- [`paper_draft.md`](paper_draft.md) is a non-submission outline/report.

**Aster premise:** no `unsafe` construct or escape hatch. The current language is scalar-only, without pointer/reference/array/heap/FFI operations. The current memory-safety property is by inexpressibility for this subset; it does not establish heap ownership inference or backend preservation.
