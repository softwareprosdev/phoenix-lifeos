import { useQuery } from "@tanstack/react-query";
import { crmService } from "@/services";

export function useContacts() {
  return useQuery({
    queryKey: ["contacts"],
    queryFn: () => crmService.listContacts(),
  });
}
