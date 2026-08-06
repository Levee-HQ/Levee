"use client";

interface QuoteCardProps {
  amount: string;
  term: string;
  premium: string;
  rate: string;
  onBuy: () => void;
  loading?: boolean;
  disabled?: boolean;
}

export function QuoteCard({
  amount,
  term,
  premium,
  rate,
  onBuy,
  loading = false,
  disabled = false,
}: QuoteCardProps) {
  return (
    <div className="card space-y-4">
      <h3 className="font-display text-lg text-levee-mark">Quote</h3>
      <div className="space-y-3">
        <div className="flex justify-between">
          <span className="text-levee-light/60 text-sm">Coverage</span>
          <span className="font-tabular text-levee-mark">{amount} USDC</span>
        </div>
        <div className="flex justify-between">
          <span className="text-levee-light/60 text-sm">Term</span>
          <span className="font-tabular text-levee-mark">{term}</span>
        </div>
        <div className="border-t border-levee-mid/30 pt-3 flex justify-between">
          <span className="text-levee-light/60 text-sm">Premium</span>
          <span className="font-tabular text-levee-mark font-semibold">
            {premium} USDC
          </span>
        </div>
        <div className="flex justify-between">
          <span className="text-levee-light/60 text-sm">Rate</span>
          <span className="font-tabular text-levee-light/70">{rate} bps</span>
        </div>
      </div>
      <button
        onClick={onBuy}
        disabled={disabled || loading}
        className="btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {loading ? "Submitting..." : "Buy cover"}
      </button>
    </div>
  );
}
