import { safeInvoke } from "./commandClient";
import type { Goal, CreateGoalInput } from "@/types";


export const goalService = {
  listGoals: () => safeInvoke<Goal[]>("Goals", "list_goals"),

  getGoal: (id: string) => safeInvoke<Goal>("Goals", "get_goal", { id }),

  createGoal: (input: CreateGoalInput) =>
    safeInvoke<Goal>("Goals", "create_goal", { payload: input }),

  updateGoal: (id: string, input: Partial<CreateGoalInput>) =>
    safeInvoke<Goal>("Goals", "update_goal", { id, payload: input }),

  deleteGoal: (id: string) => safeInvoke<void>("Goals", "delete_goal", { id }),

  listGoalsByType: (goalType: string) =>
    safeInvoke<Goal[]>("Goals", "list_goals_by_type", { goalType }),

  listActiveGoals: () => safeInvoke<Goal[]>("Goals", "list_active_goals"),

  listGoalsByAccount: (accountId: string) =>
    safeInvoke<Goal[]>("Goals", "list_goals_by_account", { accountId }),
};
