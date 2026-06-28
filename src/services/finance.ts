import { invoke } from "@tauri-apps/api/core";
import type {
  Account,
  Transaction,
  Category,
  CreateAccountInput,
  CreateTransactionInput,
  SpendingByCategory,
  CashFlow,
} from "@/types";

class FinanceServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "FinanceServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new FinanceServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const financeService = {
  listAccounts: () => safeInvoke<Account[]>("list_accounts"),

  getAccount: (id: string) => safeInvoke<Account>("get_account", { id }),

  createAccount: (input: CreateAccountInput) =>
    safeInvoke<Account>("create_account", { payload: input }),

  updateAccount: (id: string, input: Partial<CreateAccountInput>) =>
    safeInvoke<Account>("update_account", { id, payload: input }),

  deactivateAccount: (id: string) =>
    safeInvoke<void>("deactivate_account", { id }),

  listTransactions: (accountId?: string) =>
    safeInvoke<Transaction[]>("list_transactions", { accountId }),

  getTransaction: (id: string) =>
    safeInvoke<Transaction>("get_transaction", { id }),

  createTransaction: (input: CreateTransactionInput) =>
    safeInvoke<Transaction>("create_transaction", { payload: input }),

  updateTransaction: (id: string, input: Partial<CreateTransactionInput>) =>
    safeInvoke<Transaction>("update_transaction", { id, payload: input }),

  deleteTransaction: (id: string) =>
    safeInvoke<void>("delete_transaction", { id }),

  searchTransactions: (query: string) =>
    safeInvoke<Transaction[]>("search_transactions", { query }),

  listCategories: (type?: string) =>
    safeInvoke<Category[]>("list_categories", { type }),

  createCategory: (input: { name: string; type: string; parentId?: string }) =>
    safeInvoke<Category>("create_category", { payload: input }),

  updateCategory: (id: string, input: { name?: string; type?: string; parentId?: string }) =>
    safeInvoke<Category>("update_category", { id, payload: input }),

  deleteCategory: (id: string) =>
    safeInvoke<void>("delete_category", { id }),

  getNetWorth: () => safeInvoke<number>("get_net_worth"),

  getCashFlow: (startDate: string, endDate: string) =>
    safeInvoke<CashFlow>("get_cash_flow", { startDate, endDate }),

  getSpendingByCategory: (accountId: string, startDate: string, endDate: string) =>
    safeInvoke<SpendingByCategory[]>("get_spending_by_category", {
      accountId,
      startDate,
      endDate,
    }),
};
