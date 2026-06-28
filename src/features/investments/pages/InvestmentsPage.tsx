import { useState } from "react";
import { motion } from "framer-motion";
import { useInvestments, usePortfolioSummary } from "../hooks/useInvestmentData";
import { Button } from "../../../components/ui/Button";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import toast from "react-hot-toast";
import { Plus, TrendingUp, TrendingDown } from "lucide-react";
import { formatCurrency, formatPercent } from "../../../lib/utils";
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip } from "recharts";

const schema = z.object({
  symbol: z.string().min(1, "Symbol is required"),
  name: z.string().min(1, "Name is required"),
  shares: z.coerce.number().positive("Shares must be positive"),
  current_price_cents: z.coerce.number().int().positive("Price must be positive"),
  asset_class: z.enum(["stock", "etf", "mutual_fund", "crypto", "bond", "real_estate"]),
});

type FormData = z.infer<typeof schema>;

function AddInvestmentModal({ open, onClose }: { open: boolean; onClose: () => void }) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
  });

  const onSubmit = (data: FormData) => {
    console.log("Add investment:", data);
    toast.success("Investment added successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Investment">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="symbol" label="Symbol" error={errors.symbol?.message} {...register("symbol")} />
        <Input id="name" label="Name" error={errors.name?.message} {...register("name")} />
        <Input id="shares" label="Shares" type="number" step="0.001" error={errors.shares?.message} {...register("shares")} />
        <Input id="current_price_cents" label="Current Price (cents)" type="number" error={errors.current_price_cents?.message} {...register("current_price_cents")} />
        <Select
          id="asset_class"
          label="Asset Class"
          options={[
            { value: "stock", label: "Stock" },
            { value: "etf", label: "ETF" },
            { value: "mutual_fund", label: "Mutual Fund" },
            { value: "crypto", label: "Crypto" },
            { value: "bond", label: "Bond" },
            { value: "real_estate", label: "Real Estate" },
          ]}
          {...register("asset_class")}
        />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Add Investment</Button>
        </div>
      </form>
    </Modal>
  );
}

export default function InvestmentsPage() {
  const { data: investments, isLoading } = useInvestments();
  const { data: summary } = usePortfolioSummary();
  const [showAdd, setShowAdd] = useState(false);

  if (isLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="grid grid-cols-2 gap-4">
          {Array.from({ length: 4 }).map((_, i) => <CardSkeleton key={i} />)}
        </div>
      </div>
    );
  }

  const isPositive = (summary?.totalGainCents ?? 0) >= 0;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-sm font-semibold text-white">Investment Portfolio</h2>
        <Button size="sm" onClick={() => setShowAdd(true)}>
          <Plus size={14} /> Add Investment
        </Button>
      </div>

      {/* Portfolio Summary */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 mb-6">
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Total Value</p>
          <p className="text-2xl font-bold text-white mt-1">{formatCurrency(summary?.totalValueCents ?? 0)}</p>
        </div>
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Total Gain/Loss</p>
          <div className="flex items-center gap-1.5 mt-1">
            {isPositive ? <TrendingUp size={16} className="text-[hsl(142_72%_45%)]" /> : <TrendingDown size={16} className="text-[hsl(0_84%_60%)]" />}
            <span className={`text-xl font-bold ${isPositive ? "text-[hsl(142_72%_45%)]" : "text-[hsl(0_84%_60%)]"}`}>
              {isPositive ? "+" : ""}{formatCurrency(summary?.totalGainCents ?? 0)}
            </span>
            <span className={`text-xs ${isPositive ? "text-[hsl(142_72%_45%)]" : "text-[hsl(0_84%_60%)]"}`}>
              ({isPositive ? "+" : ""}{summary?.totalGainPercent.toFixed(1)}%)
            </span>
          </div>
        </div>
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Cost Basis</p>
          <p className="text-xl font-bold text-white mt-1">{formatCurrency(summary?.totalCostCents ?? 0)}</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 mb-6">
        {/* Allocation Pie */}
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Asset Allocation</p>
          <div className="h-40">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie data={summary?.allocation ?? []} dataKey="value" nameKey="name" cx="50%" cy="50%" innerRadius={45} outerRadius={70} paddingAngle={3}>
                  {(summary?.allocation ?? []).map((entry, index) => (
                    <Cell key={index} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip
                  contentStyle={{ background: "hsl(220, 15%, 14%)", border: "1px solid hsl(220, 15%, 22%)", borderRadius: 8, fontSize: 12 }}
                  formatter={(value: number) => [formatCurrency(value), undefined]}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
          <div className="space-y-1 mt-2">
            {summary?.allocation.map((item) => (
              <div key={item.name} className="flex items-center justify-between text-xs">
                <div className="flex items-center gap-1.5">
                  <div className="w-2 h-2 rounded-full" style={{ background: item.color }} />
                  <span className="text-[hsl(0_0%_50%)]">{item.name}</span>
                </div>
                <span className="text-white">{formatCurrency(item.value)}</span>
              </div>
            ))}
          </div>
        </div>

        {/* Holdings */}
        <div className="lg:col-span-2 rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Holdings</p>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="text-[11px] text-[hsl(0_0%_45%)] uppercase tracking-wider border-b border-[hsl(220_15%_20%)]">
                  <th className="text-left font-medium py-2 pr-2">Symbol</th>
                  <th className="text-left font-medium py-2 pr-2">Name</th>
                  <th className="text-right font-medium py-2 pr-2">Shares</th>
                  <th className="text-right font-medium py-2 pr-2">Price</th>
                  <th className="text-right font-medium py-2">Value</th>
                </tr>
              </thead>
              <tbody>
                {investments?.map((inv, idx) => {
                  const value = Math.round(inv.shares * inv.current_price_cents);
                  const gain = value - inv.cost_basis_cents;
                  const gainPct = inv.cost_basis_cents > 0 ? (gain / inv.cost_basis_cents) * 100 : 0;
                  return (
                    <motion.tr
                      key={inv.id}
                      initial={{ opacity: 0, y: -5 }}
                      animate={{ opacity: 1, y: 0 }}
                      transition={{ delay: idx * 0.03 }}
                      className="border-b border-[hsl(220_15%_16%)] hover:bg-[hsl(220_15%_16%/0.4)] transition-colors"
                    >
                      <td className="py-2.5 pr-2 font-semibold text-white">{inv.symbol}</td>
                      <td className="py-2.5 pr-2 text-[hsl(0_0%_50%)] text-xs">{inv.name}</td>
                      <td className="py-2.5 pr-2 text-right text-white">{inv.shares}</td>
                      <td className="py-2.5 pr-2 text-right text-[hsl(0_0%_50%)]">{formatCurrency(inv.current_price_cents)}</td>
                      <td className="py-2.5 text-right">
                        <p className="text-white font-medium">{formatCurrency(value)}</p>
                        <p className={`text-[10px] ${gain >= 0 ? "text-[hsl(142_72%_45%)]" : "text-[hsl(0_84%_60%)]"}`}>
                          {gain >= 0 ? "+" : ""}{formatPercent(gainPct)}
                        </p>
                      </td>
                    </motion.tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <AddInvestmentModal open={showAdd} onClose={() => setShowAdd(false)} />
    </motion.div>
  );
}
