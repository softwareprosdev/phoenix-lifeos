export interface CreditReport {
  id: string;
  score: number;
  score_model: string;
  bureau: string;
  utilization_rate: string;
  on_time_rate: string;
  derogatory_marks: number;
  total_accounts: number;
  open_accounts: number;
  hard_inquiries_12m: number;
  report_date: string;
  created_at: string;
}

export interface ScoreHistoryPoint {
  date: string;
  score: number;
  bureau: string;
}
