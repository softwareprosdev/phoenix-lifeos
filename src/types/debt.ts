export interface Debt {
  id: string;
  name: string;
  debt_type: DebtType;
  principal_cents: number;
  remaining_balance_cents: number;
  interest_rate: string;
  minimum_payment_cents: number;
  currency: string;
  due_date?: string;
  creditor?: string;
  created_at: string;
  updated_at: string;
}

export type DebtType =
  | "credit_card"
  | "student_loan"
  | "personal_loan"
  | "medical"
  | "auto"
  | "other";

export interface CreateDebtInput {
  name: string;
  debt_type: DebtType;
  principal_cents: number;
  interest_rate: string;
  minimum_payment_cents: number;
  currency?: string;
  due_date?: string;
  creditor?: string;
}

export interface PayoffSchedule {
  month: number;
  payment_cents: number;
  principal_cents: number;
  interest_cents: number;
  remaining_cents: number;
}

export interface DebtSummary {
  total_balance: number;
  total_min_payments: number;
  debt_count: number;
  avg_apr: number;
}

export interface DebtPaymentInput {
  debt_id: string;
  amount_cents: number;
  date: string;
}
