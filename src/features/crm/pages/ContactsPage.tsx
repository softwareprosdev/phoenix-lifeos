import { useState, useMemo } from "react";
import { motion } from "framer-motion";
import { useContacts } from "../hooks/useContactsData";
import { Button } from "../../../components/ui/Button";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Modal } from "../../../components/ui/Modal";
import { Input } from "../../../components/ui/Input";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import toast from "react-hot-toast";
import { Plus, Search, Mail, Phone, Building2 } from "lucide-react";

const schema = z.object({
  name: z.string().min(1, "Name is required"),
  role: z.string().min(1, "Role is required"),
  organization: z.string().min(1, "Organization is required"),
  email: z.string().email("Invalid email"),
  phone: z.string().min(1, "Phone is required"),
  notes: z.string().optional(),
});

type FormData = z.infer<typeof schema>;

function AddContactModal({ open, onClose }: { open: boolean; onClose: () => void }) {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
  });

  const onSubmit = (data: FormData) => {
    console.log("Add contact:", data);
    toast.success("Contact added successfully");
    reset();
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title="Add Contact">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <Input id="name" label="Name" error={errors.name?.message} {...register("name")} />
        <Input id="role" label="Role" error={errors.role?.message} {...register("role")} />
        <Input id="organization" label="Organization" error={errors.organization?.message} {...register("organization")} />
        <Input id="email" label="Email" type="email" error={errors.email?.message} {...register("email")} />
        <Input id="phone" label="Phone" error={errors.phone?.message} {...register("phone")} />
        <div className="space-y-1.5">
          <label htmlFor="notes" className="block text-xs font-medium text-[hsl(0_0%_70%)]">Notes</label>
          <textarea
            id="notes"
            rows={3}
            className="w-full px-3 py-2 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-sm text-white placeholder:text-[hsl(0_0%_40%)] focus:outline-none focus:border-[hsl(220_90%_56%/0.6)] resize-none"
            {...register("notes")}
          />
        </div>
        <div className="flex justify-end gap-2 pt-2">
          <Button type="button" variant="secondary" onClick={onClose}>Cancel</Button>
          <Button type="submit">Add Contact</Button>
        </div>
      </form>
    </Modal>
  );
}

export default function ContactsPage() {
  const { data: contacts, isLoading } = useContacts();
  const [showAdd, setShowAdd] = useState(false);
  const [search, setSearch] = useState("");

  const filtered = useMemo(() => {
    if (!contacts) return [];
    if (!search) return contacts;
    const q = search.toLowerCase();
    return contacts.filter(
      (c) =>
        c.name.toLowerCase().includes(q) ||
        c.organization?.toLowerCase().includes(q) ||
        c.email?.toLowerCase().includes(q),
    );
  }, [contacts, search]);

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
        <h2 className="text-sm font-semibold text-white">Contacts</h2>
        <Button size="sm" onClick={() => setShowAdd(true)}>
          <Plus size={14} /> Add Contact
        </Button>
      </div>

      {/* Search */}
      <div className="relative max-w-xs mb-4">
        <Search size={14} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-[hsl(0_0%_40%)]" />
        <input
          type="text"
          placeholder="Search contacts..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full h-9 pl-8 pr-3 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-sm text-white placeholder:text-[hsl(0_0%_40%)] focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
        />
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        {filtered.map((contact, idx) => (
          <motion.div
            key={contact.id}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: idx * 0.03 }}
            className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] backdrop-blur-sm border border-[hsl(220_15%_22%/0.6)]"
          >
            <div className="flex items-start justify-between mb-3">
              <div className="flex items-center gap-2.5">
                <div className="w-9 h-9 rounded-full bg-gradient-to-br from-[hsl(220_90%_56%)] to-[hsl(280_80%_60%)] flex items-center justify-center text-white text-sm font-bold">
                  {contact.name.charAt(0)}
                </div>
                <div>
                  <div className="flex items-center gap-1">
                    <p className="text-sm font-semibold text-white">{contact.name}</p>
                  </div>
                  <p className="text-[10px] text-[hsl(0_0%_45%)]">{contact.organization}</p>
                </div>
              </div>
            </div>
            <div className="space-y-1.5">
              <div className="flex items-center gap-2 text-xs">
                <Building2 size={11} className="text-[hsl(0_0%_35%)]" />
                <span className="text-[hsl(0_0%_50%)]">{contact.organization}</span>
              </div>
              <div className="flex items-center gap-2 text-xs">
                <Mail size={11} className="text-[hsl(0_0%_35%)]" />
                <span className="text-[hsl(0_0%_50%)]">{contact.email}</span>
              </div>
              <div className="flex items-center gap-2 text-xs">
                <Phone size={11} className="text-[hsl(0_0%_35%)]" />
                <span className="text-[hsl(0_0%_50%)]">{contact.phone}</span>
              </div>
            </div>
            {contact.notes && (
              <p className="text-[10px] text-[hsl(0_0%_40%)] mt-2 italic line-clamp-2">{contact.notes}</p>
            )}
          </motion.div>
        ))}
      </div>

      {filtered.length === 0 && (
        <div className="text-center py-16">
          <p className="text-sm text-[hsl(0_0%_45%)]">No contacts found{search ? " matching your search" : ""}.</p>
        </div>
      )}

      <AddContactModal open={showAdd} onClose={() => setShowAdd(false)} />
    </motion.div>
  );
}
