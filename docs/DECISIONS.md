# Decision records

## 0001 — Start from a dependency-free Rust workspace

**Date:** 2026-06-04
**Status:** Accepted

The repository had no source implementation or package metadata. Initialize a single Rust compiler binary crate with no dependencies. This minimizes bootstrap complexity and avoids locking in LLVM bindings or solver dependencies before the subset and backend design are established. Unsafe Rust is forbidden workspace-wide unless an explicit later decision justifies and audits an isolated boundary.

## 0002 — Do not claim CAOI novelty from its name or objective

**Date:** 2026-06-04
**Status:** Accepted

Cost models, borrow/region inference, uniqueness analysis, storage reuse, and optimization are established separately and in combinations. Novelty requires a specific algorithmic distinction against directly relevant prior art, plus evidence; until then CAOI is a research hypothesis/name only.

## 0003 — Establish a narrow native vertical slice before broad syntax

**Date:** 2026-06-04
**Status:** Accepted

Prefer implementing and testing a small source-to-native subset end-to-end over building broad parsers or documentation that overstates support. Unsupported syntax must be rejected explicitly. Checked arithmetic and memory safety take priority over optimizations.

## 0004 — Use restricted C emission as a bootstrap native backend

**Date:** 2026-06-04
**Status:** Accepted, temporary

The first native slice emits C only from the validated Aster AST and invokes Clang. Checked arithmetic uses compiler overflow builtins and explicit division checks. This provides an end-to-end native test path without a dependency, but does not meet the requested direct LLVM lowering architecture. C emission is a bootstrap implementation only; it must not be described as direct LLVM IR generation or as proof of safe lowering. Backend correctness remains an open obligation.

## 0005 — Treat Ghidra output as evidence and lower fail-closed

**Date:** 2026-06-04
**Status:** Accepted

The binary recovery pipeline uses Ghidra P-code/CFG and recovered metadata as evidence; decompiler C is supplementary. Unknown operations, uncertain types, and unsupported functions remain unresolved or unsafe-boundary and are not converted to stubs. The initial lowerer emits only a narrowly proven constant-return `main`, writes an explicit completeness report, and returns nonzero for partial recovery. A compiled output is not a claim of whole-program equivalence.
