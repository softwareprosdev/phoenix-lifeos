import { useQuery } from "@tanstack/react-query";
import type { BusinessAccount, Invoice } from "../types";

const mockBusinessAccounts: BusinessAccount[] = [
  { id: "1", name: "Business Checking", type: "checking", balance_cents: 12500000, institution: "Chase Business" },
  { id: "2", name: "Business Savings", type: "savings", balance_cents: 45000000, institution: "Chase Business" },
  { id: "3", name: "Merchant Account", type: "merchant", balance_cents: 850000, institution: "Stripe" },
];

const mockInvoices: Invoice[] = [
  { id: "1", number: "INV-2024-001", client: "Acme Corp", amount_cents: 2500000, status: "paid", due_date: "2024-05-15" },
  { id: "2", number: "INV-2024-002", client: "TechStart Inc", amount_cents: 1800000, status: "sent", due_date: "2024-06-30" },
  { id: "3", number: "INV-2024-003", client: "Local Cafe", amount_cents: 450000, status: "overdue", due_date: "2024-06-01" },
  { id: "4", number: "INV-2024-004", client: "Design Studio", amount_cents: 3200000, status: "draft", due_date: "2024-07-15" },
];

export function useBusinessAccounts() {
  return useQuery({
    queryKey: ["business-accounts"],
    queryFn: () => Promise.resolve(mockBusinessAccounts),
  });
}

export function useInvoices() {
  return useQuery({
    queryKey: ["invoices"],
    queryFn: () => Promise.resolve(mockInvoices),
  });
}
