#!/bin/bash
set -e

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== Building WASM ==="
cargo build -p prophet-wasm --target wasm32-unknown-unknown --release
cargo build -p tr808-wasm --target wasm32-unknown-unknown --release
cp "$ROOT/target/wasm32-unknown-unknown/release/prophet_wasm.wasm" "$ROOT/web/public/prophet-dsp.wasm"
cp "$ROOT/target/wasm32-unknown-unknown/release/tr808_wasm.wasm" "$ROOT/orbit/public/tr808.wasm"

echo "=== Building ORBIT (root /) ==="
cd "$ROOT/orbit"
npx vite build

echo "=== Building Prophet (/prophet/) ==="
cd "$ROOT/web"
npx vite build

echo "=== Merging into dist/ ==="
rm -rf "$ROOT/dist"
# ORBIT at root
cp -r "$ROOT/orbit/dist" "$ROOT/dist"
# Prophet at /prophet/
mkdir -p "$ROOT/dist/prophet"
cp -r "$ROOT/web/dist/"* "$ROOT/dist/prophet/"

echo "=== Done ==="
echo "Output: $ROOT/dist/"
echo "  /           → ORBIT drum machine"
echo "  /prophet/   → Prophet-5 synth"
ls -la "$ROOT/dist/"
