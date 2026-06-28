import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { useBudgets, useBudgetBreakdown } from "../hooks/useBudgetData";
import { BudgetCard } from "../components/BudgetCard";
import { AddBudgetModal } from "../components/AddBudgetModal";
import { Tabs } from "../../../components/ui/Tabs";
import { Button } from "../../../components/ui/Button";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Plus, X } from "lucide-react";
import { formatCurrency } from "../../../lib/utils";

export default function BudgetPage() {
  const { data: budgets, isLoading } = useBudgets();
  const [period, setPeriod] = useState("monthly");
  const [showAdd, setShowAdd] = useState(false);
  const [selectedBudgetId, setSelectedBudgetId] = useState<string | null>(null);
  const { data: breakdown } = useBudgetBreakdown(selectedBudgetId ?? "");

  if (isLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="grid grid-cols-2 gap-4">
          {Array.from({ length: 4 }).map((_, i) => <CardSkeleton key={i} />)}
        </div>
      </div>
    );
  }

  const totalSpent = budgets?.reduce((s, b) => s + b.spent_cents, 0) ?? 0;
  const totalLimit = budgets?.reduce((s, b) => s + b.limit_cents, 0) ?? 0;
  const overallPct = totalLimit > 0 ? (totalSpent / totalLimit) * 100 : 0;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-4">
          <h2 className="text-sm font-semibold text-white">Budget Overview</h2>
          <Tabs
            tabs={[
              { id: "weekly", label: "Weekly" },
              { id: "monthly", label: "Monthly" },
              { id: "yearly", label: "Yearly" },
            ]}
            active={period}
            onChange={setPeriod}
          />
        </div>
        <Button size="sm" onClick={() => setShowAdd(true)}>
          <Plus size={14} /> Add Budget
        </Button>
      </div>

      {/* Overall Budget Health */}
      <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)] mb-6">
        <div className="flex items-center justify-between mb-2">
          <p className="text-xs text-[hsl(0_0%_50%)]">Total Budget Utilization</p>
          <p className="text-xs font-medium text-white">{overallPct.toFixed(0)}%</p>
        </div>
        <div className="h-2.5 rounded-full bg-[hsl(220_15%_22%)] overflow-hidden">
          <motion.div
            initial={{ width: 0 }}
            animate={{ width: `${Math.min(overallPct, 100)}%` }}
            transition={{ duration: 0.6 }}
            className="h-full rounded-full"
            style={{
              background: overallPct > 100 ? "hsl(0, 84%, 60%)" : overallPct >= 80 ? "hsl(38, 92%, 50%)" : "hsl(142, 72%, 45%)",
            }}
          />
        </div>
        <div className="flex justify-between text-xs mt-1">
          <span className="text-[hsl(0_0%_45%)]">Spent: {formatCurrency(totalSpent)}</span>
          <span className="text-[hsl(0_0%_45%)]">Budget: {formatCurrency(totalLimit)}</span>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Budget Cards */}
        <div className="space-y-3">
          {budgets?.map((budget) => (
            <BudgetCard
              key={budget.id}
              budget={budget}
              onClick={() => setSelectedBudgetId(selectedBudgetId === budget.id ? null : budget.id)}
            />
          ))}
          {(!budgets || budgets.length === 0) && (
            <div className="text-center py-12 text-sm text-[hsl(0_0%_45%)]">
              No budgets created yet. Click "Add Budget" to get started.
            </div>
          )}
        </div>

        {/* Category Breakdown */}
        <div>
          <AnimatePresence mode="wait">
            {selectedBudgetId && breakdown ? (
              <motion.div
                key={selectedBudgetId}
                initial={{ opacity: 0, x: 10 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -10 }}
                className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]"
              >
                <div className="flex items-center justify-between mb-3">
                  <h3 className="text-sm font-semibold text-white">{breakdown.category} Breakdown</h3>
                  <button onClick={() => setSelectedBudgetId(null)} className="text-[hsl(0_0%_40%)] hover:text-white">
                    <X size={14} />
                  </button>
                </div>
                <div className="flex items-center justify-between mb-3 p-2 rounded bg-[hsl(220_15%_16%/0.5)]">
                  <span className="text-xs text-[hsl(0_0%_50%)]">
                    {formatCurrency(breakdown.spent)} of {formatCurrency(breakdown.limit)}
                  </span>
                  <span className={`text-xs font-medium ${breakdown.spent > breakdown.limit ? "text-[hsl(0_84%_60%)]" : "text-[hsl(142_72%_45%)]"}`}>
                    {breakdown.limit > 0 ? `${((breakdown.spent / breakdown.limit) * 100).toFixed(0)}%` : "N/A"}
                  </span>
                </div>
                <div className="space-y-2">
                  {breakdown.items.map((item, idx) => (
                    <div key={idx} className="flex justify-between py-1.5 border-b border-[hsl(220_15%_18%/0.5)] last:border-0">
                      <div>
                        <p className="text-sm text-white">{item.name}</p>
                        <p className="text-[10px] text-[hsl(0_0%_40%)]">{item.date}</p>
                      </div>
                      <span className="text-sm font-medium text-white">{formatCurrency(item.amount)}</span>
                    </div>
                  ))}
                </div>
              </motion.div>
            ) : (
              <motion.div
                key="empty"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex items-center justify-center h-40 rounded-lg border-2 border-dashed border-[hsl(220_15%_22%)]"
              >
                <p className="text-xs text-[hsl(0_0%_40%)]">Click a budget to see category breakdown</p>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>

      <AddBudgetModal open={showAdd} onClose={() => setShowAdd(false)} />
    </motion.div>
  );
}
