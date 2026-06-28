import { motion } from "framer-motion";
import type { Budget } from "../types";
import { formatCurrency } from "../../../lib/utils";

interface BudgetCardProps {
  budget: Budget;
  onClick: () => void;
}

export function BudgetCard({ budget, onClick }: BudgetCardProps) {
  const percentage = budget.limit_cents > 0 ? (budget.spent_cents / budget.limit_cents) * 100 : 0;

  const getStatusColor = () => {
    if (percentage > 100) return "hsl(0, 84%, 60%)";
    if (percentage >= 80) return "hsl(38, 92%, 50%)";
    return "hsl(142, 72%, 45%)";
  };

  const statusColor = getStatusColor();
  const isOver = percentage > 100;

  return (
    <motion.div
      layout
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      onClick={onClick}
      className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] backdrop-blur-sm border border-[hsl(220_15%_22%/0.6)] hover:border-[hsl(220_90%_56%/0.3)] transition-colors cursor-pointer"
    >
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-2">
          <div className="w-2.5 h-2.5 rounded-full" style={{ background: budget.color }} />
          <p className="text-sm font-semibold text-white">{budget.category}</p>
        </div>
        <span className={`text-xs font-medium ${isOver ? "text-[hsl(0_84%_60%)]" : percentage >= 80 ? "text-[hsl(38_92%_50%)]" : "text-[hsl(0_0%_50%)]"}`}>
          {percentage.toFixed(0)}%
        </span>
      </div>
      <div className="h-2 rounded-full bg-[hsl(220_15%_22%)] overflow-hidden mb-2">
        <motion.div
          initial={{ width: 0 }}
          animate={{ width: `${Math.min(percentage, 100)}%` }}
          transition={{ duration: 0.6, ease: "easeOut" }}
          className="h-full rounded-full"
          style={{ background: statusColor }}
        />
      </div>
      <div className="flex justify-between text-xs">
        <span className="text-[hsl(0_0%_50%)]">
          {formatCurrency(budget.spent_cents)} used
        </span>
        <span className="text-[hsl(0_0%_40%)]">
          {formatCurrency(budget.limit_cents)} limit
        </span>
      </div>
      {isOver && (
        <p className="text-[10px] text-[hsl(0_84%_60%)] mt-1.5 font-medium">
          Over budget by {formatCurrency(budget.spent_cents - budget.limit_cents)}
        </p>
      )}
    </motion.div>
  );
}
