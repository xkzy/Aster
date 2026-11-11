# Prior-art matrix (current summary)

**Updated:** 2026-11-12. The complete interim matrix, citation links, evidence status and limitations are maintained in [`research/novelty_matrix.md`](../../research/novelty_matrix.md). The claim-to-primary-source ledger is [`research/claim_ledger.md`](../../research/claim_ledger.md). This summary is not exhaustive.

| Direction | Closest work | What is already established | Aster implication |
|---|---|---|---|
| C/C++ → safer source | C2Rust, Laertes, Crown, C2SaferRust, Scylla, Cpp2Rust | Mechanical translation, ownership refinement, safe-subset translation, compiler/LLM-assisted rewrites, runtime wrapper lowering and optimization | No novelty from “translates C/C++ to a safer language” or “reduces unsafe code.” Specify a distinct algorithm and exact semantic guarantee. |
| Ownership/borrow inference | Rust MIR/Polonius, Oxide, Crown, C borrow-checker work, Morphic, ECOOP 2012 object ownership inference, OwnKit, Clouseau | Static loan/lifetime checking, inference of ownership annotations, flow/context-sensitive ownership analyses, automatic borrow inference in a different language | Aster’s no-`unsafe` language boundary is not itself a new inference algorithm. Current Aster has no heap model. |
| Static C memory safety | CCured, Checked C, Cyclone, Scylla | Static restrictions/analysis can discharge some obligations; other designs add runtime checks/metadata or reject uncertain programs | Report rejected inputs and all runtime mechanisms separately. “No added memory-safety checks” is not “zero runtime cost.” |
| Cost-aware choice/reuse | CCured, &inator, Perceus, Morphic, region/alias/escape/optimizer work | Cost/preference objectives and ownership-driven reuse are established in several forms | A weighted objective or combined pass list is not enough for novelty. Define a new choice space and beat composed baselines. |
| Formal reasoning/validation | CompCert, RefinedC, Oxide, formal translation validators, translational separation-logic verifiers, RustCompCert preprint | Verified compiler passes, sound ownership/refinement typing, formal validation of individual translations, and formal verifier-front-end connections | A proof/certificate alone is not novel. Need a precise new proof-producing ownership/translation algorithm and a useful target fragment. |
| Python compilation | Codon; Cython, mypyc, Numba, Nuitka, PyPy (review pending in detail) | Several restricted or runtime-backed routes to native/optimized execution | These do not establish universal Python ownership inference. Preserve identity, dynamic behavior, exceptions, and integer semantics or reject. |

## Decision

No candidate currently satisfies both a specific prior-art delta and a feasible nontrivial test in current Aster. Aster has no unsafe escape hatch and no pointer/heap constructs; the current scalar subset cannot test an ownership algorithm. See [`research/research_questions.md`](../../research/research_questions.md) and [`research/candidate_algorithms.md`](../../research/candidate_algorithms.md). No implementation is authorized by this review gate.
