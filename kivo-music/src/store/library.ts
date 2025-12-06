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
  /** 直接替换整个曲库并持久化 */
  setTracks: (tracks: MusicTrack[]) => void;
  /** 向曲库追加一首或多首歌曲（去重后持久化） */
  addTracks: (tracks: MusicTrack[] | MusicTrack) => void;
  /** 清空曲库并持久化 */
  clearLibrary: () => void;
}

/**
 * 从 track 对象中提取一个“身份 id”，用来做去重。
 * 为了兼容历史数据，按优先级尝试多个字段。
 */
function getTrackIdentity(track: MusicTrack | undefined | null): string {
  if (!track) return "";
  const anyTrack = track as any;

  if (anyTrack.trackId != null) return String(anyTrack.trackId);
  if (anyTrack.id != null) return String(anyTrack.id);
  if (anyTrack.filePath) return String(anyTrack.filePath);
  if (anyTrack.path) return String(anyTrack.path);
  if (anyTrack.location) return String(anyTrack.location);
  if (anyTrack.title) return String(anyTrack.title);

  // 万不得已兜底
  return JSON.stringify(anyTrack);
}

/**
 * 补全 / 规范化统计相关字段：
 * - addedAt: 没有则补当前时间
 * - playCount: 没有或非法则补 0
 * - lastPlayedAt: 非法日期则清空
 * - favorite: 统一为 boolean；如果存在 legacy 的 liked，则沿用其值
 *
 * 注意：不会覆盖已有合法值，只在“缺失/非法”时兜底。
 */
function normalizeTrackStats(raw: MusicTrack): MusicTrack {
  const t: any = { ...raw };
  const now = new Date().toISOString();

  // addedAt
  if (!t.addedAt) {
    t.addedAt = now;
  }

  // playCount
  const playCount = t.playCount;
  if (
    typeof playCount !== "number" ||
    !Number.isFinite(playCount) ||
    playCount < 0
  ) {
    t.playCount = 0;
  }

  // lastPlayedAt
  if (t.lastPlayedAt != null) {
    const time = Date.parse(t.lastPlayedAt);
    if (!Number.isFinite(time)) {
      t.lastPlayedAt = undefined;
    }
  }

  // favorite（兼容 legacy: liked）
  if (typeof t.favorite !== "boolean") {
    if (typeof t.liked === "boolean") {
      t.favorite = t.liked;
    } else {
      t.favorite = false;
    }
  }

  return t as MusicTrack;
}

/**
 * 对一批曲目做 normalize，自动过滤掉 null/undefined。
 */
function normalizeTrackList(
  input: MusicTrack[] | MusicTrack | null | undefined,
): MusicTrack[] {
  if (!input) return [];
  const list = Array.isArray(input) ? input : [input];
  return list
    .filter(Boolean)
    .map((item) => normalizeTrackStats(item as MusicTrack));
}

export const useLibrary = create<LibraryState>((set, get) => ({
  tracks: [],
  isLoaded: false,
  isLoading: false,

  async loadFromDisk() {
    const state = get();
    // 已经加载过或正在加载时，不再重复触发
    if (state.isLoaded || state.isLoading) {
      return;
    }

    set({ isLoading: true });

    try {
      const loaded = await loadLibrary();
      const normalized = normalizeTrackList(loaded);

      set({
        tracks: normalized,
        isLoaded: true,
        isLoading: false,
      });

      log.debug("LibraryStore", "loadFromDisk ok", {
        count: normalized.length,
      });
    } catch (error) {
      log.error("LibraryStore", "loadFromDisk error", { error });
      // 出错也标记 isLoaded，避免无限重试；曲库退化为空
      set({
        tracks: [],
        isLoaded: true,
        isLoading: false,
      });
    }
  },

  setTracks(nextTracks) {
    const normalized = normalizeTrackList(nextTracks);
    set({ tracks: normalized });
    void saveLibrary(normalized);
  },

  addTracks(input) {
    const incoming = normalizeTrackList(input);
    if (!incoming.length) return;

    const current = get().tracks ?? [];
    if (!current.length) {
      const next = incoming;
      set({ tracks: next });
      void saveLibrary(next);
      log.debug("LibraryStore", "addTracks from empty library", {
        added: next.length,
      });
      return;
    }

    const merged: MusicTrack[] = [];
    const seen = new Set<string>();

    const pushUnique = (track: MusicTrack) => {
      const key = getTrackIdentity(track);
      if (seen.has(key)) return;
      seen.add(key);
      merged.push(track);
    };

    // 先保留原有曲库顺序
    for (const track of current) {
      pushUnique(track);
    }
    // 再追加新导入曲目
    for (const track of incoming) {
      pushUnique(track);
    }

    set({ tracks: merged });
    void saveLibrary(merged);

    log.debug("LibraryStore", "addTracks merged", {
      before: current.length,
      addedRaw: incoming.length,
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
