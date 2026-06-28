import { useNetWorth } from "../hooks/useDashboardData";
import { Card } from "../../../components/ui/Card";
import { Skeleton } from "../../../components/ui/Skeleton";
import { AreaChart, Area, ResponsiveContainer, XAxis, YAxis, Tooltip } from "recharts";
import { formatCurrency } from "../../../lib/utils";
import { TrendingUp } from "lucide-react";

export function NetWorthWidget() {
  const { data, isLoading } = useNetWorth();

  if (isLoading) return <Card className="h-full"><Skeleton className="h-full" /></Card>;

  const current = data?.[data.length - 1]?.value ?? 0;
  const previous = data?.[0]?.value ?? 0;
  const change = current - previous;
  const changePercent = previous > 0 ? (change / previous) * 100 : 0;

  return (
    <Card className="h-full">
      <div className="flex items-center justify-between mb-3">
        <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider">Net Worth</p>
        <TrendingUp size={14} className="text-[hsl(142_72%_45%)]" />
      </div>
      <p className="text-2xl font-bold text-white">{formatCurrency(current)}</p>
      <div className="flex items-center gap-1.5 mt-0.5">
        <span className={`text-xs font-medium ${change >= 0 ? "text-[hsl(142_72%_45%)]" : "text-[hsl(0_84%_60%)]"}`}>
          {change >= 0 ? "+" : ""}{formatCurrency(change)} ({changePercent >= 0 ? "+" : ""}{changePercent.toFixed(1)}%)
        </span>
      </div>
      <div className="h-20 mt-3">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data}>
            <defs>
              <linearGradient id="netWorthGradient" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor="hsl(220, 90%, 56%)" stopOpacity={0.3} />
                <stop offset="100%" stopColor="hsl(220, 90%, 56%)" stopOpacity={0} />
              </linearGradient>
            </defs>
            <XAxis dataKey="date" hide />
            <YAxis hide domain={["dataMin - 500000", "dataMax + 500000"]} />
            <Tooltip
              contentStyle={{ background: "hsl(220, 15%, 14%)", border: "1px solid hsl(220, 15%, 22%)", borderRadius: 8, fontSize: 12 }}
              formatter={(value: number) => [formatCurrency(value), "Value"]}
              labelStyle={{ color: "hsl(0, 0%, 60%)" }}
            />
            <Area type="monotone" dataKey="value" stroke="hsl(220, 90%, 56%)" strokeWidth={2} fill="url(#netWorthGradient)" />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </Card>
  );
}
