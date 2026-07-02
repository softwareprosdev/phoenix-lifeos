import { safeInvoke } from "./commandClient";
import type { SpendingByCategory, CashFlow } from "@/types";


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
    safeInvoke<IncomeVsExpenses>("Reports", "get_income_vs_expenses", { startDate, endDate }),

  getMonthlyReport: (year: number) =>
    safeInvoke<MonthlyReport[]>("Reports", "get_monthly_report", { year }),

  getSpendingByCategory: (accountId: string, startDate: string, endDate: string) =>
    safeInvoke<SpendingByCategory[]>("Reports", "get_spending_by_category", {
      accountId,
      startDate,
      endDate,
    }),

  getCashFlow: (startDate: string, endDate: string) =>
    safeInvoke<CashFlow>("Reports", "get_cash_flow", { startDate, endDate }),

  getNetWorth: () => safeInvoke<number>("Reports", "get_net_worth"),

  getSpendingTrend: (months: number) =>
    safeInvoke<MonthlyReport[]>("Reports", "get_spending_trend", { months }),
};
