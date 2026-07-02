import { safeInvoke } from "./commandClient";
import type { Contact, ContactInteraction, CreateContactInput, CreateInteractionInput } from "@/types";


export const crmService = {
  listContacts: () => safeInvoke<Contact[]>("Crm", "list_contacts"),

  getContact: (id: string) => safeInvoke<Contact>("Crm", "get_contact", { id }),

  createContact: (input: CreateContactInput) =>
    safeInvoke<Contact>("Crm", "create_contact", { payload: input }),

  updateContact: (id: string, input: Partial<CreateContactInput>) =>
    safeInvoke<Contact>("Crm", "update_contact", { id, payload: input }),

  deleteContact: (id: string) => safeInvoke<void>("Crm", "delete_contact", { id }),

  searchContacts: (query: string) =>
    safeInvoke<Contact[]>("Crm", "search_contacts", { query }),

  listInteractions: (contactId: string) =>
    safeInvoke<ContactInteraction[]>("Crm", "list_contact_interactions", { contactId }),

  createInteraction: (input: CreateInteractionInput) =>
    safeInvoke<ContactInteraction>("Crm", "create_contact_interaction", { payload: input }),

  updateInteraction: (id: string, input: Partial<CreateInteractionInput>) =>
    safeInvoke<ContactInteraction>("Crm", "update_contact_interaction", { id, payload: input }),

  deleteInteraction: (id: string) =>
    safeInvoke<void>("Crm", "delete_contact_interaction", { id }),
};
