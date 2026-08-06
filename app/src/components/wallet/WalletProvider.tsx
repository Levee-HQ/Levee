"use client";

import { createContext, useContext, useState, useCallback, useEffect, type ReactNode } from "react";
import { WalletModal } from "./WalletModal";
import { TEST_WALLETS } from "@/lib/wallet";

interface WalletState {
  address: string | null;
  connected: boolean;
  network: string;
  connect: () => void;
  disconnect: () => void;
  showModal: boolean;
}

const WalletContext = createContext<WalletState>({
  address: null,
  connected: false,
  network: "TESTNET",
  connect: () => {},
  disconnect: () => {},
  showModal: false,
});

export function useWallet() {
  return useContext(WalletContext);
}

export function WalletProvider({ children }: { children: ReactNode }) {
  const [address, setAddress] = useState<string | null>(null);
  const [connected, setConnected] = useState(false);
  const [network, setNetwork] = useState("TESTNET");
  const [showModal, setShowModal] = useState(false);

  const connectFreighter = useCallback(async () => {
    try {
      if (typeof window !== "undefined" && (window as any).freighterApi) {
        const freighter = (window as any).freighterApi;
        const addr = await freighter.getAddress();
        if (addr.address) {
          setAddress(addr.address);
          setConnected(true);
          const net = await freighter.getNetworkDetails();
          setNetwork(net.network || "TESTNET");
          return;
        }
      }
      throw new Error("Freighter wallet not found. Please install the extension.");
    } catch (err) {
      throw err;
    }
  }, []);

  const connectTestWallet = useCallback(
    (walletKey: keyof typeof TEST_WALLETS) => {
      const wallet = TEST_WALLETS[walletKey];
      setAddress(wallet.publicKey);
      setConnected(true);
      setNetwork("TESTNET");
    },
    []
  );

  const connect = useCallback(() => {
    setShowModal(true);
  }, []);

  const disconnect = useCallback(() => {
    setAddress(null);
    setConnected(false);
  }, []);

  useEffect(() => {
    const checkConnection = async () => {
      try {
        if (typeof window !== "undefined" && (window as any).freighterApi) {
          const freighter = (window as any).freighterApi;
          const isConnected = await freighter.isConnected();
          if (isConnected) {
            await connectFreighter();
          }
        }
      } catch {}
    };
    checkConnection();
  }, [connectFreighter]);

  return (
    <>
      <WalletModal
        isOpen={showModal}
        onClose={() => setShowModal(false)}
        onSelectFreighter={connectFreighter}
        onSelectTestWallet={connectTestWallet}
      />
      <WalletContext.Provider value={{ address, connected, network, connect, disconnect, showModal }}>
        {children}
      </WalletContext.Provider>
    </>
  );
}
