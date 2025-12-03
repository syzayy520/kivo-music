import { create } from "zustand";
import type { MusicTrack } from "../types";

const STORAGE_KEY = "kivo-library-v1";

const loadInitialTracks = (): MusicTrack[] => {
  if (typeof window === "undefined") return [];
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    if (!Array.isArray(parsed)) return [];
    return parsed;
  } catch (e) {
    console.warn("读取本地音乐资料库失败:", e);
    return [];
  }
};

const saveTracks = (tracks: MusicTrack[]) => {
  if (typeof window === "undefined") return;
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(tracks));
  } catch (e) {
    console.warn("保存本地音乐资料库失败:", e);
  }
};

interface LibraryState {
  tracks: MusicTrack[];
  addTracks: (tracks: MusicTrack[]) => void;
  clearLibrary: () => void;
}

export const useLibrary = create<LibraryState>((set, get) => ({
  // 启动时尝试从 localStorage 里恢复
  tracks: loadInitialTracks(),

  // 导入新歌曲（自动按 path 去重）
  addTracks: (incoming) => {
    const existing = get().tracks;
    const pathSet = new Set(existing.map((t) => t.path));

    const deduped: MusicTrack[] = [];
    for (const t of incoming) {
      if (!pathSet.has(t.path)) {
        pathSet.add(t.path);
        deduped.push(t);
      }
    }

    if (!deduped.length) return; // 全是重复的就不更新

    const merged = [...existing, ...deduped];
    set({ tracks: merged });
    saveTracks(merged);
  },

  // 清空资料库（调试用 & 未来做个按钮给用户）
  clearLibrary: () => {
    set({ tracks: [] });
    saveTracks([]);
  },
}));
