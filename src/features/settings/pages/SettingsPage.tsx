import { motion } from "framer-motion";
import { useSettingsStore } from "../hooks/useSettingsData";
import { useEffect } from "react";
import { Moon, Sun, Monitor, DollarSign, Bot, Cloud, Lock, Bell, Globe } from "lucide-react";

const currencies = [
  { value: "USD", label: "USD - US Dollar" },
  { value: "EUR", label: "EUR - Euro" },
  { value: "GBP", label: "GBP - British Pound" },
  { value: "JPY", label: "JPY - Japanese Yen" },
  { value: "CAD", label: "CAD - Canadian Dollar" },
  { value: "AUD", label: "AUD - Australian Dollar" },
  { value: "CHF", label: "CHF - Swiss Franc" },
  { value: "CNY", label: "CNY - Chinese Yuan" },
];

const aiProviders = [
  { value: "ollama", label: "Ollama (Local)" },
  { value: "openai", label: "OpenAI" },
  { value: "anthropic", label: "Anthropic" },
  { value: "gemini", label: "Google Gemini" },
  { value: "none", label: "None" },
];

const lockTimeouts = [
  { value: "1", label: "1 minute" },
  { value: "5", label: "5 minutes" },
  { value: "15", label: "15 minutes" },
  { value: "30", label: "30 minutes" },
  { value: "60", label: "1 hour" },
  { value: "0", label: "Never" },
];

function SettingCard({
  icon: Icon,
  title,
  description,
  children,
}: {
  icon: React.ElementType;
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
      <div className="flex items-start gap-3">
        <div className="w-9 h-9 rounded-lg bg-[hsl(220_15%_18%)] flex items-center justify-center flex-shrink-0">
          <Icon size={16} className="text-[hsl(0_0%_55%)]" />
        </div>
        <div className="flex-1 min-w-0">
          <p className="text-sm font-medium text-white">{title}</p>
          {description && <p className="text-[11px] text-[hsl(0_0%_45%)] mt-0.5">{description}</p>}
          <div className="mt-3">{children}</div>
        </div>
      </div>
    </div>
  );
}

export default function SettingsPage() {
  const {
    theme, setTheme,
    currency, setCurrency,
    aiProvider, setAiProvider,
    syncEnabled, setSyncEnabled,
    autoLockTimeout, setAutoLockTimeout,
    notificationsEnabled, setNotificationsEnabled,
  } = useSettingsStore();

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
  }, [theme]);

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto max-w-2xl"
    >
      <h2 className="text-sm font-semibold text-white mb-4">Settings</h2>

      <div className="space-y-3">
        {/* Theme */}
        <SettingCard icon={theme === "dark" ? Moon : theme === "light" ? Sun : Monitor} title="Theme" description="Choose your preferred appearance">
          <div className="flex gap-1.5">
            {([
              { id: "dark" as const, label: "Dark", icon: Moon },
              { id: "light" as const, label: "Light", icon: Sun },
              { id: "system" as const, label: "System", icon: Monitor },
            ]).map((opt) => (
              <button
                key={opt.id}
                onClick={() => setTheme(opt.id)}
                className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-colors ${
                  theme === opt.id ? "bg-[hsl(220_90%_56%/0.2)] text-[hsl(220_90%_70%)]" : "bg-[hsl(220_15%_16%)] text-[hsl(0_0%_50%)] hover:text-white"
                }`}
              >
                <opt.icon size={12} /> {opt.label}
              </button>
            ))}
          </div>
        </SettingCard>

        {/* Currency */}
        <SettingCard icon={DollarSign} title="Currency" description="Default currency for all financial data">
          <select
            value={currency}
            onChange={(e) => setCurrency(e.target.value)}
            className="w-full h-9 px-3 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-sm text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
          >
            {currencies.map((c) => (
              <option key={c.value} value={c.value}>{c.label}</option>
            ))}
          </select>
        </SettingCard>

        {/* AI Provider */}
        <SettingCard icon={Bot} title="AI Provider" description="Select the AI model provider for insights and features">
          <select
            value={aiProvider}
            onChange={(e) => setAiProvider(e.target.value as typeof aiProvider)}
            className="w-full h-9 px-3 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-sm text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
          >
            {aiProviders.map((p) => (
              <option key={p.value} value={p.value}>{p.label}</option>
            ))}
          </select>
        </SettingCard>

        {/* Sync */}
        <SettingCard icon={Cloud} title="Sync" description="Enable cloud sync across devices">
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={syncEnabled}
              onChange={(e) => setSyncEnabled(e.target.checked)}
              className="sr-only peer"
            />
            <div className="w-9 h-5 rounded-full bg-[hsl(220_15%_22%)] peer-checked:bg-[hsl(220_90%_56%)] after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:after:translate-x-4" />
          </label>
        </SettingCard>

        {/* Notifications */}
        <SettingCard icon={Bell} title="Notifications" description="Enable in-app notifications">
          <label className="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={notificationsEnabled}
              onChange={(e) => setNotificationsEnabled(e.target.checked)}
              className="sr-only peer"
            />
            <div className="w-9 h-5 rounded-full bg-[hsl(220_15%_22%)] peer-checked:bg-[hsl(220_90%_56%)] after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:after:translate-x-4" />
          </label>
        </SettingCard>

        {/* Auto-lock */}
        <SettingCard icon={Lock} title="Auto-lock" description="Automatically lock the app after inactivity">
          <select
            value={autoLockTimeout.toString()}
            onChange={(e) => setAutoLockTimeout(Number(e.target.value))}
            className="w-full h-9 px-3 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-sm text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
          >
            {lockTimeouts.map((t) => (
              <option key={t.value} value={t.value}>{t.label}</option>
            ))}
          </select>
        </SettingCard>

        {/* Language */}
        <SettingCard icon={Globe} title="Language" description="Application language">
          <select
            value={useSettingsStore.getState().language}
            onChange={(e) => useSettingsStore.getState().setLanguage(e.target.value)}
            className="w-full h-9 px-3 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-sm text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
          >
            <option value="en-US">English (US)</option>
            <option value="en-GB">English (UK)</option>
            <option value="es">Spanish</option>
            <option value="fr">French</option>
            <option value="de">German</option>
            <option value="ja">Japanese</option>
            <option value="zh-CN">Chinese (Simplified)</option>
          </select>
        </SettingCard>
      </div>
    </motion.div>
  );
}
