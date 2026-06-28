import { useQuery } from "@tanstack/react-query";
import type { MaintenanceTask, Warranty } from "../types";

const mockTasks: MaintenanceTask[] = [
  { id: "1", title: "HVAC Filter Replacement", description: "Replace air filters", category: "home", frequency: "quarterly", last_done: "2024-03-15", next_due: "2024-06-15", cost_cents: 5000 },
  { id: "2", title: "Oil Change", description: "Toyota Camry oil change", category: "vehicle", frequency: "monthly", last_done: "2024-05-20", next_due: "2024-06-20", cost_cents: 7500 },
  { id: "3", title: "Smoke Detector Test", description: "Test all smoke detectors", category: "home", frequency: "monthly", last_done: "2024-06-01", next_due: "2024-07-01", cost_cents: 0 },
  { id: "4", title: "Phone Backup", description: "iCloud + local backup", category: "electronics", frequency: "weekly", last_done: "2024-06-14", next_due: "2024-06-21", cost_cents: 0 },
];

const mockWarranties: Warranty[] = [
  { id: "1", item: "MacBook Pro", provider: "AppleCare", expires_at: "2026-03-15", cost_cents: 34900, notes: "Covers parts and labor" },
  { id: "2", item: "Toyota Camry", provider: "Toyota", expires_at: "2027-01-20", cost_cents: 0, notes: "Factory warranty" },
];

export function useMaintenanceTasks() {
  return useQuery({
    queryKey: ["maintenance-tasks"],
    queryFn: () => Promise.resolve(mockTasks),
  });
}

export function useWarranties() {
  return useQuery({
    queryKey: ["warranties"],
    queryFn: () => Promise.resolve(mockWarranties),
  });
}
