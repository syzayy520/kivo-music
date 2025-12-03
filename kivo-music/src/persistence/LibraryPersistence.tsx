import {
  BaseDirectory,
  exists,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/plugin-fs";
import type { PlayerTrack } from "../store/player";

const LIBRARY_FILE = "kivo-library.json";

/**
 * 从磁盘读取本地音乐库
 */
export async function loadLibrary(): Promise<PlayerTrack[]> {
  try {
    // ✅ 注意：这里要用 baseDir，而不是老版本的 dir
    const hasFile = await exists(LIBRARY_FILE, {
      baseDir: BaseDirectory.AppData,
    });

    if (!hasFile) {
      console.info("[LibraryPersistence] no library file yet");
      return [];
    }

    const contents = await readTextFile(LIBRARY_FILE, {
      baseDir: BaseDirectory.AppData,
    });

    if (!contents) return [];

    let raw: unknown;
    try {
      raw = JSON.parse(contents);
    } catch (err) {
      console.warn(
        "[LibraryPersistence] JSON parse failed, ignore library file:",
        err,
      );
      return [];
    }

    if (!Array.isArray(raw)) {
      console.warn(
        "[LibraryPersistence] library file is not an array, ignore",
      );
      return [];
    }

    const tracks: PlayerTrack[] = (raw as any[])
      .map((item: any, index: number) => {
        if (!item || typeof item.filePath !== "string") return null;

        const track: PlayerTrack = {
          id:
            typeof item.id === "string"
              ? item.id
              : `legacy-${Date.now()}-${index}`,
          title:
            typeof item.title === "string" && item.title.trim()
              ? item.title
              : "未知标题",
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
        };

        return track;
      })
      .filter((t): t is PlayerTrack => t !== null);

    console.info(
      "[LibraryPersistence] loaded tracks:",
      tracks.length,
    );
    return tracks;
  } catch (err) {
    console.error("[LibraryPersistence] loadLibrary failed:", err);
    return [];
  }
}

/**
 * 把当前音乐库写回磁盘
 */
export async function saveLibrary(tracks: PlayerTrack[]): Promise<void> {
  try {
    const serializable = tracks.map((t) => ({
      id: t.id,
      title: t.title,
      artist: t.artist,
      album: t.album ?? null,
      filePath: t.filePath,
      duration: t.duration ?? null,
    }));

    const json = JSON.stringify(serializable, null, 2);

    await writeTextFile(LIBRARY_FILE, json, {
      baseDir: BaseDirectory.AppData,
    });

    console.info(
      "[LibraryPersistence] saveLibrary ok, tracks:",
      tracks.length,
    );
  } catch (err) {
    console.error("[LibraryPersistence] saveLibrary failed:", err);
  }
}
