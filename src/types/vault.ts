export interface VaultItem {
  id: string;
  item_type: VaultItemType;
  title: string;
  encrypted_data: string;
  key_version: number;
  metadata: string;
  created_at: string;
  updated_at: string;
}

export type VaultItemType = "password" | "note" | "key" | "secret";

export interface CreateVaultItemInput {
  item_type: VaultItemType;
  title: string;
  encrypted_data: string;
  key_version?: number;
  metadata?: string;
}

export interface VaultAccessLogEntry {
  id: string;
  item_id: string;
  action: "view" | "create" | "update" | "delete";
  created_at: string;
}

export interface Document {
  id: string;
  name: string;
  file_type: string;
  mime_type?: string;
  encrypted_path: string;
  ocr_text_encrypted?: string;
  extracted_metadata: string;
  tags: string[];
  source: "upload" | "email" | "scan";
  created_at: string;
  updated_at: string;
}
