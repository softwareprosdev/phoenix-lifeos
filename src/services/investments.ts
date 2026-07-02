import { safeInvoke } from "./commandClient";
import type { Investment, CreateInvestmentInput } from "@/types";


export const investmentService = {
  listInvestments: (accountId?: string) =>
    safeInvoke<Investment[]>("Investments", "list_investments", { accountId }),

  getInvestment: (id: string) => safeInvoke<Investment>("Investments", "get_investment", { id }),

  createInvestment: (input: CreateInvestmentInput) =>
    safeInvoke<Investment>("Investments", "create_investment", { payload: input }),

  updateInvestment: (id: string, input: Partial<CreateInvestmentInput>) =>
    safeInvoke<Investment>("Investments", "update_investment", { id, payload: input }),

  deleteInvestment: (id: string) => safeInvoke<void>("Investments", "delete_investment", { id }),

  listInvestmentsByType: (assetType: string) =>
    safeInvoke<Investment[]>("Investments", "list_investments_by_type", { assetType }),
};
