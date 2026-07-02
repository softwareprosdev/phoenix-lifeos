import { safeInvoke } from "./commandClient";
import type {
  Account,
  Transaction,
  Category,
  CreateAccountInput,
  CreateTransactionInput,
  CashFlow,
} from "@/types";

type NetWorthResponse = {
  net_worth_cents: number;
};

type CashFlowResponse = {
  total_income_cents: number;
  total_expenses_cents: number;
  net_cents: number;
};

type SpendingByCategoryResponse = {
  category_id?: string | null;
  category_name?: string | null;
  total_cents: number;
  transaction_count: number;
};

function normalizeCashFlow(data: CashFlow | CashFlowResponse): CashFlow {
  if ("total_income_cents" in data) {
    const income = data.total_income_cents;
    const expenses = Math.abs(data.total_expenses_cents);
    return {
      income,
      expenses,
      net: "net_cents" in data ? data.net_cents : income - expenses,
    };
  }

  return data;
}

export const financeService = {
  listAccounts: () => safeInvoke<Account[]>("Finance", "list_accounts"),

  getAccount: (id: string) => safeInvoke<Account>("Finance", "get_account", { id }),

  createAccount: (input: CreateAccountInput) =>
    safeInvoke<Account>("Finance", "create_account", { payload: input }),

  updateAccount: (id: string, input: Partial<CreateAccountInput>) =>
    safeInvoke<Account>("Finance", "update_account", { id, payload: input }),

  deactivateAccount: (id: string) =>
    safeInvoke<void>("Finance", "deactivate_account", { id }),

  listTransactions: (accountId?: string) =>
    safeInvoke<Transaction[]>("Finance", "list_transactions", { accountId }),

  getTransaction: (id: string) =>
    safeInvoke<Transaction>("Finance", "get_transaction", { id }),

  createTransaction: (input: CreateTransactionInput) =>
    safeInvoke<Transaction>("Finance", "create_transaction", { payload: input }),

  updateTransaction: (id: string, input: Partial<CreateTransactionInput>) =>
    safeInvoke<Transaction>("Finance", "update_transaction", { id, payload: input }),

  deleteTransaction: (id: string) =>
    safeInvoke<void>("Finance", "delete_transaction", { id }),

  searchTransactions: (query: string) =>
    safeInvoke<Transaction[]>("Finance", "search_transactions", { query }),

  listCategories: (type?: string) =>
    safeInvoke<Category[]>("Finance", "list_categories", { type }),

  createCategory: (input: { name: string; type: string; parentId?: string }) =>
    safeInvoke<Category>("Finance", "create_category", { payload: input }),

  updateCategory: (id: string, input: { name?: string; type?: string; parentId?: string }) =>
    safeInvoke<Category>("Finance", "update_category", { id, payload: input }),

  deleteCategory: (id: string) =>
    safeInvoke<void>("Finance", "delete_category", { id }),

  getNetWorth: async () => {
    const data = await safeInvoke<number | NetWorthResponse>("Finance", "get_net_worth");
    return typeof data === "number" ? data : data.net_worth_cents;
  },

  getCashFlow: async (startDate: string, endDate: string) => {
    const data = await safeInvoke<CashFlow | CashFlowResponse>("Finance", "get_cash_flow", {
      payload: { start: startDate, end: endDate },
    });
    return normalizeCashFlow(data);
  },

  getSpendingByCategory: async (_accountId: string, startDate: string, endDate: string) => {
    const data = await safeInvoke<SpendingByCategoryResponse[]>("Finance", "get_spending_by_category", {
      payload: { start: startDate, end: endDate },
    });
    return data.map((item) => ({
      category_id: item.category_id ?? "uncategorized",
      category_name: item.category_name ?? "Uncategorized",
      total_cents: Math.abs(item.total_cents),
      transaction_count: item.transaction_count,
    }));
  },
};
