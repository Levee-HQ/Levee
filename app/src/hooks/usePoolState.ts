"use client";

import { useState, useEffect } from "react";

export interface PoolState {
  totalAssets: number;
  lockedCapacity: number;
  totalShares: number;
  availableCapacity: number;
}

export function usePoolState(): {
  pool: PoolState | null;
  loading: boolean;
  error: string | null;
} {
  const [pool, setPool] = useState<PoolState | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setPool({
      totalAssets: 5_000,
      lockedCapacity: 2_000,
      totalShares: 5_000,
      availableCapacity: 3_000,
    });
    setLoading(false);
  }, []);

  return { pool, loading, error };
}
