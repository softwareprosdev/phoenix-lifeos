import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { creditService } from "@/services";
import type {
  BusinessCreditProfile,
  CreditAccount,
  CreditDerogatory,
  CreditInquiry,
  DisputeCase,
} from "@/types";

export function useDisputeCases() {
  return useQuery({
    queryKey: ["credit-repair", "cases"],
    queryFn: () => creditService.listDisputeCases(),
  });
}

export function useDisputeCase(id: string | null) {
  return useQuery({
    queryKey: ["credit-repair", "case", id],
    queryFn: () => creditService.getDisputeCase(id!),
    enabled: !!id,
  });
}

export function useCreateDisputeCase() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (input: Partial<DisputeCase>) => creditService.createDisputeCase(input),
    onSuccess: () => { qc.invalidateQueries({ queryKey: ["credit-repair", "cases"] }); },
  });
}

export function useUpdateDisputeCase() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: Partial<DisputeCase> }) => creditService.updateDisputeCase(id, input),
    onSuccess: () => { qc.invalidateQueries({ queryKey: ["credit-repair"] }); },
  });
}

export function useDisputeLetters(caseId: string | null) {
  return useQuery({
    queryKey: ["credit-repair", "letters", caseId],
    queryFn: () => creditService.listDisputeLetters(caseId!),
    enabled: !!caseId,
  });
}

export function useGenerateLetter() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ caseId, templateId, variables }: { caseId: string; templateId: string; variables: Record<string, string> }) =>
      creditService.generateLetter({ case_id: caseId, template_id: templateId, variables }),
    onSuccess: () => { qc.invalidateQueries({ queryKey: ["credit-repair"] }); },
  });
}

export function useLetterTemplates() {
  return useQuery({
    queryKey: ["credit-repair", "templates"],
    queryFn: () => creditService.listLetterTemplates(),
  });
}

export function useLetterTemplateByCode(code: string | null) {
  return useQuery({
    queryKey: ["credit-repair", "template", code],
    queryFn: () => creditService.getLetterTemplateByCode(code!),
    enabled: !!code,
  });
}

export function useMarkLetterSent() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ letterId, trackingNumber }: { letterId: string; trackingNumber: string }) =>
      creditService.markLetterSent(letterId, trackingNumber),
    onSuccess: () => { qc.invalidateQueries({ queryKey: ["credit-repair"] }); },
  });
}

export function useAddTimelineEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ caseId, eventType, description }: { caseId: string; eventType: string; description: string }) =>
      creditService.addTimelineEntry(caseId, eventType, description),
    onSuccess: () => { qc.invalidateQueries({ queryKey: ["credit-repair", "case"] }); },
  });
}

export function useCreditAccounts() {
  return useQuery({
    queryKey: ["credit-repair", "credit-accounts"],
    queryFn: () => creditService.listCreditAccounts(),
  });
}

export function useCreateCreditAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (input: Partial<CreditAccount>) => creditService.createCreditAccount(input),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["credit-repair", "credit-accounts"] });
    },
  });
}

export function useDeleteCreditAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => creditService.deleteCreditAccount(id),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["credit-repair", "credit-accounts"] });
    },
  });
}

export function useCreditInquiries() {
  return useQuery({
    queryKey: ["credit-repair", "inquiries"],
    queryFn: () => creditService.listCreditInquiries(),
  });
}

export function useCreateCreditInquiry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (input: Partial<CreditInquiry>) => creditService.createCreditInquiry(input),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["credit-repair", "inquiries"] });
    },
  });
}

export function useDerogatoryItems() {
  return useQuery({
    queryKey: ["credit-repair", "derogatory"],
    queryFn: () => creditService.listDerogatoryItems(),
  });
}

export function useCreateDerogatoryItem() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (input: Partial<CreditDerogatory>) => creditService.createDerogatoryItem(input),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["credit-repair", "derogatory"] });
      qc.invalidateQueries({ queryKey: ["credit-repair", "cases"] });
    },
  });
}

export function useBusinessCreditProfiles() {
  return useQuery({
    queryKey: ["credit-repair", "business-profiles"],
    queryFn: () => creditService.getBusinessCreditProfiles(),
  });
}

export function useCreateBusinessCreditProfile() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (input: Partial<BusinessCreditProfile>) =>
      creditService.createBusinessCreditProfile(input),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["credit-repair", "business-profiles"] });
    },
  });
}
