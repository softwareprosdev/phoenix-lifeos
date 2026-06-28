import { invoke } from "@tauri-apps/api/core";
import type { VaultItem, CreateVaultItemInput, VaultAccessLogEntry } from "@/types";

class VaultServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "VaultServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new VaultServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const vaultService = {
  listItems: (itemType?: string) =>
    safeInvoke<VaultItem[]>("list_vault_items", { itemType }),

  getItem: (id: string) => safeInvoke<VaultItem>("get_vault_item", { id }),

  createItem: (input: CreateVaultItemInput) =>
    safeInvoke<VaultItem>("create_vault_item", { payload: input }),

  updateItem: (id: string, input: Partial<CreateVaultItemInput>) =>
    safeInvoke<VaultItem>("update_vault_item", { id, payload: input }),

  deleteItem: (id: string) => safeInvoke<void>("delete_vault_item", { id }),

  searchItems: (query: string) =>
    safeInvoke<VaultItem[]>("search_vault_items", { query }),

  getAccessLog: (itemId: string) =>
    safeInvoke<VaultAccessLogEntry[]>("get_vault_access_log", { itemId }),

  listRecentAccess: (limit?: number) =>
    safeInvoke<VaultAccessLogEntry[]>("list_recent_vault_access", { limit: limit ?? 20 }),
};
