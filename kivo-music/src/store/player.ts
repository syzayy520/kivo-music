import { create } from "zustand";

export interface PlayerTrack {
  id: string;
  title: string;
  artist: string;
  album?: string;
  filePath: string;
  duration?: number;
}

export interface PlayerState {
  // 播放列表
  playlist: PlayerTrack[];
  currentIndex: number; // -1 表示没有当前曲目

  // 播放状态
  isPlaying: boolean;
  volume: number; // 0 ~ 1

  // 进度 & 时长
  currentTime: number;
  duration: number;

  // seek 用的中间状态（给 AudioEngine 处理）
  pendingSeek: number | null;

  // === 播放列表相关 ===
  setPlaylist: (tracks: PlayerTrack[]) => void;
  addTracks: (tracks: PlayerTrack[]) => void;

  // === 播放控制 ===
  playTrack: (index: number) => void;
  togglePlay: () => void;
  next: () => void;
  prev: () => void;

  // === 进度 / 音量 ===
  setVolume: (v: number) => void;
  setPosition: (t: number) => void;
  setDuration: (d: number) => void;
  seek: (t: number) => void;
  clearPendingSeek: () => void;
}

const clampIndex = (len: number, idx: number) => {
  if (len <= 0) return -1;
  if (idx < 0) return 0;
  if (idx >= len) return len - 1;
  return idx;
};

export const usePlayerStore = create<PlayerState>((set, get) => ({
  playlist: [],
  currentIndex: -1,
  isPlaying: false,
  volume: 1,
  currentTime: 0,
  duration: 0,
  pendingSeek: null,

  setPlaylist: (tracks) =>
    set(() => ({
      playlist: tracks,
      currentIndex: tracks.length > 0 ? 0 : -1,
    })),

  addTracks: (tracks) =>
    set((state) => ({
      playlist: [...state.playlist, ...tracks],
    })),

  playTrack: (index) =>
    set((state) => {
      const len = state.playlist.length;
      if (len === 0) return state;
      const i = clampIndex(len, index);
      return {
        currentIndex: i,
        isPlaying: true,
        currentTime: 0,
      };
    }),

  togglePlay: () =>
    set((state) => {
      if (state.playlist.length === 0) return state;
      if (state.currentIndex === -1) {
        // 没有当前曲目，从第一首开始
        return {
          ...state,
          currentIndex: 0,
          isPlaying: true,
        };
      }
      return { ...state, isPlaying: !state.isPlaying };
    }),

  next: () =>
    set((state) => {
      const len = state.playlist.length;
      if (len === 0) return state;
      if (state.currentIndex === -1) {
        return { ...state, currentIndex: 0, isPlaying: true, currentTime: 0 };
      }
      const nextIndex = state.currentIndex + 1;
      if (nextIndex >= len) {
        // 播完最后一首就停
        return { ...state, isPlaying: false };
      }
      return {
        ...state,
        currentIndex: nextIndex,
        isPlaying: true,
        currentTime: 0,
      };
    }),

  prev: () =>
    set((state) => {
      const len = state.playlist.length;
      if (len === 0) return state;
      if (state.currentIndex <= 0) {
        return { ...state, currentIndex: 0, currentTime: 0 };
      }
      return {
        ...state,
        currentIndex: state.currentIndex - 1,
        isPlaying: true,
        currentTime: 0,
      };
    }),

  setVolume: (v) =>
    set(() => ({
      volume: Math.min(1, Math.max(0, v)),
    })),

  setPosition: (t) =>
    set(() => ({
      currentTime: t,
    })),

  setDuration: (d) =>
    set(() => ({
      duration: d,
    })),

  seek: (t) =>
    set(() => ({
      pendingSeek: t,
      currentTime: t,
    })),

  clearPendingSeek: () =>
    set(() => ({
      pendingSeek: null,
    })),
}));

export default usePlayerStore;
