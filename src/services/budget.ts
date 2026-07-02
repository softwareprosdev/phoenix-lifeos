import { safeInvoke } from "./commandClient";
import type { Budget, BudgetItem, BudgetWithItems, CreateBudgetInput, BudgetProgress } from "@/types";


export const budgetService = {
  listBudgets: (startDate?: string, endDate?: string) =>
    safeInvoke<Budget[]>("Budget", "list_budgets", { startDate, endDate }),

  getBudget: (id: string) =>
    safeInvoke<Budget>("Budget", "get_budget", { id }),

  createBudget: (input: CreateBudgetInput) =>
    safeInvoke<Budget>("Budget", "create_budget", { payload: input }),

  updateBudget: (id: string, input: Partial<CreateBudgetInput>) =>
    safeInvoke<Budget>("Budget", "update_budget", { id, payload: input }),

  deleteBudget: (id: string) =>
    safeInvoke<void>("Budget", "delete_budget", { id }),

  getBudgetWithItems: (id: string) =>
    safeInvoke<BudgetWithItems>("Budget", "get_budget_with_items", { id }),

  getBudgetProgress: (id: string) =>
    safeInvoke<BudgetProgress>("Budget", "get_budget_progress", { id }),

  listBudgetItems: (budgetId: string) =>
    safeInvoke<BudgetItem[]>("Budget", "list_budget_items", { budgetId }),

  createBudgetItem: (budgetId: string, input: { category_id: string; limit_cents: number }) =>
    safeInvoke<BudgetItem>("Budget", "create_budget_item", { budgetId, payload: input }),

  updateBudgetItem: (id: string, input: { limit_cents?: number; spent_cents?: number }) =>
    safeInvoke<BudgetItem>("Budget", "update_budget_item", { id, payload: input }),

  deleteBudgetItem: (id: string) =>
    safeInvoke<void>("Budget", "delete_budget_item", { id }),
};
