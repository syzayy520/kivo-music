// src/playlists/playQueueModel.ts
import { usePlayerStore, PlayerTrack } from "../store/player";

/**
 * 播放队列相关的业务封装。
 *
 * 注意：
 * - 真正的队列就是 playerStore 里的 playlist。
 * - 这里通过 usePlayerStore.getState() 读写，UI 组件不要直接改数组。
 */

export interface QueueSnapshot {
  playlist: PlayerTrack[];
  currentIndex: number;
  isPlaying: boolean;
}

type PlayerStoreLike = {
  getState?: () => any;
  setState?: (updater: (prev: any) => any) => void;
};

function getStore(): PlayerStoreLike {
  const store = usePlayerStore as unknown as PlayerStoreLike;
  if (!store || typeof store.getState !== "function") {
    console.warn(
      "[playQueueModel] usePlayerStore.getState 不存在，无法操作队列。",
    );
  }
  return store;
}

/** 获取当前队列快照（只读） */
export function getQueueSnapshot(): QueueSnapshot {
  const store = getStore();
  const state = store.getState?.() ?? {};

  const playlist: PlayerTrack[] = Array.isArray(state.playlist)
    ? state.playlist.slice()
    : [];

  const currentIndex =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;
  const isPlaying = !!state.isPlaying;

  return { playlist, currentIndex, isPlaying };
}

/** 清空整个队列（包括当前正在播放的曲目），播放器会停止播放。 */
export function clearQueue(): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  if (typeof state.setPlaylist === "function") {
    state.setPlaylist([]);
    return;
  }

  if (typeof store.setState === "function") {
    store.setState((prev) => ({
      ...prev,
      playlist: [],
      tracks: [],
      currentIndex: -1,
      isPlaying: false,
      currentTime: 0,
      duration: 0,
      pendingSeek: null,
    }));
  }
}

/** 从队列中移除指定位置的曲目（当前播放的那首暂时不允许移除） */
export function removeFromQueue(index: number): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const list: PlayerTrack[] = Array.isArray(state.playlist)
    ? state.playlist.slice()
    : [];

  if (index < 0 || index >= list.length) return;

  const currentIndex: number =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;

  if (index === currentIndex) {
    console.warn(
      "[playQueueModel] 暂不支持直接从队列中移除当前正在播放的曲目。",
    );
    return;
  }

  const nextList = list.slice(0, index).concat(list.slice(index + 1));

  if (typeof state.setPlaylist === "function") {
    state.setPlaylist(nextList);
  } else if (typeof store.setState === "function") {
    store.setState((prev) => ({
      ...prev,
      playlist: nextList,
      tracks: nextList,
    }));
  }
}

/** 让队列从某个索引开始播放，等价于调用 playerStore.playTrack(index) */
export function playFromQueue(index: number): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const list: PlayerTrack[] = Array.isArray(state.playlist)
    ? state.playlist
    : [];
  if (index < 0 || index >= list.length) return;

  if (typeof state.playTrack === "function") {
    state.playTrack(index);
  } else {
    console.warn(
      "[playQueueModel] 当前状态缺少 playTrack 方法，无法从队列中开始播放。",
    );
  }
}

/** 在队列尾部追加一批歌曲 */
export function appendToQueue(tracks: PlayerTrack[]): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const base: PlayerTrack[] = Array.isArray(state.playlist)
    ? state.playlist.slice()
    : [];
  const additions: PlayerTrack[] = Array.isArray(tracks) ? tracks.slice() : [];
  if (additions.length === 0) return;

  const nextList = base.concat(additions);

  if (typeof state.setPlaylist === "function") {
    state.setPlaylist(nextList);
  } else if (typeof store.setState === "function") {
    store.setState((prev) => ({
      ...prev,
      playlist: nextList,
      tracks: nextList,
    }));
  }
}

/**
 * 将一批歌曲插到“下一首播放”的位置：
 * - 有当前曲目时，插在 currentIndex 后面；
 * - 没有当前曲目时，退化为 appendToQueue。
 */
export function playNext(tracks: PlayerTrack[]): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const base: PlayerTrack[] = Array.isArray(state.playlist)
    ? state.playlist.slice()
    : [];
  const additions: PlayerTrack[] = Array.isArray(tracks) ? tracks.slice() : [];
  if (additions.length === 0) return;

  const currentIndex: number =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;

  if (base.length === 0 || currentIndex < 0 || currentIndex >= base.length) {
    appendToQueue(additions);
    return;
  }

  const insertIndex = currentIndex + 1;
  const nextList = [
    ...base.slice(0, insertIndex),
    ...additions,
    ...base.slice(insertIndex),
  ];

  if (typeof state.setPlaylist === "function") {
    state.setPlaylist(nextList);
  } else if (typeof store.setState === "function") {
    store.setState((prev) => ({
      ...prev,
      playlist: nextList,
      tracks: nextList,
    }));
  }
}
