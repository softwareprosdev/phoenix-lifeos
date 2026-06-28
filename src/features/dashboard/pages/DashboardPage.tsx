import { useState } from "react";
import { motion } from "framer-motion";
import { DndContext, closestCenter, PointerSensor, useSensor, useSensors, type DragEndEvent } from "@dnd-kit/core";
import { arrayMove } from "@dnd-kit/sortable";
import { useWidgets } from "../hooks/useDashboardData";
import { NetWorthWidget } from "../components/NetWorthWidget";
import { CashFlowWidget } from "../components/CashFlowWidget";
import { BudgetHealthWidget } from "../components/BudgetHealthWidget";
import { DebtOverviewWidget } from "../components/DebtOverviewWidget";
import { CreditScoreWidget } from "../components/CreditScoreWidget";
import { RecentTransactionsWidget } from "../components/RecentTransactionsWidget";
import { AIInsightsWidget } from "../components/AIInsightsWidget";
import type { WidgetConfig } from "../types";

const widgetComponents: Record<string, React.ComponentType> = {
  "net-worth": NetWorthWidget,
  "cash-flow": CashFlowWidget,
  "budget-health": BudgetHealthWidget,
  "debt-overview": DebtOverviewWidget,
  "credit-score": CreditScoreWidget,
  "recent-transactions": RecentTransactionsWidget,
  "ai-insights": AIInsightsWidget,
};

const widgetSizes: Record<string, string> = {
  "net-worth": "col-span-2 row-span-1",
  "cash-flow": "col-span-1 row-span-1",
  "budget-health": "col-span-1 row-span-1",
  "debt-overview": "col-span-1 row-span-1",
  "credit-score": "col-span-1 row-span-1",
  "recent-transactions": "col-span-2 row-span-1",
  "ai-insights": "col-span-1 row-span-1",
};

export default function DashboardPage() {
  const { data: widgets, isLoading } = useWidgets();
  const [items, setItems] = useState<WidgetConfig[]>([]);

  useState(() => {
    if (widgets && items.length === 0) {
      setItems(widgets);
    }
  });

  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 8 } }),
  );

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (over && active.id !== over.id) {
      setItems((prev) => {
        const oldIndex = prev.findIndex((i) => i.id === active.id);
        const newIndex = prev.findIndex((i) => i.id === over.id);
        return arrayMove(prev, oldIndex, newIndex);
      });
    }
  };

  if (isLoading) {
    return (
      <div className="p-6 grid grid-cols-3 gap-4 h-full">
        {Array.from({ length: 6 }).map((_, i) => (
          <div key={i} className="rounded-lg bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)] p-4 animate-pulse">
            <div className="h-3 w-1/3 bg-[hsl(220_15%_22%)] rounded mb-3" />
            <div className="h-8 w-1/2 bg-[hsl(220_15%_22%)] rounded mb-2" />
            <div className="h-4 w-2/3 bg-[hsl(220_15%_22%)] rounded" />
          </div>
        ))}
      </div>
    );
  }

  const displayItems = items.length > 0 ? items : widgets ?? [];

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto"
    >
      <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
        <div className="grid grid-cols-3 gap-4 auto-rows-[180px]">
          {displayItems.map((widget) => {
            const Component = widgetComponents[widget.type];
            if (!Component) return null;
            return (
              <motion.div
                key={widget.id}
                layout
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.2 }}
                className={widgetSizes[widget.id] ?? "col-span-1 row-span-1"}
                style={{ touchAction: "none" }}
              >
                <Component />
              </motion.div>
            );
          })}
        </div>
      </DndContext>
    </motion.div>
  );
}
