// src/persistence/SettingsPersistence.ts
import { appDataDir, join } from "@tauri-apps/api/path";
import { exists, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

/**
 * 后续所有设置都放这里，目前只先用到封面缓存目录。
 */
export interface KivoSettings {
  /** 自定义封面缓存目录（留空则使用默认 AppData/covers） */
  coverCacheDir?: string;
}

const SETTINGS_FILE_NAME = "kivo-settings.json";

async function getSettingsFilePath(): Promise<string> {
  const dir = await appDataDir();
  return await join(dir, SETTINGS_FILE_NAME);
}

export async function loadSettings(): Promise<KivoSettings> {
  try {
    const path = await getSettingsFilePath();
    const hasFile = await exists(path);
    if (!hasFile) {
      return {};
    }
    const raw = await readTextFile(path);
    if (!raw.trim()) return {};
    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === "object") {
      return parsed as KivoSettings;
    }
    return {};
  } catch (err) {
    console.error("[SettingsPersistence] loadSettings error:", err);
    return {};
  }
}

export async function saveSettings(settings: KivoSettings): Promise<void> {
  try {
    const path = await getSettingsFilePath();
    const json = JSON.stringify(settings, null, 2);
    await writeTextFile(path, json);
  } catch (err) {
    console.error("[SettingsPersistence] saveSettings error:", err);
  }
}

/**
 * 计算“有效”的封面缓存目录：
 * - 若用户设置了 coverCacheDir，则直接用用户设置；
 * - 否则默认使用 AppData 下的 covers 目录。
 *
 * 注意：这里只负责返回路径，不会创建目录。
 * 目录创建交给 CoverCache.ensureCoversDir 来做。
 */
export async function getEffectiveCoverCacheDir(): Promise<string> {
  const settings = await loadSettings();
  if (settings.coverCacheDir && settings.coverCacheDir.trim().length > 0) {
    return settings.coverCacheDir.trim();
  }

  const appDir = await appDataDir();
  return await join(appDir, "covers");
}
