# Proof obligations (screened candidate; none discharged)

**Status:** Candidate 1 did not pass the novelty gate. This is a checklist for what a future proof-producing C→safe-Aster migration would owe, not evidence that the work is justified. All items below are **open**; see `formal_model.md` for definitions still missing.

| Obligation | Required statement/evidence | Current status / failure mode |
|---|---|---|
| C source-domain adequacy | Define a bounded C semantics and exactly which executions are defined; account for object lifetime, pointer arithmetic/provenance, integer conversions/overflow, allocation, and effects | **Open.** A test pass does not define C behavior. UB and target ABI assumptions cannot be omitted. |
| Parser/front-end soundness | Parsed AST/type/CFG corresponds to the source program in the stated C semantics | **Open.** No such front end or theorem exists in this research prototype. |
| Ownership constraint soundness | Every satisfying ownership assignment respects unique ownership, borrow exclusivity, and lifetime/non-escape constraints | **Open.** Need explicit heap graph, path joins, loops, recursive data, and calls. |
| Bounds/init proof | Each target read/write refers to initialized, in-bounds live storage | **Open.** Unknown alias/range cases must reject, not be labeled safe. |
| Translation preservation | Generated Aster has precisely the selected trace relation to source on the source-defined domain | **Open.** Need define behavior/trace, nondeterminism, failure, and equivalence vs refinement. |
| Aster type/safety soundness | Every accepted target program obeys its memory-safety property without any `unsafe` escape hatch | **Open for any future heap extension.** Current scalar verifier cannot establish this for memory-bearing constructs that do not exist. |
| Certificate soundness | Small independent checker accepts only valid source/target derivations and cannot be fooled by omitted constraints | **Open.** No certificate format/checker exists. |
| Backend preservation | Aster-to-C/native backend preserves the target semantics and safety property | **Open.** Current backend is trusted; no mechanized end-to-end theorem. |
| ABI/library/FFI boundary | External functions preserve stated ownership/effect contracts | **Open.** Current Aster has no FFI. Future boundary should be excluded or formally specified. |
| Concurrency/data races | If concurrent programs are admitted, prove safe sharing/atomic semantics and race freedom | **Excluded.** No concurrency in current Aster or candidate subset. |
| Optimization correctness | Copy elision, stack/region placement, reuse, or alias metadata preserves observations | **Open.** Each transform needs a proof/validator; standard compiler passes are a required baseline. |
| Cost claims | Cost model corresponds to measured machine behavior and comparisons use equivalent optimized baselines | **Open.** No Aster benchmark or cost theorem exists. |

## Minimum re-open condition

Do not start the prototype until (1) the target safe memory model is separately specified and authorized, (2) the candidate has a new algorithmic delta over Scylla, Crown, &inator, CCured, and established proof/validation techniques, and (3) the source semantics and theorem are fixed tightly enough that a proof and falsification experiment can be designed.
