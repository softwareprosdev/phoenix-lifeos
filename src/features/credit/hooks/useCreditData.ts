import { useQuery } from "@tanstack/react-query";
import type { CreditAccount, CreditScoreHistory, ScoreFactors } from "../types";

const mockAccounts: CreditAccount[] = [
  { id: "1", name: "Amex Platinum", credit_limit_cents: 2000000, balance_cents: 340000, apr: 22.99, issuer: "American Express", is_active: true },
  { id: "2", name: "Chase Sapphire", credit_limit_cents: 1500000, balance_cents: 0, apr: 21.99, issuer: "Chase", is_active: true },
  { id: "3", name: "Citi Double Cash", credit_limit_cents: 1000000, balance_cents: 125000, apr: 20.49, issuer: "Citi", is_active: true },
];

const mockScoreHistory: CreditScoreHistory[] = [
  { date: "Jan", score: 718 },
  { date: "Feb", score: 725 },
  { date: "Mar", score: 730 },
  { date: "Apr", score: 735 },
  { date: "May", score: 738 },
  { date: "Jun", score: 742 },
];

const mockScoreFactors: ScoreFactors = {
  utilization: { value: 18, status: "good" },
  paymentHistory: { value: 100, status: "good" },
  ageOfAccounts: { value: 5, status: "fair" },
  inquiries: { value: 2, status: "good" },
};

export function useCreditAccounts() {
  return useQuery({
    queryKey: ["credit-accounts"],
    queryFn: () => Promise.resolve(mockAccounts),
  });
}

export function useCreditScore() {
  return useQuery({
    queryKey: ["credit-score"],
    queryFn: () => Promise.resolve({ score: 742, factors: mockScoreFactors, history: mockScoreHistory }),
  });
}
