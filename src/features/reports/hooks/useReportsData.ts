import { useQuery } from "@tanstack/react-query";
import type { Report } from "../types";

const mockReports: Report[] = [
  { id: "1", name: "June 2024 Spending Analysis", type: "spending", period: "monthly", generated_at: "2024-06-15", data_url: "#" },
  { id: "2", name: "Q2 2024 Income Summary", type: "income", period: "quarterly", generated_at: "2024-06-30", data_url: "#" },
  { id: "3", name: "Net Worth Trajectory", type: "net_worth", period: "monthly", generated_at: "2024-06-15", data_url: "#" },
  { id: "4", name: "Budget vs Actual YTD", type: "budget", period: "yearly", generated_at: "2024-06-15", data_url: "#" },
  { id: "5", name: "Tax Deduction Summary", type: "tax", period: "yearly", generated_at: "2024-06-01", data_url: "#" },
];

export function useReports() {
  return useQuery({
    queryKey: ["reports"],
    queryFn: () => Promise.resolve(mockReports),
  });
}
