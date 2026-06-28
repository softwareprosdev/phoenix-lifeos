import { useQuery } from "@tanstack/react-query";
import type { Account, Transaction } from "../types";

const mockAccounts: Account[] = [
  { id: "1", name: "Checking", account_type: "checking", institution: "Chase", balance_cents: 125000, currency: "USD", is_active: true, created_at: "2024-01-01", updated_at: "2024-06-01" },
  { id: "2", name: "Savings", account_type: "savings", institution: "Chase", balance_cents: 450000, currency: "USD", is_active: true, created_at: "2024-01-01", updated_at: "2024-06-01" },
  { id: "3", name: "Credit Card", account_type: "credit", institution: "Amex", balance_cents: -340000, currency: "USD", is_active: true, created_at: "2024-02-01", updated_at: "2024-06-01" },
  { id: "4", name: "Investment Portfolio", account_type: "investment", institution: "Vanguard", balance_cents: 1250000, currency: "USD", is_active: true, created_at: "2024-01-15", updated_at: "2024-06-01" },
  { id: "5", name: "Cash", account_type: "cash", institution: "Wallet", balance_cents: 35000, currency: "USD", is_active: true, created_at: "2024-01-01", updated_at: "2024-06-01" },
];

const mockTransactions: Transaction[] = [
  { id: "1", account_id: "1", date: "2024-06-15", description: "Grocery Store Purchase", merchant: "Whole Foods", category: "Food", amount_cents: -8500, type: "expense", is_recurring: false, created_at: "2024-06-15" },
  { id: "2", account_id: "1", date: "2024-06-14", description: "Monthly Salary", merchant: "Employer Inc", category: "Income", amount_cents: 425000, type: "income", is_recurring: true, created_at: "2024-06-14" },
  { id: "3", account_id: "1", date: "2024-06-13", description: "Netflix Subscription", merchant: "Netflix", category: "Entertainment", amount_cents: -1549, type: "expense", is_recurring: true, created_at: "2024-06-13" },
  { id: "4", account_id: "3", date: "2024-06-12", description: "Gas Station", merchant: "Shell", category: "Transport", amount_cents: -4800, type: "expense", is_recurring: false, created_at: "2024-06-12" },
  { id: "5", account_id: "1", date: "2024-06-11", description: "Freelance Payment", merchant: "Client XYZ", category: "Income", amount_cents: 150000, type: "income", is_recurring: false, created_at: "2024-06-11" },
  { id: "6", account_id: "1", date: "2024-06-10", description: "Restaurant Dinner", merchant: "Olive Garden", category: "Food", amount_cents: -6200, type: "expense", is_recurring: false, created_at: "2024-06-10" },
  { id: "7", account_id: "2", date: "2024-06-09", description: "Transfer to Savings", merchant: "Internal", category: "Transfer", amount_cents: -50000, type: "expense", is_recurring: true, created_at: "2024-06-09" },
  { id: "8", account_id: "3", date: "2024-06-08", description: "Amazon Purchase", merchant: "Amazon", category: "Shopping", amount_cents: -12500, type: "expense", is_recurring: false, created_at: "2024-06-08" },
  { id: "9", account_id: "1", date: "2024-06-07", description: "Electric Bill", merchant: "Power Co", category: "Utilities", amount_cents: -13400, type: "expense", is_recurring: true, created_at: "2024-06-07" },
  { id: "10", account_id: "4", date: "2024-06-06", description: "Dividend Payment", merchant: "Vanguard", category: "Investment Income", amount_cents: 12500, type: "income", is_recurring: false, created_at: "2024-06-06" },
];

export function useAccounts() {
  return useQuery({
    queryKey: ["accounts"],
    queryFn: () => Promise.resolve(mockAccounts),
  });
}

export function useTransactions(accountId?: string) {
  return useQuery({
    queryKey: ["transactions", accountId],
    queryFn: () => {
      let result = mockTransactions;
      if (accountId) {
        result = result.filter((t) => t.account_id === accountId);
      }
      return Promise.resolve(result);
    },
  });
}
