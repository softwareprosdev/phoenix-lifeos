export interface Report {
  id: string;
  name: string;
  type: "spending" | "income" | "net_worth" | "budget" | "tax" | "custom";
  period: "monthly" | "quarterly" | "yearly";
  generated_at: string;
  data_url: string;
}
