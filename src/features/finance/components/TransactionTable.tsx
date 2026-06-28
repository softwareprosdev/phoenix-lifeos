import { motion } from "framer-motion";
import type { Transaction } from "../types";
import { formatCurrency, formatDate } from "../../../lib/utils";
import { ArrowUpRight, ArrowDownRight, Trash2 } from "lucide-react";

interface TransactionTableProps {
  transactions: Transaction[];
  onDelete?: (id: string) => void;
}

export function TransactionTable({ transactions, onDelete }: TransactionTableProps) {
  if (transactions.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-sm text-[hsl(0_0%_45%)]">No transactions found</p>
      </div>
    );
  }

  const totalIncome = transactions.filter((t) => t.type === "income").reduce((s, t) => s + t.amount_cents, 0);
  const totalExpenses = transactions.filter((t) => t.type === "expense").reduce((s, t) => s + t.amount_cents, 0);

  return (
    <div>
      <div className="flex items-center gap-4 mb-3 p-3 rounded-lg bg-[hsl(220_15%_16%/0.6)]">
        <div>
          <span className="text-[10px] text-[hsl(0_0%_45%)]">Income</span>
          <p className="text-sm font-semibold text-[hsl(142_72%_45%)]">{formatCurrency(totalIncome)}</p>
        </div>
        <div>
          <span className="text-[10px] text-[hsl(0_0%_45%)]">Expenses</span>
          <p className="text-sm font-semibold text-[hsl(0_84%_60%)]">{formatCurrency(Math.abs(totalExpenses))}</p>
        </div>
        <div>
          <span className="text-[10px] text-[hsl(0_0%_45%)]">Net</span>
          <p className={`text-sm font-semibold ${totalIncome + totalExpenses >= 0 ? "text-[hsl(142_72%_45%)]" : "text-[hsl(0_84%_60%)]"}`}>
            {formatCurrency(totalIncome + totalExpenses)}
          </p>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="text-[11px] text-[hsl(0_0%_45%)] uppercase tracking-wider border-b border-[hsl(220_15%_20%)]">
              <th className="text-left font-medium py-2 pr-2">Date</th>
              <th className="text-left font-medium py-2 pr-2">Description</th>
              <th className="text-left font-medium py-2 pr-2 hidden sm:table-cell">Merchant</th>
              <th className="text-left font-medium py-2 pr-2 hidden md:table-cell">Category</th>
              <th className="text-right font-medium py-2 pl-2">Amount</th>
              <th className="w-8 py-2" />
            </tr>
          </thead>
          <tbody>
            {transactions.map((t, index) => (
              <motion.tr
                key={t.id}
                initial={{ opacity: 0, y: -5 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: index * 0.03 }}
                className="border-b border-[hsl(220_15%_16%)] hover:bg-[hsl(220_15%_16%/0.4)] transition-colors"
              >
                <td className="py-2.5 pr-2 text-[hsl(0_0%_50%)] text-xs whitespace-nowrap">{formatDate(t.date)}</td>
                <td className="py-2.5 pr-2 text-white font-medium">{t.description}</td>
                <td className="py-2.5 pr-2 text-[hsl(0_0%_50%)] text-xs hidden sm:table-cell">{t.merchant}</td>
                <td className="py-2.5 pr-2 hidden md:table-cell">
                  <span className="px-1.5 py-0.5 rounded text-[10px] bg-[hsl(220_15%_20%)] text-[hsl(0_0%_55%)]">
                    {t.category}
                  </span>
                </td>
                <td className={`py-2.5 pl-2 text-right font-medium whitespace-nowrap ${t.type === "income" ? "text-[hsl(142_72%_45%)]" : "text-white"}`}>
                  <div className="flex items-center justify-end gap-1">
                    {t.type === "income" ? <ArrowUpRight size={12} /> : <ArrowDownRight size={12} />}
                    {t.type === "income" ? "+" : ""}{formatCurrency(Math.abs(t.amount_cents))}
                  </div>
                </td>
                <td className="py-2.5 pl-2">
                  {onDelete && (
                    <button
                      onClick={() => onDelete(t.id)}
                      className="w-6 h-6 rounded flex items-center justify-center text-[hsl(0_0%_35%)] hover:text-[hsl(0_84%_60%)] hover:bg-[hsl(0_84%_60%/0.1)] transition-colors"
                    >
                      <Trash2 size={12} />
                    </button>
                  )}
                </td>
              </motion.tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
