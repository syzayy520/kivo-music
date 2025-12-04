// src/persistence/cover-cache.index.ts

export interface KivoTrackLike {
  id?: string | number;
  trackId?: string | number;
  filePath?: string;
  path?: string;
  // 其他字段随意
  [key: string]: unknown;
}

export interface CoverRecord {
  trackId: string;
  sourcePath: string;
  cachedPath: string;
  updatedAt: string;
}

export type CoverIndex = Record<string, CoverRecord>;

export interface FolderCoverRecord {
  folderPath: string;
  cachedPath: string | null;
  hasCover: boolean;
  updatedAt: string;
}

export type FolderCoverIndex = Record<string, FolderCoverRecord>;

export interface CoverCacheStats {
  cacheDir: string;
  fileCount: number;
  totalBytes: number;
  humanReadableSize: string;
  trackEntries: number;
  folderEntries: number;
}

export interface BrokenCoverCleanupResult {
  coverChecked: number;
  coverRemoved: number;
  folderChecked: number;
  folderRemoved: number;
}

// 文件夹封面候选文件名
export const FOLDER_COVER_CANDIDATES = [
  "cover.jpg",
  "cover.jpeg",
  "cover.png",
  "folder.jpg",
  "folder.jpeg",
  "folder.png",
  "album.jpg",
  "album.jpeg",
  "album.png",
];

export function hashString(input: string): string {
  let hash = 0;
  if (!input.length) return "0";
  for (let i = 0; i < input.length; i++) {
    const chr = input.charCodeAt(i);
    hash = (hash << 5) - hash + chr;
    hash |= 0; // 32bit
  }
  return Math.abs(hash).toString(16);
}

export function formatBytes(bytes: number): string {
  if (bytes <= 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const value = bytes / Math.pow(1024, i);
  return `${value.toFixed(1)} ${units[i]}`;
}

/**
 * 把 track 映射成索引用的 key
 */
export function getTrackKey(
  track: KivoTrackLike | null | undefined,
): string {
  if (!track) return "";
  if (track.trackId !== undefined && track.trackId !== null) {
    return String(track.trackId);
  }
  if (track.id !== undefined && track.id !== null) {
    return String(track.id);
  }
  if (typeof track.filePath === "string") {
    return `path:${track.filePath}`;
  }
  if (typeof track.path === "string") {
    return `path:${track.path}`;
  }
  return hashString(JSON.stringify(track));
}

/**
 * 尝试从 track 中拿出真实文件路径
 */
export function getTrackFilePath(
  track: KivoTrackLike | null | undefined,
): string | null {
  if (!track) return null;
  if (typeof track.filePath === "string" && track.filePath.length > 0) {
    return track.filePath;
  }
  if (typeof track.path === "string" && track.path.length > 0) {
    return track.path;
  }
  return null;
}
