import { create } from "zustand";

export type PlayMode = "order" | "repeat-all" | "repeat-one";

export interface PlayerTrack {
  id: string;
  title: string;
  artist: string;
  album?: string;
  filePath: string;
  duration?: number;

  // 封面缓存相关字段（可选）
  coverId?: string;
  coverPath?: string;
}

export interface PlayerState {
  playlist: PlayerTrack[];
  // 为了兼容旧代码，提供 tracks 的别名
  tracks: PlayerTrack[];

  currentIndex: number;
  isPlaying: boolean;

  currentTime: number;
  duration: number;
  pendingSeek: number | null;

  volume: number;

  mode: PlayMode;

  setPlaylist: (tracks: PlayerTrack[]) => void;
  setTracks: (tracks: PlayerTrack[]) => void;
  playTrack: (index: number) => void;
  togglePlay: () => void;

  setVolume: (value: number) => void;
  seek: (seconds: number) => void;
  setPosition: (seconds: number) => void;
  setDuration: (seconds: number) => void;
  clearPendingSeek: () => void;

  setIsPlaying: (flag: boolean) => void;
  next: () => void;
  prev: () => void;

  setMode: (mode: PlayMode) => void;
  cycleMode: () => void;
}

function clampVolume(v: number): number {
  if (Number.isNaN(v)) return 1;
  if (v < 0) return 0;
  if (v > 1) return 1;
  return v;
}

function getTrackIdentity(t: PlayerTrack | undefined): string | null {
  if (!t) return null;
  return t.id || t.filePath || null;
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

  mode: "order",

  setPlaylist: (tracks) =>
    set((state) => {
      const nextList = Array.isArray(tracks) ? tracks.slice() : [];
      if (nextList.length === 0) {
        return {
          ...state,
          playlist: [],
          tracks: [],
          currentIndex: -1,
          isPlaying: false,
          currentTime: 0,
          duration: 0,
          pendingSeek: null,
        };
      }

      const current =
        state.currentIndex >= 0 && state.currentIndex < state.playlist.length
          ? state.playlist[state.currentIndex]
          : undefined;

      let newIndex = 0;

      const currentId = getTrackIdentity(current as PlayerTrack | undefined);
      if (currentId) {
        const found = nextList.findIndex(
          (t) => getTrackIdentity(t) === currentId,
        );
        if (found >= 0) {
          newIndex = found;
        }
      } else if (state.currentIndex >= 0 && state.currentIndex < nextList.length) {
        newIndex = state.currentIndex;
      }

      return {
        ...state,
        playlist: nextList,
        tracks: nextList,
        currentIndex: newIndex,
      };
    }),

  setTracks: (tracks) => {
    const list = Array.isArray(tracks) ? tracks : [];
    get().setPlaylist(list as PlayerTrack[]);
  },

  playTrack: (index) =>
    set((state) => {
      const list = state.playlist;
      if (!list || list.length === 0) {
        return state;
      }
      let idx = index;
      if (idx < 0) idx = 0;
      if (idx >= list.length) idx = list.length - 1;

      return {
        ...state,
        currentIndex: idx,
        isPlaying: true,
        pendingSeek: 0,
        currentTime: 0,
      };
    }),

  togglePlay: () =>
    set((state) => {
      if (!state.playlist.length) {
        return state;
      }
      return {
        ...state,
        isPlaying: !state.isPlaying,
      };
    }),

  setVolume: (value) =>
    set(() => ({
      volume: clampVolume(value),
    })),

  seek: (seconds) =>
    set((state) => ({
      ...state,
      pendingSeek: seconds,
      currentTime: seconds,
    })),

  setPosition: (seconds) =>
    set((state) => {
      if (!Number.isFinite(seconds)) return state;
      return {
        ...state,
        currentTime: seconds,
      };
    }),

  setDuration: (seconds) =>
    set((state) => {
      if (!Number.isFinite(seconds) || seconds <= 0) {
        return state;
      }
      return {
        ...state,
        duration: seconds,
      };
    }),

  clearPendingSeek: () =>
    set((state) => ({
      ...state,
      pendingSeek: null,
    })),

  setIsPlaying: (flag) =>
    set((state) => ({
      ...state,
      isPlaying: !!flag,
    })),

  next: () =>
    set((state) => {
      const list = state.playlist;
      if (!list || list.length === 0) {
        return state;
      }

      const lastIndex = list.length - 1;

      // 单曲循环：只回到本曲开头
      if (state.mode === "repeat-one") {
        return {
          ...state,
          pendingSeek: 0,
          currentTime: 0,
          isPlaying: true,
        };
      }

      // 当前还没确定 index 时，直接从 0 开始
      if (state.currentIndex < 0) {
        return {
          ...state,
          currentIndex: 0,
          pendingSeek: 0,
          currentTime: 0,
          isPlaying: true,
        };
      }

      // 已经是最后一首
      if (state.currentIndex >= lastIndex) {
        if (state.mode === "repeat-all") {
          // 列表循环：回到第一首
          return {
            ...state,
            currentIndex: 0,
            pendingSeek: 0,
            currentTime: 0,
            isPlaying: true,
          };
        }
        // 顺序播放：停在最后一首并暂停
        return {
          ...state,
          isPlaying: false,
        };
      }

      // 正常下一首
      return {
        ...state,
        currentIndex: state.currentIndex + 1,
        pendingSeek: 0,
        currentTime: 0,
        isPlaying: true,
      };
    }),

  prev: () =>
    set((state) => {
      const list = state.playlist;
      if (!list || list.length === 0) {
        return state;
      }
      if (state.currentIndex < 0) {
        return state;
      }

      // 若当前已播放超过 3 秒，先回到本曲开头
      if (state.currentTime > 3) {
        return {
          ...state,
          pendingSeek: 0,
          currentTime: 0,
        };
      }

      // 已经是第一首了，就只回到头
      if (state.currentIndex === 0) {
        return {
          ...state,
          pendingSeek: 0,
          currentTime: 0,
        };
      }

      // 正常上一首
      return {
        ...state,
        currentIndex: state.currentIndex - 1,
        pendingSeek: 0,
        currentTime: 0,
        isPlaying: true,
      };
    }),

  setMode: (mode) =>
    set((state) => ({
      ...state,
      mode,
    })),

  cycleMode: () =>
    set((state) => {
      let nextMode: PlayMode;
      if (state.mode === "order") nextMode = "repeat-all";
      else if (state.mode === "repeat-all") nextMode = "repeat-one";
      else nextMode = "order";

      return {
        ...state,
        mode: nextMode,
      };
    }),
}));

export default usePlayerStore;
