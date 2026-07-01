import { useState } from "react";
import { motion } from "framer-motion";
import { useCreditScore, useCreditAccounts } from "../hooks/useCreditData";
import { ScoreGauge } from "../components/ScoreGauge";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { LineChart, Line, ResponsiveContainer, XAxis, YAxis, Tooltip, CartesianGrid } from "recharts";
import { CreditCard, PieChart, Clock, Search, Shield } from "lucide-react";
import { formatCurrency, cn } from "../../../lib/utils";
import CreditRepairPage from "./CreditRepairPage";

export default function CreditPage() {
  const [activeTab, setActiveTab] = useState<"monitoring" | "repair">("monitoring");
  const { data: scoreData, isLoading: scoreLoading } = useCreditScore();
  const { data: accounts, isLoading: accountsLoading } = useCreditAccounts();

  const tabs = [
    { id: "monitoring" as const, label: "Monitoring", icon: Search },
    { id: "repair" as const, label: "Repair", icon: Shield },
  ];

  if (activeTab === "repair") {
    return (
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.2 }}
        className="h-full overflow-auto"
      >
        <div className="p-4 pb-0">
          <div className="flex items-center gap-1 rounded-lg bg-[hsl(220_15%_14%/0.5)] p-1 border border-[hsl(220_15%_22%/0.4)] w-fit">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={cn(
                    "inline-flex items-center gap-2 rounded px-3 py-1.5 text-xs font-medium transition-colors",
                    activeTab === tab.id
                      ? "bg-[hsl(220_90%_56%/0.15)] text-[hsl(220_90%_70%)]"
                      : "text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%/0.5)]",
                  )}
                >
                  <Icon size={14} />
                  {tab.label}
                </button>
              );
            })}
          </div>
        </div>
        <CreditRepairPage />
      </motion.div>
    );
  }

  if (scoreLoading || accountsLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="flex items-center gap-1 rounded-lg bg-[hsl(220_15%_14%/0.5)] p-1 border border-[hsl(220_15%_22%/0.4)] w-fit mb-4">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={cn(
                  "inline-flex items-center gap-2 rounded px-3 py-1.5 text-xs font-medium transition-colors",
                  activeTab === tab.id
                    ? "bg-[hsl(220_90%_56%/0.15)] text-[hsl(220_90%_70%)]"
                    : "text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%/0.5)]",
                )}
              >
                <Icon size={14} />
                {tab.label}
              </button>
            );
          })}
        </div>
        <div className="grid grid-cols-2 gap-4">
          {Array.from({ length: 4 }).map((_, i) => <CardSkeleton key={i} />)}
        </div>
      </div>
    );
  }

  const totalLimit = accounts?.reduce((s, a) => s + a.credit_limit_cents, 0) ?? 0;
  const totalBalance = accounts?.reduce((s, a) => s + a.balance_cents, 0) ?? 0;
  const utilization = totalLimit > 0 ? (totalBalance / totalLimit) * 100 : 0;

  const factorItems = scoreData ? [
    { label: "Utilization", value: `${scoreData.factors.utilization.value}%`, icon: PieChart, status: scoreData.factors.utilization.status, detail: `${utilization.toFixed(0)}% used` },
    { label: "Payment History", value: `${scoreData.factors.paymentHistory.value}%`, icon: CreditCard, status: scoreData.factors.paymentHistory.status, detail: "On time" },
    { label: "Age of Accounts", value: `${scoreData.factors.ageOfAccounts.value}yrs`, icon: Clock, status: scoreData.factors.ageOfAccounts.status, detail: "Average age" },
    { label: "Inquiries", value: `${scoreData.factors.inquiries.value}`, icon: Search, status: scoreData.factors.inquiries.status, detail: "Last 2 years" },
  ] : [];

  const scoreColor = (score: number) => {
    if (score >= 750) return "hsl(142, 72%, 45%)";
    if (score >= 700) return "hsl(196, 85%, 50%)";
    if (score >= 650) return "hsl(38, 92%, 50%)";
    return "hsl(0, 84%, 60%)";
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      <div className="flex items-center gap-1 rounded-lg bg-[hsl(220_15%_14%/0.5)] p-1 border border-[hsl(220_15%_22%/0.4)] w-fit mb-4">
        {tabs.map((tab) => {
          const Icon = tab.icon;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={cn(
                "inline-flex items-center gap-2 rounded px-3 py-1.5 text-xs font-medium transition-colors",
                activeTab === tab.id
                  ? "bg-[hsl(220_90%_56%/0.15)] text-[hsl(220_90%_70%)]"
                  : "text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%/0.5)]",
              )}
            >
              <Icon size={14} />
              {tab.label}
            </button>
          );
        })}
      </div>

      <h2 className="text-sm font-semibold text-white mb-4">Credit Monitoring</h2>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 mb-6">
        {/* Score Gauge */}
        <div className="rounded-lg p-5 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)] flex items-center justify-center">
          {scoreData && <ScoreGauge score={scoreData.score} />}
        </div>

        {/* Score Factors */}
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Score Factors</p>
          <div className="space-y-3">
            {factorItems.map((item) => {
              const Icon = item.icon;
              const statusColor = item.status === "good" ? "hsl(142, 72%, 45%)" : item.status === "fair" ? "hsl(38, 92%, 50%)" : "hsl(0, 84%, 60%)";
              return (
                <div key={item.label} className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-lg flex items-center justify-center bg-[hsl(220_15%_18%)]">
                    <Icon size={14} style={{ color: statusColor }} />
                  </div>
                  <div className="flex-1">
                    <p className="text-xs text-white">{item.label}</p>
                    <p className="text-[10px] text-[hsl(0_0%_45%)]">{item.detail}</p>
                  </div>
                  <span className="text-xs font-semibold" style={{ color: statusColor }}>{item.value}</span>
                </div>
              );
            })}
          </div>
        </div>

        {/* Utilization */}
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Credit Utilization</p>
          <div className="flex flex-col items-center justify-center h-full">
            <div className="relative w-24 h-24">
              <svg className="w-24 h-24 -rotate-90" viewBox="0 0 36 36">
                <circle cx="18" cy="18" r="15.5" fill="none" stroke="hsl(220, 15%, 22%)" strokeWidth="3" />
                <circle
                  cx="18" cy="18" r="15.5"
                  fill="none"
                  stroke={utilization > 30 ? "hsl(38, 92%, 50%)" : "hsl(142, 72%, 45%)"}
                  strokeWidth="3"
                  strokeDasharray={`${Math.min(utilization, 100)} ${100 - Math.min(utilization, 100)}`}
                  strokeLinecap="round"
                />
              </svg>
              <div className="absolute inset-0 flex items-center justify-center">
                <span className="text-lg font-bold text-white">{utilization.toFixed(0)}%</span>
              </div>
            </div>
            <p className="text-[10px] text-[hsl(0_0%_45%)] mt-2">{formatCurrency(totalBalance)} / {formatCurrency(totalLimit)}</p>
          </div>
        </div>
      </div>

      {/* Score History Chart */}
      <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)] mb-6">
        <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Score History</p>
        <div className="h-48">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={scoreData?.history ?? []}>
              <CartesianGrid strokeDasharray="3 3" stroke="hsl(220, 15%, 20%)" />
              <XAxis dataKey="date" tick={{ fontSize: 10, fill: "hsl(0, 0%, 50%)" }} axisLine={false} tickLine={false} />
              <YAxis domain={[600, 850]} tick={{ fontSize: 10, fill: "hsl(0, 0%, 50%)" }} axisLine={false} tickLine={false} />
              <Tooltip
                contentStyle={{ background: "hsl(220, 15%, 14%)", border: "1px solid hsl(220, 15%, 22%)", borderRadius: 8, fontSize: 12 }}
                labelStyle={{ color: "hsl(0, 0%, 60%)" }}
              />
              <Line type="monotone" dataKey="score" stroke={scoreColor(scoreData?.score ?? 0)} strokeWidth={2} dot={{ r: 4, fill: scoreColor(scoreData?.score ?? 0) }} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Credit Accounts */}
      <h3 className="text-sm font-semibold text-white mb-3">Credit Accounts</h3>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        {accounts?.map((account) => (
          <div
            key={account.id}
            className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]"
          >
            <p className="text-sm font-semibold text-white">{account.creditor_name}</p>
            <p className="text-[10px] text-[hsl(0_0%_45%)]">{account.account_type}</p>
            <div className="flex items-center justify-between mt-2">
              <span className="text-xs text-[hsl(0_0%_50%)]">Balance</span>
              <span className={`text-sm font-semibold ${account.balance_cents > 0 ? "text-white" : "text-[hsl(142_72%_45%)]"}`}>
                {formatCurrency(account.balance_cents)}
              </span>
            </div>
            <div className="flex items-center justify-between mt-0.5">
              <span className="text-xs text-[hsl(0_0%_50%)]">Limit</span>
              <span className="text-sm text-white">{formatCurrency(account.credit_limit_cents)}</span>
            </div>
            <div className="flex items-center justify-between mt-0.5">
              <span className="text-xs text-[hsl(0_0%_50%)]">APR</span>
              <span className="text-xs text-[hsl(0_84%_60%)]">{account.payment_status}</span>
            </div>
          </div>
        ))}
      </div>
    </motion.div>
  );
}
