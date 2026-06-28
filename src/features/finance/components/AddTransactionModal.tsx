import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { Button } from "../../../components/ui/Button";
import toast from "react-hot-toast";

const schema = z.object({
  description: z.string().min(1, "Description is required"),
  merchant: z.string().min(1, "Merchant is required"),
  category: z.string().min(1, "Category is required"),
  amount_cents: z.coerce.number().int("Must be whole number").refine((v) => v > 0, "Must be positive"),
  type: z.enum(["income", "expense"]),
  date: z.string().min(1, "Date is required"),
});

type FormData = z.infer<typeof schema>;

interface AddTransactionModalProps {
  open: boolean;
  onClose: () => void;
}

export function AddTransactionModal({ open, onClose }: AddTransactionModalProps) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
    defaultValues: { date: new Date().toISOString().split("T")[0], type: "expense" },
  });

  const onSubmit = (data: FormData) => {
    console.log("Add transaction:", data);
    toast.success("Transaction added successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Transaction">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="description" label="Description" error={errors.description?.message} {...register("description")} />
        <Input id="merchant" label="Merchant" error={errors.merchant?.message} {...register("merchant")} />
        <Input id="category" label="Category" error={errors.category?.message} {...register("category")} />
        <Input id="amount_cents" label="Amount (cents)" type="number" error={errors.amount_cents?.message} {...register("amount_cents")} />
        <Select
          id="type"
          label="Type"
          options={[
            { value: "income", label: "Income" },
            { value: "expense", label: "Expense" },
          ]}
          {...register("type")}
        />
        <Input id="date" label="Date" type="date" error={errors.date?.message} {...register("date")} />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Add Transaction</Button>
        </div>
      </form>
    </Modal>
  );
}
