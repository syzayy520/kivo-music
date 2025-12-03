// src/persistence/CoverCache.ts
//
// 封面缓存 v2 + 目录索引（B2.2）
// - covers:  track 级别封面缓存（用户手动选封面 → 复制到 AppData/covers）
// - folders: 目录级索引，目前主要用来记录「这个文件夹没有封面」，下次启动不再去试探 cover.jpg

import {
  BaseDirectory,
  exists,
  mkdir,
  readTextFile,
  writeTextFile,
  copyFile,
} from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";

// 播放队列里的 track 只要大致符合这个结构即可
export interface CoverTrackLike {
  id?: string | number;
  filePath?: string;
  path?: string;
  coverPath?: string | null;
  // 其余字段这里不关心
}

interface CoverRecord {
  trackKey: string;
  sourcePath: string;
  cachedRelativePath: string; // 相对 AppData 根目录的路径（例如 "covers/xxxx.jpg"）
  cachedAbsolutePath: string; // 绝对路径，用于 convertFileSrc
  updatedAt: string;
}

interface FolderRecord {
  folderPath: string; // 目录绝对路径
  hasCover: boolean; // 当前我们只用到 hasCover === false 的情况
  sourcePath?: string;
  cachedRelativePath?: string;
  cachedAbsolutePath?: string;
  updatedAt: string;
}

interface CoverStore {
  version: number;
  covers: CoverRecord[];
  folders: FolderRecord[];
}

const STORE_VERSION = 1;

// AppData 实际路径例如：
// C:\Users\Administrator\AppData\Roaming\com.administrator.kivo-music
// 我们在其下放：covers\*.jpg 和 covers\covers.json
const COVERS_DIR = "covers";
const COVERS_INDEX_FILE = "covers/covers.json";

async function getAppDataRoot(): Promise<string> {
  const root = await appDataDir();
  return root;
}

async function toAbsolutePath(relativePath: string): Promise<string> {
  const root = await getAppDataRoot();
  return await join(root, relativePath);
}

async function ensureCoversDir(): Promise<void> {
  const ok = await exists(COVERS_DIR, {
    baseDir: BaseDirectory.AppData,
  }).catch(() => false);

  if (!ok) {
    await mkdir(COVERS_DIR, {
      baseDir: BaseDirectory.AppData,
      recursive: true,
    });
  }
}

async function loadStore(): Promise<CoverStore> {
  try {
    const ok = await exists(COVERS_INDEX_FILE, {
      baseDir: BaseDirectory.AppData,
    }).catch(() => false);

    if (!ok) {
      return {
        version: STORE_VERSION,
        covers: [],
        folders: [],
      };
    }

    const json = await readTextFile(COVERS_INDEX_FILE, {
      baseDir: BaseDirectory.AppData,
    });
    const parsed = JSON.parse(json) as any;

    if (!parsed || typeof parsed !== "object") {
      return {
        version: STORE_VERSION,
        covers: [],
        folders: [],
      };
    }

    const version =
      typeof parsed.version === "number" && parsed.version > 0
        ? parsed.version
        : STORE_VERSION;
    const rawCovers = Array.isArray(parsed.covers) ? parsed.covers : [];
    const rawFolders = Array.isArray(parsed.folders) ? parsed.folders : [];

    const covers: CoverRecord[] = rawCovers
      .filter(
        (c: any) =>
          c &&
          typeof c.trackKey === "string" &&
          typeof c.cachedRelativePath === "string",
      )
      .map((c: any) => ({
        trackKey: String(c.trackKey),
        sourcePath: String(c.sourcePath ?? ""),
        cachedRelativePath: String(c.cachedRelativePath),
        cachedAbsolutePath: String(c.cachedAbsolutePath ?? ""),
        updatedAt: String(c.updatedAt ?? ""),
      }));

    const folders: FolderRecord[] = rawFolders
      .filter((f: any) => f && typeof f.folderPath === "string")
      .map((f: any) => ({
        folderPath: String(f.folderPath),
        hasCover: Boolean(f.hasCover),
        sourcePath:
          typeof f.sourcePath === "string" ? f.sourcePath : undefined,
        cachedRelativePath:
          typeof f.cachedRelativePath === "string"
            ? f.cachedRelativePath
            : undefined,
        cachedAbsolutePath:
          typeof f.cachedAbsolutePath === "string"
            ? f.cachedAbsolutePath
            : undefined,
        updatedAt: String(f.updatedAt ?? ""),
      }));

    return { version, covers, folders };
  } catch (error) {
    console.warn("[CoverCache] loadStore failed, use empty store:", error);
    return {
      version: STORE_VERSION,
      covers: [],
      folders: [],
    };
  }
}

async function saveStore(store: CoverStore): Promise<void> {
  try {
    await ensureCoversDir();
    const normalized: CoverStore = {
      version: store.version || STORE_VERSION,
      covers: store.covers || [],
      folders: store.folders || [],
    };
    const payload = JSON.stringify(normalized, null, 2);
    await writeTextFile(COVERS_INDEX_FILE, payload, {
      baseDir: BaseDirectory.AppData,
    });
    console.info(
      "[CoverCache] saved covers index, items:",
      normalized.covers.length,
      "folders:",
      normalized.folders.length,
    );
  } catch (error) {
    console.error("[CoverCache] saveStore failed:", error);
  }
}

// 生成 track 的唯一 key：优先 id，其次 filePath/path
function getTrackKey(track: CoverTrackLike): string | null {
  if (track.id != null) return String(track.id);
  if (track.filePath) return String(track.filePath);
  if (track.path) return String(track.path);
  return null;
}

// 简单 hash，保证文件名相对短一点
function simpleHash(input: string): string {
  let hash = 0;
  for (let i = 0; i < input.length; i++) {
    hash = (hash * 31 + input.charCodeAt(i)) | 0;
  }
  if (hash < 0) hash = -hash;
  return hash.toString(16);
}

function guessExt(path: string): string {
  const m = path.match(/\.[a-zA-Z0-9]+$/);
  return m ? m[0] : ".jpg";
}

/**
 * track 级封面缓存：
 * - 把 imagePath 指向的原图复制到 AppData/com.administrator.kivo-music/covers
 * - 更新 / 写入 covers.json
 * - 返回一个新的 track（只改 coverPath 字段，指向缓存文件的绝对路径）
 *
 * 如果复制失败，会 fallback：直接把 coverPath 设置为原图路径。
 */
export async function setCoverForTrack<T extends CoverTrackLike>(
  track: T,
  imagePath: string,
): Promise<T> {
  if (!imagePath) return track;

  const trackKey = getTrackKey(track);
  if (!trackKey) {
    console.warn(
      "[CoverCache] track has no id / filePath / path, fallback to raw imagePath.",
    );
    return { ...track, coverPath: imagePath };
  }

  await ensureCoversDir();
  const store = await loadStore();

  const ext = guessExt(imagePath);
  const hash = simpleHash(`${trackKey}:${imagePath}`);
  const fileName = `${hash}${ext}`;
  const cachedRelativePath = `${COVERS_DIR}/${fileName}`;

  try {
    await copyFile(imagePath, cachedRelativePath, {
      toPathBaseDir: BaseDirectory.AppData,
    });
  } catch (error) {
    console.error(
      "[CoverCache] copyFile failed, fallback to raw imagePath:",
      error,
    );
    return { ...track, coverPath: imagePath };
  }

  const cachedAbsolutePath = await toAbsolutePath(cachedRelativePath);

  const record: CoverRecord = {
    trackKey,
    sourcePath: imagePath,
    cachedRelativePath,
    cachedAbsolutePath,
    updatedAt: new Date().toISOString(),
  };

  const existingIndex = store.covers.findIndex(
    (c) => c.trackKey === trackKey,
  );
  if (existingIndex >= 0) {
    store.covers[existingIndex] = record;
  } else {
    store.covers.push(record);
  }

  await saveStore(store);

  return {
    ...track,
    coverPath: cachedAbsolutePath,
  };
}

/**
 * 查询：某个目录是否已经被标记为「没有封面」。
 * 用于启动时跳过对该目录的 cover.jpg 试探请求，避免重复的 500。
 */
export async function isFolderKnownNoCover(
  folderPath: string,
): Promise<boolean> {
  if (!folderPath) return false;
  try {
    const store = await loadStore();
    const rec = store.folders.find(
      (f) => f.folderPath === folderPath && f.hasCover === false,
    );
    return !!rec;
  } catch (error) {
    console.warn("[CoverCache] isFolderKnownNoCover failed:", error);
    return false;
  }
}

/**
 * 标记：某个目录「没有封面」。
 * 典型调用点：前端尝试加载 folder/cover.jpg 失败（onError）时。
 */
export async function markFolderNoCover(
  folderPath: string,
): Promise<void> {
  if (!folderPath) return;

  try {
    await ensureCoversDir();
    const store = await loadStore();
    const normalized = String(folderPath);
    const now = new Date().toISOString();

    const idx = store.folders.findIndex(
      (f) => f.folderPath === normalized,
    );
    if (idx >= 0) {
      store.folders[idx] = {
        ...store.folders[idx],
        folderPath: normalized,
        hasCover: false,
        updatedAt: now,
      };
    } else {
      store.folders.push({
        folderPath: normalized,
        hasCover: false,
        updatedAt: now,
      });
    }

    await saveStore(store);
  } catch (error) {
    console.error("[CoverCache] markFolderNoCover failed:", error);
  }
}
