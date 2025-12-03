import {
  BaseDirectory,
  exists,
  mkdir,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/plugin-fs";
import type { PlayerTrack } from "../store/player";

// 数据文件放在：$APPDATA/com.administrator.kivo-music/library/kivo-library.json
const LIBRARY_DIR = "library";
const LIBRARY_FILE_NAME = "kivo-library.json";
const LIBRARY_PATH = `${LIBRARY_DIR}/${LIBRARY_FILE_NAME}`;

/**
 * 确保 $APPDATA/com.administrator.kivo-music/library 目录存在
 */
async function ensureLibraryDirExists() {
  try {
    const hasDir = await exists(LIBRARY_DIR, {
      baseDir: BaseDirectory.AppData,
    });
    if (!hasDir) {
      await mkdir(LIBRARY_DIR, {
        baseDir: BaseDirectory.AppData,
        recursive: true,
      });
      console.info("[LibraryPersistence] created library dir");
    }
  } catch (error) {
    console.error(
      "[LibraryPersistence] ensureLibraryDirExists failed:",
      error,
    );
  }
}

/**
 * 从磁盘读取本地资料库
 */
export async function loadLibrary(): Promise<PlayerTrack[]> {
  try {
    const hasFile = await exists(LIBRARY_PATH, {
      baseDir: BaseDirectory.AppData,
    });

    if (!hasFile) {
      console.info("[LibraryPersistence] no library file yet");
      return [];
    }

    const contents = await readTextFile(LIBRARY_PATH, {
      baseDir: BaseDirectory.AppData,
    });

    const raw = JSON.parse(contents);
    if (!Array.isArray(raw)) {
      console.warn("[LibraryPersistence] library file is not an array");
      return [];
    }

    const tracks: PlayerTrack[] = raw
      .map((item: any, index: number): PlayerTrack | null => {
        if (!item || typeof item.filePath !== "string") return null;

        const track: PlayerTrack = {
          id:
            typeof item.id === "string"
              ? item.id
              : `lib-${index}-${item.filePath}`,
          title:
            typeof item.title === "string" && item.title.trim()
              ? item.title
              : "未知歌曲",
          artist:
            typeof item.artist === "string" && item.artist.trim()
              ? item.artist
              : "未知艺人",
          album:
            typeof item.album === "string" && item.album.trim()
              ? item.album
              : undefined,
          filePath: item.filePath,
          duration:
            typeof item.duration === "number" && Number.isFinite(item.duration)
              ? item.duration
              : undefined,

          // 新增：封面字段（没有就留空）
          coverId:
            typeof item.coverId === "string" && item.coverId.trim()
              ? item.coverId
              : undefined,
          coverPath:
            typeof item.coverPath === "string" && item.coverPath.trim()
              ? item.coverPath
              : undefined,
        };

        return track;
      })
      .filter((t: PlayerTrack | null): t is PlayerTrack => t !== null);

    console.info(
      "[LibraryPersistence] loaded library tracks:",
      tracks.length,
    );
    return tracks;
  } catch (error) {
    console.error("[LibraryPersistence] loadLibrary failed:", error);
    return [];
  }
}

/**
 * 把当前资料库保存到磁盘
 */
export async function saveLibrary(tracks: PlayerTrack[]): Promise<void> {
  try {
    // 先保证目录存在（不存在就创建）
    await ensureLibraryDirExists();

    const payload = JSON.stringify(tracks, null, 2);

    await writeTextFile(LIBRARY_PATH, payload, {
      baseDir: BaseDirectory.AppData,
    });

    console.info(
      "[LibraryPersistence] saveLibrary ok, tracks:",
      tracks.length,
    );
  } catch (error) {
    console.error("[LibraryPersistence] saveLibrary failed:", error);
  }
}

export default {
  loadLibrary,
  saveLibrary,
};
