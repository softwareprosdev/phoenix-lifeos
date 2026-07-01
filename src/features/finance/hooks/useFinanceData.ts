import { useQuery } from "@tanstack/react-query";
import { financeService } from "@/services";

export function useAccounts() {
  return useQuery({
    queryKey: ["accounts"],
    queryFn: () => financeService.listAccounts(),
  });
}

export function useTransactions(accountId?: string) {
  return useQuery({
    queryKey: ["transactions", accountId],
    queryFn: () => {
      if (accountId) {
        return financeService.listTransactions(accountId);
      }
      return financeService.listTransactions();
    },
  });
}
