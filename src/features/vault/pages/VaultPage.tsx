import { motion } from "framer-motion";
import { Shield } from "lucide-react";

export default function VaultPage() {
  return (
    <motion.div
      initial={{ opacity: 0, y: 12 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
      className="p-6 space-y-6"
    >
      <div className="flex items-center gap-3">
        <div className="w-10 h-10 rounded-xl bg-[hsl(220_90%_56%/0.15)] border border-[hsl(220_90%_56%/0.3)] flex items-center justify-center">
          <Shield size={20} className="text-[hsl(220_90%_65%)]" />
        </div>
        <div>
          <h2 className="text-lg font-semibold text-white">Vault</h2>
          <p className="text-sm text-[hsl(0_0%_50%)]">Encrypted password and secret storage</p>
        </div>
      </div>
    </motion.div>
  );
}
