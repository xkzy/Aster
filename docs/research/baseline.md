# Aster repository baseline (historical and current status)

**Original baseline date:** 2026-06-04. **Updated research status:** 2026-11-12.

The 2026-06-04 inspection found no compiler sources or tests at that time. Since then, the repository has gained a Rust workspace, scalar Aster lexer/parser/type checker, native C-emitting backend, strict safety gate, narrow C/C++/Rust/Python migration experiments, and Ghidra exporter/lowerer. The implementation and tests are documented in [`docs/PROGRESS.md`](../PROGRESS.md), [`docs/spec/aster-0.1.md`](../spec/aster-0.1.md), and [`docs/spec/aster-safety.md`](../spec/aster-safety.md).

The current Aster accepted language contains scalar `i32`/`bool` operations and has no pointers, references, arrays, allocation/deallocation, FFI, concurrency, or `unsafe` construct. It therefore cannot express a heap-ownership inference workload. The safety checker’s structural result is not a mechanized proof of backend semantic preservation. The Ghidra recovery pipeline is conservative and supports only a narrow constant-return case; it is not a general memory-safety or equivalence proof.

No CAOI/CAOI-P prototype, ownership-inference experiment, or performance benchmark exists. Current Aster behavior tests and the Ghidra smoke fixture are engineering verification, not research results. The evidence-led research status is maintained in the root [`research/`](../../research/) documents.
