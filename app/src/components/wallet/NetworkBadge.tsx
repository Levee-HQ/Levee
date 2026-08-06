"use client";

import { useWallet } from "./WalletProvider";

export function NetworkBadge() {
  const { network, connected } = useWallet();
  if (!connected) return null;

  const isTestnet = network === "TESTNET";

  return (
    <span
      className={`text-xs font-semibold px-2 py-1 rounded-full ${
        isTestnet
          ? "bg-levee-surface/20 text-levee-light"
          : "bg-levee-trigger/20 text-levee-trigger"
      }`}
    >
      {isTestnet ? "Testnet" : network}
    </span>
  );
}
