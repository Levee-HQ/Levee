# Deployment Readiness Checklist

**Last Updated:** August 6, 2026

## Pre-Deployment

### Environment Setup
- [ ] Install Rust 1.81 (or follow WASM_BUILD_NOTE.md workaround)
  ```bash
  rustup install 1.81
  rustup default 1.81
  ```
- [ ] Install Node 18+ (or use fnm)
- [ ] Install pnpm 9.15+
- [ ] Install Stellar CLI >= 21.0
- [ ] Install soroban CLI >= 21.0

### Network Configuration
- [ ] Choose network: `testnet` or `public`
- [ ] Set `STELLAR_RPC_URL` environment variable
  - Testnet: `https://soroban-testnet.stellar.org`
  - Public: `https://soroban-mainnet.stellar.org`
- [ ] Set `STELLAR_NETWORK` environment variable

### Keypair Setup
- [ ] Create or export admin keypair
  ```bash
  soroban keys generate admin
  export ADMIN_SECRET_KEY=$(soroban keys show admin | jq -r .secretKey)
  ```
- [ ] Create or export underwriter keypair
  ```bash
  soroban keys generate underwriter
  export UNDERWRITER_SECRET_KEY=$(soroban keys show underwriter | jq -r .secretKey)
  ```
- [ ] Fund keypairs with XLM (at least 10 XLM per key for test transactions)

### Code Validation
- [ ] All contract tests pass: `cargo test --workspace`
- [ ] SDK tests pass: `pnpm --filter @levee/sdk test`
- [ ] No clippy warnings: `cargo clippy --workspace -- -D warnings`
- [ ] Frontend builds: `pnpm -r build`

## Deployment Steps

### Step 1: Build WASM Binaries
```bash
# If Rust 1.81 works directly:
cargo build --workspace --target wasm32-unknown-unknown --release

# If you hit edition2024 issues, see WASM_BUILD_NOTE.md for alternatives
```

**Expected outputs:**
- target/wasm32-unknown-unknown/release/levee_registry.wasm
- target/wasm32-unknown-unknown/release/levee_pool.wasm
- target/wasm32-unknown-unknown/release/levee_policy.wasm
- target/wasm32-unknown-unknown/release/levee_oracle.wasm
- target/wasm32-unknown-unknown/release/levee_settlement.wasm

### Step 2: Deploy Contracts
```bash
export STELLAR_NETWORK=testnet
export STELLAR_RPC_URL=https://soroban-testnet.stellar.org

pnpm exec tsx scripts/deploy.sh
```

**Expected output:** `deployments/testnet.json` with contract IDs
```json
{
  "registry": "CXXX...",
  "pool": "CXXX...",
  "policy": "CXXX...",
  "oracle": "CXXX...",
  "settlement": "CXXX...",
  "asset": "native" // or issued asset contract address
}
```

### Step 3: Initialize System
```bash
pnpm exec tsx scripts/seed.ts
```

**Expected actions:**
- Registry: Register Oracle Deviation peril
  - deviation_threshold_bps: 500 (5%)
  - sustain_window_ledgers: 3
  - max_coverage_ratio_bps: 8000 (80%)
  - base_premium_rate_bps: 200 (2% per day)
- Pool: Deposit initial capital (underwriter's contribution)
- Policy & Settlement: Set authorized callers on pool

### Step 4: Validate Deployment
```bash
pnpm exec tsx scripts/simulate-trigger.ts
```

**Expected flow:**
1. Underwriter deposits capital into pool → receives shares
2. User buys $1,000 cover for 7 days → pool locks capacity, receives premium
3. Oracle records 3 observations above threshold → trigger fires
4. Settlement claims coverage → pool pays $1,000 (or pro-rata if insufficient)
5. Policy marked settled

### Step 5: Configure Frontend
1. Copy contract addresses from `deployments/testnet.json`
2. Edit `app/src/lib/constants.ts`:
   ```typescript
   export const TESTNET_CONTRACTS = {
     registry: "CXXX...",
     pool: "CXXX...",
     policy: "CXXX...",
     oracle: "CXXX...",
     settlement: "CXXX...",
     asset: "native", // or CUSD address
   };
   ```
3. Verify RPC endpoint is correct
4. Ensure Freighter can connect to testnet

### Step 6: Build Frontend for Deployment
```bash
pnpm -r build
```

### Step 7: Deploy Frontend
Choose one:
- **Vercel (recommended):**
  ```bash
  vercel deploy --prod
  ```
- **Self-hosted (Next.js server):**
  ```bash
  pnpm -r start
  # Runs on http://localhost:3000
  ```
- **Static export (no API routes):**
  ```bash
  # Add `output: "export"` to app/next.config.mjs
  pnpm run build
  # Deploy public/ to static hosting
  ```

## Post-Deployment

### Verification
- [ ] Navigate to frontend URL
- [ ] Connect Freighter wallet
- [ ] Verify network shows testnet
- [ ] View pool utilization (should show initial capacity)
- [ ] Click "Buy Cover" → quote displays correctly
- [ ] Calculate premium matches docs/pricing.md examples

### Monitoring
- [ ] Set up transaction monitoring on Stellar testnet
- [ ] Log contract invocations for debugging
- [ ] Track pool utilization over time
- [ ] Monitor oracle observations in trigger window

### Incident Response
- [ ] If deployment fails, check STELLAR_RPC_URL
- [ ] If WASM build fails, review WASM_BUILD_NOTE.md
- [ ] If pool is undercapitalized, add funds via deposit
- [ ] If oracle is stuck, re-run `scripts/seed.ts` to reset observations

## Rollback Plan

If deployment fails or needs immediate rollback:

1. **Stop using new contracts** — update frontend to point to previous version or disable in UI
2. **Drain pool** if funds are at risk:
   ```bash
   # Withdraw all shares as underwriter
   soroban contract invoke --id <POOL_ID> --function withdraw \
     --arg-from-file <ADMIN_KEYPAIR> --arg <SHARES>
   ```
3. **Deploy previous version** from git tag (if applicable)

## Success Criteria

✅ **Contracts deployed** to testnet with unique addresses  
✅ **Pool initialized** with underwriter capital  
✅ **Peril registered** with correct trigger parameters  
✅ **Simulation runs** without errors  
✅ **Frontend loads** and connects to deployed contracts  
✅ **User can quote** and premiums match documentation  
✅ **Wallet integration** works (Freighter connects, shows balance)

## Troubleshooting

| Error | Cause | Solution |
|-------|-------|----------|
| `soroban-sdk` build fails with `Rust 1.82+` | Version mismatch | See WASM_BUILD_NOTE.md |
| `Failed to connect to RPC` | Bad `STELLAR_RPC_URL` | Verify endpoint is reachable |
| `Insufficient funds` | Keypair not funded | Add 10+ XLM via Stellar faucet |
| `Contract not found` | Deploy didn't complete | Re-run `scripts/deploy.sh` |
| `Frontend shows error` | Contract address mismatch | Verify `app/src/lib/constants.ts` |
| `Oracle trigger never fires` | Threshold too high | Lower `deviation_threshold_bps` in seed script |

## Support & Questions

- **Documentation:** See docs/ folder and CONTRIBUTING.md
- **Issues:** Open an issue on GitHub with error logs
- **Security:** See SECURITY.md for disclosure policy

---

**Ready to deploy?** Follow the steps above in order. Estimated time: 30-60 minutes.
