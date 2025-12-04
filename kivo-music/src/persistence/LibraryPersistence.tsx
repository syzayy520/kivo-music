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
import { log } from "../utils/log";

const LIB_DIR = "library";
const LIB_FILE = `${LIB_DIR}/kivo-library.json`;
const LIB_BACKUP_FILE = `${LIB_DIR}/kivo-library.json.bak`;

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
      log.debug("LibraryPersistence", "created library dir", { dir: LIB_DIR });
    }
  } catch (error) {
    log.error("LibraryPersistence", "ensureLibraryDir error", { error });
  }
}

/**
 * 把任意对象「归一化」成 MusicTrack。
 * 兼容旧字段：
 * - filePath / path / fullPath / sourcePath
 * - title / name
 */
function normalizeTrack(raw: any, index: number): MusicTrack {
  const obj = raw && typeof raw === "object" ? raw : {};

  const rawPath: string =
    (obj.filePath as string) ||
    (obj.path as string) ||
    (obj.fullPath as string) ||
    (obj.sourcePath as string) ||
    "";

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

  log.warn(
    "LibraryPersistence",
    "unknown library JSON shape, fallback to empty list",
    { rawType: typeof raw },
  );
  return [];
}

/**
 * 构造一个“空库”文件的 payload
 */
function createEmptyLibraryPayload(): LibraryFile {
  return {
    schemaVersion: CURRENT_LIBRARY_SCHEMA_VERSION,
    tracks: [],
  };
}

/**
 * 内部工具：读取库文件原始文本。
 * - 不存在 / 为空时返回 null
 */
async function readLibraryTextSafely(): Promise<string | null> {
  await ensureLibraryDir();

  const hasFile = await exists(LIB_FILE, {
    baseDir: BaseDirectory.AppData,
  });

  if (!hasFile) {
    log.debug("LibraryPersistence", "no library file yet");
    return null;
  }

  try {
    const text = await readTextFile(LIB_FILE, {
      baseDir: BaseDirectory.AppData,
    });

    if (!text || !text.trim()) {
      log.warn("LibraryPersistence", "library file is empty");
      return null;
    }

    return text;
  } catch (error) {
    log.error("LibraryPersistence", "readTextFile failed", { error });
    return null;
  }
}

/**
 * 如果库文件 JSON 解析失败：
 * - 把原始内容备份到 .bak
 * - 写入一个“空库”文件，防止后续启动一直报错
 */
async function backupAndResetCorruptedLibraryFile(
  rawText: string,
): Promise<void> {
  try {
    await ensureLibraryDir();

    // 1. 备份原始内容
    try {
      await writeTextFile(LIB_BACKUP_FILE, rawText, {
        baseDir: BaseDirectory.AppData,
      });
      log.warn("LibraryPersistence", "backup corrupted library to .bak", {
        backupFile: LIB_BACKUP_FILE,
      });
    } catch (backupError) {
      log.error(
        "LibraryPersistence",
        "failed to write backup library file",
        { backupError },
      );
    }

    // 2. 写入空库
    try {
      const emptyPayload = createEmptyLibraryPayload();
      const emptyJson = JSON.stringify(emptyPayload, null, 2);
      await writeTextFile(LIB_FILE, emptyJson, {
        baseDir: BaseDirectory.AppData,
      });
      log.warn("LibraryPersistence", "reset corrupted library to empty file");
    } catch (resetError) {
      log.error(
        "LibraryPersistence",
        "failed to reset corrupted library file",
        { resetError },
      );
    }
  } catch (error) {
    // 理论上很少到这里，再兜一层
    log.error(
      "LibraryPersistence",
      "backupAndResetCorruptedLibraryFile fatal error",
      { error },
    );
  }
}

/**
 * 读取资料库 JSON，自动做 schema 兼容和归一化。
 */
export async function loadLibrary(): Promise<MusicTrack[]> {
  try {
    const text = await readLibraryTextSafely();
    if (!text) {
      return [];
    }

    let parsed: any;
    try {
      parsed = JSON.parse(text);
    } catch (error) {
      log.error(
        "LibraryPersistence",
        "JSON parse error, will backup & reset file",
        { error },
      );
      await backupAndResetCorruptedLibraryFile(text);
      return [];
    }

    const tracks = extractTracksFromRaw(parsed);
    log.debug("LibraryPersistence", "loaded library tracks", {
      count: tracks.length,
    });
    return tracks;
  } catch (error) {
    log.error("LibraryPersistence", "loadLibrary error", { error });
    return [];
  }
}

/**
 * 写入资料库 JSON：
 * - 永远写成 { schemaVersion, tracks: [...] } 格式
 * - 写入前会先做一次 normalize，保证结构一致
 * - 如果已有旧文件，会先备份一份 .bak（最佳努力，不影响主流程）
 */
export async function saveLibrary(tracks: MusicTrack[]): Promise<void> {
  try {
    await ensureLibraryDir();

    // 尝试备份旧文件（最佳努力，不影响后续写入）
    try {
      const existingText = await readTextFile(LIB_FILE, {
        baseDir: BaseDirectory.AppData,
      });
      if (existingText && existingText.trim()) {
        await writeTextFile(LIB_BACKUP_FILE, existingText, {
          baseDir: BaseDirectory.AppData,
        });
        log.debug("LibraryPersistence", "backup existing library before save", {
          backupFile: LIB_BACKUP_FILE,
        });
      }
    } catch {
      // 旧文件不存在或读取失败都可以忽略，不影响正常保存
    }

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

    log.debug("LibraryPersistence", "saveLibrary ok", {
      count: normalized.length,
    });
  } catch (error) {
    log.error("LibraryPersistence", "saveLibrary error", { error });
  }
}
