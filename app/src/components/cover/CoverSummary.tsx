"use client";

interface CoverSummaryProps {
  perilName: string;
  amount: string;
  term: string;
  premium: string;
  status: "Active" | "Settled" | "Expired";
  policyId: number;
}

export function CoverSummary({
  perilName,
  amount,
  term,
  premium,
  status,
  policyId,
}: CoverSummaryProps) {
  const statusColors = {
    Active: "bg-levee-surface/20 text-levee-light",
    Settled: "bg-levee-trigger/20 text-levee-trigger",
    Expired: "bg-levee-mid/20 text-levee-light/40",
  };

  return (
    <div className="card flex items-center justify-between">
      <div className="space-y-1">
        <div className="flex items-center gap-3">
          <span className="font-display text-levee-mark">{perilName}</span>
          <span
            className={`text-xs font-semibold px-2 py-0.5 rounded-full ${statusColors[status]}`}
          >
            {status}
          </span>
        </div>
        <div className="text-sm text-levee-light/60">
          Policy #{policyId} &middot; {term}
        </div>
      </div>
      <div className="text-right">
        <div className="font-tabular text-levee-mark">{amount} USDC</div>
        <div className="text-xs text-levee-light/40 font-tabular">
          {premium} USDC premium
        </div>
      </div>
    </div>
  );
}
