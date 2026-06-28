import { useQuery } from "@tanstack/react-query";
import type { Contact } from "../types";

const mockContacts: Contact[] = [
  { id: "1", name: "Sarah Johnson", role: "Financial Advisor", organization: "Merill Lynch", email: "sarah.j@merill.com", phone: "+1 (555) 123-4567", notes: "Annual review in December", is_favorite: true, created_at: "2024-01-15" },
  { id: "2", name: "Mike Chen", role: "Tax Accountant", organization: "Chen & Associates", email: "mike@chencpa.com", phone: "+1 (555) 234-5678", notes: "Great for quarterly tax planning", is_favorite: true, created_at: "2024-02-01" },
  { id: "3", name: "Emily Rodriguez", role: "Real Estate Agent", organization: "Prestige Realty", email: "emily@prestigere.com", phone: "+1 (555) 345-6789", notes: "Looking for investment properties", is_favorite: false, created_at: "2024-03-10" },
  { id: "4", name: "David Park", role: "Insurance Broker", organization: "State Farm", email: "david.park@statefarm.com", phone: "+1 (555) 456-7890", notes: "Auto + home insurance bundle", is_favorite: true, created_at: "2024-01-20" },
  { id: "5", name: "Lisa Thompson", role: "Estate Attorney", organization: "Thompson Law Group", email: "lisa@thompsonlaw.com", phone: "+1 (555) 567-8901", notes: "Working on will and trust documents", is_favorite: false, created_at: "2024-04-05" },
  { id: "6", name: "James Wilson", role: "Business Partner", organization: "Wilson Enterprises", email: "james@wilsonent.com", phone: "+1 (555) 678-9012", notes: "Potential joint venture in Q3", is_favorite: false, created_at: "2024-05-01" },
];

export function useContacts() {
  return useQuery({
    queryKey: ["contacts"],
    queryFn: () => Promise.resolve(mockContacts),
  });
}
