import type { TransactionFilters } from "../types";
import { Search } from "lucide-react";

interface TransactionFiltersProps {
  filters: TransactionFilters;
  onChange: (filters: TransactionFilters) => void;
  categories: string[];
}

export function TransactionFilters({ filters, onChange, categories }: TransactionFiltersProps) {
  return (
    <div className="flex flex-wrap items-center gap-2">
      <div className="relative flex-1 min-w-[180px] max-w-xs">
        <Search size={14} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-[hsl(0_0%_40%)]" />
        <input
          type="text"
          placeholder="Search transactions..."
          value={filters.search}
          onChange={(e) => onChange({ ...filters, search: e.target.value })}
          className="w-full h-8 pl-8 pr-3 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-xs text-white placeholder:text-[hsl(0_0%_40%)] focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
        />
      </div>
      <select
        value={filters.type}
        onChange={(e) => onChange({ ...filters, type: e.target.value as TransactionFilters["type"] })}
        className="h-8 px-2.5 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-xs text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
      >
        <option value="">All Types</option>
        <option value="income">Income</option>
        <option value="expense">Expense</option>
      </select>
      <select
        value={filters.category}
        onChange={(e) => onChange({ ...filters, category: e.target.value })}
        className="h-8 px-2.5 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-xs text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
      >
        <option value="">All Categories</option>
        {categories.map((c) => (
          <option key={c} value={c}>{c}</option>
        ))}
      </select>
      <input
        type="date"
        value={filters.dateFrom}
        onChange={(e) => onChange({ ...filters, dateFrom: e.target.value })}
        className="h-8 px-2.5 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-xs text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
      />
      <input
        type="date"
        value={filters.dateTo}
        onChange={(e) => onChange({ ...filters, dateTo: e.target.value })}
        className="h-8 px-2.5 rounded-lg bg-[hsl(220_15%_16%)] border border-[hsl(220_15%_24%)] text-xs text-white focus:outline-none focus:border-[hsl(220_90%_56%/0.6)]"
      />
    </div>
  );
}
