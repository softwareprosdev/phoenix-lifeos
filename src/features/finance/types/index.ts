export interface Account {
  id: string;
  name: string;
  account_type: "checking" | "savings" | "credit" | "investment" | "loan" | "cash";
  institution: string;
  balance_cents: number;
  currency: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Transaction {
  id: string;
  account_id: string;
  date: string;
  description: string;
  merchant: string;
  category: string;
  amount_cents: number;
  type: "income" | "expense";
  is_recurring: boolean;
  created_at: string;
}

export interface TransactionFilters {
  dateFrom: string;
  dateTo: string;
  category: string;
  type: "" | "income" | "expense";
  search: string;
}
