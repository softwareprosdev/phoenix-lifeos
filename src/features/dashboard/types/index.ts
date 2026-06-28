export interface WidgetConfig {
  id: string;
  title: string;
  type: string;
  w: number;
  h: number;
}

export interface NetWorthDataPoint {
  date: string;
  value: number;
}

export interface CashFlowItem {
  month: string;
  income: number;
  expenses: number;
}

export interface BudgetHealth {
  category: string;
  spent: number;
  limit: number;
}

export interface DebtSummary {
  totalDebt: number;
  totalMinPayments: number;
  avgApr: number;
  debtCount: number;
}

export interface CreditScoreData {
  score: number;
  change: number;
  history: { date: string; score: number }[];
}

export interface RecentTransaction {
  id: string;
  date: string;
  description: string;
  merchant: string;
  category: string;
  amountCents: number;
  type: "income" | "expense";
}

export interface AIInsight {
  id: string;
  type: "tip" | "alert" | "achievement";
  message: string;
  timestamp: string;
}
