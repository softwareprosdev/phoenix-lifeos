import { useState } from "react";
import { useRecentTransactions } from "../hooks/useDashboardData";
import { Card } from "../../../components/ui/Card";
import { Skeleton } from "../../../components/ui/Skeleton";
import { formatCurrency, formatDateShort } from "../../../lib/utils";
import { ArrowUpRight, ArrowDownRight } from "lucide-react";

export function RecentTransactionsWidget() {
  const { data, isLoading } = useRecentTransactions();
  const [showAll, setShowAll] = useState(false);
  const displayed = showAll ? data : data?.slice(0, 4);

  if (isLoading) return <Card className="h-full"><Skeleton className="h-full" /></Card>;

  return (
    <Card className="h-full">
      <div className="flex items-center justify-between mb-3">
        <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider">Recent Transactions</p>
        {data && data.length > 4 && (
          <button onClick={() => setShowAll(!showAll)} className="text-[11px] text-[hsl(220_90%_56%)] hover:text-[hsl(220_90%_70%)]">
            {showAll ? "Show Less" : "View All"}
          </button>
        )}
      </div>
      <div className="space-y-1">
        {displayed?.map((t) => (
          <div key={t.id} className="flex items-center justify-between py-1.5 border-b border-[hsl(220_15%_18%/0.5)] last:border-0">
            <div className="flex items-center gap-2.5 min-w-0">
              <div className={`w-7 h-7 rounded-lg flex items-center justify-center flex-shrink-0 ${t.type === "income" ? "bg-[hsl(142_72%_45%/0.15)]" : "bg-[hsl(0_84%_60%/0.15)]"}`}>
                {t.type === "income" ? <ArrowUpRight size={12} className="text-[hsl(142_72%_45%)]" /> : <ArrowDownRight size={12} className="text-[hsl(0_84%_60%)]" />}
              </div>
              <div className="min-w-0">
                <p className="text-sm text-white truncate">{t.description}</p>
                <p className="text-[10px] text-[hsl(0_0%_45%)]">{formatDateShort(t.date)} · {t.category}</p>
              </div>
            </div>
            <span className={`text-sm font-medium flex-shrink-0 ml-2 ${t.type === "income" ? "text-[hsl(142_72%_45%)]" : "text-white"}`}>
              {t.type === "income" ? "+" : ""}{formatCurrency(Math.abs(t.amountCents))}
            </span>
          </div>
        ))}
      </div>
    </Card>
  );
}
