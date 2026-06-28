import { useBudgetHealth } from "../hooks/useDashboardData";
import { Card } from "../../../components/ui/Card";
import { Skeleton } from "../../../components/ui/Skeleton";

export function BudgetHealthWidget() {
  const { data, isLoading } = useBudgetHealth();

  if (isLoading) return <Card className="h-full"><Skeleton className="h-full" /></Card>;

  const totalSpent = data?.reduce((s, b) => s + b.spent, 0) ?? 0;
  const totalLimit = data?.reduce((s, b) => s + b.limit, 0) ?? 0;
  const percentage = totalLimit > 0 ? (totalSpent / totalLimit) * 100 : 0;

  const getColor = (pct: number) => {
    if (pct > 100) return "hsl(0, 84%, 60%)";
    if (pct > 80) return "hsl(38, 92%, 50%)";
    return "hsl(142, 72%, 45%)";
  };

  return (
    <Card className="h-full">
      <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Budget Health</p>
      <div className="flex items-center gap-4">
        <div className="relative w-16 h-16 flex-shrink-0">
          <svg className="w-16 h-16 -rotate-90" viewBox="0 0 36 36">
            <circle cx="18" cy="18" r="15.5" fill="none" stroke="hsl(220, 15%, 22%)" strokeWidth="3" />
            <circle cx="18" cy="18" r="15.5" fill="none" stroke={getColor(percentage)} strokeWidth="3" strokeDasharray={`${percentage} ${100 - percentage}`} strokeLinecap="round" />
          </svg>
          <span className="absolute inset-0 flex items-center justify-center text-xs font-bold text-white">{Math.round(percentage)}%</span>
        </div>
        <div className="flex-1 min-w-0 space-y-1.5">
          {data?.slice(0, 3).map((b) => {
            const pct = b.limit > 0 ? (b.spent / b.limit) * 100 : 0;
            return (
              <div key={b.category}>
                <div className="flex justify-between text-[11px]">
                  <span className="text-[hsl(0_0%_60%)]">{b.category}</span>
                  <span className="text-white font-medium">{Math.round(pct)}%</span>
                </div>
                <div className="h-1 rounded-full bg-[hsl(220_15%_22%)] overflow-hidden">
                  <div className="h-full rounded-full transition-all duration-500" style={{ width: `${Math.min(pct, 100)}%`, background: getColor(pct) }} />
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </Card>
  );
}
