#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ghidra_home="${GHIDRA_INSTALL_DIR:-/opt/ghidra}"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

command -v clang >/dev/null || { echo "clang is required" >&2; exit 2; }
command -v python3 >/dev/null || { echo "python3 is required" >&2; exit 2; }
"$ghidra_home/support/analyzeHeadless" --help >/dev/null 2>&1 || true

clang -O2 -g \
    "$repo_root/ghidra-extension/tests/constant_return.c" -o "$work/original"
mkdir -p "$work/project"
"$ghidra_home/support/analyzeHeadless" "$work/project" AsterSmoke \
    -import "$work/original" \
    -scriptPath "$repo_root/ghidra-extension/ghidra_scripts" \
    -postScript AsterExport.java "$work/export.json"

cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p aster
aster="$repo_root/target/debug/aster"
if ! "$aster" ghidra lower "$work/export.json" -o "$work/recovered.as"; then
    python3 - "$work/recovered.ghidra-report.json" "$work/recovered.as" <<'PY'
import json
import os
import sys
with open(sys.argv[1], encoding="utf-8") as f:
    report = json.load(f)
assert report["complete"] is False, "partial export must not claim complete recovery"
assert os.path.isfile(sys.argv[2]), "safe subset source should still be available"
print("partial recovery correctly reported; unsupported functions were not emitted")
PY
fi
"$aster" build "$work/recovered.as" -o "$work/recovered"

python3 - "$work/original" "$work/recovered" "$work/export.json" <<'PY'
import json
import subprocess
import sys
original, recovered, export = sys.argv[1:]
with open(export, encoding="utf-8") as f:
    evidence = json.load(f)
main = [fn for fn in evidence["functions"] if fn["name"] == "main"]
assert len(main) == 1, "expected exactly one recovered main"
assert {op["opcode"] for op in main[0]["highPcode"]} == {"COPY", "RETURN"}
a = subprocess.run([original], capture_output=True)
b = subprocess.run([recovered], capture_output=True)
assert (a.returncode, a.stdout, a.stderr) == (b.returncode, b.stdout, b.stderr), (a, b)
print(f"differential smoke test passed: exit={a.returncode}, stdout/stderr identical")
PY
