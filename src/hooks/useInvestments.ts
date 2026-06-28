import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { investmentService } from "@/services";
import type { CreateInvestmentInput } from "@/types";

export const investmentKeys = {
  all: ["investments"] as const,
  list: (accountId?: string) => ["investments", "list", accountId] as const,
  detail: (id: string) => ["investments", id] as const,
};

export function useInvestments(accountId?: string) {
  return useQuery({
    queryKey: investmentKeys.list(accountId),
    queryFn: () => investmentService.listInvestments(accountId),
  });
}

export function useInvestment(id: string) {
  return useQuery({
    queryKey: investmentKeys.detail(id),
    queryFn: () => investmentService.getInvestment(id),
    enabled: !!id,
  });
}

export function useCreateInvestment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateInvestmentInput) => investmentService.createInvestment(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: investmentKeys.all });
    },
  });
}

export function useUpdateInvestment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<CreateInvestmentInput> }) =>
      investmentService.updateInvestment(id, input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: investmentKeys.all });
      queryClient.invalidateQueries({ queryKey: investmentKeys.detail(data.id) });
    },
  });
}

export function useDeleteInvestment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => investmentService.deleteInvestment(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: investmentKeys.all });
    },
  });
}
