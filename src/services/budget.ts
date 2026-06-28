import { invoke } from "@tauri-apps/api/core";
import type { Budget, BudgetItem, BudgetWithItems, CreateBudgetInput, BudgetProgress } from "@/types";

class BudgetServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "BudgetServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new BudgetServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const budgetService = {
  listBudgets: (startDate?: string, endDate?: string) =>
    safeInvoke<Budget[]>("list_budgets", { startDate, endDate }),

  getBudget: (id: string) =>
    safeInvoke<Budget>("get_budget", { id }),

  createBudget: (input: CreateBudgetInput) =>
    safeInvoke<Budget>("create_budget", { payload: input }),

  updateBudget: (id: string, input: Partial<CreateBudgetInput>) =>
    safeInvoke<Budget>("update_budget", { id, payload: input }),

  deleteBudget: (id: string) =>
    safeInvoke<void>("delete_budget", { id }),

  getBudgetWithItems: (id: string) =>
    safeInvoke<BudgetWithItems>("get_budget_with_items", { id }),

  getBudgetProgress: (id: string) =>
    safeInvoke<BudgetProgress>("get_budget_progress", { id }),

  listBudgetItems: (budgetId: string) =>
    safeInvoke<BudgetItem[]>("list_budget_items", { budgetId }),

  createBudgetItem: (budgetId: string, input: { category_id: string; limit_cents: number }) =>
    safeInvoke<BudgetItem>("create_budget_item", { budgetId, payload: input }),

  updateBudgetItem: (id: string, input: { limit_cents?: number; spent_cents?: number }) =>
    safeInvoke<BudgetItem>("update_budget_item", { id, payload: input }),

  deleteBudgetItem: (id: string) =>
    safeInvoke<void>("delete_budget_item", { id }),
};
