import type { Debt as GlobalDebt, DebtSummary, PayoffSchedule } from "@/types";

export type Debt = GlobalDebt;
export type { DebtSummary, PayoffSchedule };

export interface PayoffProjection {
  month: string;
  totalDebt: number;
}
