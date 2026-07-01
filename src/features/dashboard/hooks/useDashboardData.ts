import { useQuery } from "@tanstack/react-query";
import { financeService, debtService, creditService } from "@/services";
import type { BudgetItem } from "@/types";
import type { WidgetConfig } from "../types";

const mockWidgets: WidgetConfig[] = [
  { id: "net-worth", title: "Net Worth", type: "net-worth", w: 2, h: 1 },
  { id: "cash-flow", title: "Cash Flow", type: "cash-flow", w: 1, h: 1 },
  { id: "budget-health", title: "Budget Health", type: "budget-health", w: 1, h: 1 },
  { id: "debt-overview", title: "Debt Overview", type: "debt-overview", w: 1, h: 1 },
  { id: "credit-score", title: "Credit Score", type: "credit-score", w: 1, h: 1 },
  { id: "recent-transactions", title: "Recent Transactions", type: "recent-transactions", w: 2, h: 1 },
  { id: "ai-insights", title: "AI Insights", type: "ai-insights", w: 1, h: 1 },
];

export function useWidgets() {
  return useQuery({
    queryKey: ["dashboard", "widgets"],
    queryFn: () => Promise.resolve(mockWidgets),
  });
}

export function useNetWorth() {
  return useQuery({
    queryKey: ["dashboard", "net-worth"],
    queryFn: async () => {
      const data = await financeService.getNetWorth();
      return [{ date: new Date().toISOString().slice(0, 7), value: data }];
    },
  });
}

export function useCashFlow() {
  return useQuery({
    queryKey: ["dashboard", "cash-flow"],
    queryFn: async () => {
      const now = new Date();
      const start = new Date(now.getFullYear(), now.getMonth() - 5, 1).toISOString();
      const end = now.toISOString();
      const data = await financeService.getCashFlow(start, end);
      return [{
        month: now.toISOString().slice(0, 7),
        income: data?.income ?? 0,
        expenses: data?.expenses ?? 0,
      }];
    },
  });
}

export function useBudgetHealth() {
  return useQuery({
    queryKey: ["dashboard", "budget-health"],
    queryFn: () => Promise.resolve([] as BudgetItem[]),
  });
}

export function useDebtSummary() {
  return useQuery({
    queryKey: ["dashboard", "debt-summary"],
    queryFn: () => debtService.getDebtSummary(),
  });
}

export function useCreditScoreData() {
  return useQuery({
    queryKey: ["dashboard", "credit-score"],
    queryFn: async () => {
      const report = await creditService.getLatestReport();
      const history = await creditService.getScoreHistory();
      return {
        score: report?.score ?? 0,
        change: 0,
        history: history.map((h) => ({
          date: h.date?.slice(0, 7) ?? "",
          score: h.score,
        })),
      };
    },
  });
}

export function useRecentTransactions() {
  return useQuery({
    queryKey: ["dashboard", "recent-transactions"],
    queryFn: () => financeService.listTransactions(),
  });
}

export function useAIInsights() {
  return useQuery({
    queryKey: ["dashboard", "ai-insights"],
    queryFn: () => Promise.resolve([]),
  });
}
