import { useQuery } from "@tanstack/react-query";
import { goalService } from "@/services";

export function useGoals() {
  return useQuery({
    queryKey: ["goals"],
    queryFn: () => goalService.listGoals(),
  });
}
