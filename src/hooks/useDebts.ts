import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { debtService } from "@/services";
import type { CreateDebtInput, DebtPaymentInput } from "@/types";

export const debtKeys = {
  all: ["debts"] as const,
  detail: (id: string) => ["debts", id] as const,
  summary: ["debts", "summary"] as const,
  payoffSchedule: (debtId: string, extra?: number) => ["debts", debtId, "payoff", extra] as const,
};

export function useDebts() {
  return useQuery({
    queryKey: debtKeys.all,
    queryFn: () => debtService.listDebts(),
  });
}

export function useDebt(id: string) {
  return useQuery({
    queryKey: debtKeys.detail(id),
    queryFn: () => debtService.getDebt(id),
    enabled: !!id,
  });
}

export function useDebtSummary() {
  return useQuery({
    queryKey: debtKeys.summary,
    queryFn: () => debtService.getDebtSummary(),
  });
}

export function usePayoffSchedule(debtId: string, extraPaymentCents?: number) {
  return useQuery({
    queryKey: debtKeys.payoffSchedule(debtId, extraPaymentCents),
    queryFn: () => debtService.getPayoffSchedule(debtId, extraPaymentCents),
    enabled: !!debtId,
  });
}

export function useCreateDebt() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateDebtInput) => debtService.createDebt(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: debtKeys.all });
      queryClient.invalidateQueries({ queryKey: debtKeys.summary });
    },
  });
}

export function useUpdateDebt() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<CreateDebtInput> }) =>
      debtService.updateDebt(id, input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: debtKeys.all });
      queryClient.invalidateQueries({ queryKey: debtKeys.detail(data.id) });
      queryClient.invalidateQueries({ queryKey: debtKeys.summary });
    },
  });
}

export function useDeleteDebt() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => debtService.deleteDebt(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: debtKeys.all });
      queryClient.invalidateQueries({ queryKey: debtKeys.summary });
    },
  });
}

export function useMakeDebtPayment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: DebtPaymentInput) => debtService.makePayment(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: debtKeys.all });
      queryClient.invalidateQueries({ queryKey: debtKeys.summary });
    },
  });
}
