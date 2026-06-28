import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { vaultService } from "@/services";
import type { CreateVaultItemInput } from "@/types";

export const vaultKeys = {
  all: ["vault"] as const,
  list: (itemType?: string) => ["vault", "list", itemType] as const,
  detail: (id: string) => ["vault", id] as const,
  search: (query: string) => ["vault", "search", query] as const,
  accessLog: (itemId: string) => ["vault", itemId, "access"] as const,
};

export function useVaultItems(itemType?: string) {
  return useQuery({
    queryKey: vaultKeys.list(itemType),
    queryFn: () => vaultService.listItems(itemType),
  });
}

export function useVaultItem(id: string) {
  return useQuery({
    queryKey: vaultKeys.detail(id),
    queryFn: () => vaultService.getItem(id),
    enabled: !!id,
  });
}

export function useSearchVaultItems(query: string) {
  return useQuery({
    queryKey: vaultKeys.search(query),
    queryFn: () => vaultService.searchItems(query),
    enabled: query.length > 0,
  });
}

export function useCreateVaultItem() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateVaultItemInput) => vaultService.createItem(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: vaultKeys.all });
    },
  });
}

export function useUpdateVaultItem() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<CreateVaultItemInput> }) =>
      vaultService.updateItem(id, input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: vaultKeys.all });
      queryClient.invalidateQueries({ queryKey: vaultKeys.detail(data.id) });
    },
  });
}

export function useDeleteVaultItem() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => vaultService.deleteItem(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: vaultKeys.all });
    },
  });
}

export function useVaultAccessLog(itemId: string) {
  return useQuery({
    queryKey: vaultKeys.accessLog(itemId),
    queryFn: () => vaultService.getAccessLog(itemId),
    enabled: !!itemId,
  });
}
