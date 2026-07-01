import { useQuery } from "@tanstack/react-query";
import { maintenanceService } from "@/services";

export function useMaintenanceTasks() {
  return useQuery({
    queryKey: ["maintenance-tasks"],
    queryFn: () => maintenanceService.listTasks(),
  });
}

export function useWarranties() {
  return useQuery({
    queryKey: ["warranties"],
    queryFn: () => maintenanceService.listWarranties(),
  });
}
