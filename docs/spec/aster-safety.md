# Aster strict safety subset — current implementation contract

**Version:** 0.1 strict scalar subset
**Status:** Implemented structural verifier plus tests; not a machine-checked proof of the native backend.

## Accepted memory model

The current Aster AST has only `i32` and `bool` values; by-value function parameters/calls; initialized `let`/`var` bindings; scalar assignment; and scalar expressions. It has no pointer, reference, borrow, array, indexing, allocation, deallocation, struct, destructor, FFI, concurrency, or unsafe construct. Such syntax is rejected by parsing rather than silently mapped.

The strict verifier first invokes name/type/initialization checking, then emits explicit obligations for each function. Scalar reads are discharged by initialized binding construction and name resolution. Use-after-free/double-free, alias/lifetime, pointer arithmetic/bounds, and allocation/deallocation obligations are marked not applicable because the constructs do not exist in the accepted AST. An `Unknown` obligation is an error. `aster check` and native build both require this verifier to pass. The Rust migration also runs it on generated `.as` and records obligations in the sidecar report.

## Runtime behavior

No memory-safety enforcement checks are emitted in this subset: there are no memory-bearing language operations to check. There is no GC, runtime borrow checker, or reference counting. The native C-emitting backend has scalar stack locals and no user pointers or dynamic allocation. Checked arithmetic may call `abort` on overflow or division error; this is arithmetic semantics, not a memory-safety guard. Stack exhaustion and external compiler/runtime behavior are outside the current source-level property.

Bounds checks are absent because arrays/indexing are absent. Any future index operation must either have a statically discharged bounds proof or be rejected in strict mode; no runtime fallback is permitted.

## Soundness argument sketch (not a complete proof)

For well-typed accepted programs, every expression evaluates over scalar values in a finite environment. Name resolution/type checking prevents reads of names absent from that environment. Binding syntax always initializes before inserting a local; assignment is restricted to existing mutable bindings. No expression denotes an address or memory location, and calls pass only scalar values. Thus the source calculus cannot express dereference, invalid pointer arithmetic, out-of-bounds access, aliasing references, allocation, or destruction. This supports a source-level memory-safety argument for the accepted subset.

The implementation does **not** yet prove backend preservation. The C emitter, C compiler, ABI, stack behavior, and generated machine code are trusted. There is no mechanized preservation/progress proof; tests do not replace it. Therefore “statically memory-safe” here is a scope-bounded verifier result, not a complete formal proof of the compiled executable.

## Proof obligations and limits

| Obligation | Current disposition |
|---|---|
| Definite initialization / resolved reads | Checked by scalar type/name analysis |
| Use-after-free / double-free | Not applicable: no resources/free |
| Borrow validity / mutable alias exclusion | Not applicable: no references/borrows |
| Pointer arithmetic / index bounds | Not applicable; pointer/index syntax rejected |
| Allocation/deallocation pairing / object lifetime | Not applicable: no dynamic allocation or object destructors |
| C-emitter semantic preservation | Open, trusted implementation; not formally proved |
| Native ABI/runtime integrity and stack exhaustion | Outside the formalized source subset |

No claim of proof-assistant verification or complete mathematical soundness is made. The verifier is a conservative structural gate whose exact accepted language is deliberately tiny.
