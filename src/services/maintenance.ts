import { safeInvoke } from "./commandClient";
import type { MaintenanceTask, Warranty } from "@/features/maintenance/types";


export const maintenanceService = {
  listTasks: () => safeInvoke<MaintenanceTask[]>("Maintenance", "list_tasks"),

  createTask: (input: Record<string, unknown>) =>
    safeInvoke<MaintenanceTask>("Maintenance", "create_task", { payload: input }),

  updateTask: (id: string, input: Record<string, unknown>) =>
    safeInvoke<MaintenanceTask>("Maintenance", "update_task", { id, payload: input }),

  deleteTask: (id: string) =>
    safeInvoke<void>("Maintenance", "delete_task", { id }),

  listWarranties: () => safeInvoke<Warranty[]>("Maintenance", "list_warranties"),

  createWarranty: (input: Record<string, unknown>) =>
    safeInvoke<Warranty>("Maintenance", "create_warranty", { payload: input }),

  updateWarranty: (id: string, input: Record<string, unknown>) =>
    safeInvoke<Warranty>("Maintenance", "update_warranty", { id, payload: input }),

  deleteWarranty: (id: string) =>
    safeInvoke<void>("Maintenance", "delete_warranty", { id }),
};
