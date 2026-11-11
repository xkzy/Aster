# Ghidra-to-Aster recovery (experimental)

This pipeline treats Ghidra output as evidence, not source truth. High P-code and control-flow metadata drive the supported lowering; decompiler C is retained only for human inspection.

## Build the Ghidra extension

Build against the same Ghidra installation used at runtime (tested locally with 12.1.4):

```sh
GHIDRA_INSTALL_DIR=/opt/ghidra \
  /opt/ghidra/support/gradle/gradlew -p ghidra-extension clean buildExtension
```

The distributable zip is written under `ghidra-extension/dist/`. Install it through Ghidra's **File → Install Extensions** flow and restart Ghidra. The extension registers `AsterExport.java` in the Ghidra script manager. Running the script from the GUI prompts for an output JSON file; if a selection is active, only functions intersecting that selection are exported. To show an Aster preview dialog, build Aster (`cargo build -p aster`) and start Ghidra with `ASTER_CLI="$PWD/target/debug/aster"` in its environment. The script invokes only that Aster lowerer (not the input binary), displays the safe supported subset and report location, and labels partial/rejected output. Without `ASTER_CLI`, it exports JSON and prints setup guidance. The script can also be run directly with `-scriptPath` in headless mode.

## Headless export and lowering

```sh
GHIDRA_INSTALL_DIR=/opt/ghidra
"$GHIDRA_INSTALL_DIR/support/analyzeHeadless" /tmp/ghidra-project AsterProject \
  -import ./input-binary \
  -scriptPath "$PWD/ghidra-extension/ghidra_scripts" \
  -postScript AsterExport.java /tmp/input.ghidra.json

cargo run -p aster -- ghidra lower /tmp/input.ghidra.json -o recovered.as
```

`aster ghidra lower` writes `recovered.as` only when it can lower a supported `main`. It validates the generated Aster with the existing static safety gate and native backend, then writes `recovered.ghidra-report.json`. A nonzero exit means recovery is incomplete or rejected. A source file emitted alongside an incomplete report is a supported subset, not a complete binary replacement; unsupported functions are never stubbed or silently discarded.

To run the locally verified end-to-end fixture (requires Clang, Python 3, Gradle wrapper dependencies, and Ghidra):

```sh
GHIDRA_INSTALL_DIR=/opt/ghidra ghidra-extension/tests/run_headless_smoke.sh
```

The fixture checks the exported P-code, builds generated Aster, and compares original/generated exit status, stdout, and stderr for one execution. It deliberately uses debug information so Ghidra recovers the return type. Standard runtime functions remain unresolved, so the report must say recovery is partial.

## Export format (schema version 1)

The JSON includes:

- Ghidra version, export scope/`scopeComplete`, analyzed function count, and program language/compiler/endian/pointer-size metadata. Selection- or function-filtered exports are always marked incomplete, and the lowerer will not claim full coverage from them.
- Functions: entry/body size, calling convention, parameters, locals, return type, external/thunk/no-return flags, decompiler status and text.
- High P-code: operation mnemonic/id, sequence address/order, output and input varnodes with byte widths, address space, offset, register/constant flags, and recovered high-variable type/name where available.
- Basic-block indexes, address ranges, successor edges, and terminal operation/inputs.
- Instruction addresses and raw instruction P-code with structural operands.
- Program data types (including structure/union fields and array element metadata) and references.

The Rust importer parses this into typed IR and rejects unknown schema versions. Addresses, widths, and data-space information are preserved in the IR/report. Recovered types are explicitly treated as uncertain; the first lowerer requires a signed 32-bit integer return type.

## Current lowering boundary

Only a single-block, no-parameter function named `main` is accepted, and only when its high P-code contains exactly one 32-bit constant `COPY` to the register returned by one `RETURN`. The function must have a recovered `int`/`int32_t` type, and no thunk/no-return condition. It lowers to a scalar Aster `main` and is compiled through the existing Aster backend.

Memory operations (`LOAD`, `STORE`, pointer arithmetic, indirect call) are classified as unsafe-boundary when visible. Branches, calls, unsupported P-code, missing debug/type evidence, multiple blocks, extra functions, external functions, and all other cases are unresolved. No unsafe/placeholder Aster is emitted. Compilation is not a proof of equivalence or memory safety for the original binary.

This is an experimental first vertical slice, not a general decompiler, docked native GUI provider, ABI recovery system, ownership analyzer, or whole-program equivalence checker. Arbitrary input binaries are not automatically executed by the lowerer.
