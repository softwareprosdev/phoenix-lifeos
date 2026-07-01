import { useQuery } from "@tanstack/react-query";
import { investmentService } from "@/services";

export function useInvestments() {
  return useQuery({
    queryKey: ["investments"],
    queryFn: () => investmentService.listInvestments(),
  });
}

interface AllocationEntry {
  name: string;
  value: number;
  color: string;
}

export function usePortfolioSummary() {
  return useQuery({
    queryKey: ["portfolio-summary"],
    queryFn: async () => {
      const investments = await investmentService.listInvestments();
      const totalValueCents = investments.reduce((s, i) => s + (i.quantity ? Math.round(Number(i.quantity) * i.current_value_cents) : 0), 0);
      const totalCostCents = investments.reduce((s, i) => s + i.cost_basis_cents, 0);
      const totalGainCents = totalValueCents - totalCostCents;
      const totalGainPercent = totalCostCents > 0 ? Math.round((totalGainCents / totalCostCents) * 100) : 0;
      const allocation: AllocationEntry[] = [];

      return {
        totalValueCents,
        totalCostCents,
        totalGainCents,
        totalGainPercent,
        allocation,
      };
    },
  });
}
