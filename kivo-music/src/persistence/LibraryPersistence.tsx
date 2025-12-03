// src/persistence/LibraryPersistence.tsx
import {
  readTextFile,
  writeTextFile,
  exists,
  mkdir,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";
import {
  CURRENT_LIBRARY_SCHEMA_VERSION,
  type LibraryFile,
  type LibraryFileV1,
  type MusicTrack,
} from "../types";

const LIB_DIR = "library";
const LIB_FILE = `${LIB_DIR}/kivo-library.json`;

/**
 * 确保 AppData 下的 library 目录存在：
 * C:\Users\Administrator\AppData\Roaming\com.administrator.kivo-music\library
 */
async function ensureLibraryDir() {
  try {
    const hasDir = await exists(LIB_DIR, {
      baseDir: BaseDirectory.AppData,
    });

    if (!hasDir) {
      await mkdir(LIB_DIR, {
        baseDir: BaseDirectory.AppData,
        recursive: true,
      });
      console.log(
        "[LibraryPersistence] created library dir:",
        LIB_DIR,
      );
    }
  } catch (error) {
    console.error("[LibraryPersistence] ensureLibraryDir error:", error);
  }
}

/**
 * 根据任意对象「归一化」成 MusicTrack。
 * 兼容旧字段：
 * - filePath / path
 * - title / name
 */
function normalizeTrack(raw: any, index: number): MusicTrack {
  const obj = raw && typeof raw === "object" ? raw : {};

  const rawPath: string =
    obj.filePath || obj.path || obj.fullPath || obj.sourcePath || "";

  // 如果真的连路径都没有，就生成一个临时 ID，避免崩溃
  const safePath = typeof rawPath === "string" ? rawPath : "";

  const id: string =
    typeof obj.id === "string" && obj.id.length > 0
      ? obj.id
      : safePath || `track-${index}`;

  const filePath = safePath;
  const path = safePath;

  const fallbackTitle = (() => {
    if (!filePath) return "未命名歌曲";
    const parts = filePath.split(/[/\\]/);
    const file = parts[parts.length - 1] || filePath;
    const dotIdx = file.lastIndexOf(".");
    return dotIdx > 0 ? file.slice(0, dotIdx) : file;
  })();

  const title: string =
    typeof obj.title === "string" && obj.title.length > 0
      ? obj.title
      : typeof obj.name === "string" && obj.name.length > 0
      ? obj.name
      : fallbackTitle;

  const artist: string =
    typeof obj.artist === "string" && obj.artist.length > 0
      ? obj.artist
      : "未知艺人";

  const album: string =
    typeof obj.album === "string" && obj.album.length > 0
      ? obj.album
      : "未知专辑";

  const duration =
    typeof obj.duration === "number" && Number.isFinite(obj.duration)
      ? obj.duration
      : 0;

  const coverPath =
    typeof obj.coverPath === "string" && obj.coverPath.length > 0
      ? obj.coverPath
      : null;

  const addedAt: string | undefined =
    typeof obj.addedAt === "string" && obj.addedAt.length > 0
      ? obj.addedAt
      : undefined;

  const playCount: number | undefined =
    typeof obj.playCount === "number" && Number.isFinite(obj.playCount)
      ? obj.playCount
      : undefined;

  const lastPlayedAt: string | null | undefined =
    typeof obj.lastPlayedAt === "string"
      ? obj.lastPlayedAt
      : obj.lastPlayedAt === null
      ? null
      : undefined;

  return {
    // 先保留原始字段，避免未来扩展丢数据
    ...obj,

    // 再用规范字段覆盖一遍
    id,
    filePath,
    path,
    title,
    artist,
    album,
    duration,
    coverPath,
    addedAt,
    playCount,
    lastPlayedAt,
  };
}

/**
 * 从任意 JSON 结构中提取 track 数组：
 * - 兼容老版本：直接是数组 []
 * - 新版本：{ schemaVersion, tracks: [] }
 */
function extractTracksFromRaw(raw: any): MusicTrack[] {
  if (!raw) return [];

  // v1+ 格式：{ schemaVersion, tracks }
  if (!Array.isArray(raw) && Array.isArray((raw as LibraryFile).tracks)) {
    const file = raw as LibraryFileV1;
    const tracks = file.tracks || [];
    return tracks.map((t, idx) => normalizeTrack(t, idx));
  }

  // 老格式：直接一个数组
  if (Array.isArray(raw)) {
    return raw.map((t, idx) => normalizeTrack(t, idx));
  }

  // 其它奇怪格式：尽量宽容一点
  if (
    typeof raw === "object" &&
    Array.isArray((raw as any).libraryTracks)
  ) {
    const tracks = (raw as any).libraryTracks as any[];
    return tracks.map((t, idx) => normalizeTrack(t, idx));
  }

  console.warn(
    "[LibraryPersistence] unknown library JSON shape, fallback to empty list",
  );
  return [];
}

/**
 * 读取资料库 JSON，自动做 schema 兼容和归一化。
 */
export async function loadLibrary(): Promise<MusicTrack[]> {
  try {
    await ensureLibraryDir();

    const hasFile = await exists(LIB_FILE, {
      baseDir: BaseDirectory.AppData,
    });

    if (!hasFile) {
      console.log("[LibraryPersistence] no library file yet");
      return [];
    }

    const text = await readTextFile(LIB_FILE, {
      baseDir: BaseDirectory.AppData,
    });

    if (!text || !text.trim()) {
      console.warn("[LibraryPersistence] library file is empty");
      return [];
    }

    let parsed: any;
    try {
      parsed = JSON.parse(text);
    } catch (error) {
      console.error(
        "[LibraryPersistence] JSON parse error, will ignore file:",
        error,
      );
      return [];
    }

    const tracks = extractTracksFromRaw(parsed);
    console.log(
      "[LibraryPersistence] loaded library tracks:",
      tracks.length,
    );
    return tracks;
  } catch (error) {
    console.error("[LibraryPersistence] loadLibrary error:", error);
    return [];
  }
}

/**
 * 写入资料库 JSON：
 * - 永远写成 { schemaVersion: 1, tracks: [...] } 格式
 * - 写入前会先做一次 normalize，保证结构一致
 */
export async function saveLibrary(tracks: MusicTrack[]): Promise<void> {
  try {
    await ensureLibraryDir();

    const normalized = (tracks || []).map((t, idx) =>
      normalizeTrack(t, idx),
    );

    const payload: LibraryFile = {
      schemaVersion: CURRENT_LIBRARY_SCHEMA_VERSION,
      tracks: normalized,
    };

    const json = JSON.stringify(payload, null, 2);

    await writeTextFile(LIB_FILE, json, {
      baseDir: BaseDirectory.AppData,
    });

    console.log(
      "[LibraryPersistence] saveLibrary ok, tracks:",
      normalized.length,
    );
  } catch (error) {
    console.error("[LibraryPersistence] saveLibrary error:", error);
  }
}
