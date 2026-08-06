"use client";

import { useState } from "react";
import { UtilizationBar } from "@/components/pool/UtilizationBar";
import { DepositForm } from "@/components/pool/DepositForm";
import { PremiumYield } from "@/components/pool/PremiumYield";
import { WaterlineGauge } from "@/components/gauge/WaterlineGauge";
import { TxStatus } from "@/components/wallet/TxStatus";
import { useWallet } from "@/components/wallet/WalletProvider";

export default function UnderwritePage() {
  const { connected } = useWallet();
  const [txStatus, setTxStatus] = useState<"idle" | "signing" | "submitting" | "success" | "error">("idle");
  const [txMessage, setTxMessage] = useState("");

  const poolData = {
    totalAssets: 5000,
    lockedCapacity: 2000,
    totalShares: 5000,
    premiumYield: 4.2,
  };

  const handleDeposit = async (amount: number) => {
    if (!connected) return;
    setTxStatus("signing");
    try {
      setTxStatus("submitting");
      await new Promise((r) => setTimeout(r, 2000));
      setTxStatus("success");
      setTxMessage(`Deposited ${amount} USDC`);
    } catch {
      setTxStatus("error");
      setTxMessage("Deposit failed. Check your wallet and try again.");
    }
  };

  const handleWithdraw = async (shares: number) => {
    if (!connected) return;
    setTxStatus("signing");
    try {
      setTxStatus("submitting");
      await new Promise((r) => setTimeout(r, 2000));
      setTxStatus("success");
      setTxMessage(`Withdrawal complete`);
    } catch {
      setTxStatus("error");
      setTxMessage("Withdrawal failed. Check your wallet and try again.");
    }
  };

  return (
    <div className="space-y-8">
      <div>
        <h1 className="font-display text-3xl text-levee-mark">Underwrite</h1>
        <p className="text-levee-light/60 mt-2">
          Supply capital to the oracle deviation pool. Earn premiums from cover
          buyers.
        </p>
      </div>

      <div className="grid md:grid-cols-3 gap-6">
        <div className="card flex justify-center">
          <WaterlineGauge
            totalCapital={poolData.totalAssets}
            lockedCapital={poolData.lockedCapacity}
            triggerProximity={0.15}
            height={200}
          />
        </div>

        <div className="space-y-4">
          <UtilizationBar
            total={poolData.totalAssets}
            locked={poolData.lockedCapacity}
          />
          <PremiumYield apy={poolData.premiumYield} />
        </div>

        <div className="space-y-4">
          <DepositForm
            onDeposit={handleDeposit}
            onWithdraw={handleWithdraw}
            disabled={!connected}
          />
          <TxStatus status={txStatus} message={txMessage} />
          {!connected && (
            <p className="text-sm text-levee-light/40">
              Connect your wallet to deposit.
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
