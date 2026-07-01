import { useQuery } from "@tanstack/react-query";
import { budgetService } from "@/services";
import type { Budget, BudgetCategoryBreakdown } from "../types";

export function useBudgets() {
  return useQuery({
    queryKey: ["budgets"],
    queryFn: () => budgetService.listBudgets() as unknown as Promise<Budget[]>,
  });
}

export function useBudgetBreakdown(budgetId: string) {
  return useQuery({
    queryKey: ["budget-breakdown", budgetId],
    queryFn: () =>
      budgetService.getBudgetWithItems(budgetId).then(
        (data) =>
          ({
            category: data.name,
            spent: data.items.reduce((s, i) => s + i.spent_cents, 0),
            limit: data.total_limit_cents,
            items: data.items.map((i) => ({
              name: i.category_id ?? "Uncategorized",
              amount: i.spent_cents,
              date: "",
            })),
          }) as BudgetCategoryBreakdown,
      ),
    enabled: !!budgetId,
  });
}
