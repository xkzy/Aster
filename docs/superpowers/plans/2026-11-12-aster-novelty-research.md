# Aster Novelty Research and Candidate Validation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to execute this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Determine from primary evidence whether a precise Aster contribution survives prior-art, formal, and experimental scrutiny, and validate it only if it does.

**Architecture:** Research the closest established methods first, recording verified claims and limits in a comparison matrix. Then formulate and rank at least five candidate algorithms; only the strongest candidate that passes a novelty/feasibility gate receives a formal model and prototype. A negative novelty result is a valid outcome and prevents unnecessary compiler work.

**Tech Stack:** Primary papers and proceedings records; DOI/publisher verification; Rust prototype only if justified; existing Aster compiler; Clang/LLVM as appropriate; SMT only if the candidate requires it; reproducible scripts and tests.

**Spec:** User request, “Aster — Deep Novelty Research and Algorithm Discovery,” 2026-11-12.

## Global Constraints

- Research must precede implementation.
- Do not assume the proposed architecture is novel.
- Do not fabricate papers, citations, experimental results, or claims of novelty.
- A combination of existing tools is not automatically a research contribution.
- Do not claim mathematical correctness without an actual proof.
- Do not claim performance improvements without reproducible measurements.
- Avoid implementing all four language frontends simultaneously.
- Select the smallest source-language subset capable of testing the proposed contribution.
- The prototype must contain real analysis, source generation, compilation, and executable tests; no stubs or hardcoded benchmark answers.
- Distinguish no added safety checks from no overhead relative to unsafe implementations.
- Reject unsupported constructs when safety or semantic equivalence cannot be established.

## Review Focus

- Cross-language object identity, drop/finalizer, exception, and allocation-failure behavior: compare semantics per source and reject unsupported mismatches.
- Aliasing, pointer provenance, integer overflow, and concurrency: never infer soundness from syntax alone; state assumptions and test adversarial cases.
- Uncertain inference/type data: keep unresolved rather than silently classifying as safe.
- Performance comparisons: use optimized, equivalent baselines and repeated measurements with uncertainty.
- Publication recency/status: verify work against paper/proceedings/DOI records at the research cutoff; flag preprints and future-dated listings.

---

### Task 1: Verify closest prior art and establish a claim ledger

**Files:**
- Create/update: `research/prior_art.md`
- Create/update: `research/novelty_matrix.md`
- Update: `docs/research/literature-review.md`, `docs/research/prior-art-matrix.md`

- [ ] Verify metadata and primary-source passages for source migration (C2Rust and “Translating C to Safer Rust”), C memory safety (CCured, Checked C, Cyclone), Rust ownership (borrow checker, Polonius, Oxide, Morphic), cost-aware/uniqueness/region work (Perceus, Hylo, MLKit), and formal verification/translation validation.
- [ ] Expand via citation chaining and recent venue searches; label a source as primary, official implementation documentation, or secondary.
- [ ] For each entry, record the requested fields, exact theorem/guarantee, overhead model, limitations, Aster overlap, and specific open gap; mark unknown fields as unknown.
- [ ] Answer the “three closest directions” question from the verified matrix, not from names or abstracts alone.

### Task 2: Decide the status of the three cross-cutting claims

**Files:**
- Create/update: `research/research_questions.md`
- Update: `docs/research/research-questions.md`, `docs/research/novelty-assessment.md`

- [x] Assess unified cross-language ownership inference, static memory safety without added checks, and joint cost-aware optimization separately and in combination.
- [x] Define the accepted-program boundary and distinguish static safety instrumentation from other runtime costs and from optimized native-source baselines.
- [x] State evidence strength and unresolved literature coverage; do not convert “not found” into a universal absence claim.

### Task 3: Formulate and rank candidate algorithms

**Files:**
- Create: `research/candidate_algorithms.md`
- Update: `docs/research/novelty-assessment.md`

- [x] Propose at least five distinct candidates and compare closest prior art, precise delta, inputs/outputs, assumptions, proof obligations, complexity, scaling, minimal experiment, and falsification criteria.
- [x] Rank by novelty potential, feasibility, formal tractability, and experimental value; document why candidates are rejected or deferred.
- [x] Apply a go/no-go gate. No candidate currently has both a distinct algorithmic delta and feasible nontrivial target experiment; recorded no-go; no novelty prototype built.

**Task 3 gate result (2026-11-12): NO-GO.** No candidate survives the current novelty/feasibility gate. Tasks 4–5 (candidate formalization/prototype experiment) remain blocked unless a later primary-source review identifies a precise new algorithm and a viable safe Aster memory model. The current `formal_model.md` and `proof_obligations.md` are requirements/checklists only, not a candidate formalization or proof.

### Task 4: Formalize only the surviving candidate

**Files:**
- Create/update: `research/formal_model.md`
- Create/update: `research/proof_obligations.md`
- Create/update: `research/feasibility_limits.md`

- [ ] Define source/target semantic domains, judgments, operational semantics, memory model, ownership/lifetime/alias constraints, and cost semantics for the narrow supported subset.
- [ ] State preservation/safety/optimization propositions with assumptions; prove what is tractable and list missing lemmas for the rest.
- [ ] Identify undecidability, incomplete analyses, FFI, concurrency, and dynamic-semantics boundaries explicitly.

### Task 5: Design and run a falsifiable prototype experiment

**Files:**
- Create/update: `research/experimental_design.md`
- Create/update: `research/benchmark_results.md`
- Modify code only after Task 3 passes: exact files selected by the candidate design.

- [ ] Pre-register hypotheses, strongest applicable baselines, metrics, benchmark selection, repetitions, uncertainty estimates, and ablations before implementation.
- [ ] Implement the smallest real analysis and source-generation path; add executable differential and safety tests before broadening the subset.
- [ ] Run correctness and performance experiments reproducibly; publish negative results and avoid comparisons with unoptimized baselines only.

### Task 6: Publication and decision report

**Files:**
- Create/update: `research/paper_draft.md`
- Update: `research/novelty_assessment.md`, `docs/PROGRESS.md`

- [ ] Write only evidence-supported claims, exact closest competitors, limitations, and publication-fit rationale.
- [ ] Summarize the strongest supported novelty claim (possibly “none yet”), formal evidence, experimental evidence, failure cases, and next research steps.
- [ ] Check every citation and every quantitative claim against stored primary evidence.
