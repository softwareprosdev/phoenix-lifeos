import { useQuery } from "@tanstack/react-query";
import { financeService, reportService } from "@/services";
import { useAccounts } from "./useAccounts";
import { useDebtSummary } from "./useDebts";
import { useLatestCreditReport } from "./useCredit";
import { useActiveGoals } from "./useGoals";
import { accountKeys } from "./useAccounts";
import { startOfMonth, endOfMonth, format } from "date-fns";

export function useDashboardData() {
  const now = new Date();
  const monthStart = format(startOfMonth(now), "yyyy-MM-dd'T'HH:mm:ss'Z'");
  const monthEnd = format(endOfMonth(now), "yyyy-MM-dd'T'HH:mm:ss'Z'");

  const accounts = useAccounts();
  const debtsSummary = useDebtSummary();
  const creditReport = useLatestCreditReport();
  const activeGoals = useActiveGoals();

  const netWorth = useQuery({
    queryKey: [...accountKeys.all, "netWorth"],
    queryFn: () => financeService.getNetWorth(),
  });

  const cashFlow = useQuery({
    queryKey: ["cashflow", monthStart, monthEnd],
    queryFn: () => reportService.getCashFlow(monthStart, monthEnd),
  });

  const spendingByCategory = useQuery({
    queryKey: ["spendingByCategory", monthStart, monthEnd],
    queryFn: () => reportService.getSpendingByCategory("", monthStart, monthEnd),
    enabled: accounts.data !== undefined && accounts.data.length > 0,
  });

  return {
    accounts,
    netWorth,
    cashFlow,
    spendingByCategory,
    debtsSummary,
    creditReport,
    activeGoals,
    isLoading:
      accounts.isLoading ||
      netWorth.isLoading ||
      cashFlow.isLoading,
    isError:
      accounts.isError ||
      netWorth.isError ||
      cashFlow.isError,
  };
}
