export interface Debt {
  id: string;
  name: string;
  balance_cents: number;
  apr: number;
  min_payment_cents: number;
  original_balance_cents: number;
  creditor: string;
  due_date: string;
  created_at: string;
}

export interface PayoffProjection {
  month: string;
  totalDebt: number;
}
