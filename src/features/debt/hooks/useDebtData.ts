import { useQuery } from "@tanstack/react-query";
import { debtService } from "@/services";

export function useDebts() {
  return useQuery({
    queryKey: ["debts"],
    queryFn: () => debtService.listDebts(),
  });
}

export function usePayoffProjection(method: "snowball" | "avalanche") {
  return useQuery({
    queryKey: ["debt-projection", method],
    queryFn: () => debtService.getPayoffSchedule("", method === "snowball" ? 0 : 0),
  });
}
