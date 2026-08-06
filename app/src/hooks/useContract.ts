"use client";

import { useState, useCallback } from "react";

interface ContractCallState<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
}

export function useContract<T>() {
  const [state, setState] = useState<ContractCallState<T>>({
    data: null,
    loading: false,
    error: null,
  });

  const execute = useCallback(async (fn: () => Promise<T>) => {
    setState({ data: null, loading: true, error: null });
    try {
      const result = await fn();
      setState({ data: result, loading: false, error: null });
      return result;
    } catch (err) {
      const message = err instanceof Error ? err.message : "Unknown error";
      setState({ data: null, loading: false, error: message });
      throw err;
    }
  }, []);

  return { ...state, execute };
}
