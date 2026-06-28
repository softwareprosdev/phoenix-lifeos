export interface Contact {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  organization?: string;
  notes?: string;
  created_at: string;
  updated_at: string;
  deleted_at?: string;
}

export interface ContactInteraction {
  id: string;
  contact_id: string;
  interaction_type: InteractionType;
  subject?: string;
  notes?: string;
  date: string;
  created_at: string;
}

export type InteractionType = "call" | "email" | "meeting" | "note" | "other";

export interface CreateContactInput {
  name: string;
  email?: string;
  phone?: string;
  organization?: string;
  notes?: string;
}

export interface CreateInteractionInput {
  contact_id: string;
  interaction_type: InteractionType;
  subject?: string;
  notes?: string;
  date: string;
}
