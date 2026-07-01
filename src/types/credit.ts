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

export interface CreditAccount {
  id: string;
  creditor_name: string;
  account_type: string;
  account_number_masked: string;
  bureau: string;
  opened_date: string;
  reported_date: string;
  credit_limit_cents: number;
  balance_cents: number;
  past_due_cents: number;
  payment_status: string;
  account_status: string;
  remarks: string;
  created_at: string;
}

export interface CreditInquiry {
  id: string;
  bureau: string;
  inquiry_date: string;
  creditor_name: string;
  inquiry_type: string;
  created_at: string;
}

export interface CreditDerogatory {
  id: string;
  bureau: string;
  derogatory_type: string;
  creditor_name: string;
  account_number_masked: string;
  date_reported: string;
  date_expected_removal: string;
  description: string;
  created_at: string;
}

export interface DisputeLetterTemplate {
  id: string;
  template_code: string;
  name: string;
  description: string;
  legal_basis: string;
  recipient_type: string;
  variable_schema: Record<string, string>;
  default_content: string;
  created_at: string;
}

export interface NegativeItem {
  id: string;
  creditor_name: string;
  bureau: string;
  account_number_masked: string;
  negative_type: string;
  date_reported: string;
  amount_cents: number;
  description: string;
  created_at: string;
}

export interface DisputeCase {
  id: string;
  case_number: string;
  dispute_type: string;
  status: string;
  creditor_name: string;
  bureau: string;
  reason: string;
  notes: string;
  created_at: string;
  updated_at: string;
  won_at: string | null;
}

export interface DisputeLetter {
  id: string;
  case_id: string;
  template_id: string;
  template_code: string;
  recipient_name: string;
  recipient_address: string;
  content: string;
  tracking_number: string | null;
  sent_at: string | null;
  created_at: string;
}

export interface DisputeTimelineEntry {
  id: string;
  case_id: string;
  event_type: string;
  description: string;
  created_at: string;
}

export interface BusinessCreditProfile {
  id: string;
  business_name: string;
  ein: string;
  industry: string;
  business_credit_score: number;
  paydex_score: number | null;
  credit_limit_cents: number;
  data_provider: string;
  created_at: string;
  updated_at: string;
}

export interface VendorCreditAccount {
  id: string;
  business_profile_id: string;
  vendor_name: string;
  account_type: string;
  account_number_masked: string;
  credit_limit_cents: number;
  balance_cents: number;
  payment_terms: string;
  payment_status: string;
  created_at: string;
}

export interface DisputeCaseDetail {
  case: DisputeCase;
  letters: DisputeLetter[];
  timeline: DisputeTimelineEntry[];
  negative_item: NegativeItem | null;
}

export interface GenerateLetterInput {
  case_id: string;
  template_id: string;
  variables: Record<string, string>;
}
