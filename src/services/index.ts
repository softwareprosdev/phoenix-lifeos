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
import { maintenanceService } from "./maintenance";
import { businessService } from "./business";
import { dashboardService } from "./dashboard";

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
export { maintenanceService };
export { businessService };
export { dashboardService };

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
  maintenance: maintenanceService,
  business: businessService,
  dashboard: dashboardService,
} as const;
