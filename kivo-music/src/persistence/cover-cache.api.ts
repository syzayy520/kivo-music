// src/persistence/cover-cache.api.ts
import { exists, copyFile, readDir, size } from "@tauri-apps/plugin-fs";
import { dirname, extname, join } from "@tauri-apps/api/path";

import {
  ensureCoverCacheDirExists,
  getCoverCacheDirPath,
} from "./cover-cache.files";
import {
  loadCoverIndex,
  saveCoverIndex,
  loadFolderCoverIndex,
  saveFolderCoverIndex,
  cleanupBrokenCoverEntries,
} from "./cover-cache.index";
import type { FolderCoverIndex } from "./cover-cache.index";

// ------------ 类型 ------------

export interface KivoTrackLike {
  trackId?: string | number;
  id?: string | number;
  filePath?: string;
  path?: string;
  location?: string;
  coverPath?: string;
  cover?: string;
  [key: string]: unknown;
}

export interface CoverCacheStats {
  cacheDir: string;
  fileCount: number;
  totalBytes: number;
  humanReadableSize: string;
  trackEntries: number;
  folderEntries: number;
}

const FOLDER_COVER_CANDIDATES = [
  "cover.jpg",
  "folder.jpg",
  "folder.png",
  "cover.png",
  "front.jpg",
  "front.png",
  "album.jpg",
  "album.png",
];

// ------------ 内部工具 ------------

function hashString(input: string): string {
  let hash = 0;
  if (!input.length) return "0";
  for (let i = 0; i < input.length; i++) {
    const chr = input.charCodeAt(i);
    hash = (hash << 5) - hash + chr;
    hash |= 0;
  }
  return String(Math.abs(hash));
}

function getTrackKey(track: KivoTrackLike | null | undefined): string {
  if (!track) return "null";
  if (track.trackId != null) return String(track.trackId);
  if (track.id != null) return String(track.id);
  if (typeof track.filePath === "string") return `path:${track.filePath}`;
  if (typeof track.path === "string") return `path:${track.path}`;
  if (typeof track.location === "string") return `loc:${track.location}`;
  return hashString(JSON.stringify(track));
}

function getTrackFilePath(
  track: KivoTrackLike | null | undefined,
): string | null {
  if (!track) return null;
  if (typeof track.filePath === "string" && track.filePath.length > 0) {
    return track.filePath;
  }
  if (typeof track.path === "string" && track.path.length > 0) {
    return track.path;
  }
  if (typeof track.location === "string" && track.location.length > 0) {
    return track.location;
  }
  return null;
}

/**
 * 确保给定文件夹有一条 folder-cover 记录：
 * - 若已有且 cachedPath 存在则直接返回；
 * - 否则在目录内扫描常见封面文件并记录；
 * - 若仍找不到，则记一条 cachedPath=null，避免下次反复扫描。
 */
async function ensureFolderCover(
  folderPath: string,
  folderIndex: FolderCoverIndex,
): Promise<string | null> {
  const existing = folderIndex[folderPath];

  if (existing && existing.cachedPath) {
    try {
      if (await exists(existing.cachedPath)) {
        return existing.cachedPath;
      }
    } catch {
      // ignore，后面会重新扫描
    }
  }

  try {
    const entries = await readDir(folderPath);
    const names = new Set(entries.map((e) => e.name));
    for (const candidate of FOLDER_COVER_CANDIDATES) {
      if (names.has(candidate)) {
        const candidatePath = await join(folderPath, candidate);
        if (await exists(candidatePath)) {
          folderIndex[folderPath] = {
            folderPath,
            cachedPath: candidatePath,
            updatedAt: new Date().toISOString(),
          };
          await saveFolderCoverIndex(folderIndex);
          return candidatePath;
        }
      }
    }
  } catch (error) {
    console.warn(
      "[CoverCache] 扫描文件夹封面失败:",
      folderPath,
      error,
    );
  }

  folderIndex[folderPath] = {
    folderPath,
    cachedPath: null,
    updatedAt: new Date().toISOString(),
  };
  await saveFolderCoverIndex(folderIndex);
  return null;
}

// ------------ 对外 API ------------

/** 封面缓存目录（会确保存在） */
export async function getCoverCacheDir(): Promise<string> {
  return getCoverCacheDirPath();
}

/** 确保封面缓存目录和两个索引文件都准备好。 */
export async function ensureCoverCacheReady(): Promise<void> {
  await ensureCoverCacheDirExists();
  await saveCoverIndex({});
  await saveFolderCoverIndex({});
}

/** 根据 track 在 covers.json 中查找已缓存的封面。 */
export async function getCachedCoverPath(
  track: KivoTrackLike,
): Promise<string | null> {
  const key = getTrackKey(track);
  const index = await loadCoverIndex();
  const rec = index[key];
  if (!rec || !rec.cachedPath) return null;

  try {
    if (await exists(rec.cachedPath)) {
      return rec.cachedPath;
    }
  } catch (error) {
    console.warn(
      "[CoverCache] 检查缓存封面文件失败，将忽略并删除索引:",
      rec.cachedPath,
      error,
    );
  }

  delete index[key];
  await saveCoverIndex(index);
  return null;
}

/**
 * 为某个 track 设置封面：
 * - 把源文件复制到封面缓存目录
 * - 在 covers.json 中记录索引
 * - 返回缓存文件路径
 */
export async function setCoverForTrack(
  track: KivoTrackLike,
  sourcePath: string,
): Promise<string | null> {
  const cacheDir = await ensureCoverCacheDirExists();
  const index = await loadCoverIndex();

  const key = getTrackKey(track);
  const ext = extname(sourcePath) || ".jpg";
  const fileName = `cover-${hashString(key)}${ext}`;
  const destPath = await join(cacheDir, fileName);

  try {
    await copyFile(sourcePath, destPath);
  } catch (error) {
    console.error("[CoverCache] 复制封面文件失败:", {
      sourcePath,
      destPath,
      error,
    });
    return null;
  }

  index[key] = {
    trackKey: key,
    cachedPath: destPath,
    updatedAt: new Date().toISOString(),
  };

  await saveCoverIndex(index);
  return destPath;
}

/**
 * 综合解析一个 track 的封面路径：
 * 1. track.coverPath / cover
 * 2. covers.json 索引
 * 3. 文件夹自动封面
 */
export async function resolveCoverPathForTrack(
  track: KivoTrackLike,
): Promise<string | null> {
  if (typeof track.coverPath === "string" && track.coverPath.length > 0) {
    try {
      if (await exists(track.coverPath)) {
        return track.coverPath;
      }
    } catch {
      // ignore
    }
  }
  if (typeof track.cover === "string" && track.cover.length > 0) {
    try {
      if (await exists(track.cover)) {
        return track.cover;
      }
    } catch {
      // ignore
    }
  }

  const cached = await getCachedCoverPath(track);
  if (cached) return cached;

  const filePath = getTrackFilePath(track);
  if (filePath) {
    const folder = await dirname(filePath);
    const folderIndex = await loadFolderCoverIndex();
    const folderCover = await ensureFolderCover(folder, folderIndex);
    if (folderCover) {
      return folderCover;
    }
  }

  return null;
}

/** 统计封面缓存占用情况。 */
export async function getCoverCacheStats(): Promise<CoverCacheStats> {
  const dir = await getCoverCacheDirPath();
  const coverIndex = await loadCoverIndex();
  const folderIndex = await loadFolderCoverIndex();

  const uniquePaths = new Set<string>();

  for (const rec of Object.values(coverIndex)) {
    if (rec.cachedPath) uniquePaths.add(rec.cachedPath);
  }
  for (const rec of Object.values(folderIndex)) {
    if (rec.cachedPath) uniquePaths.add(rec.cachedPath);
  }

  let totalBytes = 0;

  for (const p of uniquePaths) {
    try {
      const fileSize = await size(p);
      totalBytes += Number(fileSize || 0);
    } catch {
      // 忽略单个文件错误
    }
  }

  const fileCount = uniquePaths.size;
  const humanReadableSize = formatBytes(totalBytes);

  return {
    cacheDir: dir,
    fileCount,
    totalBytes,
    humanReadableSize,
    trackEntries: Object.keys(coverIndex).length,
    folderEntries: Object.keys(folderIndex).length,
  };
}

/** 清空封面缓存索引（不删除图片本身）。 */
export async function clearCoverCache(): Promise<void> {
  await saveCoverIndex({});
  await saveFolderCoverIndex({});
}

/** 自检 & 清理坏记录（具体统计逻辑在 cover-cache.index.ts）。 */
export { cleanupBrokenCoverEntries };

/**
 * 旧版 → 新版 封面缓存结构迁移入口。
 * 目前只是确保目录和索引文件存在；参数暂时保留以兼容调用方。
 */
export async function migrateCoverCache(
  _oldDir?: string,
  _newDir?: string,
): Promise<void> {
  await ensureCoverCacheReady();
}

// ------------ 工具：formatBytes ------------

export function formatBytes(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes <= 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  let index = 0;
  let value = bytes;
  while (value >= 1024 && index < units.length - 1) {
    value /= 1024;
    index += 1;
  }
  return `${value.toFixed(1)} ${units[index]}`;
}
