import { financeService } from "./finance";
import { budgetService } from "./budget";
import { debtService } from "./debt";
import { creditService } from "./credit";
import { goalService } from "./goals";
import { investmentService } from "./investments";
import { vaultService } from "./vault";
import { crmService } from "./crm";
import { reportService } from "./reports";
import { settingsService } from "./settings";

export { financeService };
export { budgetService };
export { debtService };
export { creditService };
export { goalService };
export { investmentService };
export { vaultService };
export { crmService };
export { reportService };
export { settingsService };

export const services = {
  finance: financeService,
  budget: budgetService,
  debt: debtService,
  credit: creditService,
  goals: goalService,
  investments: investmentService,
  vault: vaultService,
  crm: crmService,
  reports: reportService,
  settings: settingsService,
} as const;
