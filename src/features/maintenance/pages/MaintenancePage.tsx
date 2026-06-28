import { useState } from "react";
import { motion } from "framer-motion";
import { useMaintenanceTasks, useWarranties } from "../hooks/useMaintenanceData";
import { Button } from "../../../components/ui/Button";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import toast from "react-hot-toast";
import { Plus, Wrench, Shield, Home, Car, Smartphone, Clock } from "lucide-react";
import { formatDate, formatCurrency } from "../../../lib/utils";

const taskSchema = z.object({
  title: z.string().min(1, "Title is required"),
  description: z.string().min(1, "Description is required"),
  category: z.enum(["home", "vehicle", "electronics", "other"]),
  frequency: z.enum(["once", "daily", "weekly", "monthly", "quarterly", "yearly"]),
  cost_cents: z.coerce.number().int().min(0, "Cost must be >= 0"),
});

type TaskFormData = z.infer<typeof taskSchema>;

function AddTaskModal({ open, onClose }: { open: boolean; onClose: () => void }) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<TaskFormData>({
    resolver: zodResolver(taskSchema),
    defaultValues: { category: "home", frequency: "monthly", cost_cents: 0 },
  });

  const onSubmit = (data: TaskFormData) => {
    console.log("Add task:", data);
    toast.success("Task added successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Maintenance Task">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="title" label="Title" error={errors.title?.message} {...register("title")} />
        <Input id="description" label="Description" error={errors.description?.message} {...register("description")} />
        <Select
          id="category"
          label="Category"
          options={[
            { value: "home", label: "Home" },
            { value: "vehicle", label: "Vehicle" },
            { value: "electronics", label: "Electronics" },
            { value: "other", label: "Other" },
          ]}
          {...register("category")}
        />
        <Select
          id="frequency"
          label="Frequency"
          options={[
            { value: "once", label: "One Time" },
            { value: "daily", label: "Daily" },
            { value: "weekly", label: "Weekly" },
            { value: "monthly", label: "Monthly" },
            { value: "quarterly", label: "Quarterly" },
            { value: "yearly", label: "Yearly" },
          ]}
          {...register("frequency")}
        />
        <Input id="cost_cents" label="Cost (cents)" type="number" error={errors.cost_cents?.message} {...register("cost_cents")} />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Add Task</Button>
        </div>
      </form>
    </Modal>
  );
}

export default function MaintenancePage() {
  const { data: tasks, isLoading } = useMaintenanceTasks();
  const { data: warranties } = useWarranties();
  const [showAdd, setShowAdd] = useState(false);
  const [tab, setTab] = useState<"tasks" | "warranties">("tasks");

  if (isLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="grid grid-cols-2 gap-4">
          {Array.from({ length: 3 }).map((_, i) => <CardSkeleton key={i} />)}
        </div>
      </div>
    );
  }

  const categoryIcons: Record<string, React.ElementType> = { home: Home, vehicle: Car, electronics: Smartphone, other: Wrench };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-sm font-semibold text-white">Maintenance</h2>
        {tab === "tasks" && (
          <Button size="sm" onClick={() => setShowAdd(true)}>
            <Plus size={14} /> Add Task
          </Button>
        )}
      </div>

      <div className="flex gap-1.5 mb-4">
        <button
          onClick={() => setTab("tasks")}
          className={`px-3 py-1.5 rounded-lg text-xs font-medium transition-colors ${tab === "tasks" ? "bg-[hsl(220_90%_56%/0.2)] text-[hsl(220_90%_70%)]" : "bg-[hsl(220_15%_16%)] text-[hsl(0_0%_50%)] hover:text-white"}`}
        >
          <Wrench size={12} className="inline mr-1" /> Tasks
        </button>
        <button
          onClick={() => setTab("warranties")}
          className={`px-3 py-1.5 rounded-lg text-xs font-medium transition-colors ${tab === "warranties" ? "bg-[hsl(220_90%_56%/0.2)] text-[hsl(220_90%_70%)]" : "bg-[hsl(220_15%_16%)] text-[hsl(0_0%_50%)] hover:text-white"}`}
        >
          <Shield size={12} className="inline mr-1" /> Warranties
        </button>
      </div>

      {tab === "tasks" ? (
        <div className="space-y-3">
          {tasks?.map((task) => {
            const Icon = categoryIcons[task.category] ?? Wrench;
            return (
              <div key={task.id} className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-lg bg-[hsl(220_15%_18%)] flex items-center justify-center">
                    <Icon size={15} className="text-[hsl(0_0%_50%)]" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-semibold text-white">{task.title}</p>
                    <p className="text-[10px] text-[hsl(0_0%_45%)]">{task.description}</p>
                    <div className="flex items-center gap-3 mt-1">
                      <span className="text-[10px] text-[hsl(0_0%_40%)]">Next: {formatDate(task.next_due)}</span>
                      <span className="text-[10px] text-[hsl(0_0%_40%)] capitalize">{task.frequency}</span>
                      {task.cost_cents > 0 && <span className="text-[10px] text-[hsl(0_0%_40%)]">{formatCurrency(task.cost_cents)}</span>}
                    </div>
                  </div>
                  <div className="flex-shrink-0">
                    <Clock size={14} className="text-[hsl(38_92%_50%)]" />
                  </div>
                </div>
              </div>
            );
          })}
          {(!tasks || tasks.length === 0) && (
            <div className="text-center py-12 text-sm text-[hsl(0_0%_45%)]">
              No maintenance tasks. Click "Add Task" to get started.
            </div>
          )}
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
          {warranties?.map((w) => (
            <div key={w.id} className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
              <p className="text-sm font-semibold text-white">{w.item}</p>
              <p className="text-[10px] text-[hsl(0_0%_45%)]">{w.provider}</p>
              <div className="flex justify-between mt-2 text-xs">
                <span className="text-[hsl(0_0%_50%)]">Expires</span>
                <span className="text-white">{formatDate(w.expires_at)}</span>
              </div>
              {w.cost_cents > 0 && (
                <div className="flex justify-between text-xs mt-0.5">
                  <span className="text-[hsl(0_0%_50%)]">Cost</span>
                  <span className="text-white">{formatCurrency(w.cost_cents)}</span>
                </div>
              )}
            </div>
          ))}
        </div>
      )}

      <AddTaskModal open={showAdd} onClose={() => setShowAdd(false)} />
    </motion.div>
  );
}
