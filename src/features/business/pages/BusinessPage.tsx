import { useState } from "react";
import { motion } from "framer-motion";
import { useBusinessAccounts, useInvoices } from "../hooks/useBusinessData";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Button } from "../../../components/ui/Button";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Plus, Building2, FileText } from "lucide-react";
import toast from "react-hot-toast";
import { formatCurrency, formatDate } from "../../../lib/utils";
import { BarChart, Bar, ResponsiveContainer, XAxis, YAxis, Tooltip } from "recharts";

const invoiceSchema = z.object({
  number: z.string().min(1, "Invoice number is required"),
  client: z.string().min(1, "Client is required"),
  amount_cents: z.coerce.number().int().positive("Amount must be positive"),
  due_date: z.string().min(1, "Due date is required"),
});

type InvoiceFormData = z.infer<typeof invoiceSchema>;

function AddInvoiceModal({ open, onClose }: { open: boolean; onClose: () => void }) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<InvoiceFormData>({
    resolver: zodResolver(invoiceSchema),
  });

  const onSubmit = (data: InvoiceFormData) => {
    console.log("Add invoice:", data);
    toast.success("Invoice created successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Invoice">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="number" label="Invoice Number" error={errors.number?.message} {...register("number")} />
        <Input id="client" label="Client" error={errors.client?.message} {...register("client")} />
        <Input id="amount_cents" label="Amount (cents)" type="number" error={errors.amount_cents?.message} {...register("amount_cents")} />
        <Input id="due_date" label="Due Date" type="date" error={errors.due_date?.message} {...register("due_date")} />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Create Invoice</Button>
        </div>
      </form>
    </Modal>
  );
}

export default function BusinessPage() {
  const { data: accounts, isLoading: acctLoading } = useBusinessAccounts();
  const { data: invoices, isLoading: invLoading } = useInvoices();
  const [showAdd, setShowAdd] = useState(false);

  if (acctLoading || invLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="grid grid-cols-2 gap-4">
          {Array.from({ length: 4 }).map((_, i) => <CardSkeleton key={i} />)}
        </div>
      </div>
    );
  }

  const totalBalance = accounts?.reduce((s, a) => s + a.balance_cents, 0) ?? 0;
  const sentAmount = invoices?.filter((i) => i.status === "sent" || i.status === "overdue").reduce((s, i) => s + i.amount_cents, 0) ?? 0;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-sm font-semibold text-white">Business</h2>
        <Button size="sm" onClick={() => setShowAdd(true)}>
          <Plus size={14} /> Add Invoice
        </Button>
      </div>

      {/* Summary */}
      <div className="grid grid-cols-3 gap-4 mb-6">
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Total Balance</p>
          <p className="text-xl font-bold text-white mt-1">{formatCurrency(totalBalance)}</p>
        </div>
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Pending Invoices</p>
          <p className="text-xl font-bold text-[hsl(38_92%_50%)] mt-1">{formatCurrency(sentAmount)}</p>
        </div>
        <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
          <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Accounts</p>
          <p className="text-xl font-bold text-white mt-1">{accounts?.length ?? 0}</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-6">
        {/* Accounts */}
        <div>
          <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Business Accounts</p>
          <div className="space-y-3">
            {accounts?.map((acct) => (
              <div key={acct.id} className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
                <div className="flex items-center gap-2.5 mb-2">
                  <Building2 size={14} className="text-[hsl(0_0%_40%)]" />
                  <p className="text-sm font-semibold text-white">{acct.name}</p>
                  <span className="text-[10px] px-1.5 py-0.5 rounded bg-[hsl(220_15%_20%)] text-[hsl(0_0%_50%)] capitalize">{acct.type}</span>
                </div>
                <p className="text-[10px] text-[hsl(0_0%_45%)]">{acct.institution}</p>
                <p className="text-lg font-bold text-white mt-1">{formatCurrency(acct.balance_cents)}</p>
              </div>
            ))}
          </div>
        </div>

        {/* Invoices */}
        <div>
          <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Recent Invoices</p>
          <div className="space-y-3">
            {invoices?.map((inv) => {
              const statusColors: Record<string, string> = {
                paid: "text-[hsl(142_72%_45%)]",
                sent: "text-[hsl(220_90%_56%)]",
                overdue: "text-[hsl(0_84%_60%)]",
                draft: "text-[hsl(0_0%_40%)]",
              };
              return (
                <div key={inv.id} className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
                  <div className="flex items-center justify-between mb-1">
                    <div className="flex items-center gap-2">
                      <FileText size={13} className="text-[hsl(0_0%_40%)]" />
                      <p className="text-sm font-semibold text-white">{inv.number}</p>
                    </div>
                    <span className={`text-[10px] font-medium capitalize ${statusColors[inv.status]}`}>{inv.status}</span>
                  </div>
                  <p className="text-xs text-[hsl(0_0%_50%)]">{inv.client}</p>
                  <div className="flex items-center justify-between mt-1">
                    <span className="text-xs text-[hsl(0_0%_45%)]">Due {formatDate(inv.due_date)}</span>
                    <span className="text-sm font-semibold text-white">{formatCurrency(inv.amount_cents)}</span>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </div>

      <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
        <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider mb-3">Income Overview</p>
        <div className="h-32">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart
              data={[
                { month: "Jan", income: 15000 },
                { month: "Feb", income: 12000 },
                { month: "Mar", income: 18000 },
                { month: "Apr", income: 16500 },
                { month: "May", income: 21000 },
                { month: "Jun", income: 19500 },
              ]}
            >
              <XAxis dataKey="month" tick={{ fontSize: 10, fill: "hsl(0, 0%, 50%)" }} axisLine={false} tickLine={false} />
              <YAxis hide />
              <Tooltip
                contentStyle={{ background: "hsl(220, 15%, 14%)", border: "1px solid hsl(220, 15%, 22%)", borderRadius: 8, fontSize: 12 }}
                formatter={(value: number) => [formatCurrency(value * 100), "Income"]}
              />
              <Bar dataKey="income" fill="hsl(142, 72%, 45%)" radius={[4, 4, 0, 0]} maxBarSize={24} />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      <AddInvoiceModal open={showAdd} onClose={() => setShowAdd(false)} />
    </motion.div>
  );
}
