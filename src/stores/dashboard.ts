import { create } from "zustand";
import { persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

export interface Widget {
  id: string;
  type: string;
  title: string;
  x: number;
  y: number;
  w: number;
  h: number;
  visible: boolean;
}

interface DashboardState {
  widgets: Widget[];
  setWidgets: (widgets: Widget[]) => void;
  updateWidget: (id: string, partial: Partial<Widget>) => void;
  toggleWidget: (id: string) => void;
  moveWidget: (id: string, x: number, y: number) => void;
  resizeWidget: (id: string, w: number, h: number) => void;
  addWidget: (widget: Widget) => void;
  removeWidget: (id: string) => void;
}

const DEFAULT_WIDGETS: Widget[] = [
  { id: "net-worth", type: "netWorth", title: "Net Worth", x: 0, y: 0, w: 2, h: 1, visible: true },
  { id: "cash-flow", type: "cashFlow", title: "Monthly Cash Flow", x: 2, y: 0, w: 2, h: 1, visible: true },
  { id: "recent-transactions", type: "recentTransactions", title: "Recent Transactions", x: 0, y: 1, w: 2, h: 2, visible: true },
  { id: "budget-overview", type: "budgetOverview", title: "Budget Overview", x: 2, y: 1, w: 1, h: 1, visible: true },
  { id: "credit-score", type: "creditScore", title: "Credit Score", x: 3, y: 1, w: 1, h: 1, visible: true },
  { id: "goals-progress", type: "goalsProgress", title: "Goals Progress", x: 2, y: 2, w: 2, h: 1, visible: true },
  { id: "upcoming-bills", type: "upcomingBills", title: "Upcoming Bills", x: 0, y: 3, w: 1, h: 1, visible: true },
  { id: "investment-summary", type: "investmentSummary", title: "Investments", x: 1, y: 3, w: 1, h: 1, visible: true },
  { id: "debt-summary", type: "debtSummary", title: "Debt Summary", x: 2, y: 3, w: 1, h: 1, visible: true },
  { id: "recent-events", type: "recentEvents", title: "Recent Activity", x: 3, y: 3, w: 1, h: 1, visible: true },
];

export const useDashboardStore = create<DashboardState>()(
  persist(
    immer((set) => ({
      widgets: DEFAULT_WIDGETS,

      setWidgets: (widgets) =>
        set((state) => {
          state.widgets = widgets;
        }),

      updateWidget: (id, partial) =>
        set((state) => {
          const idx = state.widgets.findIndex((w) => w.id === id);
          if (idx !== -1) {
            Object.assign(state.widgets[idx], partial);
          }
        }),

      toggleWidget: (id) =>
        set((state) => {
          const idx = state.widgets.findIndex((w) => w.id === id);
          if (idx !== -1) {
            state.widgets[idx].visible = !state.widgets[idx].visible;
          }
        }),

      moveWidget: (id, x, y) =>
        set((state) => {
          const idx = state.widgets.findIndex((w) => w.id === id);
          if (idx !== -1) {
            state.widgets[idx].x = x;
            state.widgets[idx].y = y;
          }
        }),

      resizeWidget: (id, w, h) =>
        set((state) => {
          const idx = state.widgets.findIndex((w) => w.id === id);
          if (idx !== -1) {
            state.widgets[idx].w = w;
            state.widgets[idx].h = h;
          }
        }),

      addWidget: (widget) =>
        set((state) => {
          state.widgets.push(widget);
        }),

      removeWidget: (id) =>
        set((state) => {
          state.widgets = state.widgets.filter((w) => w.id !== id);
        }),
    })),
    { name: "phoenix-dashboard-layout" },
  ),
);
