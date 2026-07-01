import { useQuery } from "@tanstack/react-query";
import { vaultService } from "@/services";

export function useVaultItems() {
  return useQuery({
    queryKey: ["vault-items"],
    queryFn: () => vaultService.listItems(),
  });
}
