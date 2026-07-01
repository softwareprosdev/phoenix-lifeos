import { invoke } from "@tauri-apps/api/core";
import type { MaintenanceTask, Warranty } from "@/features/maintenance/types";

class MaintenanceServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "MaintenanceServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new MaintenanceServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const maintenanceService = {
  listTasks: () => safeInvoke<MaintenanceTask[]>("list_tasks"),

  createTask: (input: Record<string, unknown>) =>
    safeInvoke<MaintenanceTask>("create_task", { payload: input }),

  updateTask: (id: string, input: Record<string, unknown>) =>
    safeInvoke<MaintenanceTask>("update_task", { id, payload: input }),

  deleteTask: (id: string) =>
    safeInvoke<void>("delete_task", { id }),

  listWarranties: () => safeInvoke<Warranty[]>("list_warranties"),

  createWarranty: (input: Record<string, unknown>) =>
    safeInvoke<Warranty>("create_warranty", { payload: input }),

  updateWarranty: (id: string, input: Record<string, unknown>) =>
    safeInvoke<Warranty>("update_warranty", { id, payload: input }),

  deleteWarranty: (id: string) =>
    safeInvoke<void>("delete_warranty", { id }),
};
