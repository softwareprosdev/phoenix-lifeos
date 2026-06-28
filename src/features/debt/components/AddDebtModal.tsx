import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Button } from "../../../components/ui/Button";
import toast from "react-hot-toast";

const schema = z.object({
  name: z.string().min(1, "Name is required"),
  creditor: z.string().min(1, "Creditor is required"),
  balance_cents: z.coerce.number().int().positive("Balance must be positive"),
  apr: z.coerce.number().min(0, "APR must be positive").max(100, "APR must be <= 100"),
  min_payment_cents: z.coerce.number().int().positive("Min payment must be positive"),
  original_balance_cents: z.coerce.number().int().positive(),
});

type FormData = z.infer<typeof schema>;

interface AddDebtModalProps {
  open: boolean;
  onClose: () => void;
}

export function AddDebtModal({ open, onClose }: AddDebtModalProps) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
  });

  const onSubmit = (data: FormData) => {
    console.log("Add debt:", data);
    toast.success("Debt added successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Debt">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="name" label="Debt Name" error={errors.name?.message} {...register("name")} />
        <Input id="creditor" label="Creditor" error={errors.creditor?.message} {...register("creditor")} />
        <Input id="balance_cents" label="Balance (cents)" type="number" error={errors.balance_cents?.message} {...register("balance_cents")} />
        <Input id="apr" label="APR (%)" type="number" step="0.01" error={errors.apr?.message} {...register("apr")} />
        <Input id="min_payment_cents" label="Min Payment (cents)" type="number" error={errors.min_payment_cents?.message} {...register("min_payment_cents")} />
        <Input id="original_balance_cents" label="Original Balance (cents)" type="number" error={errors.original_balance_cents?.message} {...register("original_balance_cents")} />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Add Debt</Button>
        </div>
      </form>
    </Modal>
  );
}
