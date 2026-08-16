#!/usr/bin/env bash
set -euo pipefail

NETWORK="${1:-testnet}"
echo "Deploying Levee contracts to $NETWORK..."

if [ ! -f .env ]; then
  echo "Error: .env file not found. Copy .env.example and fill in values."
  exit 1
fi

source .env

echo "Building WASM artifacts..."
cargo build --workspace --target wasm32-unknown-unknown --release

CONTRACTS=(levee-registry levee-pool levee-policy levee-oracle levee-settlement)
declare -A CONTRACT_IDS

for contract in "${CONTRACTS[@]}"; do
  WASM="target/wasm32-unknown-unknown/release/${contract//-/_}.wasm"
  if [ ! -f "$WASM" ]; then
    echo "Error: WASM not found at $WASM"
    exit 1
  fi

  echo "Deploying $contract..."
  CONTRACT_ID=$(stellar contract deploy \
    --wasm "$WASM" \
    --source "$DEPLOYER_SECRET_KEY" \
    --network "$NETWORK" \
    2>&1)
  CONTRACT_IDS[$contract]="$CONTRACT_ID"
  echo "  $contract: $CONTRACT_ID"
done

cat > "deployments/${NETWORK}.json" <<EOF
{
  "network": "$NETWORK",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contracts": {
    "registry": "${CONTRACT_IDS[levee-registry]}",
    "pool": "${CONTRACT_IDS[levee-pool]}",
    "policy": "${CONTRACT_IDS[levee-policy]}",
    "oracle": "${CONTRACT_IDS[levee-oracle]}",
    "settlement": "${CONTRACT_IDS[levee-settlement]}"
  }
}
EOF

echo ""
echo "Deployment complete. Contract IDs written to deployments/${NETWORK}.json"
