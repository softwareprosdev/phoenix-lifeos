import { invoke } from "@tauri-apps/api/core";
import type { SpendingByCategory, CashFlow } from "@/types";

class ReportServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "ReportServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new ReportServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export interface IncomeVsExpenses {
  income_cents: number;
  expense_cents: number;
  net_cents: number;
}

export interface MonthlyReport {
  month: string;
  income_cents: number;
  expense_cents: number;
  net_cents: number;
}

export const reportService = {
  getIncomeVsExpenses: (startDate: string, endDate: string) =>
    safeInvoke<IncomeVsExpenses>("get_income_vs_expenses", { startDate, endDate }),

  getMonthlyReport: (year: number) =>
    safeInvoke<MonthlyReport[]>("get_monthly_report", { year }),

  getSpendingByCategory: (accountId: string, startDate: string, endDate: string) =>
    safeInvoke<SpendingByCategory[]>("get_spending_by_category", {
      accountId,
      startDate,
      endDate,
    }),

  getCashFlow: (startDate: string, endDate: string) =>
    safeInvoke<CashFlow>("get_cash_flow", { startDate, endDate }),

  getNetWorth: () => safeInvoke<number>("get_net_worth"),

  getSpendingTrend: (months: number) =>
    safeInvoke<MonthlyReport[]>("get_spending_trend", { months }),
};
