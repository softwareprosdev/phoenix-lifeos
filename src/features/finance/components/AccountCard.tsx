import { motion } from "framer-motion";
import type { Account } from "../types";
import { formatCurrency } from "../../../lib/utils";
import { Badge } from "../../../components/ui/Badge";
import { Wallet, PiggyBank, CreditCard, TrendingUp, HandCoins, DollarSign } from "lucide-react";

const iconMap: Record<string, React.ElementType> = {
  checking: Wallet,
  savings: PiggyBank,
  credit: CreditCard,
  investment: TrendingUp,
  loan: HandCoins,
  cash: DollarSign,
};

const colorMap: Record<string, string> = {
  checking: "hsl(220, 90%, 56%)",
  savings: "hsl(142, 72%, 45%)",
  credit: "hsl(0, 84%, 60%)",
  investment: "hsl(280, 80%, 60%)",
  loan: "hsl(38, 92%, 50%)",
  cash: "hsl(196, 85%, 50%)",
};

interface AccountCardProps {
  account: Account;
  selected: boolean;
  onClick: () => void;
}

export function AccountCard({ account, selected, onClick }: AccountCardProps) {
  const Icon = iconMap[account.account_type] ?? Wallet;
  const color = colorMap[account.account_type] ?? "hsl(220, 90%, 56%)";
  const isNegative = account.balance_cents < 0;

  return (
    <motion.div
      layout
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      onClick={onClick}
      className={`rounded-lg p-4 cursor-pointer transition-all duration-200 border ${
        selected
          ? "bg-[hsl(220_90%_56%/0.1)] border-[hsl(220_90%_56%/0.4)]"
          : "bg-[hsl(220_15%_14%/0.7)] backdrop-blur-sm border-[hsl(220_15%_22%/0.6)] hover:border-[hsl(220_90%_56%/0.3)]"
      }`}
    >
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2.5">
          <div className="w-9 h-9 rounded-lg flex items-center justify-center" style={{ background: `${color}20` }}>
            <Icon size={16} style={{ color }} />
          </div>
          <div>
            <p className="text-sm font-semibold text-white">{account.name}</p>
            <p className="text-[10px] text-[hsl(0_0%_45%)]">{account.institution}</p>
          </div>
        </div>
        <Badge variant={account.account_type === "credit" ? "danger" : "default"}>
          {account.account_type}
        </Badge>
      </div>
      <p className={`text-lg font-bold ${isNegative ? "text-[hsl(0_84%_60%)]" : "text-white"}`}>
        {isNegative ? "-" : ""}{formatCurrency(Math.abs(account.balance_cents))}
      </p>
    </motion.div>
  );
}
