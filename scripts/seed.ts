import { execSync } from "child_process";
import { readFileSync } from "fs";

const NETWORK = process.argv[2] || "testnet";
const deployments = JSON.parse(
  readFileSync(`deployments/${NETWORK}.json`, "utf-8")
);
const { registry, pool, policy, oracle, settlement } = deployments.contracts;

function stellar(cmd: string): string {
  const full = `stellar ${cmd} --network ${NETWORK}`;
  console.log(`> ${full}`);
  return execSync(full, { encoding: "utf-8" }).trim();
}

console.log("Seeding Levee on", NETWORK);
console.log("Contracts:", deployments.contracts);

console.log("\n1. Initializing registry...");
stellar(
  `contract invoke --id ${registry} -- init --admin ${process.env.DEPLOYER_PUBLIC_KEY}`
);

console.log("\n2. Registering oracle deviation peril...");
stellar(
  `contract invoke --id ${registry} -- register_peril ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--id ORACLE1 ` +
    `--config '{"kind":"OracleDeviation","target_protocol":"${pool}","oracle_source":"${oracle}","deviation_threshold_bps":500,"sustain_window_ledgers":3,"max_coverage_ratio_bps":8000,"base_premium_rate_bps":200,"active":true}'`
);

console.log("\n3. Initializing pool...");
stellar(
  `contract invoke --id ${pool} -- init ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--asset CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC ` +
    `--peril ORACLE1`
);

console.log("\n4. Initializing policy...");
stellar(
  `contract invoke --id ${policy} -- init ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--registry ${registry} ` +
    `--pool ${pool} ` +
    `--asset CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`
);

console.log("\n5. Initializing oracle...");
stellar(
  `contract invoke --id ${oracle} -- init ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--registry ${registry}`
);

console.log("\n6. Initializing settlement...");
stellar(
  `contract invoke --id ${settlement} -- init ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--policy_contract ${policy} ` +
    `--oracle_contract ${oracle} ` +
    `--pool_contract ${pool} ` +
    `--asset CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`
);

console.log("\n7. Setting authorized callers on pool...");
stellar(
  `contract invoke --id ${pool} -- set_authorized_caller ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--caller ${policy}`
);
stellar(
  `contract invoke --id ${pool} -- set_authorized_caller2 ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--caller ${settlement}`
);

console.log("\n8. Setting authorized settlement on policy...");
stellar(
  `contract invoke --id ${policy} -- set_authorized_settlement ` +
    `--admin ${process.env.DEPLOYER_PUBLIC_KEY} ` +
    `--settlement ${settlement}`
);

console.log("\nSeed complete.");
