export interface CreditAccount {
  id: string;
  name: string;
  credit_limit_cents: number;
  balance_cents: number;
  apr: number;
  issuer: string;
  is_active: boolean;
}

export interface CreditScoreHistory {
  date: string;
  score: number;
}

export interface ScoreFactors {
  utilization: { value: number; status: "good" | "fair" | "poor" };
  paymentHistory: { value: number; status: "good" | "fair" | "poor" };
  ageOfAccounts: { value: number; status: "good" | "fair" | "poor" };
  inquiries: { value: number; status: "good" | "fair" | "poor" };
}
