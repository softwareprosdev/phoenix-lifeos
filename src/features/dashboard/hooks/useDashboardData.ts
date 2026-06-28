import { useQuery } from "@tanstack/react-query";
import type { NetWorthDataPoint, CashFlowItem, BudgetHealth, DebtSummary, CreditScoreData, RecentTransaction, AIInsight, WidgetConfig } from "../types";

const mockWidgets: WidgetConfig[] = [
  { id: "net-worth", title: "Net Worth", type: "net-worth", w: 2, h: 1 },
  { id: "cash-flow", title: "Cash Flow", type: "cash-flow", w: 1, h: 1 },
  { id: "budget-health", title: "Budget Health", type: "budget-health", w: 1, h: 1 },
  { id: "debt-overview", title: "Debt Overview", type: "debt-overview", w: 1, h: 1 },
  { id: "credit-score", title: "Credit Score", type: "credit-score", w: 1, h: 1 },
  { id: "recent-transactions", title: "Recent Transactions", type: "recent-transactions", w: 2, h: 1 },
  { id: "ai-insights", title: "AI Insights", type: "ai-insights", w: 1, h: 1 },
];

const mockNetWorth: NetWorthDataPoint[] = [
  { date: "Jan", value: 12500000 },
  { date: "Feb", value: 12750000 },
  { date: "Mar", value: 13200000 },
  { date: "Apr", value: 12950000 },
  { date: "May", value: 13500000 },
  { date: "Jun", value: 14200000 },
];

const mockCashFlow: CashFlowItem[] = [
  { month: "Jan", income: 850000, expenses: 520000 },
  { month: "Feb", income: 850000, expenses: 490000 },
  { month: "Mar", income: 920000, expenses: 560000 },
  { month: "Apr", income: 850000, expenses: 510000 },
  { month: "May", income: 880000, expenses: 540000 },
  { month: "Jun", income: 950000, expenses: 530000 },
];

const mockBudgetHealth: BudgetHealth[] = [
  { category: "Housing", spent: 1800, limit: 2000 },
  { category: "Food", spent: 650, limit: 600 },
  { category: "Transport", spent: 320, limit: 400 },
  { category: "Entertainment", spent: 180, limit: 200 },
  { category: "Utilities", spent: 280, limit: 300 },
];

const mockDebtSummary: DebtSummary = {
  totalDebt: 45200000,
  totalMinPayments: 125000,
  avgApr: 18.5,
  debtCount: 4,
};

const mockCreditScore: CreditScoreData = {
  score: 742,
  change: 12,
  history: [
    { date: "Jan", score: 718 },
    { date: "Feb", score: 725 },
    { date: "Mar", score: 730 },
    { date: "Apr", score: 735 },
    { date: "May", score: 738 },
    { date: "Jun", score: 742 },
  ],
};

const mockRecentTransactions: RecentTransaction[] = [
  { id: "1", date: "2024-06-15", description: "Grocery Store", merchant: "Whole Foods", category: "Food", amountCents: -8500, type: "expense" },
  { id: "2", date: "2024-06-14", description: "Salary Deposit", merchant: "Employer Inc", category: "Income", amountCents: 425000, type: "income" },
  { id: "3", date: "2024-06-13", description: "Netflix Subscription", merchant: "Netflix", category: "Entertainment", amountCents: -1549, type: "expense" },
  { id: "4", date: "2024-06-12", description: "Gas Station", merchant: "Shell", category: "Transport", amountCents: -4800, type: "expense" },
  { id: "5", date: "2024-06-11", description: "Freelance Payment", merchant: "Client XYZ", category: "Income", amountCents: 150000, type: "income" },
];

const mockAIInsights: AIInsight[] = [
  { id: "1", type: "tip", message: "You could save $120/month by reducing dining out", timestamp: "2024-06-15" },
  { id: "2", type: "alert", message: "Credit card payment due in 3 days", timestamp: "2024-06-15" },
  { id: "3", type: "achievement", message: "Net worth up 13.6% this year! 🎉", timestamp: "2024-06-15" },
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
    queryFn: () => Promise.resolve(mockNetWorth),
  });
}

export function useCashFlow() {
  return useQuery({
    queryKey: ["dashboard", "cash-flow"],
    queryFn: () => Promise.resolve(mockCashFlow),
  });
}

export function useBudgetHealth() {
  return useQuery({
    queryKey: ["dashboard", "budget-health"],
    queryFn: () => Promise.resolve(mockBudgetHealth),
  });
}

export function useDebtSummary() {
  return useQuery({
    queryKey: ["dashboard", "debt-summary"],
    queryFn: () => Promise.resolve(mockDebtSummary),
  });
}

export function useCreditScoreData() {
  return useQuery({
    queryKey: ["dashboard", "credit-score"],
    queryFn: () => Promise.resolve(mockCreditScore),
  });
}

export function useRecentTransactions() {
  return useQuery({
    queryKey: ["dashboard", "recent-transactions"],
    queryFn: () => Promise.resolve(mockRecentTransactions),
  });
}

export function useAIInsights() {
  return useQuery({
    queryKey: ["dashboard", "ai-insights"],
    queryFn: () => Promise.resolve(mockAIInsights),
  });
}
