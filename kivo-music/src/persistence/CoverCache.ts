// src/persistence/CoverCache.ts
//
// 管理封面缓存：
// - 封面仓库目录（默认 AppData/covers，以后可以在设置里改）
// - covers.json 索引：trackId -> { sourcePath, cachedPath, updatedAt }
// - 负责在用户选择封面时，把图片复制到封面仓库目录
//
// 现在额外提供 getCachedCoverPath，播放时可以自动把封面补回来。

import { join } from "@tauri-apps/api/path";
import {
  exists,
  readTextFile,
  writeTextFile,
  mkdir,
  copyFile,
} from "@tauri-apps/plugin-fs";
import type { PlayerTrack } from "../store/player";
import { getEffectiveCoverCacheDir } from "./SettingsPersistence";

export interface CoverIndexEntry {
  trackId: string;
  /** 用户当时选择的原始图片路径 */
  sourcePath: string;
  /** 实际用于显示的缓存路径（封面仓库里的路径） */
  cachedPath: string;
  updatedAt: string;
}

export type CoverIndex = Record<string, CoverIndexEntry>;

const COVERS_INDEX_FILE_NAME = "covers.json";

/** 从路径里粗暴提取扩展名（包含点），如果没有就返回空字符串 */
function getExtensionFromPath(path: string): string {
  const normalized = path.replace(/\\/g, "/");
  const lastSlash = normalized.lastIndexOf("/");
  const lastDot = normalized.lastIndexOf(".");
  if (lastDot <= lastSlash) return "";
  return normalized.slice(lastDot);
}

/** 很简单的字符串 hash，用来给封面文件起一个比较稳定的名字 */
function simpleHash(input: string): string {
  let hash = 0;
  for (let i = 0; i < input.length; i++) {
    hash = (hash * 31 + input.charCodeAt(i)) | 0;
  }
  // 转成无符号再转 16 进制，避免出现负号
  return (hash >>> 0).toString(16);
}

/**
 * 确保封面缓存目录存在：
 * - 默认使用 AppData/covers
 * - 以后如果用户在设置里改成 D 盘之类的目录，这里会自动使用新的目录
 */
async function ensureCoversDir(): Promise<string> {
  const dir = await getEffectiveCoverCacheDir();

  try {
    const hasDir = await exists(dir);
    if (!hasDir) {
      await mkdir(dir);
      console.log("[CoverCache] created covers dir:", dir);
    }
  } catch (err) {
    console.error("[CoverCache] ensureCoversDir error:", err);
  }

  return dir;
}

async function getIndexFilePath(): Promise<string> {
  const dir = await ensureCoversDir();
  return await join(dir, COVERS_INDEX_FILE_NAME);
}

export async function loadCoverIndex(): Promise<CoverIndex> {
  try {
    const indexPath = await getIndexFilePath();
    const hasIndex = await exists(indexPath);
    if (!hasIndex) {
      return {};
    }

    const raw = await readTextFile(indexPath);
    if (!raw.trim()) return {};

    const parsed = JSON.parse(raw) as CoverIndex;
    if (!parsed || typeof parsed !== "object") {
      return {};
    }
    return parsed;
  } catch (err) {
    console.error("[CoverCache] loadCoverIndex error:", err);
    return {};
  }
}

export async function saveCoverIndex(index: CoverIndex): Promise<void> {
  try {
    const indexPath = await getIndexFilePath();
    const json = JSON.stringify(index, null, 2);
    await writeTextFile(indexPath, json);
  } catch (err) {
    console.error("[CoverCache] saveCoverIndex error:", err);
  }
}

/**
 * 播放时根据 track.id 在 covers.json 中找缓存封面路径：
 * - 有记录且文件存在 → 返回 cachedPath
 * - 否则返回 undefined
 */
export async function getCachedCoverPath(
  track: PlayerTrack,
): Promise<string | undefined> {
  if (!track || !track.id) return;

  const index = await loadCoverIndex();
  const entry = index[track.id];
  if (!entry || !entry.cachedPath) {
    return;
  }

  try {
    const ok = await exists(entry.cachedPath);
    if (!ok) {
      return;
    }
  } catch (err) {
    console.error("[CoverCache] exists(cachedPath) error:", err);
    return;
  }

  return entry.cachedPath;
}

/**
 * 为某个 track 记录 / 更新封面：
 *
 * - 会尝试把 sourcePath 对应的图片复制到封面仓库目录；
 * - 然后把 cachedPath 设为仓库里的路径；
 * - 并记录到 covers.json；
 * - 返回 cachedPath（给 track.coverPath 使用）。
 *
 * 如果复制失败，会退回直接使用 sourcePath，这样至少还能显示封面。
 */
export async function setCoverForTrack(
  track: PlayerTrack,
  sourcePath: string,
): Promise<string | undefined> {
  if (!track || !track.id) {
    console.warn("[CoverCache] setCoverForTrack: track has no id", track);
    return;
  }

  const trackId = track.id;
  const index = await loadCoverIndex();
  const prev = index[trackId];

  const coversDir = await ensureCoversDir();

  let cachedPath: string;

  // 如果已经有索引，并且 sourcePath 没变，而且缓存文件还存在，就直接用之前的 cachedPath
  if (
    prev &&
    prev.sourcePath === sourcePath &&
    prev.cachedPath &&
    (await exists(prev.cachedPath))
  ) {
    cachedPath = prev.cachedPath;
  } else {
    // 需要重新复制一份到封面仓库
    const ext = getExtensionFromPath(sourcePath) || ".jpg";
    const fileName = `${simpleHash(
      sourcePath + "|" + Date.now().toString(),
    )}${ext}`;
    const destPath = await join(coversDir, fileName);

    try {
      await copyFile(sourcePath, destPath);
      console.log("[CoverCache] copyFile:", sourcePath, "->", destPath);
      cachedPath = destPath;
    } catch (err) {
      console.error(
        "[CoverCache] copyFile failed, fallback to sourcePath:",
        err,
      );
      // 复制失败就直接使用原路径，至少不至于封面直接消失
      cachedPath = sourcePath;
    }
  }

  index[trackId] = {
    trackId,
    sourcePath,
    cachedPath,
    updatedAt: new Date().toISOString(),
  };

  await saveCoverIndex(index);

  return cachedPath;
}
