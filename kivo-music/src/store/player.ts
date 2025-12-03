import { create } from "zustand";

export interface PlayerTrack {
  id: string;
  title: string;
  artist: string;
  album?: string;
  filePath: string;
  duration?: number;

  // 封面相关字段（已经在库里用上了）
  coverId?: string;   // 封面唯一 ID（未来可用专辑名/哈希等）
  coverPath?: string; // 封面在本地缓存后的绝对路径
}

export type PlayMode = "normal" | "repeat-one" | "repeat-all";

export interface PlayerState {
  // 播放列表 / 队列
  playlist: PlayerTrack[];
  /** 兼容历史字段：有的地方还叫 tracks，这里始终与 playlist 保持一致 */
  tracks: PlayerTrack[];

  // 播放指针
  currentIndex: number; // -1 表示当前没有曲目
  isPlaying: boolean;

  // 进度信息
  currentTime: number;
  duration: number;
  pendingSeek: number | null;

  // 音量 / 模式
  volume: number;
  mode: PlayMode;

  // === actions ===
  /** 覆盖整个播放列表，尽量保留 currentIndex */
  setPlaylist: (tracks: PlayerTrack[]) => void;
  /** setPlaylist 的兼容别名 */
  setTracks: (tracks: PlayerTrack[]) => void;

  /** 切换到指定索引并开始播放 */
  playTrack: (index: number) => void;

  /** 播放 / 暂停 */
  togglePlay: () => void;
  setIsPlaying: (value: boolean) => void;

  /** 音量 0–1 */
  setVolume: (value: number) => void;

  /** 由 <audio> 驱动的实时位置 */
  setPosition: (t: number) => void;
  setDuration: (d: number) => void;

  /** 用户拖动进度条时调用 */
  seek: (t: number) => void;
  clearPendingSeek: () => void;

  /** 切到下一首 / 上一首 */
  next: () => void;
  prev: () => void;
}

function clampIndex(len: number, index: number): number {
  if (len <= 0) return -1;
  if (index < 0) return 0;
  if (index >= len) return len - 1;
  return index;
}

export const usePlayerStore = create<PlayerState>((set, get) => ({
  playlist: [],
  tracks: [],
  currentIndex: -1,
  isPlaying: false,
  currentTime: 0,
  duration: 0,
  pendingSeek: null,
  volume: 1,
  mode: "normal",

  setPlaylist: (tracks: PlayerTrack[]) =>
    set((state) => {
      const len = tracks.length;
      let newIndex = state.currentIndex;

      if (len === 0) {
        newIndex = -1;
      } else if (newIndex < 0 || newIndex >= len) {
        // 之前索引已经越界了，就回到第一首
        newIndex = 0;
      }

      return {
        playlist: tracks,
        tracks, // 保持同步
        currentIndex: newIndex,
      };
    }),

  setTracks: (tracks: PlayerTrack[]) =>
    set((state) => {
      const len = tracks.length;
      let newIndex = state.currentIndex;

      if (len === 0) {
        newIndex = -1;
      } else if (newIndex < 0 || newIndex >= len) {
        newIndex = 0;
      }

      return {
        playlist: tracks,
        tracks,
        currentIndex: newIndex,
      };
    }),

  playTrack: (index: number) =>
    set((state) => {
      const len = state.playlist.length;
      if (len === 0) {
        return state;
      }
      const clamped = clampIndex(len, index);
      if (clamped === -1) {
        return {
          ...state,
          currentIndex: -1,
          isPlaying: false,
          currentTime: 0,
          duration: 0,
        };
      }
      return {
        ...state,
        currentIndex: clamped,
        isPlaying: true,
        pendingSeek: null,
      };
    }),

  togglePlay: () =>
    set((state) => ({
      isPlaying: !state.isPlaying,
    })),

  setIsPlaying: (value: boolean) =>
    set(() => ({
      isPlaying: value,
    })),

  setVolume: (value: number) =>
    set(() => ({
      volume: Math.min(1, Math.max(0, value)),
    })),

  setPosition: (t: number) =>
    set(() => ({
      currentTime: t,
    })),

  setDuration: (d: number) =>
    set(() => ({
      duration: d,
    })),

  seek: (t: number) =>
    set(() => ({
      pendingSeek: t,
      currentTime: t,
    })),

  clearPendingSeek: () =>
    set(() => ({
      pendingSeek: null,
    })),

  next: () => {
    const state = get();
    const len = state.playlist.length;
    if (len === 0) return;

    let nextIndex = state.currentIndex + 1;
    if (nextIndex >= len) {
      if (state.mode === "repeat-all") {
        nextIndex = 0;
      } else {
        // normal / repeat-one：到末尾就停
        set({
          isPlaying: false,
        });
        return;
      }
    }

    set({
      currentIndex: nextIndex,
      isPlaying: true,
      pendingSeek: null,
    });
  },

  prev: () => {
    const state = get();
    const len = state.playlist.length;
    if (len === 0) return;

    let prevIndex = state.currentIndex - 1;
    if (prevIndex < 0) {
      if (state.mode === "repeat-all") {
        prevIndex = len - 1;
      } else {
        prevIndex = 0;
      }
    }

    set({
      currentIndex: prevIndex,
      isPlaying: true,
      pendingSeek: null,
    });
  },
}));

export default usePlayerStore;
