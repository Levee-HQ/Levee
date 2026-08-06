"use client";

import { CoverSummary } from "@/components/cover/CoverSummary";
import { useWallet } from "@/components/wallet/WalletProvider";

export default function PositionsPage() {
  const { connected } = useWallet();

  const mockPolicies = [
    {
      policyId: 1,
      perilName: "Oracle Deviation",
      amount: "1,000",
      term: "7 days",
      premium: "14.00",
      status: "Active" as const,
    },
    {
      policyId: 2,
      perilName: "Oracle Deviation",
      amount: "500",
      term: "14 days",
      premium: "14.00",
      status: "Expired" as const,
    },
  ];

  if (!connected) {
    return (
      <div className="space-y-8">
        <h1 className="font-display text-3xl text-levee-mark">Positions</h1>
        <div className="card text-center py-16">
          <p className="text-levee-light/40 text-lg">
            Connect your wallet to see your positions.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      <h1 className="font-display text-3xl text-levee-mark">Positions</h1>

      <section className="space-y-4">
        <h2 className="text-sm font-semibold uppercase tracking-wider text-levee-surface">
          Cover positions
        </h2>
        {mockPolicies.length > 0 ? (
          <div className="space-y-3">
            {mockPolicies.map((p) => (
              <CoverSummary key={p.policyId} {...p} />
            ))}
          </div>
        ) : (
          <div className="card text-center py-8">
            <p className="text-levee-light/40">
              No cover positions yet. Buy cover to get started.
            </p>
          </div>
        )}
      </section>

      <section className="space-y-4">
        <h2 className="text-sm font-semibold uppercase tracking-wider text-levee-surface">
          Underwriting positions
        </h2>
        <div className="card">
          <div className="flex justify-between items-center">
            <div>
              <div className="text-levee-mark font-display">Oracle Deviation Pool</div>
              <div className="text-sm text-levee-light/60">0 shares</div>
            </div>
            <div className="text-right">
              <div className="font-tabular text-levee-mark">0.00 USDC</div>
              <div className="text-xs text-levee-light/40">value</div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
