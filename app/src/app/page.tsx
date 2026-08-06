"use client";

import Link from "next/link";
import { WaterlineGauge } from "@/components/gauge/WaterlineGauge";

export default function Home() {
  return (
    <div className="space-y-16">
      <section className="pt-12 pb-8 text-center space-y-6">
        <h1 className="font-display text-5xl sm:text-6xl text-levee-mark leading-tight">
          A levee does not stop the river
        </h1>
        <p className="text-xl text-levee-light/70 max-w-2xl mx-auto">
          It decides where the water goes when the river rises. Parametric cover
          for Stellar DeFi — automatic payouts, no claims process.
        </p>
        <div className="flex items-center justify-center gap-4 pt-4">
          <Link href="/cover" className="btn-primary">
            Buy cover
          </Link>
          <Link href="/underwrite" className="btn-secondary">
            Supply capital
          </Link>
        </div>
      </section>

      <section className="grid md:grid-cols-3 gap-6">
        <div className="card space-y-4">
          <div className="text-levee-surface font-semibold text-sm uppercase tracking-wider">
            Buyers
          </div>
          <h3 className="font-display text-xl text-levee-mark">
            Protection that pays itself
          </h3>
          <p className="text-levee-light/60 text-sm leading-relaxed">
            Pick a protocol, choose a peril, set your coverage amount and term.
            If the trigger fires, the payout is automatic. Nothing to file,
            nothing to argue.
          </p>
        </div>
        <div className="card space-y-4">
          <div className="text-levee-surface font-semibold text-sm uppercase tracking-wider">
            Underwriters
          </div>
          <h3 className="font-display text-xl text-levee-mark">
            Earn premiums on idle capital
          </h3>
          <p className="text-levee-light/60 text-sm leading-relaxed">
            Deposit USDC into a peril-specific pool. Earn premiums from cover
            buyers. Capital is locked only for the term of the cover it backs.
          </p>
        </div>
        <div className="card space-y-4">
          <div className="text-levee-surface font-semibold text-sm uppercase tracking-wider">
            Triggers
          </div>
          <h3 className="font-display text-xl text-levee-mark">
            Parametric and transparent
          </h3>
          <p className="text-levee-light/60 text-sm leading-relaxed">
            Every trigger condition is fully specified up front. Oracle deviation
            beyond a threshold, sustained across multiple ledgers. No adjuster,
            no jurisdiction.
          </p>
        </div>
      </section>

      <section className="card flex flex-col md:flex-row items-center gap-8">
        <div className="w-full md:w-1/3 flex justify-center">
          <WaterlineGauge
            totalCapital={5000}
            lockedCapital={2000}
            triggerProximity={0.3}
            height={240}
          />
        </div>
        <div className="flex-1 space-y-4">
          <h3 className="font-display text-2xl text-levee-mark">
            The waterline gauge
          </h3>
          <p className="text-levee-light/60 leading-relaxed">
            Pool capital as water level. Coverage sold as the marked flood
            stage. Current trigger proximity as a rising line. One glance tells
            you the state of the system.
          </p>
        </div>
      </section>
    </div>
  );
}
