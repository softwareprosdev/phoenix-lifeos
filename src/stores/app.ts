import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import type { Theme } from "@/types";

interface AppState {
  sidebarCollapsed: boolean;
  activePage: string;
  theme: Theme;
  searchQuery: string;
  setSidebarCollapsed: (collapsed: boolean) => void;
  toggleSidebar: () => void;
  setActivePage: (page: string) => void;
  setTheme: (theme: Theme) => void;
  setSearchQuery: (query: string) => void;
}

export const useAppStore = create<AppState>()(
  immer((set) => ({
    sidebarCollapsed: false,
    activePage: "dashboard",
    theme: "dark",
    searchQuery: "",

    setSidebarCollapsed: (collapsed) =>
      set((state) => {
        state.sidebarCollapsed = collapsed;
      }),

    toggleSidebar: () =>
      set((state) => {
        state.sidebarCollapsed = !state.sidebarCollapsed;
      }),

    setActivePage: (page) =>
      set((state) => {
        state.activePage = page;
      }),

    setTheme: (theme) =>
      set((state) => {
        state.theme = theme;
      }),

    setSearchQuery: (query) =>
      set((state) => {
        state.searchQuery = query;
      }),
  })),
);
