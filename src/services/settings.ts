import { safeInvoke } from "./commandClient";
import type { AppSettings } from "@/types";


export const settingsService = {
  getSettings: () => safeInvoke<AppSettings>("Settings", "get_settings"),

  updateSettings: (input: Partial<AppSettings>) =>
    safeInvoke<AppSettings>("Settings", "update_settings", { payload: input }),

  getSetting: (key: string) => safeInvoke<string | null>("Settings", "get_setting", { key }),

  setSetting: (key: string, value: string) =>
    safeInvoke<void>("Settings", "set_setting", { key, value }),

  resetSettings: () => safeInvoke<void>("Settings", "reset_settings"),
};
