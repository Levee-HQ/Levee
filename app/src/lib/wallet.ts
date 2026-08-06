export async function getFreighterApi() {
  if (typeof window === "undefined") return null;
  const freighter = (window as any).freighterApi;
  if (!freighter) return null;
  return freighter;
}

export async function isFreighterInstalled(): Promise<boolean> {
  const api = await getFreighterApi();
  return api !== null;
}

// Test wallet addresses for demo/testing (testnet only). Public keys only —
// secret keys must never live in frontend source, even fabricated ones.
export const TEST_WALLETS = {
  admin: {
    publicKey: "GAVDWZTD6RQBVMZN6PTWDVKX6YSWFRHSJZ4GXE3QLKNQP3DNNZFDWMA",
    name: "Admin",
  },
  underwriter: {
    publicKey: "GBCSGTG4JRYSQFHXWXQV4SDKHQBJHDRDMVDJ4HZZVZJ2H7HJMBLQJJH",
    name: "Underwriter",
  },
  user: {
    publicKey: "GCCQXQSDGSSXVNWZBLCSLNDQSKCSVHSVCSDFLKGHSDFHGSLDFHGLSDFHGSD",
    name: "User (Buyer)",
  },
} as const;
