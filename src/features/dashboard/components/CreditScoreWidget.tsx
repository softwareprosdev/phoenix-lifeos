import { useCreditScoreData } from "../hooks/useDashboardData";
import { Card } from "../../../components/ui/Card";
import { Skeleton } from "../../../components/ui/Skeleton";
import { LineChart, Line, ResponsiveContainer, XAxis, Tooltip } from "recharts";

function getScoreColor(score: number) {
  if (score >= 750) return "hsl(142, 72%, 45%)";
  if (score >= 700) return "hsl(196, 85%, 50%)";
  if (score >= 650) return "hsl(38, 92%, 50%)";
  return "hsl(0, 84%, 60%)";
}

function getScoreLabel(score: number) {
  if (score >= 800) return "Exceptional";
  if (score >= 740) return "Very Good";
  if (score >= 670) return "Good";
  if (score >= 580) return "Fair";
  return "Poor";
}

export function CreditScoreWidget() {
  const { data, isLoading } = useCreditScoreData();

  if (isLoading) return <Card className="h-full"><Skeleton className="h-full" /></Card>;

  const color = getScoreColor(data?.score ?? 0);

  return (
    <Card className="h-full">
      <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Credit Score</p>
      <div className="flex items-end gap-3">
        <div>
          <p className="text-2xl font-bold" style={{ color }}>{data?.score}</p>
          <p className="text-[11px] text-[hsl(0_0%_50%)]" style={{ color }}>{getScoreLabel(data?.score ?? 0)}</p>
          {data && data.change !== 0 && (
            <p className={`text-[10px] ${data.change > 0 ? "text-[hsl(142_72%_45%)]" : "text-[hsl(0_84%_60%)]"}`}>
              {data.change > 0 ? "+" : ""}{data.change} pts
            </p>
          )}
        </div>
        <div className="flex-1 h-14">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data?.history ?? []}>
              <XAxis dataKey="date" hide />
              <Tooltip
                contentStyle={{ background: "hsl(220, 15%, 14%)", border: "1px solid hsl(220, 15%, 22%)", borderRadius: 8, fontSize: 12 }}
                formatter={(value: number) => [value, "Score"]}
                labelStyle={{ color: "hsl(0, 0%, 60%)" }}
              />
              <Line type="monotone" dataKey="score" stroke={color} strokeWidth={2} dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
    </Card>
  );
}
