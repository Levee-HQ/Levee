"use client";

import { useState } from "react";
import { TEST_WALLETS } from "@/lib/wallet";

interface WalletModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSelectFreighter: () => Promise<void>;
  onSelectTestWallet: (wallet: keyof typeof TEST_WALLETS) => void;
}

export function WalletModal({
  isOpen,
  onClose,
  onSelectFreighter,
  onSelectTestWallet,
}: WalletModalProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleFreighter = async () => {
    setLoading(true);
    setError(null);
    try {
      await onSelectFreighter();
      onClose();
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : "Failed to connect Freighter wallet"
      );
    } finally {
      setLoading(false);
    }
  };

  const handleTestWallet = (walletKey: keyof typeof TEST_WALLETS) => {
    onSelectTestWallet(walletKey);
    onClose();
  };

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/50 z-40"
        onClick={onClose}
        role="presentation"
      />

      {/* Modal */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div className="bg-levee-deep border border-levee-mid rounded-lg shadow-xl max-w-md w-full">
          {/* Header */}
          <div className="border-b border-levee-mid px-6 py-4">
            <h2 className="font-display text-xl text-levee-mark">
              Connect Wallet
            </h2>
            <p className="text-levee-light/60 text-sm mt-1">
              Choose how you want to connect
            </p>
          </div>

          {/* Content */}
          <div className="p-6 space-y-3">
            {/* Freighter Option */}
            <button
              onClick={handleFreighter}
              disabled={loading}
              className="w-full p-4 border border-levee-surface rounded-lg hover:bg-levee-mid/20 transition-colors text-left disabled:opacity-50"
            >
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 bg-levee-surface rounded-full flex items-center justify-center">
                  🔐
                </div>
                <div>
                  <div className="font-semibold text-levee-mark">
                    Freighter Wallet
                  </div>
                  <div className="text-xs text-levee-light/60">
                    Browser extension (recommended)
                  </div>
                </div>
              </div>
            </button>

            {/* Divider */}
            <div className="relative py-2">
              <div className="absolute inset-0 flex items-center">
                <div className="w-full border-t border-levee-mid/30" />
              </div>
              <div className="relative flex justify-center text-xs">
                <span className="px-2 bg-levee-deep text-levee-light/50">
                  Test Wallets
                </span>
              </div>
            </div>

            {/* Test Wallet Options */}
            {(Object.entries(TEST_WALLETS) as Array<[keyof typeof TEST_WALLETS, any]>).map(
              ([key, wallet]) => (
                <button
                  key={key}
                  onClick={() => handleTestWallet(key)}
                  className="w-full p-4 border border-levee-surface/50 rounded-lg hover:bg-levee-mid/20 transition-colors text-left"
                >
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 bg-levee-surface/30 rounded-full flex items-center justify-center">
                      {key === "admin" && "👑"}
                      {key === "underwriter" && "💰"}
                      {key === "user" && "🛡️"}
                    </div>
                    <div>
                      <div className="font-semibold text-levee-mark">
                        {wallet.name}
                      </div>
                      <div className="text-xs text-levee-light/60 font-tabular">
                        {wallet.publicKey.slice(0, 8)}...
                        {wallet.publicKey.slice(-6)}
                      </div>
                    </div>
                  </div>
                </button>
              )
            )}

            {error && (
              <div className="p-3 bg-levee-trigger/10 border border-levee-trigger/30 rounded text-levee-trigger text-sm">
                {error}
              </div>
            )}
          </div>

          {/* Footer */}
          <div className="border-t border-levee-mid px-6 py-4 flex justify-end">
            <button
              onClick={onClose}
              className="px-4 py-2 text-levee-light/70 hover:text-levee-mark transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
