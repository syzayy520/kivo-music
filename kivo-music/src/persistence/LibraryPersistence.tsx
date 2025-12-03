// src/persistence/LibraryPersistence.tsx
import {
  readTextFile,
  writeTextFile,
  exists,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";
import type { PlayerTrack } from "../store/player";

const LIBRARY_FILE = "kivo-library.json";

/**
 * 从磁盘读取资料库
 */
export async function loadLibrary(): Promise<PlayerTrack[]> {
  try {
    const hasFile = await exists(LIBRARY_FILE, {
      baseDir: BaseDirectory.AppData,
    });
    if (!hasFile) {
      return [];
    }

    const contents = await readTextFile(LIBRARY_FILE, {
      baseDir: BaseDirectory.AppData,
    });

    const raw = JSON.parse(contents);
    if (!Array.isArray(raw)) return [];

    const tracks: PlayerTrack[] = raw
      .map((item: any, index: number) => {
        if (!item || typeof item.filePath !== "string") return null;

        const track: PlayerTrack = {
          id: typeof item.id === "string" ? item.id : `lib-${index}`,
          title: typeof item.title === "string" ? item.title : "未知标题",
          artist: typeof item.artist === "string" ? item.artist : "未知艺人",
          album: typeof item.album === "string" ? item.album : undefined,
          filePath: item.filePath,
          duration:
            typeof item.duration === "number" ? item.duration : undefined,
        };

        return track;
      })
      .filter((t: PlayerTrack | null): t is PlayerTrack => t !== null);

    return tracks;
  } catch (error) {
    console.error("[LibraryPersistence] loadLibrary failed:", error);
    return [];
  }
}

/**
 * 把当前资料库写入磁盘
 */
export async function saveLibrary(tracks: PlayerTrack[]): Promise<void> {
  try {
    const data = JSON.stringify(tracks, null, 2);
    await writeTextFile(LIBRARY_FILE, data, {
      baseDir: BaseDirectory.AppData,
    });
  } catch (error) {
    console.error("[LibraryPersistence] saveLibrary failed:", error);
  }
}
