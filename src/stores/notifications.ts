import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

export interface Notification {
  id: string;
  type: "success" | "error" | "info" | "warning";
  title: string;
  message?: string;
  timestamp: number;
  read: boolean;
  action?: {
    label: string;
    onClick: () => void;
  };
}

interface NotificationState {
  notifications: Notification[];
  addNotification: (notification: Omit<Notification, "id" | "timestamp" | "read">) => string;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  dismissNotification: (id: string) => void;
  clearAll: () => void;
  unreadCount: () => number;
}

let _idCounter = 0;
function nextId(): string {
  _idCounter += 1;
  return `notif-${_idCounter}-${Date.now()}`;
}

export const useNotificationStore = create<NotificationState>()(
  immer((set, get) => ({
    notifications: [],

    addNotification: (notif) => {
      const id = nextId();
      set((state) => {
        state.notifications.push({
          ...notif,
          id,
          timestamp: Date.now(),
          read: false,
        });
      });
      return id;
    },

    markAsRead: (id) =>
      set((state) => {
        const idx = state.notifications.findIndex((n) => n.id === id);
        if (idx !== -1) {
          state.notifications[idx].read = true;
        }
      }),

    markAllAsRead: () =>
      set((state) => {
        for (const n of state.notifications) {
          n.read = true;
        }
      }),

    dismissNotification: (id) =>
      set((state) => {
        state.notifications = state.notifications.filter((n) => n.id !== id);
      }),

    clearAll: () =>
      set((state) => {
        state.notifications = [];
      }),

    unreadCount: () => {
      return get().notifications.filter((n) => !n.read).length;
    },
  })),
);
