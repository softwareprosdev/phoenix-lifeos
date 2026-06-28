import { useQuery } from "@tanstack/react-query";
import type { Debt, PayoffProjection } from "../types";

const mockDebts: Debt[] = [
  { id: "1", name: "Student Loan", balance_cents: 2400000, apr: 5.5, min_payment_cents: 25000, original_balance_cents: 3000000, creditor: "Sallie Mae", due_date: "2024-07-15", created_at: "2024-01-01" },
  { id: "2", name: "Credit Card", balance_cents: 340000, apr: 22.99, min_payment_cents: 12000, original_balance_cents: 500000, creditor: "Amex", due_date: "2024-07-10", created_at: "2024-01-01" },
  { id: "3", name: "Car Loan", balance_cents: 1850000, apr: 4.2, min_payment_cents: 38000, original_balance_cents: 2500000, creditor: "Toyota Finance", due_date: "2024-07-20", created_at: "2024-01-01" },
  { id: "4", name: "Personal Loan", balance_cents: 750000, apr: 9.99, min_payment_cents: 18000, original_balance_cents: 1000000, creditor: "SoFi", due_date: "2024-07-05", created_at: "2024-02-01" },
];

const mockSnowballProjection: PayoffProjection[] = [
  { month: "Month 0", totalDebt: 5340000 },
  { month: "Month 3", totalDebt: 4800000 },
  { month: "Month 6", totalDebt: 4200000 },
  { month: "Month 9", totalDebt: 3500000 },
  { month: "Month 12", totalDebt: 2800000 },
  { month: "Month 15", totalDebt: 2000000 },
  { month: "Month 18", totalDebt: 1200000 },
  { month: "Month 21", totalDebt: 500000 },
  { month: "Month 24", totalDebt: 0 },
];

const mockAvalancheProjection: PayoffProjection[] = [
  { month: "Month 0", totalDebt: 5340000 },
  { month: "Month 3", totalDebt: 4900000 },
  { month: "Month 6", totalDebt: 4400000 },
  { month: "Month 9", totalDebt: 3800000 },
  { month: "Month 12", totalDebt: 3100000 },
  { month: "Month 15", totalDebt: 2300000 },
  { month: "Month 18", totalDebt: 1500000 },
  { month: "Month 21", totalDebt: 700000 },
  { month: "Month 24", totalDebt: 0 },
];

export function useDebts() {
  return useQuery({
    queryKey: ["debts"],
    queryFn: () => Promise.resolve(mockDebts),
  });
}

export function usePayoffProjection(method: "snowball" | "avalanche") {
  return useQuery({
    queryKey: ["debt-projection", method],
    queryFn: () => Promise.resolve(method === "snowball" ? mockSnowballProjection : mockAvalancheProjection),
  });
}
