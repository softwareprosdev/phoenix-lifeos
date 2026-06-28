export interface AppSettings {
  theme: "dark" | "light" | "system";
  currency: string;
  aiProvider: "ollama" | "openai" | "anthropic" | "gemini" | "none";
  syncEnabled: boolean;
  autoLockTimeout: number;
  language: string;
  notificationsEnabled: boolean;
}
