import { useQuery } from "@tanstack/react-query";
import type { Goal } from "../types";

const mockGoals: Goal[] = [
  { id: "1", name: "Emergency Fund", target_amount_cents: 5000000, current_amount_cents: 3500000, deadline: "2024-12-31", category: "emergency", color: "hsl(220, 90%, 56%)", created_at: "2024-01-01" },
  { id: "2", name: "New Car", target_amount_cents: 3000000, current_amount_cents: 1000000, deadline: "2025-06-30", category: "purchase", color: "hsl(142, 72%, 45%)", created_at: "2024-01-01" },
  { id: "3", name: "Retirement", target_amount_cents: 50000000, current_amount_cents: 12500000, deadline: "2050-12-31", category: "retirement", color: "hsl(38, 92%, 50%)", created_at: "2024-01-01" },
  { id: "4", name: "Vacation", target_amount_cents: 800000, current_amount_cents: 350000, deadline: "2024-09-01", category: "savings", color: "hsl(280, 80%, 60%)", created_at: "2024-02-01" },
  { id: "5", name: "Debt Free", target_amount_cents: 5340000, current_amount_cents: 3800000, deadline: "2025-12-31", category: "debt", color: "hsl(0, 84%, 60%)", created_at: "2024-01-01" },
];

export function useGoals() {
  return useQuery({
    queryKey: ["goals"],
    queryFn: () => Promise.resolve(mockGoals),
  });
}
