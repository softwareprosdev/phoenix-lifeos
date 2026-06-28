import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { crmService } from "@/services";
import type { CreateContactInput, CreateInteractionInput } from "@/types";

export const contactKeys = {
  all: ["contacts"] as const,
  detail: (id: string) => ["contacts", id] as const,
  search: (query: string) => ["contacts", "search", query] as const,
  interactions: (contactId: string) => ["contacts", contactId, "interactions"] as const,
};

export function useContacts() {
  return useQuery({
    queryKey: contactKeys.all,
    queryFn: () => crmService.listContacts(),
  });
}

export function useContact(id: string) {
  return useQuery({
    queryKey: contactKeys.detail(id),
    queryFn: () => crmService.getContact(id),
    enabled: !!id,
  });
}

export function useSearchContacts(query: string) {
  return useQuery({
    queryKey: contactKeys.search(query),
    queryFn: () => crmService.searchContacts(query),
    enabled: query.length > 0,
  });
}

export function useCreateContact() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateContactInput) => crmService.createContact(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: contactKeys.all });
    },
  });
}

export function useUpdateContact() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<CreateContactInput> }) =>
      crmService.updateContact(id, input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: contactKeys.all });
      queryClient.invalidateQueries({ queryKey: contactKeys.detail(data.id) });
    },
  });
}

export function useDeleteContact() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => crmService.deleteContact(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: contactKeys.all });
    },
  });
}

export function useContactInteractions(contactId: string) {
  return useQuery({
    queryKey: contactKeys.interactions(contactId),
    queryFn: () => crmService.listInteractions(contactId),
    enabled: !!contactId,
  });
}

export function useCreateInteraction() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: CreateInteractionInput) => crmService.createInteraction(input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: contactKeys.interactions(data.contact_id),
      });
    },
  });
}
