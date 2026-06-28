import { useAIInsights } from "../hooks/useDashboardData";
import { Card } from "../../../components/ui/Card";
import { Skeleton } from "../../../components/ui/Skeleton";
import { Lightbulb, AlertTriangle, Award } from "lucide-react";

const iconMap = {
  tip: Lightbulb,
  alert: AlertTriangle,
  achievement: Award,
};

const colorMap = {
  tip: "hsl(220, 90%, 56%)",
  alert: "hsl(38, 92%, 50%)",
  achievement: "hsl(142, 72%, 45%)",
};

export function AIInsightsWidget() {
  const { data, isLoading } = useAIInsights();

  if (isLoading) return <Card className="h-full"><Skeleton className="h-full" /></Card>;

  return (
    <Card className="h-full">
      <div className="flex items-center justify-between mb-3">
        <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider">AI Insights</p>
        <Lightbulb size={14} className="text-[hsl(38_92%_50%)]" />
      </div>
      <div className="space-y-2">
        {data?.map((insight) => {
          const Icon = iconMap[insight.type];
          const color = colorMap[insight.type];
          return (
            <div key={insight.id} className="flex gap-2.5 p-2 rounded-lg bg-[hsl(220_15%_16%/0.5)]">
              <div className="mt-0.5 flex-shrink-0">
                <Icon size={14} style={{ color }} />
              </div>
              <div>
                <p className="text-xs text-white leading-relaxed">{insight.message}</p>
              </div>
            </div>
          );
        })}
      </div>
    </Card>
  );
}
