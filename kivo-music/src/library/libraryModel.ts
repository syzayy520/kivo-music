// 路径：src/library/libraryModel.ts
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
  addedAt?: string | null;
  favorite?: boolean;
  playCount?: number;
  lastPlayedAt?: string | null;
  rating?: number;
  [key: string]: any;
}

export function getTrackIdentity(
  track: LibraryTrack | null | undefined,
): string | null {
  if (!track) return null;
  if (track.trackId != null) return String(track.trackId);
  if (track.id != null) return String(track.id);
  if (track.filePath) return track.filePath;
  if (track.path) return track.path;
  if (track.location) return track.location;
  return null;
}

export function getTrackTitle(track: LibraryTrack): string {
  if (!track) return "未知曲目";
  if (track.title) return track.title;
  if (track.name) return track.name;

  const raw = track.filePath || track.path || track.location || "未知曲目";
  const fileName = String(raw).split(/[\\/]/).pop() || String(raw);
  const dotIndex = fileName.lastIndexOf(".");
  return dotIndex > 0 ? fileName.slice(0, dotIndex) : fileName;
}

export function getTrackArtist(track: LibraryTrack): string {
  return track?.artist || "未知艺人";
}

export function getTrackAlbum(track: LibraryTrack): string {
  return track?.album || "未分专辑";
}

async function persistLibraryAsync(tracks: LibraryTrack[]): Promise<void> {
  try {
    await saveLibrary(tracks as any[]);
  } catch (error) {
    console.error("[libraryModel] saveLibrary error:", error);
  }
}

export function getCurrentLibraryTracks(): LibraryTrack[] {
  const store: any = useLibrary as any;
  if (!store || typeof store.getState !== "function") return [];
  const state = store.getState();
  return Array.isArray(state?.tracks) ? state.tracks : [];
}

export function saveCurrentLibrary(): void {
  const tracks = getCurrentLibraryTracks();
  void persistLibraryAsync(tracks);
}

export function safeUpdateLibraryTracks(
  updater: (tracks: LibraryTrack[]) => LibraryTrack[],
): void {
  const store: any = useLibrary as any;
  if (!store || typeof store.getState !== "function" || typeof store.setState !== "function") {
    console.warn("[libraryModel] useLibrary 没有 getState/setState，跳过持久化更新。");
    return;
  }

  const prevState = store.getState();
  const prevTracks: LibraryTrack[] = Array.isArray(prevState?.tracks)
    ? prevState.tracks
    : [];

  const nextTracks = updater(prevTracks);
  store.setState({ ...prevState, tracks: nextTracks });
  void persistLibraryAsync(nextTracks);
}

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

export function startPlaylistFrom(tracks: LibraryTrack[], startIndex = 0): void {
  if (!tracks || tracks.length === 0) return;

  const store: any = usePlayerStore as any;
  if (!store || typeof store.setState !== "function" || typeof store.getState !== "function") {
    console.warn("[libraryModel] usePlayerStore 缺少 setState/getState，无法启动播放队列。");
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
