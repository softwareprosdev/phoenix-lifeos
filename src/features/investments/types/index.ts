export interface Investment {
  id: string;
  symbol: string;
  name: string;
  shares: number;
  cost_basis_cents: number;
  current_price_cents: number;
  asset_class: "stock" | "etf" | "mutual_fund" | "crypto" | "bond" | "real_estate";
}

export interface PortfolioSummary {
  totalValueCents: number;
  totalCostCents: number;
  totalGainCents: number;
  totalGainPercent: number;
  allocation: { name: string; value: number; color: string }[];
}
