// src/playlists/playQueueModel.ts
import { usePlayerStore } from "../store/player";
import type { PlayerTrack } from "../types/track";

/**
 * 播放队列相关的业务封装。
 *
 * 约定：
 * - 队列的真实数据存放在 playerStore.playlist 中；
 * - 所有对队列的写操作都通过这里提供的方法完成；
 * - UI 组件只读 playlist / currentIndex，不直接改数组。
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

/**
 * 获取全局的 player store。
 */
function getStore(): PlayerStoreLike {
  const store = usePlayerStore as unknown as PlayerStoreLike;
  if (!store || typeof store.getState !== "function") {
    // 理论上不会发生，做一层兜底日志即可。
    console.warn(
      "[playQueueModel] usePlayerStore.getState 不存在，无法操作队列。",
    );
  }
  return store;
}

/**
 * 从状态中安全地拿到 playlist 副本。
 */
function getPlaylistFromState(state: any): PlayerTrack[] {
  if (!state) return [];
  return Array.isArray(state.playlist)
    ? (state.playlist as PlayerTrack[]).slice()
    : [];
}

/**
 * 将新的 playlist 写回 store。
 * - 优先使用 state.setPlaylist 保持和核心逻辑一致；
 * - 兼容没有 setPlaylist 的情况，直接用 setState 覆盖。
 */
function applyPlaylist(
  nextList: PlayerTrack[],
  store: PlayerStoreLike,
  state: any,
): void {
  if (typeof state?.setPlaylist === "function") {
    state.setPlaylist(nextList);
    return;
  }

  if (typeof store.setState === "function") {
    store.setState((prev) => ({
      ...prev,
      playlist: nextList,
      // 为了兼容旧代码，tracks 始终与 playlist 保持一致
      tracks: nextList,
    }));
  }
}

/**
 * 获取当前队列快照（只读）
 */
export function getQueueSnapshot(): QueueSnapshot {
  const store = getStore();
  const state = store.getState?.() ?? {};

  const playlist = getPlaylistFromState(state);
  const currentIndex =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;
  const isPlaying = !!state.isPlaying;

  return { playlist, currentIndex, isPlaying };
}

/**
 * 清空整个队列（包括当前正在播放的曲目），播放器会停止播放。
 */
export function clearQueue(): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  if (typeof state.setPlaylist === "function") {
    // 交给核心逻辑处理重置细节
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

/**
 * 从队列中移除指定位置的曲目（当前播放的那首暂时不允许移除）
 */
export function removeFromQueue(index: number): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const list = getPlaylistFromState(state);
  if (index < 0 || index >= list.length) return;

  const currentIndex: number =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;

  // 当前播放的曲目由 UI 控制不显示删除按钮，这里做一次兜底保护
  if (index === currentIndex) {
    console.warn(
      "[playQueueModel] 暂不支持直接从队列中移除当前正在播放的曲目。",
    );
    return;
  }

  const nextList = list.slice(0, index).concat(list.slice(index + 1));
  applyPlaylist(nextList, store, state);
}

/**
 * 让队列从某个索引开始播放，等价于调用 playerStore.playTrack(index)
 */
export function playFromQueue(index: number): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const list = Array.isArray(state.playlist) ? state.playlist : [];
  if (index < 0 || index >= list.length) return;

  if (typeof state.playTrack === "function") {
    state.playTrack(index);
  } else {
    console.warn(
      "[playQueueModel] 当前状态缺少 playTrack 方法，无法从队列中开始播放。",
    );
  }
}

/**
 * 在队列尾部追加一批歌曲。
 * - 不改变当前播放的位置；
 * - 追加的曲目会出现在 Up Next 的队尾。
 */
export function appendToQueue(tracks: PlayerTrack[]): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const base = getPlaylistFromState(state);
  const additions: PlayerTrack[] = Array.isArray(tracks) ? tracks.slice() : [];
  if (additions.length === 0) return;

  const nextList = base.concat(additions);
  applyPlaylist(nextList, store, state);
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

  const base = getPlaylistFromState(state);
  const additions: PlayerTrack[] = Array.isArray(tracks) ? tracks.slice() : [];
  if (additions.length === 0) return;

  const currentIndex: number =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;

  if (base.length === 0 || currentIndex < 0 || currentIndex >= base.length) {
    appendToQueue(additions);
    return;
  }

  const insertIndex = currentIndex + 1;
  const nextList: PlayerTrack[] = [
    ...base.slice(0, insertIndex),
    ...additions,
    ...base.slice(insertIndex),
  ];

  applyPlaylist(nextList, store, state);
}

/**
 * 在当前队列内移动一首歌的位置。
 *
 * 用法示例：
 * - 上移一行：moveInQueue(index, index - 1)
 * - 下移一行：moveInQueue(index, index + 1)
 *
 * 设计要点：
 * - 尽量保持当前播放曲目不变，currentIndex 由 setPlaylist 内部根据 track 身份重新计算；
 * - 索引越界、队列长度 <= 1 时不做任何修改。
 */
export function moveInQueue(fromIndex: number, toIndex: number): void {
  const store = getStore();
  const state = store.getState?.();
  if (!state) return;

  const base = getPlaylistFromState(state);
  const length = base.length;
  if (length <= 1) return;

  if (fromIndex < 0 || fromIndex >= length) return;

  let target = toIndex;
  if (target < 0) target = 0;
  if (target >= length) target = length - 1;

  if (fromIndex === target) return;

  const working = base.slice();
  const [moved] = working.splice(fromIndex, 1);
  if (!moved) return;

  // 此时 working 长度为 length - 1，target 仍然是“期望最终位置”，允许插到末尾。
  if (target > working.length) {
    target = working.length;
  }
  working.splice(target, 0, moved);

  applyPlaylist(working, store, state);
}
