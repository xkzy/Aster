# Known limitations

As of 2026-06-04:

- Aster is a bootstrap compiler subset. It has no pointer/reference/array/heap constructs or ownership/borrow checker. Its structural scalar safety verifier is not a mechanized proof of the backend.
- Native builds emit restricted C and invoke Clang; there is no direct LLVM IR backend, MIR, formal semantics, benchmark, or demonstrated performance result.
- Strict source migration is experimental and intentionally narrow: Rust accepts a tiny standalone scalar subset; C/C++ accepts constant-return `main`; Python accepts `import sys` followed by `sys.exit(<signed i32 literal>)`. Other inputs are rejected. Cargo projects, general language semantics, dependencies, and broad differential suites are not supported.
- Ghidra integration is built and tested locally against Ghidra 12.1.4. The Java extension script exports versioned JSON evidence and can show a modal Aster subset preview when launched from the GUI with `ASTER_CLI` configured. There is no docked native GUI provider; GUI usage is through the Ghidra script manager/dialog.
- Ghidra lowering accepts only a single-block, no-parameter `main` whose recovered type is signed 32-bit integer and whose high P-code is exactly a constant `COPY` to the returned register plus `RETURN`. Other functions are reported unresolved (or unsafe-boundary when memory/indirect operations are visible) and are not emitted. Aster source may therefore be a partial function recovery, never a claim of whole-program equivalence.
- Ghidra type recovery is fallible. The supported lowerer requires a recovered `int`/`int32_t` type; stripped binaries without that evidence are rejected even if the code may be simple.
- The headless smoke fixture compares one run of a known constant-return binary. There is no general behavioral-equivalence harness for arbitrary binaries, inputs, environments, nondeterminism, or side effects.
- No general source maps, assembly generation, ownership recovery, pointer provenance/aliasing/lifetime proof, heap modeling, call reconstruction, ABI adaptation, or decompiler-C-based translation exists.
- CAOI remains a research hypothesis/name. A full primary-source literature review and any novelty claim are pending.
