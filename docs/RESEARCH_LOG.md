# Research log

## 2026-06-04 — Initial repository and prior-art search

- Inventory: repository was effectively empty; see `docs/research/baseline.md`.
- Environment: Rust/Cargo 1.99.1, Clang 22.1.8, GCC 16.2.1, LLVM 22.1.8.
- Web search located primary/official references for Oxide, Perceus, Hylo mutable value semantics, Polonius, Mojo ownership, LLVM parameter attributes, Swift SIL ownership, Nim ARC/ORC, Austral linearity, and MLKit regions. The Morphic search result describes a publication titled “Fully-Automatic Type Inference for Borrows with Lifetimes” associated with OOPSLA 2026; because the date is 2026-06-04 and the result's metadata may postdate this date, its publication status must be verified before citing it as published prior art.
- Initial novelty caution: the proposed ingredients overlap known work in automatic borrow/lifetime inference, uniqueness/reuse, region analysis, and alias/effect analysis. “Cost-aware joint inference” is not itself evidence of novelty. No CAOI novelty claim is made.
- No algorithm choice, benchmarks, or proof results yet.
- Implemented/tested only a small lexer slice after adding failing API tests; later added parser, scalar type checker, restricted C-emitting native build, and experimental Rust migration. This remains a prototype.

## 2026-06-04 — Rust migration frontend investigation

- Installed compiler toolchain inventory shows stable rustc 1.99.1 and exact rustup toolchain 1.99.1 available; prototype pins `rustc 1.99.1` and `syn = 2.0.119`.
- Experiment: `rustc --emit=mir` succeeds and emits typed MIR, while its output explicitly warns that the text format is human-facing and may change. The migrator therefore invokes pinned rustc for semantic validation and emits MIR, but does not parse the unstable text or extract ownership facts.
- Rust Compiler Development Guide says external `rustc_driver`/`rustc_interface` use compiler internals. The prototype avoids those APIs; no claim that rustc internal APIs are stable.
- `syn` parses source ASTs but is not a type checker; pair with pinned rustc validation.
- Existing C2Rust is C-to-Rust, not reverse direction. It does not address Rust-to-Aster guarantees.
- Implemented tiny Rust subset with successful generated Aster native compile plus single-run exit/stdout/stderr comparison. Tests reject unsupported constructs and Cargo ancestry. No full Rust or Cargo migration claims.


## Sources identified for direct inspection

- Oxide: Aaron Weiss, Daniel Patterson, and Amal Ahmed (paper author metadata should be checked against the paper), “Oxide: The Essence of Rust,” https://arxiv.org/abs/1903.00982
- Perceus: Alex Reinking, Ningning Xie, Leonardo de Moura, Daan Leijen, “Perceus: Garbage Free Reference Counting with Reuse,” PLDI 2021, https://doi.org/10.1145/3453483.3454032
- Hylo: Dimi Racordon et al., “Native Implementation of Mutable Value Semantics,” https://arxiv.org/abs/2106.12678
- Rust MIR borrow checking: https://rustc-dev-guide.rust-lang.org/borrow_check.html
- Polonius repository/rules: https://github.com/rust-lang/polonius and https://rust-lang.github.io/polonius/rules.html
- Mojo ownership/lifetimes: https://mojolang.org/docs/manual/values/ownership/ and https://mojolang.org/docs/manual/values/lifetimes/
- LLVM LangRef: https://llvm.org/docs/LangRef.html
- Swift SIL ownership: https://github.com/swiftlang/swift/blob/main/docs/SIL/Ownership.md
- Nim memory management: https://nim-lang.org/docs/mm.html
- Austral specification: https://austral-lang.org/spec/spec.html
- MLKit region inference: Mads Tofte and Jean-Pierre Talpin, “Region-Based Memory Management,” Information and Computation 132(2), 109–176 (1997), https://ropas.snu.ac.kr/lib/dock/ToTa1997.pdf
- C++ guaranteed copy elision proposal (related, not equivalent to general copy optimization): https://open-std.org/jtc1/sc22/wg21/docs/papers/2016/p0135r1.html
