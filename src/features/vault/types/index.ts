export interface VaultItem {
  id: string;
  title: string;
  category: "password" | "note" | "key" | "secret";
  value: string;
  last_updated: string;
  created_at: string;
}
