# Literature review — initial evidence and research plan

**Date checked:** 2026-06-04. This is a preliminary review, not a claim of comprehensive coverage. Full-paper and implementation review remains outstanding; URLs below are primary or official sources where possible.

## Rust, MIR, and Polonius

Rust's borrow checker analyzes MIR after type checking; the rustc development guide describes the MIR borrow-checking pipeline. Polonius develops a fact/rule formulation of borrow checking, including loans, origins and region subset relations. Its repository provides implementations and test inputs. These are ownership/lifetime validity analyses, not a joint execution-cost optimizer. Any Aster optimization must refine/obey a separate safety judgment. Sources: [rustc-dev-guide: borrow check](https://rustc-dev-guide.rust-lang.org/borrow_check.html); [Polonius repository](https://github.com/rust-lang/polonius); [Polonius rules](https://rust-lang.github.io/polonius/rules.html).

## Oxide

Aaron Weiss et al., “Oxide: The Essence of Rust,” arXiv:1903.00982 (latest listed version v4, 2021). The abstract states that Oxide formalizes a source-level Rust-like language with fully annotated types, treats lifetimes as approximations of reference provenance, computes this information through a substructural typing judgment, and proves type safety with progress and preservation. This is prior art for a formal ownership/borrow core, not evidence of automated cost-based optimization. Source: [arXiv record and abstract](https://arxiv.org/abs/1903.00982). Verify exact author list and technical claims in the paper before final citation.

## Hylo and mutable value semantics

Dimi Racordon et al., “Native Implementation of Mutable Value Semantics,” arXiv:2106.12678. The paper addresses efficient native implementation of mutable value semantics, making it a close comparison for assignment/value behavior and ownership-guided copy avoidance. The arXiv source was fetched; detailed algorithm/formal obligations remain to be extracted from the paper. Source: [arXiv](https://arxiv.org/abs/2106.12678); [Hylo specification](https://github.com/hylo-lang/specification/blob/main/spec.md).

## Perceus and Koka

Alex Reinking, Ningning Xie, Leonardo de Moura, and Daan Leijen, “Perceus: Garbage Free Reference Counting with Reuse,” PLDI 2021, DOI [10.1145/3453483.3454032](https://doi.org/10.1145/3453483.3454032). Search metadata and the authors' Microsoft Research PDF describe compile-time insertion of precise reference-count operations and reuse of uniquely referenced nodes; the claimed garbage-free property is qualified for cycle-free programs. This is clear prior art for uniqueness-based reuse and copy avoidance, although the runtime model differs from Aster's intended strict static ownership core. The DOI endpoint did not fetch in this session; paper PDF: [Microsoft Research](https://www.microsoft.com/en-us/research/wp-content/uploads/2021/06/perceus-pldi21.pdf).

## Morphic

Search surfaced William Brandon, Benjamin Driscoll, Frank Dai, Jonathan Ragan-Kelley, Mae Milano, and Alex Aiken, “Fully-Automatic Type Inference for Borrows with Lifetimes,” associated with SPLASH/OOPSLA 2026. Since this review is dated 2026-06-04 and some search records may be future-dated or stale, publication status and accessible manuscript version must be verified. If valid, it is directly relevant and may subsume the proposed automatic borrow/lifetime inference goal. Sources: [project page](https://www.cs.princeton.edu/~mpmilano/project/morphic/); [conference listing](https://2026.splashcon.org/details/oopsla-2026/22/Fully-Automatic-Type-Inference-for-Borrows-with-Lifetimes).

## Other named systems (initial official-source leads)

- **Vale/Valen:** Vale project README and design pages describe single ownership and generational references; status and design have changed, and this is not to be conflated with automatic borrow inference. [Vale README](https://github.com/ValeLang/Vale/blob/master/README.md), [Vale vision](https://vale.dev/vision/vision).
- **Austral:** specification defines linearity/resource consumption rules. [Austral specification](https://austral-lang.org/spec/spec.html).
- **Swift:** language memory-safety/exclusivity documentation and compiler SIL ownership documents cover exclusivity and ARC optimization. [Memory safety](https://docs.swift.org/latest/documentation/the-swift-programming-language/memorysafety/), [SIL ownership](https://github.com/swiftlang/swift/blob/main/docs/SIL/Ownership.md).
- **Nim:** official documentation distinguishes ARC and ORC memory-management modes. [Nim memory management](https://nim-lang.org/docs/mm.html).
- **Mojo:** official ownership and lifetime documentation describes borrowing, ownership transfer, and origins. [Ownership](https://mojolang.org/docs/manual/values/ownership/), [lifetimes](https://mojolang.org/docs/manual/values/lifetimes/).
- **MLKit:** Tofte and Talpin, “Region-Based Memory Management,” *Information and Computation* 132(2), 109–176 (1997), presents region inference. [Paper](https://ropas.snu.ac.kr/lib/dock/ToTa1997.pdf).
- **LLVM:** `noalias`, `readonly`, and `nocapture` are semantic promises to the optimizer with distinct meanings; emitting unsupported attributes can invalidate optimizer correctness. [LLVM LangRef](https://llvm.org/docs/LangRef.html#parameter-attributes).
- **Copy elision:** WG21 P0135R1 changes C++ value-category/initialization rules for guaranteed elision in selected cases; it is not a general cost-based ownership algorithm. [P0135R1](https://open-std.org/jtc1/sc22/wg21/docs/papers/2016/p0135r1.html).

## Preliminary synthesis

The initial CAOI proposal combines established concerns: borrow/lifetime inference, uniqueness, dead-copy elimination, region/escape analysis, and storage reuse. This search does not establish a novel algorithm. “Jointly cost-aware” may be a useful engineering formulation, but novelty needs a concrete decision procedure, a distinction from Morphic/Perceus/Hylo/standard compiler pipelines, and controlled results. CAOI-P adds familiar PGO, specialization, vectorization and interprocedural optimization dimensions; combining them under one objective alone is not a novelty result.

## Remaining review

Read primary papers in full; confirm author/publication metadata; inspect the exact inference rules and implementations; add separation logic, uniqueness/linear type systems, alias/escape analysis, region inference refinements, copy-on-write, ARC optimization, and proof-carrying transformations; cite specific sections/pages and avoid treating marketing documentation as formal proof. No experiment or theorem is claimed here.
