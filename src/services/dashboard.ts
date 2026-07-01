import { financeService } from "./finance";
import { debtService } from "./debt";
import { creditService } from "./credit";

export const dashboardService = {
  getNetWorth: () => financeService.getNetWorth(),

  getCashFlow: (startDate: string, endDate: string) =>
    financeService.getCashFlow(startDate, endDate),

  getDebtSummary: () => debtService.getDebtSummary(),

  getCreditScore: () => creditService.getLatestReport(),

  getRecentTransactions: (_limit = 10) =>
    financeService.listTransactions(),
};
