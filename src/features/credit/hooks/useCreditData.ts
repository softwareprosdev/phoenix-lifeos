import { useQuery } from "@tanstack/react-query";
import { creditService } from "@/services";

export function useCreditAccounts() {
  return useQuery({
    queryKey: ["credit-accounts"],
    queryFn: () => creditService.listCreditAccounts(),
  });
}

interface CreditScoreFactors {
  utilization: { value: number; status: string }
  paymentHistory: { value: number; status: string }
  ageOfAccounts: { value: number; status: string }
  inquiries: { value: number; status: string }
}

export function useCreditScore() {
  return useQuery({
    queryKey: ["credit-score"],
    queryFn: async () => {
      const latest = await creditService.getLatestReport();
      const history = await creditService.getScoreHistory();
      return {
        score: latest?.score ?? 0,
        history,
        factors: {
          utilization: { value: 35, status: "fair" },
          paymentHistory: { value: 97, status: "good" },
          ageOfAccounts: { value: 8, status: "good" },
          inquiries: { value: 2, status: "good" },
        } as CreditScoreFactors,
      };
    },
  });
}
