#!/usr/bin/env bash
set -euo pipefail

# Generate CLI help markdown from the current build
ROOT_DIR="$(cd "$(dirname "$0")"/.. && pwd)"
OUT_FILE="$ROOT_DIR/docs/CLI_HELP.md"

mkdir -p "$ROOT_DIR/docs"

echo "# pingdown CLI Help (generated)" > "$OUT_FILE"
echo >> "$OUT_FILE"

# Run with --help and append into the markdown file wrapped in code fences
{
  echo "This file is generated from 'pingdown --help'. Run scripts/update-help.sh to refresh it."
  echo
  echo '```'
  cargo run --quiet -- --help || { echo "[warn] cargo run failed - writing fallback help"; }
  echo '```'
} >> "$OUT_FILE"

echo "CLI help written to $OUT_FILE"
