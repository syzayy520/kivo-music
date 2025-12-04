// src/persistence/cover-cache.files.ts
import { exists, mkdir } from "@tauri-apps/plugin-fs";
import { join } from "@tauri-apps/api/path";
import { getEffectiveCoverCacheDir } from "./SettingsPersistence";
import type { CoverIndex, FolderCoverIndex } from "./cover-cache.index";
import { safeReadJson, safeWriteJson } from "./jsonSafe";
import { log } from "../utils/log";

export const COVERS_INDEX_FILE = "covers.json";
export const FOLDER_COVERS_INDEX_FILE = "folder-covers.json";

/**
 * 确保目录存在
 */
export async function ensureDirExists(dir: string): Promise<void> {
  try {
    if (!(await exists(dir))) {
      await mkdir(dir, { recursive: true });
    }
  } catch (error) {
    log.error("CoverCache", "ensureDirExists 失败", { dir, error });
  }
}

async function getIndexFilePath(
  fileName: string,
  cacheDirOverride?: string,
): Promise<string> {
  const cacheDir = cacheDirOverride ?? (await getEffectiveCoverCacheDir());
  await ensureDirExists(cacheDir);
  return await join(cacheDir, fileName);
}

export async function loadCoverIndex(
  cacheDirOverride?: string,
): Promise<CoverIndex> {
  const indexPath = await getIndexFilePath(
    COVERS_INDEX_FILE,
    cacheDirOverride,
  );
  return await safeReadJson<CoverIndex>(indexPath, {});
}

export async function saveCoverIndex(
  index: CoverIndex,
  cacheDirOverride?: string,
): Promise<void> {
  const indexPath = await getIndexFilePath(
    COVERS_INDEX_FILE,
    cacheDirOverride,
  );
  await safeWriteJson<CoverIndex>(indexPath, index ?? {});
}

export async function loadFolderCoverIndex(
  cacheDirOverride?: string,
): Promise<FolderCoverIndex> {
  const indexPath = await getIndexFilePath(
    FOLDER_COVERS_INDEX_FILE,
    cacheDirOverride,
  );
  return await safeReadJson<FolderCoverIndex>(indexPath, {});
}

export async function saveFolderCoverIndex(
  index: FolderCoverIndex,
  cacheDirOverride?: string,
): Promise<void> {
  const indexPath = await getIndexFilePath(
    FOLDER_COVERS_INDEX_FILE,
    cacheDirOverride,
  );
  await safeWriteJson<FolderCoverIndex>(indexPath, index ?? {});
}

/**
 * 返回当前生效的封面缓存目录（会确保目录存在）
 */
export async function getCoverCacheDir(): Promise<string> {
  const cacheDir = await getEffectiveCoverCacheDir();
  await ensureDirExists(cacheDir);
  return cacheDir;
}
