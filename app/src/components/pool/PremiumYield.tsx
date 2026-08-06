"use client";

interface PremiumYieldProps {
  apy: number;
}

export function PremiumYield({ apy }: PremiumYieldProps) {
  return (
    <div className="card">
      <div className="text-sm text-levee-light/60">Premium yield (APY)</div>
      <div className="font-tabular text-2xl text-levee-mark mt-1">
        {apy.toFixed(1)}%
      </div>
    </div>
  );
}
