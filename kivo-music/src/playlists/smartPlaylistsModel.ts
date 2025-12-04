// src/playlists/smartPlaylistsModel.ts
import {
  LibraryTrack,
  getTrackIdentity,
} from "../library/libraryModel";

export type PlaylistMode =
  | "queue"
  | "recentAdded"
  | "recentPlayed"
  | "mostPlayed"
  | "favorites";

function uniqueTracks(tracks: LibraryTrack[]): LibraryTrack[] {
  const seen = new Set<string>();
  const result: LibraryTrack[] = [];

  for (const t of tracks) {
    const id = getTrackIdentity(t) ?? JSON.stringify(t);
    if (seen.has(id)) continue;
    seen.add(id);
    result.push(t);
  }

  return result;
}

export function buildSmartPlaylist(
  mode: PlaylistMode,
  opts: {
    libraryTracks: LibraryTrack[];
    currentQueue: LibraryTrack[];
  },
): LibraryTrack[] {
  const { libraryTracks, currentQueue } = opts;

  if (mode === "queue") {
    return Array.isArray(currentQueue) ? currentQueue.slice() : [];
  }

  const tracks = Array.isArray(libraryTracks) ? libraryTracks : [];

  switch (mode) {
    case "recentAdded": {
      const sorted = tracks.slice().sort((a, b) => {
        const aTime = a.addedAt ? Date.parse(a.addedAt) : 0;
        const bTime = b.addedAt ? Date.parse(b.addedAt) : 0;
        return bTime - aTime;
      });
      return uniqueTracks(sorted).slice(0, 200);
    }
    case "recentPlayed": {
      const withPlay = tracks.filter(
        (t) => t.lastPlayedAt && !Number.isNaN(Date.parse(t.lastPlayedAt)),
      );
      withPlay.sort((a, b) => {
        const aTime = a.lastPlayedAt ? Date.parse(a.lastPlayedAt) : 0;
        const bTime = b.lastPlayedAt ? Date.parse(b.lastPlayedAt) : 0;
        return bTime - aTime;
      });
      return uniqueTracks(withPlay).slice(0, 200);
    }
    case "mostPlayed": {
      const withCount = tracks.filter(
        (t) => typeof t.playCount === "number" && (t.playCount ?? 0) > 0,
      );
      withCount.sort((a, b) => {
        const aCount = a.playCount ?? 0;
        const bCount = b.playCount ?? 0;
        if (bCount !== aCount) return bCount - aCount;

        const aTime = a.lastPlayedAt ? Date.parse(a.lastPlayedAt) : 0;
        const bTime = b.lastPlayedAt ? Date.parse(b.lastPlayedAt) : 0;
        return bTime - aTime;
      });
      return uniqueTracks(withCount).slice(0, 200);
    }
    case "favorites": {
      const fav = tracks.filter((t) => !!t.favorite);
      return uniqueTracks(fav).slice(0, 200);
    }
    default:
      return [];
  }
}

export function getPlaylistModeLabel(mode: PlaylistMode): string {
  switch (mode) {
    case "queue":
      return "当前队列";
    case "recentAdded":
      return "最近添加";
    case "recentPlayed":
      return "最近播放";
    case "mostPlayed":
      return "最常播放";
    case "favorites":
      return "喜欢的歌曲";
    default:
      return "当前队列";
  }
}

export function getPlaylistModeDescription(mode: PlaylistMode): string {
  switch (mode) {
    case "queue":
      return "显示当前实际播放队列，可以直接调整或清空。";
    case "recentAdded":
      return "按加入时间从近到远列出最近导入的歌曲。";
    case "recentPlayed":
      return "按最近播放时间从近到远列出你听过的歌曲。";
    case "mostPlayed":
      return "按播放次数从高到低列出你最常听的歌曲。";
    case "favorites":
      return "只显示已标记为“喜欢”的歌曲。";
    default:
      return "";
  }
}
