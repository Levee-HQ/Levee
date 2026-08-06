"use client";

import { useState } from "react";
import { QuoteCard } from "@/components/cover/QuoteCard";
import { TermSelector } from "@/components/cover/TermSelector";
import { TxStatus } from "@/components/wallet/TxStatus";
import { useWallet } from "@/components/wallet/WalletProvider";
import { computePremium } from "@/lib/pricing";

export default function CoverPage() {
  const { connected } = useWallet();
  const [amount, setAmount] = useState("");
  const [termLedgers, setTermLedgers] = useState(17_280);
  const [txStatus, setTxStatus] = useState<"idle" | "signing" | "submitting" | "success" | "error">("idle");
  const [txMessage, setTxMessage] = useState("");

  const parsedAmount = parseFloat(amount) || 0;
  const quote = parsedAmount > 0
    ? computePremium({
        amount: BigInt(Math.floor(parsedAmount * 10_000_000)),
        term_ledgers: termLedgers,
        base_premium_rate_bps: 200,
      })
    : null;

  const handleBuy = async () => {
    if (!connected || !quote) return;
    setTxStatus("signing");
    try {
      setTxStatus("submitting");
      await new Promise((r) => setTimeout(r, 2000));
      setTxStatus("success");
      setTxMessage("Cover bought");
    } catch {
      setTxStatus("error");
      setTxMessage("Transaction failed. Check your wallet and try again.");
    }
  };

  return (
    <div className="space-y-8">
      <div>
        <h1 className="font-display text-3xl text-levee-mark">Buy cover</h1>
        <p className="text-levee-light/60 mt-2">
          Protection against oracle deviation on Blend pools.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-8">
        <div className="space-y-6">
          <div className="card space-y-4">
            <label className="block text-sm font-medium text-levee-light/70">
              Coverage amount (USDC)
            </label>
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="1000"
              className="input-field w-full"
              min="0"
              step="100"
            />
          </div>

          <div className="card space-y-4">
            <label className="block text-sm font-medium text-levee-light/70">
              Term
            </label>
            <TermSelector value={termLedgers} onChange={setTermLedgers} />
          </div>

          <TxStatus status={txStatus} message={txMessage} />
        </div>

        <div>
          {quote && parsedAmount > 0 ? (
            <QuoteCard
              amount={parsedAmount.toLocaleString()}
              term={`${Math.floor(termLedgers / 17_280)} day${termLedgers >= 34_560 ? "s" : ""}`}
              premium={(Number(quote.premium) / 10_000_000).toFixed(2)}
              rate={quote.rate_bps.toString()}
              onBuy={handleBuy}
              loading={txStatus === "signing" || txStatus === "submitting"}
              disabled={!connected}
            />
          ) : (
            <div className="card text-center py-12">
              <p className="text-levee-light/40">
                Enter an amount to see your quote.
              </p>
            </div>
          )}

          {!connected && (
            <p className="text-center text-sm text-levee-light/40 mt-4">
              Connect your wallet to buy cover.
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
