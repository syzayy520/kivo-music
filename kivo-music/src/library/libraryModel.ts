// 路径：src/library/libraryModel.ts
//
// 说明：
//   负责定义资料库中的曲目模型（LibraryTrack），
//   并提供围绕曲目的基础操作：
//   - 统一标题 / 艺人 / 专辑 / identity 的读取逻辑；
//   - 更新播放统计（播放次数 / 最近播放时间）；
//   - 切换“喜欢”状态；
//   - 启动播放队列；
//   - 从 store 中取出当前曲库并持久化到磁盘。
//
//   这里保留对 useLibrary / usePlayerStore / saveLibrary 的依赖，
//   以兼容现有调用方（例如 LibraryPage）。

import { useLibrary } from "../store/library";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";

export type SortKey = "default" | "title" | "artist" | "album" | "recent";

/**
 * 资料库中的曲目类型。
 *
 * 为了兼容历史数据，这里字段都比较宽松，
 * 新代码建议逐步向更统一的结构靠拢。
 */
export interface LibraryTrack {
  id?: string | number;
  trackId?: string | number;

  // 文件路径相关
  filePath?: string;
  path?: string;
  location?: string;

  // 基本元信息
  title?: string;
  name?: string;
  artist?: string;
  album?: string;

  // 运行时元数据
  addedAt?: string | null;
  favorite?: boolean;
  playCount?: number;
  lastPlayedAt?: string | null;
  rating?: number;

  // 其它扩展字段
  [key: string]: any;
}

/**
 * 统一获取曲目 identity（用于高亮 / 去重 / 比对）。
 */
export function getTrackIdentity(
  track: LibraryTrack | null | undefined,
): string | null {
  if (!track) return null;

  if (track.trackId != null) return String(track.trackId);
  if (track.id != null) return String(track.id);

  const rawPath =
    track.filePath ?? (track as any).path ?? (track as any).location;
  if (typeof rawPath === "string" && rawPath.trim()) {
    return rawPath;
  }

  const title = getTrackTitle(track);
  const artist = getTrackArtist(track);
  if (title || artist) return `${artist}::${title}`;

  return null;
}

/**
 * 统一获取标题。
 */
export function getTrackTitle(track: LibraryTrack): string {
  if (!track) return "未知曲目";
  if (track.title && typeof track.title === "string") return track.title;
  if (track.name && typeof track.name === "string") return track.name;

  const raw =
    track.filePath || track.path || track.location || "未知曲目";
  const fileName = String(raw).split(/[\\/]/).pop() || String(raw);
  const dotIndex = fileName.lastIndexOf(".");
  return dotIndex > 0 ? fileName.slice(0, dotIndex) : fileName;
}

/**
 * 统一获取艺人。
 */
export function getTrackArtist(track: LibraryTrack): string {
  if (!track) return "未知艺人";
  if (track.artist && typeof track.artist === "string") return track.artist;

  const anyTrack = track as any;

  if (typeof anyTrack.artistName === "string" && anyTrack.artistName.trim()) {
    return anyTrack.artistName;
  }

  if (Array.isArray(anyTrack.artists) && anyTrack.artists.length > 0) {
    const joined = anyTrack.artists.filter(Boolean).join(", ");
    if (joined.trim()) return joined;
  }

  return "未知艺人";
}

/**
 * 统一获取专辑。
 */
export function getTrackAlbum(track: LibraryTrack): string | null {
  if (!track) return null;
  if (track.album && typeof track.album === "string") return track.album;

  const anyTrack = track as any;
  if (typeof anyTrack.albumName === "string" && anyTrack.albumName.trim()) {
    return anyTrack.albumName;
  }

  return null;
}

/**
 * 从 useLibrary store 中取出当前曲库。
 */
function getCurrentLibraryTracks(): LibraryTrack[] {
  const store: any = useLibrary as any;
  if (!store || typeof store.getState !== "function") return [];
  const state = store.getState();
  return Array.isArray(state?.tracks) ? state.tracks : [];
}

/**
 * 异步将曲库持久化到磁盘。
 */
async function persistLibraryAsync(
  tracks: LibraryTrack[],
): Promise<void> {
  try {
    // saveLibrary 接受 MusicTrack[]，这里认为 LibraryTrack 与之兼容
    await saveLibrary(tracks as any);
  } catch (error) {
    console.error("[libraryModel] saveLibrary error", error);
  }
}

/**
 * 立即保存当前曲库（供外部调用）。
 *
 * 典型用法：导入 / 清空曲库后手动触发一次持久化。
 */
export function saveCurrentLibrary(): void {
  const tracks = getCurrentLibraryTracks();
  void persistLibraryAsync(tracks);
}

/**
 * 在保持状态/持久化一致的前提下，安全更新曲库。
 *
 * - 先从 useLibrary 取出当前 tracks；
 * - 调用 updater 生成新的 tracks；
 * - setState 写回；
 * - 再异步调用 saveLibrary 持久化。
 */
export function safeUpdateLibraryTracks(
  updater: (tracks: LibraryTrack[]) => LibraryTrack[],
): void {
  const store: any = useLibrary as any;
  if (
    !store ||
    typeof store.getState !== "function" ||
    typeof store.setState !== "function"
  ) {
    console.warn(
      "[libraryModel] useLibrary 没有 getState/setState，跳过持久化更新。",
    );
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
 * 切换“喜欢”状态。
 */
export function toggleFavorite(track: LibraryTrack): void {
  const identity = getTrackIdentity(track);
  if (!identity) return;

  safeUpdateLibraryTracks((tracks) =>
    tracks.map((t) => {
      if (getTrackIdentity(t) !== identity) return t;
      const current = !!t.favorite;
      return {
        ...t,
        favorite: !current,
      };
    }),
  );
}

/**
 * 为指定曲目记录一次播放统计。
 *
 * - playCount + 1
 * - lastPlayedAt 更新为当前时间
 */
export function bumpPlayStatsForTrack(track: LibraryTrack): void {
  const identity = getTrackIdentity(track);
  if (!identity) return;

  safeUpdateLibraryTracks((tracks) =>
    tracks.map((t) => {
      if (getTrackIdentity(t) !== identity) return t;

      const currentCount =
        typeof t.playCount === "number" ? t.playCount : 0;

      return {
        ...t,
        playCount: currentCount + 1,
        lastPlayedAt: new Date().toISOString(),
      };
    }),
  );
}

/**
 * 启动一个播放队列，从指定索引开始播放。
 *
 * - tracks 为 LibraryTrack 数组；
 * - startIndex 为起始播放位置；
 * - 直接操作 usePlayerStore 的 playlist 等字段。
 */
export function startPlaylistFrom(
  tracks: LibraryTrack[],
  startIndex = 0,
): void {
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

  const clampedIndex = Math.min(
    Math.max(startIndex, 0),
    tracks.length - 1,
  );

  store.setState((prev: any) => ({
    ...prev,
    playlist: tracks,
    currentIndex: clampedIndex,
    isPlaying: true,
    pendingSeek: 0,
    currentTime: 0,
  }));
}
