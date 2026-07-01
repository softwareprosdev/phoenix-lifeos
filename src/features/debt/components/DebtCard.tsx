import { motion } from "framer-motion";
import type { Debt } from "../types";
import { formatCurrency } from "../../../lib/utils";
import { Building2 } from "lucide-react";

interface DebtCardProps {
  debt: Debt;
}

export function DebtCard({ debt }: DebtCardProps) {
  const payoffPct = debt.principal_cents > 0
    ? ((debt.principal_cents - debt.remaining_balance_cents) / debt.principal_cents) * 100
    : 0;

  return (
    <motion.div
      layout
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] backdrop-blur-sm border border-[hsl(220_15%_22%/0.6)]"
    >
      <div className="flex items-start justify-between mb-3">
        <div>
          <div className="flex items-center gap-2">
            <Building2 size={14} className="text-[hsl(0_0%_40%)]" />
            <p className="text-sm font-semibold text-white">{debt.name}</p>
          </div>
          <p className="text-[10px] text-[hsl(0_0%_45%)] mt-0.5">{debt.creditor}</p>
        </div>
        <span className="text-xs font-semibold text-[hsl(0_84%_60%)]">{Number(debt.interest_rate).toFixed(1)}% APR</span>
      </div>

      <div className="flex items-end justify-between mb-3">
        <div>
          <p className="text-lg font-bold text-white">{formatCurrency(debt.remaining_balance_cents)}</p>
          <p className="text-[10px] text-[hsl(0_0%_45%)]">Min: {formatCurrency(debt.minimum_payment_cents)}/mo</p>
        </div>
        <p className="text-[11px] text-[hsl(0_0%_50%)]">
          {payoffPct.toFixed(0)}% paid off
        </p>
      </div>

      <div className="h-2 rounded-full bg-[hsl(220_15%_22%)] overflow-hidden">
        <motion.div
          initial={{ width: 0 }}
          animate={{ width: `${payoffPct}%` }}
          transition={{ duration: 0.6 }}
          className="h-full rounded-full"
          style={{ background: payoffPct > 50 ? "hsl(142, 72%, 45%)" : "hsl(38, 92%, 50%)" }}
        />
      </div>
    </motion.div>
  );
}
