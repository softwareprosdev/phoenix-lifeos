import { useCashFlow } from "../hooks/useDashboardData";
import { Card } from "../../../components/ui/Card";
import { Skeleton } from "../../../components/ui/Skeleton";
import { BarChart, Bar, ResponsiveContainer, XAxis, YAxis, Tooltip } from "recharts";
import { formatCurrency } from "../../../lib/utils";

export function CashFlowWidget() {
  const { data, isLoading } = useCashFlow();

  if (isLoading) return <Card className="h-full"><Skeleton className="h-full" /></Card>;

  return (
    <Card className="h-full">
      <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Cash Flow</p>
      <div className="h-28">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart data={data}>
            <XAxis dataKey="month" tick={{ fontSize: 10, fill: "hsl(0, 0%, 50%)" }} axisLine={false} tickLine={false} />
            <YAxis hide />
            <Tooltip
              contentStyle={{ background: "hsl(220, 15%, 14%)", border: "1px solid hsl(220, 15%, 22%)", borderRadius: 8, fontSize: 12 }}
              formatter={(value: number) => [formatCurrency(value), undefined]}
              labelStyle={{ color: "hsl(0, 0%, 60%)" }}
            />
            <Bar dataKey="income" fill="hsl(142, 72%, 45%)" radius={[2, 2, 0, 0]} maxBarSize={12} />
            <Bar dataKey="expenses" fill="hsl(0, 84%, 60%)" radius={[2, 2, 0, 0]} maxBarSize={12} />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </Card>
  );
}
