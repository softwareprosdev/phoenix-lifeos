export interface Account {
  id: string;
  name: string;
  account_type: AccountType;
  institution?: string;
  balance_cents: number;
  currency: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export type AccountType =
  | "checking"
  | "savings"
  | "credit"
  | "loan"
  | "cash"
  | "investment"
  | "business";

export interface Transaction {
  id: string;
  account_id: string;
  amount_cents: number;
  currency: string;
  transaction_type: TransactionType;
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

export type TransactionType = "income" | "expense" | "transfer";

export interface Category {
  id: string;
  name: string;
  type: "income" | "expense";
  parent_id?: string;
  created_at: string;
}

export interface CreateAccountInput {
  name: string;
  account_type: AccountType;
  institution?: string;
  balance_cents: number;
  currency?: string;
}

export interface CreateTransactionInput {
  account_id: string;
  amount_cents: number;
  currency?: string;
  transaction_type: TransactionType;
  category_id?: string;
  merchant?: string;
  description?: string;
  date: string;
  tags?: string[];
  is_recurring?: boolean;
}

export interface SpendingByCategory {
  category_id: string;
  category_name: string;
  total_cents: number;
  transaction_count: number;
}

export interface CashFlow {
  income: number;
  expenses: number;
  net: number;
}
