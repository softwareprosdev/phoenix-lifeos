import { useDebtSummary } from "../hooks/useDashboardData";
import { Card } from "../../../components/ui/Card";
import { Skeleton } from "../../../components/ui/Skeleton";
import { formatCurrency } from "../../../lib/utils";
import { TrendingDown } from "lucide-react";

export function DebtOverviewWidget() {
  const { data, isLoading } = useDebtSummary();

  if (isLoading) return <Card className="h-full"><Skeleton className="h-full" /></Card>;

  return (
    <Card className="h-full">
      <div className="flex items-center justify-between mb-3">
        <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider">Debt Overview</p>
        <TrendingDown size={14} className="text-[hsl(0_84%_60%)]" />
      </div>
      <p className="text-2xl font-bold text-white">{formatCurrency(data?.totalDebt ?? 0)}</p>
      <div className="grid grid-cols-3 gap-2 mt-3">
        <div>
          <p className="text-[10px] text-[hsl(0_0%_50%)]">Accounts</p>
          <p className="text-sm font-semibold text-white">{data?.debtCount}</p>
        </div>
        <div>
          <p className="text-[10px] text-[hsl(0_0%_50%)]">Avg APR</p>
          <p className="text-sm font-semibold text-[hsl(0_84%_60%)]">{data?.avgApr.toFixed(1)}%</p>
        </div>
        <div>
          <p className="text-[10px] text-[hsl(0_0%_50%)]">Min Payment</p>
          <p className="text-sm font-semibold text-white">{formatCurrency(data?.totalMinPayments ?? 0)}</p>
        </div>
      </div>
    </Card>
  );
}
