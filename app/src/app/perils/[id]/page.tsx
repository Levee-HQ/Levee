"use client";

import { WaterlineGauge } from "@/components/gauge/WaterlineGauge";

export default function PerilDetailPage() {
  const peril = {
    id: "ORACLE1",
    name: "Oracle Deviation",
    description: "Triggers when the reported oracle price deviates from the reference feed beyond the threshold, sustained across multiple ledgers.",
    threshold_bps: 500,
    sustain_window: 3,
    max_coverage_ratio_bps: 8000,
    base_premium_rate_bps: 200,
    active: true,
    pool: {
      totalAssets: 5000,
      lockedCapacity: 2000,
      triggerProximity: 0.15,
    },
  };

  return (
    <div className="space-y-8">
      <div>
        <div className="flex items-center gap-3">
          <h1 className="font-display text-3xl text-levee-mark">
            {peril.name}
          </h1>
          <span
            className={`text-xs font-semibold px-2 py-1 rounded-full ${
              peril.active
                ? "bg-levee-surface/20 text-levee-light"
                : "bg-red-900/20 text-red-300"
            }`}
          >
            {peril.active ? "Active" : "Inactive"}
          </span>
        </div>
        <p className="text-levee-light/60 mt-2 max-w-2xl">
          {peril.description}
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-8">
        <div className="card flex justify-center py-8">
          <WaterlineGauge
            totalCapital={peril.pool.totalAssets}
            lockedCapital={peril.pool.lockedCapacity}
            triggerProximity={peril.pool.triggerProximity}
            height={280}
          />
        </div>

        <div className="space-y-4">
          <div className="card space-y-3">
            <h3 className="text-sm font-semibold uppercase tracking-wider text-levee-surface">
              Trigger parameters
            </h3>
            <div className="space-y-2">
              <Row label="Deviation threshold" value={`${peril.threshold_bps} bps (${(peril.threshold_bps / 100).toFixed(1)}%)`} />
              <Row label="Sustain window" value={`${peril.sustain_window} ledgers`} />
              <Row label="Max coverage ratio" value={`${peril.max_coverage_ratio_bps} bps (${(peril.max_coverage_ratio_bps / 100).toFixed(0)}%)`} />
              <Row label="Base premium rate" value={`${peril.base_premium_rate_bps} bps`} />
            </div>
          </div>

          <div className="card space-y-3">
            <h3 className="text-sm font-semibold uppercase tracking-wider text-levee-surface">
              Pool state
            </h3>
            <div className="space-y-2">
              <Row label="Total capital" value={`${peril.pool.totalAssets.toLocaleString()} USDC`} />
              <Row label="Locked" value={`${peril.pool.lockedCapacity.toLocaleString()} USDC`} />
              <Row label="Available" value={`${(peril.pool.totalAssets - peril.pool.lockedCapacity).toLocaleString()} USDC`} />
              <Row label="Trigger proximity" value={`${(peril.pool.triggerProximity * 100).toFixed(1)}%`} highlight />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function Row({ label, value, highlight = false }: { label: string; value: string; highlight?: boolean }) {
  return (
    <div className="flex justify-between text-sm">
      <span className="text-levee-light/60">{label}</span>
      <span className={`font-tabular ${highlight ? "text-levee-trigger font-semibold" : "text-levee-mark"}`}>
        {value}
      </span>
    </div>
  );
}
