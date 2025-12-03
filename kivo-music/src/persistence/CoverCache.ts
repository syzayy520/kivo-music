// src/persistence/CoverCache.ts
//
// 封面缓存 v2（修正版路径）：
// - 把用户选择的图片复制到 $APPDATA/com.administrator.kivo-music/covers 目录
// - 用 covers.json 维护一个简单索引
// - 返回带有 coverPath（指向缓存文件绝对路径）的 track

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
  cachedRelativePath: string; // 相对 AppData 根目录（com.administrator.kivo-music）的路径
  cachedAbsolutePath: string; // 绝对路径，用于 convertFileSrc
  updatedAt: string;
}

interface CoverStore {
  version: number;
  covers: CoverRecord[];
}

// 这里直接用相对 AppData 根目录的路径：
// AppData 实际是 C:\Users\Administrator\AppData\Roaming\com.administrator.kivo-music
const COVERS_DIR = "covers";
const COVERS_INDEX_FILE = "covers/covers.json";

async function getAppDataRoot(): Promise<string> {
  // 例如：C:\Users\Administrator\AppData\Roaming\com.administrator.kivo-music\
  const root = await appDataDir();
  return root;
}

// 把相对 AppData 的路径拼成绝对路径
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
      return { version: 1, covers: [] };
    }

    const json = await readTextFile(COVERS_INDEX_FILE, {
      baseDir: BaseDirectory.AppData,
    });
    const parsed = JSON.parse(json) as any;

    if (!parsed || typeof parsed !== "object") {
      return { version: 1, covers: [] };
    }

    const version =
      typeof parsed.version === "number" && parsed.version > 0
        ? parsed.version
        : 1;
    const rawCovers = Array.isArray(parsed.covers) ? parsed.covers : [];

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

    return { version, covers };
  } catch (error) {
    console.warn("[CoverCache] loadStore failed, use empty store:", error);
    return { version: 1, covers: [] };
  }
}

async function saveStore(store: CoverStore): Promise<void> {
  try {
    await ensureCoversDir();
    const payload = JSON.stringify(store, null, 2);
    await writeTextFile(COVERS_INDEX_FILE, payload, {
      baseDir: BaseDirectory.AppData,
    });
    console.info(
      "[CoverCache] saved covers index, items:",
      store.covers.length,
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
 * 主函数：
 * - 把 imagePath 指向的原图复制到 AppData/com.administrator.kivo-music/covers
 * - 更新 / 写入 covers.json
 * - 返回一个新的 track（只改 coverPath 字段，指向缓存文件的绝对路径）
 *
 * 如果复制失败，会 fallback：直接把 coverPath 设置为原图路径（行为和旧版一致）。
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
