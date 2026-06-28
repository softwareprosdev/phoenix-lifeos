export interface Goal {
  id: string;
  name: string;
  goal_type: GoalType;
  target_amount_cents: number;
  current_amount_cents: number;
  monthly_contribution_cents: number;
  currency: string;
  deadline?: string;
  linked_account_id?: string;
  created_at: string;
  updated_at: string;
}

export type GoalType =
  | "savings"
  | "emergency_fund"
  | "home_purchase"
  | "vehicle"
  | "retirement"
  | "education"
  | "vacation"
  | "debt_payoff"
  | "custom";

export interface CreateGoalInput {
  name: string;
  goal_type: GoalType;
  target_amount_cents: number;
  current_amount_cents?: number;
  monthly_contribution_cents?: number;
  currency?: string;
  deadline?: string;
  linked_account_id?: string;
}
