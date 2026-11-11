# Candidate algorithms and novelty gate

**Status:** screened at paper level on 2026-11-12; none currently passes the gate. Scores are qualitative and provisional, not measured. “Novelty potential” is not a claim of novelty.

## Ranking summary

Scale: H = high, M = medium, L = low. Feasibility reflects the existing Aster implementation and likely prototype burden.

| Rank | Candidate | Novelty potential | Feasibility | Formal tractability | Experimental value | Gate |
|---:|---|:---:|:---:|:---:|:---:|---|
| 1 | Proof-producing, ownership-guided C→safe-Aster translation | M (could become H only with a new decision procedure/certificate) | L | M | H | **Fail for now:** no distinct algorithm yet; Aster has no heap/memory language to target |
| 2 | Staged static C memory-safety analysis with reject-not-instrument policy | L–M | L–M | M | M | **Fail:** Rust-like C borrow analysis and safe-C systems already cover much of this design point |
| 3 | Compiler-oracle/CEGIS migration with edit minimization and abstention | L | M–H | L–M | M | **Fail:** close to Laertes, Crown, Scylla workflows; objective and certificate not specified |
| 4 | Cost-constrained ownership/representation inference (MaxSMT/ILP) | L | M | M | H | **Fail:** CCured and &inator already cover cost-minimizing choices under safety/semantic constraints |
| 5 | One ownership IR for C, C++, Rust, and Python with semantic adapters | L | L | L | M | **Fail:** broad integration, no new inference rule; incompatible source semantics dominate |
| 6 | Ownership-fact-guided Aster backend optimization | L | L–M | M | H | **Fail:** current Aster has no memory-bearing operations; standard alias/escape/reuse optimizations are strong baselines |

## Candidate 1 — proof-producing, ownership-guided C→safe-Aster translation

**Hypothesis:** For a bounded, defined-behavior subset of C with arrays and heap allocation, infer a safe Aster representation and emit a small independently checked certificate proving that the generated program refines the source on defined executions and satisfies Aster’s static memory-safety rules. Aster has no `unsafe` escape hatch; unresolved operations are rejected.

**Input / output.** Input: Clang-parsed C plus target ABI/configuration, with functions, local aggregates/arrays, allocation, pointer arithmetic, branches and loops in a precisely bounded subset. Output: safe Aster source, source map, a proof/constraint certificate, explicit rejected/unresolved cases, and a manifest of any target/runtime checks. This requires a new Aster memory model; it is not implementable as a meaningful extension of today’s scalar-only subset without a separate language-design decision.

**Proposed procedure.** Build typed control-flow and memory-access graphs; infer candidate object/region/unique-borrow facts; enumerate only target representations expressible in safe Aster; generate side conditions for bounds, initialization, alias exclusion, allocation lifetime and each rewrite; solve or conservatively reject; emit translation plus a certificate checked by a small independent checker. The certificate must cover the whole body, not only interface types.

**Closest prior art and potential delta.** Scylla already translates a restricted C subset to safe Rust with pointer-arithmetic and borrow analyses, but explicitly reports no translation-correctness guarantee. &inator reasons about semantically correct, least-cost C/Rust interfaces but not bodies. Crown infers ownership from C2Rust MIR. CCured chooses pointer representations to minimize runtime checks. Translation validation, CompCert, RefinedC and translational verifier frameworks establish mature proof techniques. The only plausible delta is a *specific* body-level ownership synthesis/checking algorithm and a nontrivial independently checkable whole-translation certificate for a memory-bearing Aster subset—not “we use SMT,” “we produce safe code,” or “we prove it.” No such algorithm is specified yet.

**Assumptions / proof obligations.** Define C’s object and pointer semantics, UB domain, integer widths/overflow, allocation failure, aliasing, and observable traces; prove every generated operation is expressible and safe in Aster; establish type/ownership inference soundness; prove source-to-target trace inclusion for defined executions; establish certificate-checker soundness; prove the backend preserves Aster semantics. FFI, concurrency, volatile/atomic effects, pointer-to-integer conversions, unions, setjmp/longjmp, and unmodeled allocation behavior must initially be rejected.

**Complexity / scaling.** Candidate ownership choices form a finite constraint problem, but solver worst-case complexity and practical scaling are not established. The design must report timeouts/unknowns as rejected, not “safe”; report certificate size/checking time separately from synthesis time.

**Minimal falsifiable experiment if reconsidered.** After an Aster memory model exists, use small C programs with loops, aliasing, interior pointers, allocation/free, and boundary cases; compare accepted programs, output safety, certificate checking, and behavior against Scylla, C2Rust+Crown/Laertes, and hand-written safe Aster. Pin compilers/flags; use differential traces on defined inputs and sanitizer/instrumented runs only as bug-finding support, not as proof. Report rejection rate, edit count, compile/certificate time, binary size, runtime, and explicit instrumentation. Any comparison must include an equally optimized native C baseline.

**Falsification.** Fail if all accepted examples are scalar/trivial; if the certificate is merely a generic translation validator with no new ownership rule; if existing systems plus a standard proof/validation framework reproduce the same decisions and guarantees; if the subset is too narrow to test meaningful ownership; or if Aster must add unchecked/`unsafe` escape hatches.

**Gate decision:** **Fail for implementation now.** The conceptual guarantee is attractive, but no algorithmic delta over Scylla + &inator/Crown + established validation/proof techniques has been established, and current Aster cannot express the needed target memory operations.

## Candidate 2 — statically safe C memory subset; reject rather than instrument

**Input / output.** C AST/CFG and a target ABI; emit only programs whose nullness, initialization, lifetime, aliasing, and bounds obligations are statically discharged, otherwise reject. Aster target would need safe array/owner/borrow constructs, with no `unsafe` or runtime borrow checks.

**Procedure sketch.** Points-to and object-lifetime analysis; CFG loan/region dataflow; bounds-range proof; type-directed lowering to Aster owner/borrow types; reject unknown joins or unsupported provenance. Complexity depends on pointer analysis and range proof; no tractability claim without an exact fragment.

**Closest prior art / delta.** “Foundations for a Rust-Like Borrow Checker for C,” Crown, Scylla, Checked C, CCured, Cyclone, and safe Rust cover adjacent pieces. “No runtime checks” is not a new property by itself; the potential gap would have to be a new sound, useful pointer/bounds analysis for an Aster-specific semantic fragment, not simply choosing rejection over instrumentation.

**Proofs / test.** Define memory model and prove no invalid access for accepted programs; reject soundly whenever analysis is unknown. Test adversarial aliasing, pointer arithmetic, lifetime escapes, integer overflow in bounds expressions and malformed allocation pairs. Benchmark static acceptance against the closest checker and runtime costs against instrumented and native baselines.

**Falsification / rank.** If acceptance is no better than a basic borrow checker or the subset forbids all interesting pointer behavior, this is not a research contribution. **Fail currently:** high overlap and no Aster memory type system.

## Candidate 3 — compiler-oracle CEGIS migration with source-edit minimization

**Input / output.** Restricted C or C++ and a safe Aster candidate; output an edit sequence, generated Aster, rejection explanation and differential tests. Generate one candidate edit at a time; use the Aster parser/type/safety checker as oracle; use a source/target trace checker for semantic constraints; rank legal candidates by a declared edit/runtime/proof cost.

**Closest prior art / delta.** Laertes uses compiler-guided search/refactoring; Crown does ownership-guided Rust retyping; Scylla asks for manual source restructuring; Cpp2Rust optimizes generated Rust. Aster has no unsafe escape hatch, but “use the compiler as a checker” and “minimize edits” are not novel. A delta needs a new proof-aware search algorithm with measurable reduction in human changes without unsafe output or semantic loss.

**Proofs / test.** Every edit rule needs a precondition/postcondition or validation certificate; search failure must not imply input invalid. Evaluate success/rejection, edit distance, semantic behavior, and human-equivalent effort on fixed examples. Compare against the same translator with greedy edits and against existing pipelines.

**Complexity / falsification.** Search can explode with interacting rewrites; compiler feedback is incomplete and may not point to the cause. Fail if the tool only wraps existing compiler errors or if the minimum is arbitrary (token edit distance hides readability/semantic redesign). **Fail currently:** too close to existing CEGIS/refactoring systems.

## Candidate 4 — cost-constrained ownership/representation inference

**Input / output.** A finite program representation and a set of legal alternatives (owned value, immutable borrow, mutable borrow, clone/copy, region allocation, etc.); output a legal Aster program minimizing an explicit cost vector. A weighted scalar cost is only an engineering choice unless justified; a Pareto frontier may be more honest.

**Procedure sketch.** Encode ownership, source-semantic, target type and safety constraints, plus declared costs, as MaxSMT/ILP; use deterministic tie-breaking; emit proof/constraint evidence. Any unknown or timeout returns no result. Expected solver scalability is unknown and should not be assumed.

**Closest prior art / delta.** &inator explicitly pursues correct and least-cost C-to-Rust interfaces. CCured’s whole-program pointer-kind inference explicitly minimizes runtime check cost while preserving safety. Perceus and Morphic optimize ownership-related runtime operations. Potential delta only if Aster has a materially new choice space/semantics/certificate and controlled experiments beat composition of these systems.

**Falsification / rank.** If a standard cost optimizer or sequential analysis yields the same choices; if weights are tuned post hoc; if “cost” ignores edit/proof burden; or if improvements vanish under equal compiler flags, the claim fails. **Fail currently:** multiobjective inference is not itself novel.

## Candidate 5 — cross-language ownership IR with source-specific adapters

**Input / output.** C, C++, Rust and Python frontends map a common bounded fragment into an ownership/effect/identity IR; a target Aster backend emits only safe forms and reports source-specific exclusions.

**Procedure sketch.** Define language-specific extraction judgments and semantic adapters; preserve object identity, destructor/finalizer effects, exceptions and integer behavior; map only equivalent constructs to the IR; reject ambiguity. The central risk is that the common subset becomes too small or the adapters become four unrelated frontends.

**Closest prior art / delta.** Existing language migrators and frontends cover pairwise paths; static analyzers and compiler IRs normalize syntax. The proposal has no novel inference rule yet. Cross-language support count is not an algorithmic contribution.

**Proofs / experiment.** Prove each adapter preserves its source fragment into the IR, then prove IR-to-Aster lowering; test identity, drop order, exceptions, aliasing, overflow and finalization. Compare per-language coverage and semantics against mature translators, not only a shared toy corpus.

**Falsification / rank.** If each adapter encodes distinct semantics and shares only syntax/AST scaffolding, there is no unified ownership algorithm. **Fail:** poor tractability and no evidence of a useful common fragment.

## Candidate 6 — ownership-fact-guided backend optimization

**Input / output.** Well-typed Aster plus checked ownership/escape/immutability summaries; output native code with alias attributes, stack/region placement, copy elimination or reuse only when justified.

**Procedure sketch.** Derive facts from source typing and CFG; verify each fact against the backend memory model; lower facts to optimization metadata; run a standard optimizer. In the current scalar-only Aster there are no heap/alias/copy operations, so this candidate cannot be evaluated without language expansion.

**Closest prior art / delta.** LLVM/GCC already implement alias, escape, inlining, copy and allocation-placement analyses; CompCert/Alive2/translation validation cover verified transformations; Perceus and Morphic make ownership facts operational. The delta must be a specific class of optimization unavailable to those passes, with proof and measured codegen difference.

**Falsification / rank.** Fail if the same optimized IR/binary results from existing passes/attributes, or if the benchmark involves no memory operations. **Fail:** current Aster baseline has no relevant operations.

## Overall gate

No candidate has both (a) a specific, source-backed technical delta over its closest prior art and (b) a feasible nontrivial experiment in current Aster. The strongest lead (Candidate 1) remains a question, not a selected contribution. Do not implement a research prototype until the literature gap is narrowed and the target memory model is explicitly authorized/designed. A negative result is a valid deliverable.
