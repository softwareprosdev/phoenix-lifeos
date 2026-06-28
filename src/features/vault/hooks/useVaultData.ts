import { useQuery } from "@tanstack/react-query";
import type { VaultItem } from "../types";

const mockVaultItems: VaultItem[] = [
  { id: "1", title: "Email Password", category: "password", value: "s3cur3P@ss!", last_updated: "2024-06-01", created_at: "2024-01-15" },
  { id: "2", title: "Bank PIN", category: "key", value: "1234", last_updated: "2024-05-20", created_at: "2024-01-15" },
  { id: "3", title: "WiFi Password", category: "password", value: "wifi_pass_123", last_updated: "2024-06-10", created_at: "2024-02-01" },
  { id: "4", title: "Investment Strategy Notes", category: "note", value: "Rebalance quarterly to maintain 80/20 split between stocks and bonds. Target retirement date: 2050.", last_updated: "2024-06-15", created_at: "2024-03-10" },
  { id: "5", title: "API Key - OpenAI", category: "secret", value: "sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", last_updated: "2024-06-12", created_at: "2024-04-01" },
  { id: "6", title: "SSH Private Key", category: "key", value: "-----BEGIN OPENSSH PRIVATE KEY-----\nxxxxxxxx\n-----END OPENSSH PRIVATE KEY-----", last_updated: "2024-05-01", created_at: "2024-01-20" },
  { id: "7", title: "Tax Filing Notes", category: "note", value: "Estimated taxes due: $12,500 for Q2 2024. Extension filed.", last_updated: "2024-06-05", created_at: "2024-04-15" },
  { id: "8", title: "Wallet Seed Phrase", category: "secret", value: "word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12", last_updated: "2024-01-01", created_at: "2024-01-01" },
];

export function useVaultItems() {
  return useQuery({
    queryKey: ["vault-items"],
    queryFn: () => Promise.resolve(mockVaultItems),
  });
}
