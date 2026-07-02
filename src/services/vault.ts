import { safeInvoke } from "./commandClient";
import type { VaultItem, CreateVaultItemInput, VaultAccessLogEntry } from "@/types";


export const vaultService = {
  listItems: (itemType?: string) =>
    safeInvoke<VaultItem[]>("Vault", "list_vault_items", { itemType }),

  getItem: (id: string) => safeInvoke<VaultItem>("Vault", "get_vault_item", { id }),

  createItem: (input: CreateVaultItemInput) =>
    safeInvoke<VaultItem>("Vault", "create_vault_item", { payload: input }),

  updateItem: (id: string, input: Partial<CreateVaultItemInput>) =>
    safeInvoke<VaultItem>("Vault", "update_vault_item", { id, payload: input }),

  deleteItem: (id: string) => safeInvoke<void>("Vault", "delete_vault_item", { id }),

  searchItems: (query: string) =>
    safeInvoke<VaultItem[]>("Vault", "search_vault_items", { query }),

  getAccessLog: (itemId: string) =>
    safeInvoke<VaultAccessLogEntry[]>("Vault", "get_vault_access_log", { itemId }),

  listRecentAccess: (limit?: number) =>
    safeInvoke<VaultAccessLogEntry[]>("Vault", "list_recent_vault_access", { limit: limit ?? 20 }),
};
