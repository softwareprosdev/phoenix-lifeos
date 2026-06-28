import { invoke } from "@tauri-apps/api/core";
import type { Contact, ContactInteraction, CreateContactInput, CreateInteractionInput } from "@/types";

class CrmServiceError extends Error {
  constructor(message: string, public cause?: unknown) {
    super(message);
    this.name = "CrmServiceError";
  }
}

async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (err) {
    throw new CrmServiceError(`IPC call '${cmd}' failed`, err);
  }
}

export const crmService = {
  listContacts: () => safeInvoke<Contact[]>("list_contacts"),

  getContact: (id: string) => safeInvoke<Contact>("get_contact", { id }),

  createContact: (input: CreateContactInput) =>
    safeInvoke<Contact>("create_contact", { payload: input }),

  updateContact: (id: string, input: Partial<CreateContactInput>) =>
    safeInvoke<Contact>("update_contact", { id, payload: input }),

  deleteContact: (id: string) => safeInvoke<void>("delete_contact", { id }),

  searchContacts: (query: string) =>
    safeInvoke<Contact[]>("search_contacts", { query }),

  listInteractions: (contactId: string) =>
    safeInvoke<ContactInteraction[]>("list_contact_interactions", { contactId }),

  createInteraction: (input: CreateInteractionInput) =>
    safeInvoke<ContactInteraction>("create_contact_interaction", { payload: input }),

  updateInteraction: (id: string, input: Partial<CreateInteractionInput>) =>
    safeInvoke<ContactInteraction>("update_contact_interaction", { id, payload: input }),

  deleteInteraction: (id: string) =>
    safeInvoke<void>("delete_contact_interaction", { id }),
};
