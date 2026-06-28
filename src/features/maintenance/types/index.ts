export interface MaintenanceTask {
  id: string;
  title: string;
  description: string;
  category: "home" | "vehicle" | "electronics" | "other";
  frequency: "once" | "daily" | "weekly" | "monthly" | "quarterly" | "yearly";
  last_done: string;
  next_due: string;
  cost_cents: number;
}

export interface Warranty {
  id: string;
  item: string;
  provider: string;
  expires_at: string;
  cost_cents: number;
  notes: string;
}
