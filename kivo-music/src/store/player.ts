// src/store/player.ts
import { create } from "zustand";
import {
  PlayMode as CorePlayMode,
  computeNextIndex,
  computePrevIndex,
} from "../playerStateMachine";

export type PlayMode = CorePlayMode;

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

function clampIndex(index: number, list: unknown[]): number {
  if (!Array.isArray(list) || list.length === 0) return -1;
  if (index < 0) return 0;
  if (index >= list.length) return list.length - 1;
  return index;
}

/**
 * 尝试从 track 对象中解析出“身份 Id”，用于在列表变动后保持当前播放行。
 */
function getTrackIdentity(track: PlayerTrack | undefined): string | null {
  if (!track) return null;
  if (track.id) return String(track.id);
  if (track.filePath) return track.filePath;

  const anyTrack = track as any;
  if (anyTrack.identity) return String(anyTrack.identity);
  if (anyTrack.path) return String(anyTrack.path);
  if (anyTrack.location) return String(anyTrack.location);

  return null;
}

export const usePlayerStore = create<PlayerState>((set) => ({
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

  setTracks: (tracks) =>
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

      return {
        ...state,
        playlist: nextList,
        tracks: nextList,
        currentIndex: clampIndex(state.currentIndex, nextList),
      };
    }),

  playTrack: (index) =>
    set((state) => {
      const list = state.playlist;
      if (!list || list.length === 0) {
        return state;
      }

      const safeIndex = clampIndex(index, list);
      if (safeIndex < 0) {
        return state;
      }

      return {
        ...state,
        currentIndex: safeIndex,
        pendingSeek: 0,
        currentTime: 0,
        isPlaying: true,
      };
    }),

  togglePlay: () =>
    set((state) => {
      if (!state.playlist || state.playlist.length === 0) {
        return state;
      }

      // 如果还没有 currentIndex，默认从 0 开始播放
      const newIndex =
        state.currentIndex < 0
          ? 0
          : clampIndex(state.currentIndex, state.playlist);

      return {
        ...state,
        currentIndex: newIndex,
        isPlaying: !state.isPlaying,
      };
    }),

  setVolume: (value) =>
    set((state) => ({
      ...state,
      volume: clampVolume(value),
    })),

  seek: (seconds) =>
    set((state) => ({
      ...state,
      pendingSeek: Number.isFinite(seconds) ? Math.max(seconds, 0) : 0,
    })),

  setPosition: (seconds) =>
    set((state) => ({
      ...state,
      currentTime: Number.isFinite(seconds) ? Math.max(seconds, 0) : 0,
    })),

  setDuration: (seconds) =>
    set((state) => ({
      ...state,
      duration: Number.isFinite(seconds) ? Math.max(seconds, 0) : 0,
    })),

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

      const cursor = {
        length: list.length,
        currentIndex: state.currentIndex,
      };

      const nextIndex = computeNextIndex(cursor, state.mode);

      // 顺序播放且已经到尾：停止播放，不改 index
      if (nextIndex === -1) {
        return {
          ...state,
          isPlaying: false,
        };
      }

      return {
        ...state,
        currentIndex: nextIndex,
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

      // 若当前已播放超过 3 秒，先回到本曲开头
      if (state.currentTime > 3) {
        return {
          ...state,
          pendingSeek: 0,
          currentTime: 0,
        };
      }

      const cursor = {
        length: list.length,
        currentIndex:
          state.currentIndex < 0
            ? 0
            : clampIndex(state.currentIndex, list),
      };

      const prevIndex = computePrevIndex(cursor, state.mode);

      // prevIndex 可能等于当前 index（例如 repeat-one / index=0 时），
      // 这里仍然把它当成“回到本曲开头处理”，不强制改变播放状态。
      if (prevIndex === state.currentIndex) {
        return {
          ...state,
          pendingSeek: 0,
          currentTime: 0,
        };
      }

      return {
        ...state,
        currentIndex: prevIndex,
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
