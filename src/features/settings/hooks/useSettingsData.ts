import { create } from "zustand";
import type { AppSettings } from "../types";

interface SettingsStore extends AppSettings {
  setTheme: (theme: AppSettings["theme"]) => void;
  setCurrency: (currency: string) => void;
  setAiProvider: (provider: AppSettings["aiProvider"]) => void;
  setSyncEnabled: (enabled: boolean) => void;
  setAutoLockTimeout: (timeout: number) => void;
  setLanguage: (lang: string) => void;
  setNotificationsEnabled: (enabled: boolean) => void;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  theme: "dark",
  currency: "USD",
  aiProvider: "ollama",
  syncEnabled: true,
  autoLockTimeout: 5,
  language: "en-US",
  notificationsEnabled: true,
  setTheme: (theme) => set({ theme }),
  setCurrency: (currency) => set({ currency }),
  setAiProvider: (aiProvider) => set({ aiProvider }),
  setSyncEnabled: (syncEnabled) => set({ syncEnabled }),
  setAutoLockTimeout: (autoLockTimeout) => set({ autoLockTimeout }),
  setLanguage: (language) => set({ language }),
  setNotificationsEnabled: (notificationsEnabled) => set({ notificationsEnabled }),
}));
