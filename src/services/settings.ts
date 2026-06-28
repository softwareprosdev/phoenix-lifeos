import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "@/types";

class SettingsServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "SettingsServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new SettingsServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const settingsService = {
  getSettings: () => safeInvoke<AppSettings>("get_settings"),

  updateSettings: (input: Partial<AppSettings>) =>
    safeInvoke<AppSettings>("update_settings", { payload: input }),

  getSetting: (key: string) => safeInvoke<string | null>("get_setting", { key }),

  setSetting: (key: string, value: string) =>
    safeInvoke<void>("set_setting", { key, value }),

  resetSettings: () => safeInvoke<void>("reset_settings"),
};
