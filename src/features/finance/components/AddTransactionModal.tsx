import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { Button } from "../../../components/ui/Button";
import { useCreateTransaction } from "@/hooks";
import toast from "react-hot-toast";

const schema = z.object({
  description: z.string().min(1, "Description is required"),
  merchant: z.string().optional(),
  category_id: z.string().optional(),
  amount_cents: z.coerce.number().int("Must be whole number").refine((v) => v > 0, "Must be positive"),
  transaction_type: z.enum(["income", "expense"]),
  date: z.string().min(1, "Date is required"),
});

type FormData = z.infer<typeof schema>;

interface AddTransactionModalProps {
  open: boolean;
  onClose: () => void;
}

export function AddTransactionModal({ open, onClose }: AddTransactionModalProps) {
  const createTransaction = useCreateTransaction();
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
    defaultValues: { date: new Date().toISOString().split("T")[0], transaction_type: "expense" },
  });

  const onSubmit = async (data: FormData) => {
    try {
      await createTransaction.mutateAsync({
        account_id: "",
        ...data,
        amount_cents: data.transaction_type === "expense" ? -Math.abs(data.amount_cents) : data.amount_cents,
      });
      toast.success("Transaction added successfully");
      reset();
      onClose();
    } catch {
      toast.error("Failed to add transaction");
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Transaction">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="description" label="Description" error={errors.description?.message} {...register("description")} />
        <Input id="merchant" label="Merchant" error={errors.merchant?.message} {...register("merchant")} />
        <Input id="category_id" label="Category ID" error={errors.category_id?.message} {...register("category_id")} />
        <Input id="amount_cents" label="Amount (cents)" type="number" error={errors.amount_cents?.message} {...register("amount_cents")} />
        <Select
          id="transaction_type"
          label="Type"
          options={[
            { value: "income", label: "Income" },
            { value: "expense", label: "Expense" },
          ]}
          {...register("transaction_type")}
        />
        <Input id="date" label="Date" type="date" error={errors.date?.message} {...register("date")} />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit" isLoading={createTransaction.isPending}>Add Transaction</Button>
        </div>
      </form>
    </Modal>
  );
}
