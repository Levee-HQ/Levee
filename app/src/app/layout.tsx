import type { Metadata } from "next";
import "@/styles/globals.css";
import { WalletProvider } from "@/components/wallet/WalletProvider";
import { ConnectButton } from "@/components/wallet/ConnectButton";
import { NetworkBadge } from "@/components/wallet/NetworkBadge";
import Link from "next/link";

export const metadata: Metadata = {
  title: "Levee — Parametric Cover for Stellar DeFi",
  description:
    "Automatic payouts for oracle manipulation, stablecoin depegs, and lending-pool bad debt.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <WalletProvider>
          <div className="min-h-screen flex flex-col">
            <header className="border-b border-levee-mid/30 px-6 py-4">
              <nav className="max-w-6xl mx-auto flex items-center justify-between">
                <div className="flex items-center gap-8">
                  <Link
                    href="/"
                    className="font-display text-2xl text-levee-mark"
                  >
                    Levee
                  </Link>
                  <div className="hidden sm:flex items-center gap-6">
                    <Link
                      href="/cover"
                      className="text-levee-light/70 hover:text-levee-mark transition-colors"
                    >
                      Buy cover
                    </Link>
                    <Link
                      href="/underwrite"
                      className="text-levee-light/70 hover:text-levee-mark transition-colors"
                    >
                      Underwrite
                    </Link>
                    <Link
                      href="/positions"
                      className="text-levee-light/70 hover:text-levee-mark transition-colors"
                    >
                      Positions
                    </Link>
                  </div>
                </div>
                <div className="flex items-center gap-4">
                  <NetworkBadge />
                  <ConnectButton />
                </div>
              </nav>
            </header>
            <main className="flex-1 max-w-6xl mx-auto w-full px-6 py-8">
              {children}
            </main>
            <footer className="border-t border-levee-mid/30 px-6 py-6">
              <div className="max-w-6xl mx-auto text-center text-levee-light/40 text-sm">
                Levee v0 — Testnet only. Contracts unaudited. Do not deposit
                real funds.
              </div>
            </footer>
          </div>
        </WalletProvider>
      </body>
    </html>
  );
}
