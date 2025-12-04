// src/persistence/cover-cache.api.ts
import {
  copyFile,
  exists,
  readDir,
  remove,
  size,
} from "@tauri-apps/plugin-fs";
import { basename, dirname, extname, join } from "@tauri-apps/api/path";
import { loadSettings } from "./SettingsPersistence";
import {
  KivoTrackLike,
  FolderCoverRecord,
  CoverCacheStats,
  BrokenCoverCleanupResult,
  FOLDER_COVER_CANDIDATES,
  hashString,
  formatBytes,
  getTrackKey,
  getTrackFilePath,
} from "./cover-cache.index";
import {
  ensureDirExists,
  getCoverCacheDir as getCoverCacheDirInternal,
  loadCoverIndex,
  saveCoverIndex,
  loadFolderCoverIndex,
  saveFolderCoverIndex,
} from "./cover-cache.files";

/**
 * 预热封面缓存系统：
 * - 确保封面缓存目录存在；
 * - 预读 covers.json / folder-covers.json。
 */
export async function ensureCoverCacheReady(): Promise<void> {
  const cacheDir = await getCoverCacheDirInternal();
  await ensureDirExists(cacheDir);
  await Promise.all([
    loadCoverIndex(cacheDir),
    loadFolderCoverIndex(cacheDir),
  ]);
}

/**
 * 对外暴露的封面缓存目录
 */
export async function getCoverCacheDir(): Promise<string> {
  return getCoverCacheDirInternal();
}

/**
 * 把一张图片复制到封面缓存目录，并在 covers.json 中记录
 * 返回缓存后的文件绝对路径
 */
export async function setCoverForTrack(
  track: KivoTrackLike,
  sourcePath: string,
): Promise<string> {
  const cacheDir = await getCoverCacheDirInternal();
  await ensureDirExists(cacheDir);

  const ext = await extname(sourcePath);
  const trackKey = getTrackKey(track);
  const baseName = (await basename(sourcePath)) || "cover";
  const hash = hashString(`${trackKey}:${sourcePath}:${Date.now()}`);
  const fileName = `${hash}-${baseName}${ext || ".jpg"}`;
  const destPath = await join(cacheDir, fileName);

  try {
    await copyFile(sourcePath, destPath);
  } catch (error) {
    console.error("[CoverCache] copyFile 失败:", { sourcePath, destPath }, error);
    throw error;
  }

  const index = await loadCoverIndex(cacheDir);
  index[trackKey] = {
    trackId: trackKey,
    sourcePath,
    cachedPath: destPath,
    updatedAt: new Date().toISOString(),
  };
  await saveCoverIndex(index, cacheDir);

  return destPath;
}

/**
 * 仅按 trackId 查询封面缓存，不做文件夹扫描
 */
export async function getCachedCoverPath(
  track: KivoTrackLike,
): Promise<string | null> {
  const cacheDir = await getCoverCacheDirInternal();
  const index = await loadCoverIndex(cacheDir);
  const trackKey = getTrackKey(track);
  const record = index[trackKey];

  if (!record || !record.cachedPath) {
    return null;
  }

  try {
    if (await exists(record.cachedPath)) {
      return record.cachedPath;
    }

    // 文件不存在了，顺便清理索引
    delete index[trackKey];
    await saveCoverIndex(index, cacheDir);
  } catch (error) {
    console.error("[CoverCache] 校验 cachedPath 失败:", record.cachedPath, error);
  }

  return null;
}

/**
 * 在文件夹内扫描 cover.jpg / folder.jpg 等，
 * 结果会缓存在 folder-covers.json 中（包括“没有封面”的情况）
 */
export async function getFolderCoverForTrack(
  track: KivoTrackLike,
): Promise<string | null> {
  const filePath = getTrackFilePath(track);
  if (!filePath) return null;

  const folderPath = await dirname(filePath);
  const cacheDir = await getCoverCacheDirInternal();
  const folderIndex = await loadFolderCoverIndex(cacheDir);
  const existing = folderIndex[folderPath];

  // 如果已经知道“没有封面”，直接返回 null，避免反复扫描
  if (existing && existing.hasCover === false) {
    return null;
  }

  // 如果之前已经扫描过并且文件还在，直接返回
  if (existing && existing.cachedPath) {
    try {
      if (await exists(existing.cachedPath)) {
        return existing.cachedPath;
      }
    } catch (error) {
      console.warn(
        "[CoverCache] 文件夹封面缓存失效，准备重新扫描:",
        existing.cachedPath,
        error,
      );
    }
  }

  // 重新扫描文件夹
  let candidateSource: string | null = null;

  try {
    const entries = await readDir(folderPath);
    for (const entry of entries) {
      if (!entry.isFile) continue;
      const nameLower = (entry.name ?? "").toLowerCase();
      if (FOLDER_COVER_CANDIDATES.includes(nameLower)) {
        candidateSource = await join(folderPath, entry.name ?? "");
        break;
      }
    }
  } catch (error) {
    console.error("[CoverCache] readDir 扫描文件夹封面失败:", folderPath, error);
  }

  const now = new Date().toISOString();
  const folderRecord: FolderCoverRecord = {
    folderPath,
    cachedPath: null,
    hasCover: false,
    updatedAt: now,
  };

  if (!candidateSource) {
    // 这个文件夹没有封面，记一笔，以后就不再重复尝试
    folderIndex[folderPath] = folderRecord;
    await saveFolderCoverIndex(folderIndex, cacheDir);
    return null;
  }

  // 找到了候选封面，复制到缓存目录
  const baseName = await basename(candidateSource);
  const ext = await extname(candidateSource);
  const hash = hashString(folderPath);
  const destFileName = `${hash}-${baseName}${ext || ".jpg"}`;
  const destPath = await join(cacheDir, destFileName);

  try {
    if (!(await exists(destPath))) {
      await copyFile(candidateSource, destPath);
    }
  } catch (error) {
    console.error(
      "[CoverCache] 复制文件夹封面失败:",
      { candidateSource, destPath },
      error,
    );
    // 扫描到了但复制失败，不缓存，防止死循环
    folderIndex[folderPath] = folderRecord;
    await saveFolderCoverIndex(folderIndex, cacheDir);
    return null;
  }

  // 更新文件夹索引
  folderRecord.cachedPath = destPath;
  folderRecord.hasCover = true;
  folderRecord.updatedAt = now;
  folderIndex[folderPath] = folderRecord;
  await saveFolderCoverIndex(folderIndex, cacheDir);

  // 给当前 track 也建一条 Track 索引，统一走 covers.json
  const trackKey = getTrackKey(track);
  const coverIndex = await loadCoverIndex(cacheDir);
  coverIndex[trackKey] = {
    trackId: trackKey,
    sourcePath: candidateSource,
    cachedPath: destPath,
    updatedAt: now,
  };
  await saveCoverIndex(coverIndex, cacheDir);

  return destPath;
}

/**
 * 统一的封面解析函数：
 * 1. 先用 trackId 在 covers.json 里找；
 * 2. 再看设置里是否允许“文件夹自动封面”，如果允许再按文件夹扫描；
 */
export async function resolveCoverPathForTrack(
  track: KivoTrackLike,
): Promise<string | null> {
  // 1. 先看 track 级缓存
  const byTrack = await getCachedCoverPath(track);
  if (byTrack) return byTrack;

  // 2. 再根据设置决定要不要用文件夹自动封面
  let enableFolderAutoCover = true;
  try {
    const settings = await loadSettings();
    // 老版本 settings 里可能没有 enableFolderAutoCover 字段，这里兜底 true
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    enableFolderAutoCover = (settings as any).enableFolderAutoCover ?? true;
  } catch (error) {
    console.error(
      "[CoverCache] 读取设置失败，默认启用文件夹自动封面:",
      error,
    );
  }

  if (!enableFolderAutoCover) {
    return null;
  }

  const byFolder = await getFolderCoverForTrack(track);
  if (byFolder) return byFolder;

  return null;
}

/**
 * 统计当前封面缓存的信息（文件数量 / 大小 / 索引条数）
 */
export async function getCoverCacheStats(): Promise<CoverCacheStats> {
  const cacheDir = await getCoverCacheDirInternal();
  await ensureDirExists(cacheDir);

  let fileCount = 0;
  let totalBytes = 0;

  try {
    const entries = await readDir(cacheDir);
    for (const entry of entries) {
      if (!entry.isFile) continue;
      const fullPath = await join(cacheDir, entry.name ?? "");
      try {
        const info = await size(fullPath);
        const bytes =
          typeof info === "number" ? info : (info as any).size ?? 0;
        totalBytes += bytes;
        fileCount += 1;
      } catch (error) {
        console.error("[CoverCache] 统计文件大小失败:", fullPath, error);
      }
    }
  } catch (error) {
    console.error("[CoverCache] 遍历缓存目录失败:", cacheDir, error);
  }

  const coverIndex = await loadCoverIndex(cacheDir);
  const folderIndex = await loadFolderCoverIndex(cacheDir);

  return {
    cacheDir,
    fileCount,
    totalBytes,
    humanReadableSize: formatBytes(totalBytes),
    trackEntries: Object.keys(coverIndex).length,
    folderEntries: Object.keys(folderIndex).length,
  };
}

/**
 * 清空当前封面缓存目录（仅磁盘文件 + 两个索引文件）
 * 不会动 kivo-library.json
 */
export async function clearCoverCache(): Promise<void> {
  const cacheDir = await getCoverCacheDirInternal();
  await ensureDirExists(cacheDir);

  try {
    const entries = await readDir(cacheDir);
    for (const entry of entries) {
      if (!entry.isFile) continue;
      const fullPath = await join(cacheDir, entry.name ?? "");
      try {
        await remove(fullPath);
      } catch (error) {
        console.error("[CoverCache] 删除缓存文件失败:", fullPath, error);
      }
    }
  } catch (error) {
    console.error("[CoverCache] 清空缓存目录时 readDir 失败:", cacheDir, error);
  }

  await saveCoverIndex({}, cacheDir);
  await saveFolderCoverIndex({}, cacheDir);
}

/**
 * 清理“损坏的封面索引”
 */
export async function cleanupBrokenCoverEntries(): Promise<BrokenCoverCleanupResult> {
  const cacheDir = await getCoverCacheDirInternal();
  await ensureDirExists(cacheDir);

  const coverIndex = await loadCoverIndex(cacheDir);
  const folderIndex = await loadFolderCoverIndex(cacheDir);

  let coverChecked = 0;
  let coverRemoved = 0;
  let folderChecked = 0;
  let folderRemoved = 0;

  // Track 封面索引
  for (const key of Object.keys(coverIndex)) {
    const rec = coverIndex[key];
    coverChecked += 1;

    try {
      if (!(await exists(rec.cachedPath))) {
        delete coverIndex[key];
        coverRemoved += 1;
      }
    } catch (error) {
      console.warn(
        "[CoverCache] 检查封面文件失败，将移除索引:",
        rec.cachedPath,
        error,
      );
      delete coverIndex[key];
      coverRemoved += 1;
    }
  }

  // 文件夹封面索引
  for (const key of Object.keys(folderIndex)) {
    const rec = folderIndex[key];
    folderChecked += 1;

    if (!rec.cachedPath) {
      // “无封面”的标记留着即可
      continue;
    }

    try {
      if (!(await exists(rec.cachedPath))) {
        delete folderIndex[key];
        folderRemoved += 1;
      }
    } catch (error) {
      console.warn(
        "[CoverCache] 检查文件夹封面失败，将移除索引:",
        rec.cachedPath,
        error,
      );
      delete folderIndex[key];
      folderRemoved += 1;
    }
  }

  await saveCoverIndex(coverIndex, cacheDir);
  await saveFolderCoverIndex(folderIndex, cacheDir);

  return {
    coverChecked,
    coverRemoved,
    folderChecked,
    folderRemoved,
  };
}

/**
 * 把封面缓存从 oldDir 迁移到 newDir：
 * - 复制旧目录下所有文件到新目录
 * - 更新 covers.json / folder-covers.json 里的 cachedPath
 * - 尝试清理旧目录下的封面文件
 */
export async function migrateCoverCache(
  oldDir: string,
  newDir: string,
): Promise<void> {
  if (!oldDir || !newDir || oldDir === newDir) return;

  await ensureDirExists(newDir);

  const coverIndex = await loadCoverIndex(oldDir);
  const folderIndex = await loadFolderCoverIndex(oldDir);

  // 复制所有文件
  try {
    const entries = await readDir(oldDir);
    for (const entry of entries) {
      if (!entry.isFile) continue;
      const srcPath = await join(oldDir, entry.name ?? "");
      const destPath = await join(newDir, entry.name ?? "");
      try {
        if (!(await exists(destPath))) {
          await copyFile(srcPath, destPath);
        }
      } catch (error) {
        console.error(
          "[CoverCache] 迁移封面文件失败:",
          { srcPath, destPath },
          error,
        );
      }
    }
  } catch (error) {
    console.error("[CoverCache] 迁移封面时 readDir 失败:", oldDir, error);
  }

  const replaceDir = (p: string | null): string | null => {
    if (!p) return p;
    if (p.startsWith(oldDir)) {
      return newDir + p.slice(oldDir.length);
    }
    return p;
  };

  // 更新索引里的 cachedPath
  for (const key of Object.keys(coverIndex)) {
    const rec = coverIndex[key];
    rec.cachedPath = replaceDir(rec.cachedPath) ?? rec.cachedPath;
  }
  for (const key of Object.keys(folderIndex)) {
    const rec = folderIndex[key];
    rec.cachedPath = replaceDir(rec.cachedPath) ?? rec.cachedPath;
  }

  // 将新的索引写到新目录下
  await saveCoverIndex(coverIndex, newDir);
  await saveFolderCoverIndex(folderIndex, newDir);

  // 尝试清理旧目录中的文件（不强求全部成功）
  try {
    const entries = await readDir(oldDir);
    for (const entry of entries) {
      if (!entry.isFile) continue;
      const fullPath = await join(oldDir, entry.name ?? "");
      try {
        await remove(fullPath);
      } catch (error) {
        console.warn("[CoverCache] 删除旧缓存文件失败:", fullPath, error);
      }
    }
  } catch (error) {
    console.error("[CoverCache] 清理旧封面目录失败:", oldDir, error);
  }
}
