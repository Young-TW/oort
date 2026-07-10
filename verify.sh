#!/usr/bin/env bash
# Verify Oort solutions locally via oort3's simulator.
#
# Usage:
#   ./verify.sh              # test all src/N.rs
#   ./verify.sh 1 3 5        # test specific levels
#   ./verify.sh --verbose 1  # pass flags through
#   ./verify.sh all --verbose --rounds 50
set -uo pipefail

OORT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OORT3_DIR="${OORT3_DIR:-/Users/young/code/Young-TW/oort3}"
VERIFY_BIN="$OORT3_DIR/target/release/oort-verify"

# Ensure rustup (with wasm32 target) is on PATH; compiler spawns rustc via rustup.
if [[ -x /opt/homebrew/opt/rustup/bin/rustup ]]; then
  export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
fi

if [[ ! -x "$VERIFY_BIN" ]]; then
  echo "Error: oort-verify not found at $VERIFY_BIN" >&2
  echo "Build it first: (cd $OORT3_DIR && cargo build --release --bin oort-verify)" >&2
  exit 1
fi

# Separate flags (start with -) from level args
flags=()
levels=()
for arg in "$@"; do
  if [[ "$arg" == -* ]]; then
    flags+=("$arg")
  else
    levels+=("$arg")
  fi
done

# Default: all src/N.rs
if [[ ${#levels[@]} -eq 0 ]] || [[ " ${levels[*]} " == *" all "* ]]; then
  levels=()
  while IFS= read -r f; do
    [[ "$f" == */main.rs ]] && continue
    levels+=("$(basename "$f" .rs)")
  done < <(ls "$OORT_DIR"/src/[0-9]*.rs 2>/dev/null | sort -V)
fi

if [[ ${#levels[@]} -eq 0 ]]; then
  echo "No solution files found in $OORT_DIR/src/" >&2
  exit 1
fi

echo "Verifying ${#levels[@]} solution(s) in $OORT3_DIR"
echo "─────────────────────────────────────────────"

pass=0
fail=0
failed_levels=()
for lvl in "${levels[@]}"; do
  file="$OORT_DIR/src/$lvl.rs"
  if [[ ! -f "$file" ]]; then
    echo "[$lvl] SKIP: $file not found"
    fail=$((fail+1))
    failed_levels+=("$lvl")
    continue
  fi
  echo "[$lvl] $file"
  if "$VERIFY_BIN" "$file" "${flags[@]}" 2>&1; then
    pass=$((pass+1))
  else
    fail=$((fail+1))
    failed_levels+=("$lvl")
  fi
  echo
done

echo "─────────────────────────────────────────────"
echo "Result: $pass passed, $fail failed"
if [[ $fail -gt 0 ]]; then
  echo "Failed: ${failed_levels[*]}"
  exit 1
fi
