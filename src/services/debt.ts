import { invoke } from "@tauri-apps/api/core";
import type { Debt, CreateDebtInput, PayoffSchedule, DebtSummary, DebtPaymentInput } from "@/types";

class DebtServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "DebtServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new DebtServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const debtService = {
  listDebts: () => safeInvoke<Debt[]>("list_debts"),

  getDebt: (id: string) => safeInvoke<Debt>("get_debt", { id }),

  createDebt: (input: CreateDebtInput) =>
    safeInvoke<Debt>("create_debt", { payload: input }),

  updateDebt: (id: string, input: Partial<CreateDebtInput>) =>
    safeInvoke<Debt>("update_debt", { id, payload: input }),

  deleteDebt: (id: string) => safeInvoke<void>("delete_debt", { id }),

  getDebtSummary: () => safeInvoke<DebtSummary>("get_debt_summary"),

  getPayoffSchedule: (debtId: string, extraPaymentCents?: number) =>
    safeInvoke<PayoffSchedule[]>("get_payoff_schedule", {
      debtId,
      extraPaymentCents: extraPaymentCents ?? 0,
    }),

  makePayment: (input: DebtPaymentInput) =>
    safeInvoke<Debt>("make_debt_payment", { payload: input }),

  listDebtsByType: (debtType: string) =>
    safeInvoke<Debt[]>("list_debts_by_type", { debtType }),
};
