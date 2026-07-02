import { safeInvoke } from "./commandClient";
import type { BusinessAccount, Invoice } from "@/features/business/types";


export const businessService = {
  listBusinesses: () => safeInvoke<BusinessAccount[]>("Business", "list_businesses"),

  createBusiness: (input: Record<string, unknown>) =>
    safeInvoke<BusinessAccount>("Business", "create_business", { payload: input }),

  updateBusiness: (id: string, input: Record<string, unknown>) =>
    safeInvoke<BusinessAccount>("Business", "update_business", { id, payload: input }),

  deleteBusiness: (id: string) =>
    safeInvoke<void>("Business", "delete_business", { id }),

  listInvoices: () => safeInvoke<Invoice[]>("Business", "list_invoices"),

  createInvoice: (input: Record<string, unknown>) =>
    safeInvoke<Invoice>("Business", "create_invoice", { payload: input }),

  updateInvoice: (id: string, input: Record<string, unknown>) =>
    safeInvoke<Invoice>("Business", "update_invoice", { id, payload: input }),

  deleteInvoice: (id: string) =>
    safeInvoke<void>("Business", "delete_invoice", { id }),
};
