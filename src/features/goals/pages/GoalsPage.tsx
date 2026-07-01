import { useState } from "react";
import { motion } from "framer-motion";
import { useGoals } from "../hooks/useGoalsData";
import { Button } from "../../../components/ui/Button";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import toast from "react-hot-toast";
import { Plus, Target } from "lucide-react";
import { formatCurrency } from "../../../lib/utils";

const schema = z.object({
  name: z.string().min(1, "Name is required"),
  target_amount_cents: z.coerce.number().int().positive("Target must be positive"),
  current_amount_cents: z.coerce.number().int().min(0, "Current must be >= 0"),
  deadline: z.string().min(1, "Deadline is required"),
  category: z.enum(["savings", "investment", "debt", "purchase", "emergency", "retirement"]),
});

type FormData = z.infer<typeof schema>;

function AddGoalModal({ open, onClose }: { open: boolean; onClose: () => void }) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
    defaultValues: { current_amount_cents: 0, category: "savings" },
  });

  const onSubmit = (data: FormData) => {
    console.log("Add goal:", data);
    toast.success("Goal created successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Goal">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="name" label="Goal Name" error={errors.name?.message} {...register("name")} />
        <Input id="target_amount_cents" label="Target Amount (cents)" type="number" error={errors.target_amount_cents?.message} {...register("target_amount_cents")} />
        <Input id="current_amount_cents" label="Current Amount (cents)" type="number" error={errors.current_amount_cents?.message} {...register("current_amount_cents")} />
        <Input id="deadline" label="Deadline" type="date" error={errors.deadline?.message} {...register("deadline")} />
        <Select
          id="category"
          label="Category"
          options={[
            { value: "savings", label: "Savings" },
            { value: "investment", label: "Investment" },
            { value: "debt", label: "Debt" },
            { value: "purchase", label: "Purchase" },
            { value: "emergency", label: "Emergency" },
            { value: "retirement", label: "Retirement" },
          ]}
          {...register("category")}
        />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Create Goal</Button>
        </div>
      </form>
    </Modal>
  );
}

const GOAL_COLORS = ["hsl(220, 90%, 56%)", "hsl(142, 72%, 45%)", "hsl(38, 92%, 50%)", "hsl(0, 84%, 60%)", "hsl(280, 80%, 60%)", "hsl(196, 85%, 50%)"];
const goalColor = (id: string) => GOAL_COLORS[id.charCodeAt(0) % GOAL_COLORS.length];

export default function GoalsPage() {
  const { data: goals, isLoading } = useGoals();
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

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-sm font-semibold text-white">Financial Goals</h2>
        <Button size="sm" onClick={() => setShowAdd(true)}>
          <Plus size={14} /> Add Goal
        </Button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {goals?.map((goal) => {
          const progress = goal.target_amount_cents > 0 ? (goal.current_amount_cents / goal.target_amount_cents) * 100 : 0;
          const remaining = goal.target_amount_cents - goal.current_amount_cents;
          const deadlineDate = goal.deadline ? new Date(goal.deadline) : null;
          const daysLeft = deadlineDate ? Math.max(0, Math.ceil((deadlineDate.getTime() - Date.now()) / (1000 * 60 * 60 * 24))) : 0;
          const monthlySuggested = daysLeft > 0 ? remaining / (daysLeft / 30) : remaining;

          return (
            <motion.div
              key={goal.id}
              layout
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] backdrop-blur-sm border border-[hsl(220_15%_22%/0.6)]"
            >
              <div className="flex items-center gap-2.5 mb-3">
                <div className="w-9 h-9 rounded-lg flex items-center justify-center" style={{ background: `${goalColor(goal.id)}20` }}>
                  <Target size={16} style={{ color: goalColor(goal.id) }} />
                </div>
                <div className="min-w-0">
                  <p className="text-sm font-semibold text-white truncate">{goal.name}</p>
                  <p className="text-[10px] text-[hsl(0_0%_45%)] capitalize">{goal.goal_type.replace(/_/g, " ")}</p>
                </div>
              </div>

              {/* Progress Circle */}
              <div className="flex items-center gap-4 mb-3">
                <div className="relative w-16 h-16 flex-shrink-0">
                  <svg className="w-16 h-16 -rotate-90" viewBox="0 0 36 36">
                    <circle cx="18" cy="18" r="15.5" fill="none" stroke="hsl(220, 15%, 22%)" strokeWidth="3" />
                    <circle
                      cx="18" cy="18" r="15.5"
                      fill="none"
                      stroke={goalColor(goal.id)}
                      strokeWidth="3"
                      strokeDasharray={`${Math.min(progress, 100)} ${100 - Math.min(progress, 100)}`}
                      strokeLinecap="round"
                      style={{ transition: "stroke-dasharray 0.6s ease-out" }}
                    />
                  </svg>
                  <div className="absolute inset-0 flex items-center justify-center">
                    <span className="text-xs font-bold text-white">{Math.round(progress)}%</span>
                  </div>
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between text-xs mb-1">
                    <span className="text-[hsl(0_0%_50%)]">Target</span>
                    <span className="text-white font-medium">{formatCurrency(goal.target_amount_cents)}</span>
                  </div>
                  <div className="flex justify-between text-xs">
                    <span className="text-[hsl(0_0%_50%)]">Current</span>
                    <span className="text-white font-medium">{formatCurrency(goal.current_amount_cents)}</span>
                  </div>
                </div>
              </div>

              {daysLeft > 0 && remaining > 0 && (
                <div className="p-2 rounded bg-[hsl(220_15%_16%/0.5)]">
                  <div className="flex justify-between text-[10px]">
                    <span className="text-[hsl(0_0%_45%)]">Monthly suggestion</span>
                    <span className="text-[hsl(220_90%_56%)] font-medium">{formatCurrency(Math.round(monthlySuggested))}/mo</span>
                  </div>
                  <div className="flex justify-between text-[10px]">
                    <span className="text-[hsl(0_0%_45%)]">Days remaining</span>
                    <span className="text-white">{daysLeft} days</span>
                  </div>
                </div>
              )}
            </motion.div>
          );
        })}
      </div>

      {(!goals || goals.length === 0) && (
        <div className="text-center py-16">
          <Target size={32} className="mx-auto text-[hsl(0_0%_30%)] mb-3" />
          <p className="text-sm text-[hsl(0_0%_45%)]">No goals set yet. Start by adding a financial goal.</p>
        </div>
      )}

      <AddGoalModal open={showAdd} onClose={() => setShowAdd(false)} />
    </motion.div>
  );
}
