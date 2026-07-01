import { invoke } from "@tauri-apps/api/core";
import type { BusinessAccount, Invoice } from "@/features/business/types";

class BusinessServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "BusinessServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new BusinessServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const businessService = {
  listBusinesses: () => safeInvoke<BusinessAccount[]>("list_businesses"),

  createBusiness: (input: Record<string, unknown>) =>
    safeInvoke<BusinessAccount>("create_business", { payload: input }),

  updateBusiness: (id: string, input: Record<string, unknown>) =>
    safeInvoke<BusinessAccount>("update_business", { id, payload: input }),

  deleteBusiness: (id: string) =>
    safeInvoke<void>("delete_business", { id }),

  listInvoices: () => safeInvoke<Invoice[]>("list_invoices"),

  createInvoice: (input: Record<string, unknown>) =>
    safeInvoke<Invoice>("create_invoice", { payload: input }),

  updateInvoice: (id: string, input: Record<string, unknown>) =>
    safeInvoke<Invoice>("update_invoice", { id, payload: input }),

  deleteInvoice: (id: string) =>
    safeInvoke<void>("delete_invoice", { id }),
};
