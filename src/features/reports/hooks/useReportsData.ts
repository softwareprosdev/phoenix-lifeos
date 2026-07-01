import { useQuery } from "@tanstack/react-query";
import { reportService } from "@/services";
import type { Report } from "../types";

export function useReports() {
  return useQuery({
    queryKey: ["reports"],
    queryFn: () => reportService.getSpendingTrend(12) as unknown as Promise<Report[]>,
  });
}
