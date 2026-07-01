import { useQuery } from "@tanstack/react-query";
import { businessService } from "@/services";

export function useBusinessAccounts() {
  return useQuery({
    queryKey: ["business-accounts"],
    queryFn: () => businessService.listBusinesses(),
  });
}

export function useInvoices() {
  return useQuery({
    queryKey: ["invoices"],
    queryFn: () => businessService.listInvoices(),
  });
}
