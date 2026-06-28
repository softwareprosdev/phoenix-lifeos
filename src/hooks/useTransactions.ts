import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { financeService } from "@/services";
import type { CreateTransactionInput } from "@/types";

export const transactionKeys = {
  all: ["transactions"] as const,
  list: (accountId?: string) => ["transactions", "list", accountId] as const,
  detail: (id: string) => ["transactions", id] as const,
  search: (query: string) => ["transactions", "search", query] as const,
};

export function useTransactions(accountId?: string) {
  return useQuery({
    queryKey: transactionKeys.list(accountId),
    queryFn: () => financeService.listTransactions(accountId),
  });
}

export function useTransaction(id: string) {
  return useQuery({
    queryKey: transactionKeys.detail(id),
    queryFn: () => financeService.getTransaction(id),
    enabled: !!id,
  });
}

export function useCreateTransaction() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateTransactionInput) => financeService.createTransaction(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: transactionKeys.all });
    },
  });
}

export function useUpdateTransaction() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<CreateTransactionInput> }) =>
      financeService.updateTransaction(id, input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: transactionKeys.all });
      queryClient.invalidateQueries({ queryKey: transactionKeys.detail(data.id) });
    },
  });
}

export function useDeleteTransaction() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => financeService.deleteTransaction(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: transactionKeys.all });
    },
  });
}

export function useSearchTransactions(query: string) {
  return useQuery({
    queryKey: transactionKeys.search(query),
    queryFn: () => financeService.searchTransactions(query),
    enabled: query.length > 0,
  });
}
