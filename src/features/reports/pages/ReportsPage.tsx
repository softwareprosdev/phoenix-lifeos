import { useState } from "react";
import { motion } from "framer-motion";
import { useReports } from "../hooks/useReportsData";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Button } from "../../../components/ui/Button";
import { FileText, Download, BarChart3, TrendingUp, Target, Receipt, Plus } from "lucide-react";
import { formatDate } from "../../../lib/utils";

const typeIcons: Record<string, React.ElementType> = {
  spending: Receipt,
  income: TrendingUp,
  net_worth: BarChart3,
  budget: Target,
  tax: FileText,
  custom: FileText,
};

const typeColors: Record<string, string> = {
  spending: "hsl(0, 84%, 60%)",
  income: "hsl(142, 72%, 45%)",
  net_worth: "hsl(220, 90%, 56%)",
  budget: "hsl(38, 92%, 50%)",
  tax: "hsl(280, 80%, 60%)",
  custom: "hsl(196, 85%, 50%)",
};

export default function ReportsPage() {
  const { data: reports, isLoading } = useReports();
  const [filter, setFilter] = useState("all");

  if (isLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="grid grid-cols-2 gap-4">
          {Array.from({ length: 4 }).map((_, i) => <CardSkeleton key={i} />)}
        </div>
      </div>
    );
  }

  const filtered = filter === "all" ? reports : reports?.filter((r) => r.type === filter);

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-sm font-semibold text-white">Reports</h2>
        <Button size="sm">
          <Plus size={14} /> Generate Report
        </Button>
      </div>

      {/* Type Filter */}
      <div className="flex gap-1.5 mb-4 flex-wrap">
        {["all", "spending", "income", "net_worth", "budget", "tax"].map((type) => (
          <button
            key={type}
            onClick={() => setFilter(type)}
            className={`px-3 py-1.5 rounded-lg text-xs font-medium capitalize transition-colors ${
              filter === type ? "bg-[hsl(220_90%_56%/0.2)] text-[hsl(220_90%_70%)]" : "bg-[hsl(220_15%_16%)] text-[hsl(0_0%_50%)] hover:text-white"
            }`}
          >
            {type === "net_worth" ? "Net Worth" : type}
          </button>
        ))}
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        {filtered?.map((report, idx) => {
          const Icon = typeIcons[report.type] ?? FileText;
          const color = typeColors[report.type] ?? "hsl(220, 90%, 56%)";
          return (
            <motion.div
              key={report.id}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: idx * 0.03 }}
              className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] backdrop-blur-sm border border-[hsl(220_15%_22%/0.6)] hover:border-[hsl(220_90%_56%/0.3)] transition-colors cursor-pointer"
            >
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-center gap-2.5">
                  <div className="w-9 h-9 rounded-lg flex items-center justify-center" style={{ background: `${color}20` }}>
                    <Icon size={16} style={{ color }} />
                  </div>
                  <div>
                    <p className="text-sm font-semibold text-white">{report.name}</p>
                    <p className="text-[10px] text-[hsl(0_0%_45%)] capitalize">{report.period} · {report.type}</p>
                  </div>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-[10px] text-[hsl(0_0%_40%)]">Generated {formatDate(report.generated_at)}</span>
                <button className="flex items-center gap-1 text-[10px] text-[hsl(220_90%_56%)] hover:text-[hsl(220_90%_70%)]">
                  <Download size={12} /> Download
                </button>
              </div>
            </motion.div>
          );
        })}
      </div>

      {(!filtered || filtered.length === 0) && (
        <div className="text-center py-16">
          <BarChart3 size={32} className="mx-auto text-[hsl(0_0%_30%)] mb-3" />
          <p className="text-sm text-[hsl(0_0%_45%)]">No reports found. Generate your first report to get started.</p>
        </div>
      )}
    </motion.div>
  );
}
