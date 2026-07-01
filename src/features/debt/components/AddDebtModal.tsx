import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { Button } from "../../../components/ui/Button";
import toast from "react-hot-toast";

const schema = z.object({
  name: z.string().min(1, "Name is required"),
  creditor: z.string().min(1, "Creditor is required").optional(),
  debt_type: z.enum(["credit_card", "student_loan", "personal_loan", "medical", "auto", "other"]),
  principal_cents: z.coerce.number().int().positive("Principal must be positive"),
  interest_rate: z.coerce.number().min(0, "APR must be positive").max(100, "APR must be <= 100"),
  minimum_payment_cents: z.coerce.number().int().positive("Min payment must be positive"),
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
        <Select id="debt_type" label="Debt Type" options={[
          { value: "credit_card", label: "Credit Card" },
          { value: "student_loan", label: "Student Loan" },
          { value: "personal_loan", label: "Personal Loan" },
          { value: "medical", label: "Medical" },
          { value: "auto", label: "Auto" },
          { value: "other", label: "Other" },
        ]} {...register("debt_type")} />
        <Input id="principal_cents" label="Principal (cents)" type="number" error={errors.principal_cents?.message} {...register("principal_cents")} />
        <Input id="interest_rate" label="APR (%)" type="number" step="0.01" error={errors.interest_rate?.message} {...register("interest_rate")} />
        <Input id="minimum_payment_cents" label="Min Payment (cents)" type="number" error={errors.minimum_payment_cents?.message} {...register("minimum_payment_cents")} />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Add Debt</Button>
        </div>
      </form>
    </Modal>
  );
}
