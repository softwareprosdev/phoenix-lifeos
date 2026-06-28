import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { goalService } from "@/services";
import type { CreateGoalInput } from "@/types";

export const goalKeys = {
  all: ["goals"] as const,
  detail: (id: string) => ["goals", id] as const,
  active: ["goals", "active"] as const,
  byType: (goalType: string) => ["goals", "type", goalType] as const,
};

export function useGoals() {
  return useQuery({
    queryKey: goalKeys.all,
    queryFn: () => goalService.listGoals(),
  });
}

export function useGoal(id: string) {
  return useQuery({
    queryKey: goalKeys.detail(id),
    queryFn: () => goalService.getGoal(id),
    enabled: !!id,
  });
}

export function useActiveGoals() {
  return useQuery({
    queryKey: goalKeys.active,
    queryFn: () => goalService.listActiveGoals(),
  });
}

export function useCreateGoal() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateGoalInput) => goalService.createGoal(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: goalKeys.all });
    },
  });
}

export function useUpdateGoal() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<CreateGoalInput> }) =>
      goalService.updateGoal(id, input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: goalKeys.all });
      queryClient.invalidateQueries({ queryKey: goalKeys.detail(data.id) });
    },
  });
}

export function useDeleteGoal() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => goalService.deleteGoal(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: goalKeys.all });
    },
  });
}
