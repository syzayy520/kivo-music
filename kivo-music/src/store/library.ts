// src/store/library.ts
import { create } from "zustand";
import type { MusicTrack } from "../types";
import { loadLibrary, saveLibrary } from "../persistence/LibraryPersistence";
import { log } from "../utils/log";

export interface LibraryState {
  /** 当前曲库中的所有曲目 */
  tracks: MusicTrack[];
  /** 是否已经从磁盘加载过一次 */
  isLoaded: boolean;
  /** 正在从磁盘加载 */
  isLoading: boolean;

  /** 从磁盘加载曲库（幂等，多次调用也只会真正加载一次） */
  loadFromDisk: () => Promise<void>;

  /** 用一批 tracks 替换当前曲库，并持久化到磁盘 */
  setTracks: (tracks: MusicTrack[]) => void;

  /** 往现有曲库里追加若干首歌（支持单首或数组），自动去重 + 持久化 */
  addTracks: (tracks: MusicTrack[] | MusicTrack) => void;

  /** 清空曲库并持久化 */
  clearLibrary: () => void;
}

/**
 * 生成一个用于去重的 key：
 * - 优先使用 id
 * - 其次使用 filePath / path
 */
function makeTrackKey(t: MusicTrack): string {
  const rawPath =
    (t.filePath as string | undefined) ||
    (t.path as string | undefined) ||
    "";
  const safePath = typeof rawPath === "string" ? rawPath : "";
  const id = typeof t.id === "string" ? t.id : "";
  return `${id}::${safePath}`;
}

export const useLibrary = create<LibraryState>((set, get) => ({
  tracks: [],
  isLoaded: false,
  isLoading: false,

  async loadFromDisk() {
    // 已经加载过就不重复来了
    if (get().isLoaded || get().isLoading) {
      return;
    }

    set({ isLoading: true });
    try {
      const loaded = await loadLibrary();
      set({
        tracks: loaded ?? [],
        isLoaded: true,
        isLoading: false,
      });
      log.debug("LibraryStore", "loaded library from disk", {
        count: loaded.length,
      });
    } catch (error) {
      log.error("LibraryStore", "loadFromDisk error", { error });
      set({ isLoading: false, isLoaded: true });
    }
  },

  setTracks(nextTracks) {
    const safe = Array.isArray(nextTracks) ? nextTracks : [];
    set({ tracks: safe });
    void saveLibrary(safe);
  },

  addTracks(input) {
    const incoming = Array.isArray(input) ? input : [input];
    if (!incoming.length) return;

    const current = get().tracks ?? [];
    const existingKeySet = new Set(current.map(makeTrackKey));
    const merged: MusicTrack[] = [...current];

    for (const t of incoming) {
      const key = makeTrackKey(t);
      if (existingKeySet.has(key)) {
        // 已有同一首歌 → 跳过，避免重复
        continue;
      }
      existingKeySet.add(key);
      merged.push(t);
    }

    set({ tracks: merged });
    void saveLibrary(merged);

    log.debug("LibraryStore", "addTracks merged", {
      before: current.length,
      added: incoming.length,
      after: merged.length,
    });
  },

  clearLibrary() {
    set({ tracks: [] });
    void saveLibrary([]);
    log.warn("LibraryStore", "library cleared");
  },
}));

// --- 自动在应用启动时加载一次曲库（最佳努力，不影响主流程） ---
void (async () => {
  try {
    await useLibrary.getState().loadFromDisk();
  } catch (error) {
    log.error("LibraryStore", "auto loadFromDisk error", { error });
  }
})();
