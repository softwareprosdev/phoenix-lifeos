import { invoke } from "@tauri-apps/api/core";
import type { Investment, CreateInvestmentInput } from "@/types";

class InvestmentServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "InvestmentServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new InvestmentServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const investmentService = {
  listInvestments: (accountId?: string) =>
    safeInvoke<Investment[]>("list_investments", { accountId }),

  getInvestment: (id: string) => safeInvoke<Investment>("get_investment", { id }),

  createInvestment: (input: CreateInvestmentInput) =>
    safeInvoke<Investment>("create_investment", { payload: input }),

  updateInvestment: (id: string, input: Partial<CreateInvestmentInput>) =>
    safeInvoke<Investment>("update_investment", { id, payload: input }),

  deleteInvestment: (id: string) => safeInvoke<void>("delete_investment", { id }),

  listInvestmentsByType: (assetType: string) =>
    safeInvoke<Investment[]>("list_investments_by_type", { assetType }),
};
