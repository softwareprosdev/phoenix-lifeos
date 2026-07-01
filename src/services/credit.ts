import { invoke } from "@tauri-apps/api/core";
import type {
  CreditReport, ScoreHistoryPoint,
  DisputeCase, DisputeLetter, DisputeLetterTemplate,
  DisputeTimelineEntry, DisputeCaseDetail, GenerateLetterInput,
  CreditAccount, CreditInquiry, CreditDerogatory,
  BusinessCreditProfile,
} from "@/types";

class CreditServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "CreditServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new CreditServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const creditService = {
  getLatestReport: () => safeInvoke<CreditReport | null>("get_latest_credit_report"),

  getReport: (id: string) => safeInvoke<CreditReport>("get_credit_report", { id }),

  listReports: (startDate?: string, endDate?: string) =>
    safeInvoke<CreditReport[]>("list_credit_reports", { startDate, endDate }),

  getScoreHistory: () => safeInvoke<ScoreHistoryPoint[]>("get_score_history"),

  createReport: (input: { score: number; bureau: string; score_model?: string }) =>
    safeInvoke<CreditReport>("create_credit_report", { payload: input }),

  deleteReport: (id: string) => safeInvoke<void>("delete_credit_report", { id }),

  listDisputeCases: () => safeInvoke<DisputeCase[]>("list_dispute_cases"),

  createDisputeCase: (input: Partial<DisputeCase>) =>
    safeInvoke<DisputeCase>("create_dispute_case", { payload: input }),

  getDisputeCase: (id: string) => safeInvoke<DisputeCaseDetail>("get_dispute_case", { id }),

  updateDisputeCase: (id: string, input: Partial<DisputeCase>) =>
    safeInvoke<DisputeCase>("update_dispute_case", { id, payload: input }),

  listDisputeLetters: (caseId: string) =>
    safeInvoke<DisputeLetter[]>("list_dispute_letters", { caseId }),

  generateLetter: (input: GenerateLetterInput) =>
    safeInvoke<DisputeLetter>("generate_letter", { payload: input }),

  listLetterTemplates: () => safeInvoke<DisputeLetterTemplate[]>("list_letter_templates"),

  getLetterTemplateByCode: (code: string) =>
    safeInvoke<DisputeLetterTemplate | null>("get_letter_template_by_code", { code }),

  markLetterSent: (letterId: string, trackingNumber: string) =>
    safeInvoke<DisputeLetter>("mark_letter_sent", { letterId, trackingNumber }),

  addTimelineEntry: (caseId: string, eventType: string, description: string) =>
    safeInvoke<DisputeTimelineEntry>("add_timeline_entry", { caseId, eventType, description }),

  listCreditAccounts: () => safeInvoke<CreditAccount[]>("list_credit_accounts"),

  createCreditAccount: (input: Partial<CreditAccount>) =>
    safeInvoke<CreditAccount>("create_credit_account", { payload: input }),

  listCreditInquiries: () => safeInvoke<CreditInquiry[]>("list_credit_inquiries"),

  listDerogatoryItems: () => safeInvoke<CreditDerogatory[]>("list_derogatory_items"),

  getBusinessCreditProfiles: () => safeInvoke<BusinessCreditProfile[]>("get_business_credit_profiles"),
};
