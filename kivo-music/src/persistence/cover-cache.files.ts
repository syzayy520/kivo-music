// src/persistence/cover-cache.files.ts
import {
  exists,
  mkdir,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/plugin-fs";
import { join } from "@tauri-apps/api/path";
import { getEffectiveCoverCacheDir } from "./SettingsPersistence";

// 确保封面缓存目录存在，并返回绝对路径
export async function ensureCoverCacheDirExists(): Promise<string> {
  const dir = await getEffectiveCoverCacheDir();
  if (!(await exists(dir))) {
    await mkdir(dir, { recursive: true });
  }
  return dir;
}

// 获取封面缓存目录路径（会顺便确保目录存在）
export async function getCoverCacheDirPath(): Promise<string> {
  return ensureCoverCacheDirExists();
}

// 获取某个索引文件的完整路径（例如 covers.json）
export async function getIndexFilePath(fileName: string): Promise<string> {
  const dir = await ensureCoverCacheDirExists();
  return await join(dir, fileName);
}

// 安全读取 JSON 文件；损坏或不存在时返回 fallback
export async function readJsonFile<T>(
  path: string,
  fallback: T,
): Promise<T> {
  try {
    if (!(await exists(path))) return fallback;
    const raw = await readTextFile(path);
    if (!raw.trim()) return fallback;
    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === "object") {
      return parsed as T;
    }
  } catch (error) {
    console.warn("[CoverCacheFiles] 读取 JSON 失败，将使用默认值:", {
      path,
      error,
    });
  }
  return fallback;
}

// 安全写入 JSON 文件
export async function writeJsonFile<T>(
  path: string,
  data: T,
): Promise<void> {
  try {
    const json = JSON.stringify(data ?? {}, null, 2);
    await writeTextFile(path, json);
  } catch (error) {
    console.error("[CoverCacheFiles] 写入 JSON 失败:", {
      path,
      error,
    });
  }
}
