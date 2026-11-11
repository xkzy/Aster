# Strict migration policy and current frontend status

A migration is reported successful only after generated `.as` passes the Aster static safety gate, native compilation, and the frontend's supported behavioral checks. No unsafe compatibility mode, garbage collector, runtime borrow checker, hidden reference counting, or runtime memory-safety check is allowed. Unknown/unimplemented constructs fail closed.

| Input | Current behavior | Success status |
|---|---|---|
| Native Aster `.as` | Scalar-only parser/typechecker/verifier; C-emitting native bootstrap backend | Supported subset only; backend proof remains open |
| Rust `.rs` | Pinned rustc validation plus narrow `syn` source translation; Aster verify/build and one original-vs-output execution comparison | Tiny scalar subset only; details in `rust-subset.md` |
| C `.c` | Explicit strict-mode refusal: no verified frontend | Not supported |
| C++ `.cpp`, `.cc`, `.cxx` | Explicit strict-mode refusal: no verified frontend | Not supported |
| Python `.py` | Explicit strict-mode refusal: no verified frontend | Not supported |
| Cargo project | Explicit refusal; no Cargo workspace/dependency/build-script/proc-macro/test analysis | Not supported |

The CLI refusal for C, C++, and Python creates no output and identifies the missing frontend. Rust migration similarly refuses unsupported syntax and Cargo ancestry. These gaps are not compatibility fallbacks.

The current verifier's proof obligations are narrow because accepted source has no pointers, references, arrays, dynamic allocations, or destructors. See `../spec/aster-safety.md` for obligations and the unproved backend assumptions. No universal language soundness or migration equivalence theorem is claimed.
