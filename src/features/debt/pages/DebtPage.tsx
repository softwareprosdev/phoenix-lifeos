import { useState } from "react";
import { motion } from "framer-motion";
import { useDebts, usePayoffProjection } from "../hooks/useDebtData";
import { DebtCard } from "../components/DebtCard";
import { AddDebtModal } from "../components/AddDebtModal";
import { Button } from "../../../components/ui/Button";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Plus, Snowflake, Triangle } from "lucide-react";
import { formatCurrency } from "../../../lib/utils";
import { AreaChart, Area, ResponsiveContainer, XAxis, YAxis, Tooltip, CartesianGrid } from "recharts";

export default function DebtPage() {
  const { data: debts, isLoading } = useDebts();
  const [showAdd, setShowAdd] = useState(false);
  const [strategy, setStrategy] = useState<"snowball" | "avalanche">("snowball");
  const { data: projection } = usePayoffProjection(strategy);

  if (isLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="grid grid-cols-2 gap-4">
          {Array.from({ length: 4 }).map((_, i) => <CardSkeleton key={i} />)}
        </div>
      </div>
    );
  }

  const totalBalance = debts?.reduce((s, d) => s + d.remaining_balance_cents, 0) ?? 0;
  const totalMinPayments = debts?.reduce((s, d) => s + d.minimum_payment_cents, 0) ?? 0;
  const avgApr = debts && debts.length > 0 ? debts.reduce((s, d) => s + Number(d.interest_rate), 0) / debts.length : 0;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-sm font-semibold text-white">Debt Management</h2>
        <Button size="sm" onClick={() => setShowAdd(true)}>
          <Plus size={14} /> Add Debt
        </Button>
      </div>

      {/* Summary Stats */}
      <div className="grid grid-cols-3 gap-4 mb-6">
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Total Debt</p>
          <p className="text-xl font-bold text-[hsl(0_84%_60%)] mt-1">{formatCurrency(totalBalance)}</p>
        </div>
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Avg APR</p>
          <p className="text-xl font-bold text-white mt-1">{avgApr.toFixed(1)}%</p>
        </div>
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Min Payments</p>
          <p className="text-xl font-bold text-white mt-1">{formatCurrency(totalMinPayments)}/mo</p>
        </div>
      </div>

      {/* Strategy Toggle */}
      <div className="flex items-center gap-2 mb-4">
        <button
          onClick={() => setStrategy("snowball")}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-colors ${
            strategy === "snowball" ? "bg-[hsl(220_90%_56%/0.2)] text-[hsl(220_90%_70%)]" : "bg-[hsl(220_15%_16%)] text-[hsl(0_0%_50%)] hover:text-white"
          }`}
        >
          <Snowflake size={12} /> Snowball
        </button>
        <button
          onClick={() => setStrategy("avalanche")}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-colors ${
            strategy === "avalanche" ? "bg-[hsl(220_90%_56%/0.2)] text-[hsl(220_90%_70%)]" : "bg-[hsl(220_15%_16%)] text-[hsl(0_0%_50%)] hover:text-white"
          }`}
        >
          <Triangle size={12} /> Avalanche
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Debt Cards */}
        <div className="space-y-3">
          {debts?.map((debt) => (
            <DebtCard key={debt.id} debt={debt} />
          ))}
        </div>

        {/* Payoff Timeline */}
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">
            {strategy === "snowball" ? "Snowball" : "Avalanche"} Payoff Timeline
          </p>
          <div className="h-48">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={projection ?? []}>
                <defs>
                  <linearGradient id="debtGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="hsl(0, 84%, 60%)" stopOpacity={0.3} />
                    <stop offset="100%" stopColor="hsl(0, 84%, 60%)" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(220, 15%, 20%)" />
                <XAxis dataKey="month" tick={{ fontSize: 10, fill: "hsl(0, 0%, 50%)" }} axisLine={false} tickLine={false} />
                <YAxis tick={{ fontSize: 10, fill: "hsl(0, 0%, 50%)" }} axisLine={false} tickLine={false} tickFormatter={(v) => `$${(v / 1000000).toFixed(0)}M`} />
                <Tooltip
                  contentStyle={{ background: "hsl(220, 15%, 14%)", border: "1px solid hsl(220, 15%, 22%)", borderRadius: 8, fontSize: 12 }}
                  formatter={(value: number) => [formatCurrency(value), "Total Debt"]}
                  labelStyle={{ color: "hsl(0, 0%, 60%)" }}
                />
                <Area type="monotone" dataKey="totalDebt" stroke="hsl(0, 84%, 60%)" strokeWidth={2} fill="url(#debtGradient)" />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      <AddDebtModal open={showAdd} onClose={() => setShowAdd(false)} />
    </motion.div>
  );
}
