// src/persistence/CoverCache.ts
import {
  copyFile,
  exists,
  mkdir,
  readDir,
  readTextFile,
  writeTextFile,
  size,
} from "@tauri-apps/plugin-fs";
import { dirname, extname, join } from "@tauri-apps/api/path";
import { getEffectiveCoverCacheDir } from "./SettingsPersistence";

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

const COVERS_INDEX_FILE = "covers.json";
const FOLDER_COVERS_INDEX_FILE = "folder-covers.json";

const FOLDER_COVER_CANDIDATES = [
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

async function getCoverCacheDirInternal(): Promise<string> {
  const dir = await getEffectiveCoverCacheDir();
  if (!(await exists(dir))) {
    await mkdir(dir, { recursive: true });
  }
  return dir;
}

async function getIndexPath(fileName: string): Promise<string> {
  const dir = await getCoverCacheDirInternal();
  return await join(dir, fileName);
}

async function loadJsonFile<T>(path: string, fallback: T): Promise<T> {
  try {
    if (!(await exists(path))) return fallback;
    const raw = await readTextFile(path);
    if (!raw.trim()) return fallback;
    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === "object") {
      return parsed as T;
    }
  } catch (error) {
    console.warn("[CoverCache] 读取 JSON 失败，将使用默认值:", path, error);
  }
  return fallback;
}

async function saveJsonFile<T>(path: string, data: T): Promise<void> {
  try {
    const json = JSON.stringify(data ?? {}, null, 2);
    await writeTextFile(path, json);
  } catch (error) {
    console.error("[CoverCache] 写入 JSON 失败:", path, error);
  }
}

async function loadCoverIndex(): Promise<CoverIndex> {
  const indexPath = await getIndexPath(COVERS_INDEX_FILE);
  return await loadJsonFile<CoverIndex>(indexPath, {});
}

async function saveCoverIndex(index: CoverIndex): Promise<void> {
  const indexPath = await getIndexPath(COVERS_INDEX_FILE);
  await saveJsonFile<CoverIndex>(indexPath, index ?? {});
}

async function loadFolderCoverIndex(): Promise<FolderCoverIndex> {
  const indexPath = await getIndexPath(FOLDER_COVERS_INDEX_FILE);
  return await loadJsonFile<FolderCoverIndex>(indexPath, {});
}

async function saveFolderCoverIndex(index: FolderCoverIndex): Promise<void> {
  const indexPath = await getIndexPath(FOLDER_COVERS_INDEX_FILE);
  await saveJsonFile<FolderCoverIndex>(indexPath, index ?? {});
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
async function ensureFolderCover(folderPath: string): Promise<string | null> {
  const folderIndex = await loadFolderCoverIndex();
  const existing = folderIndex[folderPath];

  if (existing && existing.cachedPath) {
    if (await exists(existing.cachedPath)) {
      return existing.cachedPath;
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

/**
 * 封面缓存目录（会确保存在）
 */
export async function getCoverCacheDir(): Promise<string> {
  return await getCoverCacheDirInternal();
}

/**
 * 确保封面缓存目录和两个索引文件都准备好。
 */
export async function ensureCoverCacheReady(): Promise<void> {
  const dir = await getCoverCacheDirInternal();
  await saveJsonFile(await join(dir, COVERS_INDEX_FILE), {});
  await saveJsonFile(await join(dir, FOLDER_COVERS_INDEX_FILE), {});
}

/**
 * 根据 track 在 covers.json 中查找已缓存的封面。
 */
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
): Promise<string> {
  const cacheDir = await getCoverCacheDirInternal();
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
    throw error;
  }

  index[key] = {
    trackId: key,
    sourcePath,
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
    if (await exists(track.coverPath)) {
      return track.coverPath;
    }
  }
  if (typeof track.cover === "string" && track.cover.length > 0) {
    if (await exists(track.cover)) {
      return track.cover;
    }
  }

  const cached = await getCachedCoverPath(track);
  if (cached) return cached;

  const filePath = getTrackFilePath(track);
  if (filePath) {
    const folder = await dirname(filePath);
    const folderCover = await ensureFolderCover(folder);
    if (folderCover) {
      return folderCover;
    }
  }

  return null;
}

/**
 * 统计封面缓存占用情况。
 */
export async function getCoverCacheStats(): Promise<CoverCacheStats> {
  const dir = await getCoverCacheDirInternal();
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
      const fileSize = await size(p); // plugin-fs 的 size 返回 number
      totalBytes += Number(fileSize || 0);
    } catch {
      // ignore
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

/**
 * 清空封面缓存索引：
 * - 不必删除图片文件本身，只要把 covers.json / folder-covers.json 置空
 */
export async function clearCoverCache(): Promise<void> {
  const dir = await getCoverCacheDirInternal();
  const coversPath = await join(dir, COVERS_INDEX_FILE);
  const folderPath = await join(dir, FOLDER_COVERS_INDEX_FILE);
  await saveJsonFile(coversPath, {});
  await saveJsonFile(folderPath, {});
}

/**
 * 清理所有指向不存在文件的封面索引。
 * 返回总共清理掉多少条记录（track + folder）。
 */
export async function cleanupBrokenCoverEntries(): Promise<number> {
  const coverIndex = await loadCoverIndex();
  const folderIndex = await loadFolderCoverIndex();

  let removed = 0;

  // 清理 track 封面索引
  for (const key of Object.keys(coverIndex)) {
    const rec = coverIndex[key];
    if (!rec.cachedPath) continue;

    try {
      const ok = await exists(rec.cachedPath);
      if (!ok) {
        delete coverIndex[key];
        removed++;
      }
    } catch {
      delete coverIndex[key];
      removed++;
    }
  }

  // 清理文件夹封面索引：文件不存在则把 cachedPath 置空
  for (const key of Object.keys(folderIndex)) {
    const rec = folderIndex[key];
    if (!rec.cachedPath) continue;

    let broken = false;
    try {
      const ok = await exists(rec.cachedPath);
      broken = !ok;
    } catch {
      broken = true;
    }

    if (broken) {
      folderIndex[key] = {
        ...rec,
        cachedPath: null,
        updatedAt: new Date().toISOString(),
      };
      removed++;
    }
  }

  await saveCoverIndex(coverIndex);
  await saveFolderCoverIndex(folderIndex);

  return removed;
}

/**
 * 旧版 → 新版 封面缓存结构迁移入口。
 *
 * 目前新旧结构差异不大，这里做的事情很简单：
 * - 确保缓存目录和索引文件存在
 * 以后如果需要调整目录结构 / 重命名索引文件，可以在这里扩展具体迁移逻辑。
 */
export async function migrateCoverCache(): Promise<void> {
  await ensureCoverCacheReady();
}

function formatBytes(bytes: number): string {
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
