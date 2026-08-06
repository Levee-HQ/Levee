"use client";

import { useWallet } from "./WalletProvider";

export function ConnectButton() {
  const { address, connected, connect, disconnect } = useWallet();

  if (connected && address) {
    const short = `${address.slice(0, 4)}...${address.slice(-4)}`;
    return (
      <button onClick={disconnect} className="btn-secondary text-sm px-4 py-2">
        {short}
      </button>
    );
  }

  return (
    <button onClick={connect} className="btn-primary text-sm px-4 py-2">
      Connect wallet
    </button>
  );
}
