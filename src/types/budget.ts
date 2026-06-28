export interface Budget {
  id: string;
  name: string;
  period: "weekly" | "monthly" | "yearly";
  total_limit_cents: number;
  currency: string;
  start_date: string;
  end_date: string;
  created_at: string;
}

export interface BudgetItem {
  id: string;
  budget_id: string;
  category_id?: string;
  limit_cents: number;
  spent_cents: number;
}

export interface BudgetWithItems extends Budget {
  items: BudgetItem[];
}

export interface CreateBudgetInput {
  name: string;
  period: string;
  total_limit_cents: number;
  start_date: string;
  end_date: string;
  items: { category_id: string; limit_cents: number }[];
}

export interface BudgetProgress {
  budget: BudgetWithItems;
  total_spent_cents: number;
  percent_used: number;
  is_over_budget: boolean;
}
