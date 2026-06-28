export interface BusinessAccount {
  id: string;
  name: string;
  type: "checking" | "savings" | "credit" | "merchant";
  balance_cents: number;
  institution: string;
}

export interface Invoice {
  id: string;
  number: string;
  client: string;
  amount_cents: number;
  status: "draft" | "sent" | "paid" | "overdue";
  due_date: string;
}
