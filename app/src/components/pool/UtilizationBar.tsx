"use client";

interface UtilizationBarProps {
  total: number;
  locked: number;
}

export function UtilizationBar({ total, locked }: UtilizationBarProps) {
  const utilization = total > 0 ? (locked / total) * 100 : 0;

  return (
    <div className="card space-y-3">
      <div className="flex justify-between text-sm">
        <span className="text-levee-light/60">Pool utilization</span>
        <span className="font-tabular text-levee-mark">{utilization.toFixed(1)}%</span>
      </div>
      <div className="h-2 bg-levee-deep rounded-full overflow-hidden">
        <div
          className="h-full bg-levee-surface rounded-full transition-all duration-300"
          style={{ width: `${Math.min(utilization, 100)}%` }}
        />
      </div>
      <div className="flex justify-between text-xs text-levee-light/40 font-tabular">
        <span>{locked.toLocaleString()} locked</span>
        <span>{total.toLocaleString()} total</span>
      </div>
    </div>
  );
}
