export interface Goal {
  id: string;
  name: string;
  target_amount_cents: number;
  current_amount_cents: number;
  deadline: string;
  category: "savings" | "investment" | "debt" | "purchase" | "emergency" | "retirement";
  color: string;
  created_at: string;
}
