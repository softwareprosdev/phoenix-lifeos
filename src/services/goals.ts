import { invoke } from "@tauri-apps/api/core";
import type { Goal, CreateGoalInput } from "@/types";

class GoalServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "GoalServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new GoalServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const goalService = {
  listGoals: () => safeInvoke<Goal[]>("list_goals"),

  getGoal: (id: string) => safeInvoke<Goal>("get_goal", { id }),

  createGoal: (input: CreateGoalInput) =>
    safeInvoke<Goal>("create_goal", { payload: input }),

  updateGoal: (id: string, input: Partial<CreateGoalInput>) =>
    safeInvoke<Goal>("update_goal", { id, payload: input }),

  deleteGoal: (id: string) => safeInvoke<void>("delete_goal", { id }),

  listGoalsByType: (goalType: string) =>
    safeInvoke<Goal[]>("list_goals_by_type", { goalType }),

  listActiveGoals: () => safeInvoke<Goal[]>("list_active_goals"),

  listGoalsByAccount: (accountId: string) =>
    safeInvoke<Goal[]>("list_goals_by_account", { accountId }),
};
