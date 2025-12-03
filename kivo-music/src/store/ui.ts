import { create } from "zustand";

export type Page = "library" | "playlist" | "nowplaying";

interface UIState {
  page: Page;              // 当前页面
  sidebarOpen: boolean;    // 左侧栏是否展开
  theme: "light" | "dark";

  setPage: (p: Page) => void;
  toggleSidebar: () => void;
  setTheme: (t: "light" | "dark") => void;
}

export const useUI = create<UIState>((set) => ({
  page: "library",
  sidebarOpen: true,
  theme: "light",

  setPage: (p) => set({ page: p }),
  toggleSidebar: () =>
    set((s) => ({ sidebarOpen: !s.sidebarOpen })),
  setTheme: (t) => set({ theme: t }),
}));
