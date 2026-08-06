import { execSync } from "child_process";
import { readFileSync } from "fs";

const NETWORK = process.argv[2] || "testnet";
const deployments = JSON.parse(
  readFileSync(`deployments/${NETWORK}.json`, "utf-8")
);
const { pool, policy, oracle, settlement } = deployments.contracts;

function stellar(cmd: string): string {
  const full = `stellar ${cmd} --network ${NETWORK}`;
  console.log(`> ${full}`);
  return execSync(full, { encoding: "utf-8" }).trim();
}

console.log("=== Levee Trigger Simulation ===\n");
console.log("Network:", NETWORK);
console.log("This script demonstrates the full payout lifecycle.\n");

console.log("Step 1: Deposit capital into pool...");
stellar(
  `contract invoke --id ${pool} -- deposit ` +
    `--from ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--amount 50000000000`
);
console.log("  Deposited 5,000 USDC\n");

console.log("Step 2: Buy cover...");
const policyId = stellar(
  `contract invoke --id ${policy} -- buy ` +
    `--buyer ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--peril ORACLE1 ` +
    `--amount 10000000000 ` +
    `--term_ledgers 17280`
);
console.log(`  Policy ID: ${policyId}\n`);

console.log("Step 3: Simulate oracle deviation (3 consecutive observations above threshold)...");
for (let i = 0; i < 3; i++) {
  const deviation = 600 + i * 100;
  stellar(
    `contract invoke --id ${oracle} -- record_observation ` +
      `--caller ${process.env.DEPLOYER_PUBLIC_KEY} ` +
      `--peril ORACLE1 ` +
      `--deviation_bps ${deviation}`
  );
  console.log(`  Observation ${i + 1}: ${deviation} bps deviation`);
}

console.log("\nStep 4: Check trigger state...");
const triggered = stellar(
  `contract invoke --id ${oracle} -- is_triggered --peril ORACLE1`
);
console.log(`  Triggered: ${triggered}\n`);

if (triggered === "true") {
  console.log("Step 5: Claim payout...");
  const payout = stellar(
    `contract invoke --id ${settlement} -- claim ` +
      `--caller ${process.env.DEPLOYER_PUBLIC_KEY} ` +
      `--policy_id ${policyId}`
  );
  console.log(`  Payout: ${payout} stroops (${Number(payout) / 10_000_000} USDC)\n`);
  console.log("=== Payout executed successfully ===");
} else {
  console.log("Trigger not met. Deviation observations may need higher values or more ledgers.\n");
}
