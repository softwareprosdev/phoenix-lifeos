import { useQuery } from "@tanstack/react-query";
import type { Investment, PortfolioSummary } from "../types";

const mockInvestments: Investment[] = [
  { id: "1", symbol: "VTI", name: "Vanguard Total Stock Market", shares: 50, cost_basis_cents: 12000000, current_price_cents: 26000, asset_class: "etf" },
  { id: "2", symbol: "VXUS", name: "Vanguard Total International", shares: 30, cost_basis_cents: 1800000, current_price_cents: 6200, asset_class: "etf" },
  { id: "3", symbol: "BND", name: "Vanguard Total Bond Market", shares: 40, cost_basis_cents: 3200000, current_price_cents: 7200, asset_class: "bond" },
  { id: "4", symbol: "AAPL", name: "Apple Inc", shares: 10, cost_basis_cents: 1500000, current_price_cents: 19500, asset_class: "stock" },
  { id: "5", symbol: "BTC", name: "Bitcoin", shares: 0.5, cost_basis_cents: 1500000, current_price_cents: 3400000, asset_class: "crypto" },
];

const mockPortfolioSummary: PortfolioSummary = {
  totalValueCents: 3161000,
  totalCostCents: 2900000,
  totalGainCents: 261000,
  totalGainPercent: 9.0,
  allocation: [
    { name: "US Stocks", value: 1300000, color: "hsl(220, 90%, 56%)" },
    { name: "International", value: 186000, color: "hsl(142, 72%, 45%)" },
    { name: "Bonds", value: 288000, color: "hsl(38, 92%, 50%)" },
    { name: "Tech", value: 195000, color: "hsl(280, 80%, 60%)" },
    { name: "Crypto", value: 170000, color: "hsl(0, 84%, 60%)" },
  ],
};

export function useInvestments() {
  return useQuery({
    queryKey: ["investments"],
    queryFn: () => Promise.resolve(mockInvestments),
  });
}

export function usePortfolioSummary() {
  return useQuery({
    queryKey: ["portfolio-summary"],
    queryFn: () => Promise.resolve(mockPortfolioSummary),
  });
}
