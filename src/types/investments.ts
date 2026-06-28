export interface Investment {
  id: string;
  account_id: string;
  asset_type: AssetType;
  symbol?: string;
  name: string;
  quantity: string;
  cost_basis_cents: number;
  current_value_cents: number;
  currency: string;
  created_at: string;
  updated_at: string;
}

export type AssetType =
  | "stock"
  | "bond"
  | "etf"
  | "mutual_fund"
  | "crypto"
  | "real_estate"
  | "commodity"
  | "cash"
  | "other";

export interface CreateInvestmentInput {
  account_id: string;
  asset_type: AssetType;
  symbol?: string;
  name: string;
  quantity: string;
  cost_basis_cents: number;
  current_value_cents: number;
  currency?: string;
}
