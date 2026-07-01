export interface Account {
  id: string;
  name: string;
  account_type: "checking" | "savings" | "credit" | "investment" | "loan" | "cash" | "business";
  institution?: string;
  balance_cents: number;
  currency: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Transaction {
  id: string;
  account_id: string;
  amount_cents: number;
  currency: string;
  transaction_type: "income" | "expense" | "transfer";
  category_id?: string;
  merchant?: string;
  description?: string;
  date: string;
  tags: string[];
  is_recurring: boolean;
  created_at: string;
  updated_at: string;
  deleted_at?: string;
}

export interface TransactionFilters {
  dateFrom: string;
  dateTo: string;
  category: string;
  type: "" | "income" | "expense";
  search: string;
}
