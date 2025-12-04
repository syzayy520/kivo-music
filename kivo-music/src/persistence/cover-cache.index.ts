// src/persistence/cover-cache.index.ts
import { exists } from "@tauri-apps/plugin-fs";
import {
  getIndexFilePath,
  readJsonFile,
  writeJsonFile,
} from "./cover-cache.files";

// ==== 类型定义 ====

export interface CoverRecord {
  trackKey: string;
  cachedPath: string;
  // 这些字段是可选的，兼容旧结构 / 预留扩展
  sourcePath?: string;
  trackId?: string | number;
  updatedAt?: string;
}

export type CoverIndex = Record<string, CoverRecord>;

export interface FolderCoverRecord {
  folderPath: string;
  cachedPath: string | null;
  updatedAt?: string;
}

export type FolderCoverIndex = Record<string, FolderCoverRecord>;

// Settings 面板会用到的清理结果类型
export interface BrokenCoverCleanupResult {
  coverChecked: number;
  coverRemoved: number;
  folderChecked: number;
  folderRemoved: number;
}

// ==== 索引读写 ====

const COVERS_INDEX_FILE = "covers.json";
const FOLDER_COVERS_INDEX_FILE = "folder-covers.json";

export async function loadCoverIndex(): Promise<CoverIndex> {
  const path = await getIndexFilePath(COVERS_INDEX_FILE);
  const data = await readJsonFile<CoverIndex | CoverRecord[] | null>(
    path,
    {},
  );

  // 兼容老格式：如果是数组就转成 map
  if (Array.isArray(data)) {
    const map: CoverIndex = {};
    for (const item of data as CoverRecord[]) {
      if (!item || !item.trackKey) continue;
      map[item.trackKey] = item;
    }
    return map;
  }

  // 默认就是 Record<string, CoverRecord> 或空对象
  return data ?? {};
}

export async function saveCoverIndex(index: CoverIndex): Promise<void> {
  const path = await getIndexFilePath(COVERS_INDEX_FILE);
  await writeJsonFile(path, index);
}

export async function loadFolderCoverIndex(): Promise<FolderCoverIndex> {
  const path = await getIndexFilePath(FOLDER_COVERS_INDEX_FILE);
  const data =
    await readJsonFile<FolderCoverIndex | FolderCoverRecord[] | null>(
      path,
      {},
    );

  // 兼容老格式：数组形式
  if (Array.isArray(data)) {
    const map: FolderCoverIndex = {};
    for (const item of data as FolderCoverRecord[]) {
      if (!item || !item.folderPath) continue;
      map[item.folderPath] = item;
    }
    return map;
  }

  return data ?? {};
}

export async function saveFolderCoverIndex(
  index: FolderCoverIndex,
): Promise<void> {
  const path = await getIndexFilePath(FOLDER_COVERS_INDEX_FILE);
  await writeJsonFile(path, index);
}

// ==== 自检 & 清理坏记录 ====

export async function cleanupBrokenCoverEntries(): Promise<BrokenCoverCleanupResult> {
  const coverIndex = await loadCoverIndex();
  const folderIndex = await loadFolderCoverIndex();

  let coverChecked = 0;
  let coverRemoved = 0;
  let folderChecked = 0;
  let folderRemoved = 0;

  // Track 封面索引：删除指向不存在文件的记录
  for (const key of Object.keys(coverIndex)) {
    const rec = coverIndex[key];
    coverChecked += 1;
    if (!rec.cachedPath || !(await exists(rec.cachedPath))) {
      delete coverIndex[key];
      coverRemoved += 1;
    }
  }

  // 文件夹封面索引：把失效的 cachedPath 置为 null（保留记录避免下次重复扫）
  for (const key of Object.keys(folderIndex)) {
    const rec = folderIndex[key];
    folderChecked += 1;
    if (rec.cachedPath && !(await exists(rec.cachedPath))) {
      folderIndex[key] = { ...rec, cachedPath: null };
      folderRemoved += 1;
    }
  }

  await saveCoverIndex(coverIndex);
  await saveFolderCoverIndex(folderIndex);

  return {
    coverChecked,
    coverRemoved,
    folderChecked,
    folderRemoved,
  };
}
