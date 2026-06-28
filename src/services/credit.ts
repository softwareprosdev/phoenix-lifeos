import { invoke } from "@tauri-apps/api/core";
import type { CreditReport, ScoreHistoryPoint } from "@/types";

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
};
