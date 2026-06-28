import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { Button } from "../../../components/ui/Button";
import toast from "react-hot-toast";

const schema = z.object({
  category: z.string().min(1, "Category is required"),
  limit_cents: z.coerce.number().int().positive("Limit must be positive"),
  period: z.enum(["weekly", "monthly", "yearly"]),
});

type FormData = z.infer<typeof schema>;

interface AddBudgetModalProps {
  open: boolean;
  onClose: () => void;
}

export function AddBudgetModal({ open, onClose }: AddBudgetModalProps) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
    defaultValues: { period: "monthly" },
  });

  const onSubmit = (data: FormData) => {
    console.log("Add budget:", data);
    toast.success("Budget created successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Budget">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="category" label="Category" error={errors.category?.message} {...register("category")} />
        <Input id="limit_cents" label="Limit (cents)" type="number" error={errors.limit_cents?.message} {...register("limit_cents")} />
        <Select
          id="period"
          label="Period"
          options={[
            { value: "weekly", label: "Weekly" },
            { value: "monthly", label: "Monthly" },
            { value: "yearly", label: "Yearly" },
          ]}
          {...register("period")}
        />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Create Budget</Button>
        </div>
      </form>
    </Modal>
  );
}
