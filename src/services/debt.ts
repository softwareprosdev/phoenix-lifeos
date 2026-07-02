import { safeInvoke } from "./commandClient";
import type { Debt, CreateDebtInput, PayoffSchedule, DebtSummary, DebtPaymentInput } from "@/types";


export const debtService = {
  listDebts: () => safeInvoke<Debt[]>("Debt", "list_debts"),

  getDebt: (id: string) => safeInvoke<Debt>("Debt", "get_debt", { id }),

  createDebt: (input: CreateDebtInput) =>
    safeInvoke<Debt>("Debt", "create_debt", { payload: input }),

  updateDebt: (id: string, input: Partial<CreateDebtInput>) =>
    safeInvoke<Debt>("Debt", "update_debt", { id, payload: input }),

  deleteDebt: (id: string) => safeInvoke<void>("Debt", "delete_debt", { id }),

  getDebtSummary: () => safeInvoke<DebtSummary>("Debt", "get_debt_summary"),

  getPayoffSchedule: (debtId: string, extraPaymentCents?: number) =>
    safeInvoke<PayoffSchedule[]>("Debt", "get_payoff_schedule", {
      debtId,
      extraPaymentCents: extraPaymentCents ?? 0,
    }),

  makePayment: (input: DebtPaymentInput) =>
    safeInvoke<Debt>("Debt", "make_debt_payment", { payload: input }),

  listDebtsByType: (debtType: string) =>
    safeInvoke<Debt[]>("Debt", "list_debts_by_type", { debtType }),
};
