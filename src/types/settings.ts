export type Theme = "dark" | "light" | "system";
export type AiProvider = "ollama" | "openai" | "anthropic" | "gemini" | "none";

export interface AppSettings {
  theme: Theme;
  language: string;
  currency: string;
  ai_provider: AiProvider;
  sync_enabled: boolean;
  auto_lock_minutes: number;
}
