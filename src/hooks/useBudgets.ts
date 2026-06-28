import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { budgetService } from "@/services";
import type { CreateBudgetInput } from "@/types";

export const budgetKeys = {
  all: ["budgets"] as const,
  detail: (id: string) => ["budgets", id] as const,
  progress: (id: string) => ["budgets", id, "progress"] as const,
  items: (budgetId: string) => ["budgets", budgetId, "items"] as const,
};

export function useBudgets() {
  return useQuery({
    queryKey: budgetKeys.all,
    queryFn: () => budgetService.listBudgets(),
  });
}

export function useBudget(id: string) {
  return useQuery({
    queryKey: budgetKeys.detail(id),
    queryFn: () => budgetService.getBudget(id),
    enabled: !!id,
  });
}

export function useBudgetWithItems(id: string) {
  return useQuery({
    queryKey: [...budgetKeys.detail(id), "withItems"],
    queryFn: () => budgetService.getBudgetWithItems(id),
    enabled: !!id,
  });
}

export function useBudgetProgress(id: string) {
  return useQuery({
    queryKey: budgetKeys.progress(id),
    queryFn: () => budgetService.getBudgetProgress(id),
    enabled: !!id,
  });
}

export function useCreateBudget() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateBudgetInput) => budgetService.createBudget(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: budgetKeys.all });
    },
  });
}

export function useUpdateBudget() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<CreateBudgetInput> }) =>
      budgetService.updateBudget(id, input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: budgetKeys.all });
      queryClient.invalidateQueries({ queryKey: budgetKeys.detail(data.id) });
    },
  });
}

export function useDeleteBudget() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => budgetService.deleteBudget(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: budgetKeys.all });
    },
  });
}

export function useCreateBudgetItem(budgetId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: { category_id: string; limit_cents: number }) =>
      budgetService.createBudgetItem(budgetId, input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: budgetKeys.items(budgetId) });
    },
  });
}

export function useDeleteBudgetItem() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => budgetService.deleteBudgetItem(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: budgetKeys.all });
    },
  });
}
