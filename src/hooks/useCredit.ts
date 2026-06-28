import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { creditService } from "@/services";

export const creditKeys = {
  all: ["credit"] as const,
  latest: ["credit", "latest"] as const,
  list: ["credit", "list"] as const,
  history: ["credit", "history"] as const,
};

export function useLatestCreditReport() {
  return useQuery({
    queryKey: creditKeys.latest,
    queryFn: () => creditService.getLatestReport(),
  });
}

export function useCreditReports() {
  return useQuery({
    queryKey: creditKeys.list,
    queryFn: () => creditService.listReports(),
  });
}

export function useScoreHistory() {
  return useQuery({
    queryKey: creditKeys.history,
    queryFn: () => creditService.getScoreHistory(),
  });
}

export function useCreateCreditReport() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: { score: number; bureau: string; score_model?: string }) =>
      creditService.createReport(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: creditKeys.all });
    },
  });
}
