export interface Budget {
  id: string;
  category: string;
  limit_cents: number;
  spent_cents: number;
  period: "weekly" | "monthly" | "yearly";
  color: string;
  created_at: string;
}

export interface BudgetCategoryBreakdown {
  category: string;
  spent: number;
  limit: number;
  items: { name: string; amount: number; date: string }[];
}
