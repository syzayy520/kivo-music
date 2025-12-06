// 路径：src/library/libraryModel.ts
// 职责：
// - 定义曲库里使用的 LibraryTrack 结构（在 MusicTrack 基础上补充运行时字段）
// - 统一 track identity（避免 id/path 各写各的）
// - 提供常用的库操作：排序字段、收藏切换、播放统计、自库启动播放队列等
// - 所有对库的写操作最终都通过 saveLibrary 落盘

import { useLibrary } from "../store/library";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";

export type SortKey = "default" | "title" | "artist" | "album" | "recent";

export interface LibraryTrack {
  id?: string | number;
  trackId?: string | number;

  filePath?: string;
  path?: string;
  location?: string;

  title?: string;
  name?: string;
  artist?: string;
  album?: string;

  // 运行时统计 & 用户偏好
  addedAt?: string | null;
  favorite?: boolean;
  playCount?: number;
  lastPlayedAt?: string | null;
  rating?: number;

  // 允许扩展字段
  [key: string]: any;
}

/**
 * 从一条 track 中提取“身份标识”，用于在库数组中找同一首歌。
 * 优先级：
 * - track.id / track.trackId
 * - filePath / path / location
 * - title + artist 兜底
 */
export function getTrackIdentity(track: LibraryTrack | null | undefined): string | null {
  if (!track) return null;

  const idPart = track.id ?? track.trackId;
  if (idPart != null) return String(idPart);

  const filePath = track.filePath ?? track.path ?? track.location;
  if (filePath) return String(filePath);

  const title = track.title ?? track.name;
  if (!title) return null;

  const artist = track.artist ?? "";
  return `${title}::${artist}`;
}

// 统一从 track 里取「标题 / 艺人 / 专辑」，并把旧的中文占位符当成空值处理
export function getTrackTitle(track: LibraryTrack): string {
  const anyTrack = track as any;
  const raw: string =
    (typeof anyTrack?.title === "string" && anyTrack.title) ||
    (typeof anyTrack?.name === "string" && anyTrack.name) ||
    "";

  const trimmed = raw.trim();
  if (!trimmed) return "";
  return trimmed;
}

export function getTrackArtist(track: LibraryTrack): string {
  const anyTrack = track as any;
  const raw: string =
    (typeof anyTrack?.artist === "string" && anyTrack.artist) || "";

  const trimmed = raw.trim();
  // 之前写到库里的中文占位符，一律当成「没有艺人」
  if (!trimmed || trimmed === "未知艺人") {
    return "";
  }
  return trimmed;
}

export function getTrackAlbum(track: LibraryTrack): string {
  const anyTrack = track as any;
  const raw: string =
    (typeof anyTrack?.album === "string" && anyTrack.album) || "";

  const trimmed = raw.trim();
  // 之前有两种写法：未分专辑 / 未知专辑，都当成「没有专辑」
  if (
    !trimmed ||
    trimmed === "未分专辑" ||
    trimmed === "未知专辑"
  ) {
    return "";
  }
  return trimmed;
}



/**
 * 把当前库写回磁盘。
 * 注意：内部使用异步函数，但对外暴露为 fire-and-forget。
 */
async function persistLibraryAsync(tracks: LibraryTrack[]): Promise<void> {
  try {
    await saveLibrary(tracks as any[]);
  } catch (error) {
    // 这里用 console.error 即可，不打断主流程
    console.error("[libraryModel] saveLibrary error:", error);
  }
}

/**
 * 读取当前库中的所有曲目。
 */
export function getCurrentLibraryTracks(): LibraryTrack[] {
  const store: any = useLibrary as any;
  if (!store || typeof store.getState !== "function") return [];
  const state = store.getState();
  return Array.isArray(state?.tracks) ? state.tracks : [];
}

/**
 * 主动触发一次“保存当前曲库到磁盘”。
 */
export function saveCurrentLibrary(): void {
  const tracks = getCurrentLibraryTracks();
  void persistLibraryAsync(tracks);
}

/**
 * 安全更新库：
 * - 从 useLibrary 取出当前 tracks
 * - 通过 updater 生成新的数组
 * - setState 写回 store
 * - 异步调用 saveLibrary 落盘
 */
export function safeUpdateLibraryTracks(
  updater: (tracks: LibraryTrack[]) => LibraryTrack[],
): void {
  const store: any = useLibrary as any;
  if (!store || typeof store.getState !== "function" || typeof store.setState !== "function") {
    console.warn("[libraryModel] useLibrary 缺少 getState/setState，无法更新库。");
    return;
  }

  const prevState = store.getState();
  const prevTracks: LibraryTrack[] = Array.isArray(prevState?.tracks)
    ? prevState.tracks
    : [];

  const nextTracks = updater(prevTracks);

  store.setState({
    ...prevState,
    tracks: nextTracks,
  });

  void persistLibraryAsync(nextTracks);
}

/**
 * 切换“收藏”状态：
 * - favorite 从 true → false / false → true
 * - 只在库里改一份，播放队列里的对象通过引用保持一致
 */
export function toggleFavorite(track: LibraryTrack): void {
  const identity = getTrackIdentity(track);
  if (!identity) return;

  safeUpdateLibraryTracks((tracks) =>
    tracks.map((t) => {
      if (getTrackIdentity(t) !== identity) return t;
      const current = !!t.favorite;
      return { ...t, favorite: !current };
    }),
  );
}

/**
 * 播放统计：
 * - 每次调用视为“播放次数 +1”
 * - 同时更新 lastPlayedAt 为当前时间
 *
 * 约定：
 * - 目前在 HtmlAudioBackend / AudioEngine 那边，在“播放结束时”调用；
 * - 也可以在其他场景（例如手动标记“已播放”）显式调用。
 */
export function bumpPlayStatsForTrack(track: LibraryTrack): void {
  const identity = getTrackIdentity(track);
  if (!identity) return;

  safeUpdateLibraryTracks((tracks) =>
    tracks.map((t) => {
      if (getTrackIdentity(t) !== identity) return t;
      const currentCount = typeof t.playCount === "number" ? t.playCount : 0;
      return {
        ...t,
        playCount: currentCount + 1,
        lastPlayedAt: new Date().toISOString(),
      };
    }),
  );
}

/**
 * 从一组 LibraryTrack 启动一个播放队列，并从指定位置开始播放。
 *
 * 约定：
 * - tracks 为库里的曲目数组（保持原引用即可）
 * - startIndex 会被 clamp 到合法范围
 * - 播放状态（playlist/currentIndex/isPlaying/pendingSeek/currentTime）统一在这里设置
 *
 * 注意：
 * - 播放统计不在这里做，避免和“结束播放时的统计”重复。
 */
export function startPlaylistFrom(tracks: LibraryTrack[], startIndex = 0): void {
  if (!tracks || tracks.length === 0) return;

  const store: any = usePlayerStore as any;
  if (
    !store ||
    typeof store.setState !== "function" ||
    typeof store.getState !== "function"
  ) {
    console.warn(
      "[libraryModel] usePlayerStore 缺少 setState/getState，无法启动播放队列。",
    );
    return;
  }

  const clampedIndex = Math.min(Math.max(startIndex, 0), tracks.length - 1);

  store.setState((prev: any) => ({
    ...prev,
    playlist: tracks,
    currentIndex: clampedIndex,
    isPlaying: true,
    pendingSeek: 0,
    currentTime: 0,
  }));
}
