# Prior-art matrix (initial, source leads)

Status labels: **reviewed abstract/docs** is not a substitute for a full paper/implementation review. Entries marked **pending** need primary-source study before algorithm design or novelty claims.

| Work/system | Memory model / mechanism | Analysis or implementation | Guarantees / trade-offs relevant to Aster | Aster implication |
|---|---|---|---|---|
| Rust MIR borrow checker | Ownership, loans, regions over MIR places and CFG | MIR dataflow; Polonius explores Datalog/Datafrog loan relations | Static borrow checking; source-level borrow inference is not a cost optimizer | Baseline for checking; inspect exact current implementation |
| Oxide (Weiss, Patterson, Ahmed et al.; verify author list from PDF) | Formal source-level Rust subset; lifetimes approximate reference provenance | Substructural typing judgment; progress/preservation proof | Formalizes borrow safety, not an optimal cost decision procedure | Safety layer must remain independent from optional optimization |
| Polonius | Loans, origins, subset relations, CFG points | Datalog/Datafrog formulation and subsequent implementation variants | More location-sensitive relationships; practical scalability/status varies | Related dataflow foundation, not CAOI novelty by itself |
| Hylo / mutable value semantics | Value-semantic mutation with exclusive access | Native implementation uses ownership/exclusivity to avoid copies | Closely overlaps value semantics plus storage/copy optimization | Directly relevant; investigate uniqueness and guaranteed in-place mutation |
| Perceus / Koka | Reference-counted heap values, uniqueness at runtime | Compile-time insertion of precise RC; reuse when unique | Garbage-free for cycle-free data under its model; runtime RC remains central | Reuse analysis and copy avoidance predate CAOI proposal |
| Morphic | Rust-like borrow semantics with inferred borrow/lifetime types (publication status requires date check) | Fully automatic type inference for borrows with lifetimes | Directly challenges automatic borrow/lifetime inference novelty | Check formal restrictions, algorithm and source before differentiating |
| Vale / Valen | Vale single ownership/generational references; designs evolve | Vale's design differs from conventional static borrow checking | References can incur generation checks; implementation and status need checking | Do not describe it as established automatic lifetime inference |
| Austral | Linear types; resources consumed exactly once | Static linearity checking | Strong resource discipline, not general inferred borrowing | Prior art for unique resource transfer |
| Swift | Value/reference types, ARC, exclusivity; SIL ownership | ARC insertion/optimization, exclusivity enforcement | ARC runtime costs can be optimized; value semantics need copy-on-write in some APIs | Inspect SIL ownership and ARC optimizer |
| Nim ARC/ORC | Deterministic reference counting, ORC adds cycle handling | Compiler-inserted memory management | Runtime management modes and semantics differ from strict zero-overhead core | Relevant trade-off, not equivalent guarantee |
| Mojo | Ownership, immutable/mutable borrows, origins/lifetimes | Compiler checks transfers and reference origins | Automatic in common cases, annotations needed in harder origin cases | Contemporary ergonomics comparator; verify versioned docs |
| MLKit | Region-allocated values and region lifetimes | Type/effect-based region inference | No tracing GC needed in region model; regions can retain excess memory | Region inference and lifetime allocation are known |
| LLVM | IR memory model / attributes | Optimizer relies on caller/callee promises (`noalias`, `readonly`, `nocapture`) | Incorrect attributes can make IR semantics invalid; they are not checks | Lower attributes only from proven facts, never inferred optimistically |
| C/C++ copy elision | Object construction/value categories | Guaranteed direct construction in selected cases; optional NRVO | Narrow language-level object-creation rules, not arbitrary ownership optimization | Not evidence for general CAOI novelty |

## Next literature tasks

Read complete primary sources, record exact publication details and formal assumptions, inspect relevant implementations (rustc/Polonius, Koka, Swift SIL, Morphic/Hylo/Vale), and update this matrix with claims and limitations supported by page/section citations. Also review escape/alias analysis, effect systems, separation logic, abstract interpretation, proof-carrying optimization, and vectorization literature.
