#!/usr/bin/env bash
set -euo pipefail

echo "Regenerating TypeScript bindings from built WASM..."

NETWORK="${1:-testnet}"
OUT_DIR="packages/sdk/src/clients"

stellar contract build

mkdir -p "$OUT_DIR"

CONTRACTS=(levee-registry levee-pool levee-policy levee-oracle levee-settlement)

for contract in "${CONTRACTS[@]}"; do
  WASM="target/wasm32-unknown-unknown/release/${contract//-/_}.wasm"
  if [ ! -f "$WASM" ]; then
    echo "Warning: $WASM not found, skipping $contract"
    continue
  fi

  NAME="${contract#levee-}"
  echo "Generating bindings for $contract..."
  stellar contract bindings typescript \
    --wasm "$WASM" \
    --output-dir "$OUT_DIR/$NAME" \
    --overwrite \
    2>/dev/null || echo "  Warning: binding generation failed for $contract (may need contract ID on $NETWORK)"
done

echo "Done. Bindings written to $OUT_DIR/"
