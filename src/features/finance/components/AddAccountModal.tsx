import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { Select } from "../../../components/ui/Select";
import { Button } from "../../../components/ui/Button";
import { useCreateAccount } from "@/hooks";
import toast from "react-hot-toast";

const schema = z.object({
  name: z.string().min(1, "Name is required"),
  account_type: z.enum(["checking", "savings", "credit", "investment", "loan", "cash", "business"]),
  institution: z.string().optional(),
  balance_cents: z.coerce.number().int("Balance must be a whole number"),
  currency: z.string().default("USD"),
});

type FormData = z.infer<typeof schema>;

interface AddAccountModalProps {
  open: boolean;
  onClose: () => void;
}

export function AddAccountModal({ open, onClose }: AddAccountModalProps) {
  const createAccount = useCreateAccount();
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
    defaultValues: { currency: "USD", balance_cents: 0 },
  });

  const onSubmit = async (data: FormData) => {
    try {
      await createAccount.mutateAsync(data);
      toast.success("Account added successfully");
      reset();
      onClose();
    } catch {
      toast.error("Failed to add account");
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Account">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="name" label="Account Name" error={errors.name?.message} {...register("name")} />
        <Select
          id="account_type"
          label="Account Type"
          error={errors.account_type?.message}
          options={[
            { value: "checking", label: "Checking" },
            { value: "savings", label: "Savings" },
            { value: "credit", label: "Credit Card" },
            { value: "investment", label: "Investment" },
            { value: "loan", label: "Loan" },
            { value: "cash", label: "Cash" },
            { value: "business", label: "Business" },
          ]}
          {...register("account_type")}
        />
        <Input id="institution" label="Institution" error={errors.institution?.message} {...register("institution")} />
        <Input id="balance_cents" label="Balance (cents)" type="number" error={errors.balance_cents?.message} {...register("balance_cents")} />
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit" isLoading={createAccount.isPending}>Add Account</Button>
        </div>
      </form>
    </Modal>
  );
}
