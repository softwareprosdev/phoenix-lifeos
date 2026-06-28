import { useState, useMemo } from "react";
import { motion } from "framer-motion";
import { useAccounts, useTransactions } from "../hooks/useFinanceData";
import { AccountCard } from "../components/AccountCard";
import { TransactionTable } from "../components/TransactionTable";
import { TransactionFilters } from "../components/TransactionFilters";
import { AddAccountModal } from "../components/AddAccountModal";
import { AddTransactionModal } from "../components/AddTransactionModal";
import { CardSkeleton } from "../../../components/ui/Skeleton";
import { Button } from "../../../components/ui/Button";
import { Plus, RotateCcw } from "lucide-react";
import toast from "react-hot-toast";
import type { TransactionFilters as TF } from "../types";

export default function FinancePage() {
  const { data: accounts, isLoading: accountsLoading } = useAccounts();
  const [selectedAccount, setSelectedAccount] = useState<string | undefined>();
  const { data: transactions, isLoading: txLoading } = useTransactions(selectedAccount);
  const [showAddAccount, setShowAddAccount] = useState(false);
  const [showAddTx, setShowAddTx] = useState(false);
  const [filters, setFilters] = useState<TF>({ dateFrom: "", dateTo: "", category: "", type: "", search: "" });

  const categories = useMemo(() => {
    const cats = new Set(transactions?.map((t) => t.category) ?? []);
    return Array.from(cats);
  }, [transactions]);

  const filteredTransactions = useMemo(() => {
    if (!transactions) return [];
    return transactions.filter((t) => {
      if (filters.type && t.type !== filters.type) return false;
      if (filters.category && t.category !== filters.category) return false;
      if (filters.search && !t.description.toLowerCase().includes(filters.search.toLowerCase()) && !t.merchant.toLowerCase().includes(filters.search.toLowerCase())) return false;
      if (filters.dateFrom && t.date < filters.dateFrom) return false;
      if (filters.dateTo && t.date > filters.dateTo) return false;
      return true;
    });
  }, [transactions, filters]);

  if (accountsLoading) {
    return (
      <div className="p-6 space-y-4">
        <div className="grid grid-cols-3 gap-4">
          {Array.from({ length: 3 }).map((_, i) => <CardSkeleton key={i} />)}
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
      {/* Accounts Section */}
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-sm font-semibold text-white">Accounts</h2>
        <Button size="sm" onClick={() => setShowAddAccount(true)}>
          <Plus size={14} /> Add Account
        </Button>
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 gap-3 mb-6">
        {accounts?.map((account) => (
          <AccountCard
            key={account.id}
            account={account}
            selected={selectedAccount === account.id}
            onClick={() => setSelectedAccount(selectedAccount === account.id ? undefined : account.id)}
          />
        ))}
      </div>

      {/* Transactions Section */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <h2 className="text-sm font-semibold text-white">Transactions</h2>
          {selectedAccount && (
            <button
              onClick={() => setSelectedAccount(undefined)}
              className="flex items-center gap-1 text-[11px] text-[hsl(220_90%_56%)] hover:text-[hsl(220_90%_70%)]"
            >
              <RotateCcw size={12} /> Clear filter
            </button>
          )}
        </div>
        <Button size="sm" onClick={() => setShowAddTx(true)}>
          <Plus size={14} /> Add Transaction
        </Button>
      </div>

      <TransactionFilters filters={filters} onChange={setFilters} categories={categories} />

      <div className="mt-3">
        {txLoading ? (
          <div className="space-y-2">
            {Array.from({ length: 5 }).map((_, i) => (
              <div key={i} className="h-10 rounded bg-[hsl(220_15%_14%/0.5)] animate-pulse" />
            ))}
          </div>
        ) : (
          <TransactionTable transactions={filteredTransactions} onDelete={() => toast.success("Transaction deleted")} />
        )}
      </div>

      <AddAccountModal open={showAddAccount} onClose={() => setShowAddAccount(false)} />
      <AddTransactionModal open={showAddTx} onClose={() => setShowAddTx(false)} />
    </motion.div>
  );
}
