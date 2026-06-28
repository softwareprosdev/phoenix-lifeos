import { useQuery } from "@tanstack/react-query";
import type { Budget, BudgetCategoryBreakdown } from "../types";

const mockBudgets: Budget[] = [
  { id: "1", category: "Housing", limit_cents: 200000, spent_cents: 180000, period: "monthly", color: "hsl(220, 90%, 56%)", created_at: "2024-01-01" },
  { id: "2", category: "Food", limit_cents: 60000, spent_cents: 65000, period: "monthly", color: "hsl(142, 72%, 45%)", created_at: "2024-01-01" },
  { id: "3", category: "Transport", limit_cents: 40000, spent_cents: 32000, period: "monthly", color: "hsl(38, 92%, 50%)", created_at: "2024-01-01" },
  { id: "4", category: "Entertainment", limit_cents: 20000, spent_cents: 18000, period: "monthly", color: "hsl(280, 80%, 60%)", created_at: "2024-01-01" },
  { id: "5", category: "Utilities", limit_cents: 30000, spent_cents: 28000, period: "monthly", color: "hsl(196, 85%, 50%)", created_at: "2024-01-01" },
  { id: "6", category: "Shopping", limit_cents: 25000, spent_cents: 22000, period: "monthly", color: "hsl(0, 84%, 60%)", created_at: "2024-01-01" },
];

export function useBudgets() {
  return useQuery({
    queryKey: ["budgets"],
    queryFn: () => Promise.resolve(mockBudgets),
  });
}

export function useBudgetBreakdown(budgetId: string) {
  const breakdowns: Record<string, BudgetCategoryBreakdown> = {
    "1": { category: "Housing", spent: 180000, limit: 200000, items: [{ name: "Rent", amount: 150000, date: "2024-06-01" }, { name: "Internet", amount: 8000, date: "2024-06-01" }, { name: "Insurance", amount: 22000, date: "2024-06-05" }] },
    "2": { category: "Food", spent: 65000, limit: 60000, items: [{ name: "Groceries", amount: 42000, date: "2024-06-03" }, { name: "Dining Out", amount: 18000, date: "2024-06-10" }, { name: "Coffee Shops", amount: 5000, date: "2024-06-12" }] },
  };
  return useQuery({
    queryKey: ["budget-breakdown", budgetId],
    queryFn: () => Promise.resolve(breakdowns[budgetId] ?? { category: "", spent: 0, limit: 0, items: [] }),
  });
}
