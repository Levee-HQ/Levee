"use client";

import { useState } from "react";

interface DepositFormProps {
  onDeposit: (amount: number) => void;
  onWithdraw: (shares: number) => void;
  disabled?: boolean;
}

export function DepositForm({ onDeposit, onWithdraw, disabled }: DepositFormProps) {
  const [amount, setAmount] = useState("");
  const [mode, setMode] = useState<"deposit" | "withdraw">("deposit");

  const handleSubmit = () => {
    const val = parseFloat(amount);
    if (val <= 0 || isNaN(val)) return;
    if (mode === "deposit") {
      onDeposit(val);
    } else {
      onWithdraw(val);
    }
    setAmount("");
  };

  return (
    <div className="card space-y-4">
      <div className="flex gap-2">
        <button
          onClick={() => setMode("deposit")}
          className={`flex-1 py-2 rounded-lg text-sm font-medium transition-colors ${
            mode === "deposit"
              ? "bg-levee-surface text-levee-mark"
              : "text-levee-light/60 hover:text-levee-mark"
          }`}
        >
          Deposit
        </button>
        <button
          onClick={() => setMode("withdraw")}
          className={`flex-1 py-2 rounded-lg text-sm font-medium transition-colors ${
            mode === "withdraw"
              ? "bg-levee-surface text-levee-mark"
              : "text-levee-light/60 hover:text-levee-mark"
          }`}
        >
          Withdraw
        </button>
      </div>
      <input
        type="number"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
        placeholder={mode === "deposit" ? "Amount to deposit" : "Shares to withdraw"}
        className="input-field w-full"
        min="0"
      />
      <button
        onClick={handleSubmit}
        disabled={disabled || !amount || parseFloat(amount) <= 0}
        className="btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {mode === "deposit" ? "Deposit" : "Withdraw"}
      </button>
    </div>
  );
}
