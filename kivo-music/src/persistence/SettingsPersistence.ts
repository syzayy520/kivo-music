// src/persistence/SettingsPersistence.ts
import { exists, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";

export type Theme = "system" | "light" | "dark";

export interface AppSettings {
  /** schema 版本，未来升级 settings 时可以用来做兼容 */
  version: 1;
  /** 主题：system = 跟随操作系统（目前等价于 light，占位） */
  theme: Theme;
  /** 预留：封面缓存目录（绝对路径），null 表示使用默认的 AppData/covers */
  coverCacheDir: string | null;
}

export const DEFAULT_SETTINGS: AppSettings = {
  version: 1,
  theme: "system",
  coverCacheDir: null,
};

const SETTINGS_FILE_NAME = "settings.json";

async function getSettingsFilePath(): Promise<string> {
  const root = await appDataDir();
  return await join(root, SETTINGS_FILE_NAME);
}

/**
 * 从磁盘加载设置。
 * 读取失败或不存在时，返回默认设置。
 */
export async function loadSettings(): Promise<AppSettings> {
  try {
    const path = await getSettingsFilePath();
    const has = await exists(path);
    if (!has) {
      return { ...DEFAULT_SETTINGS };
    }

    const text = await readTextFile(path);
    if (!text || !text.trim()) {
      return { ...DEFAULT_SETTINGS };
    }

    const raw = JSON.parse(text) as Partial<AppSettings>;

    const theme: Theme =
      raw.theme === "light" || raw.theme === "dark" || raw.theme === "system"
        ? raw.theme
        : DEFAULT_SETTINGS.theme;

    const coverCacheDir: string | null =
      typeof raw.coverCacheDir === "string" ? raw.coverCacheDir : null;

    return {
      version: 1,
      theme,
      coverCacheDir,
    };
  } catch (error) {
    console.error("[SettingsPersistence] loadSettings error:", error);
    return { ...DEFAULT_SETTINGS };
  }
}

/**
 * 覆盖保存完整设置。
 */
export async function saveSettings(settings: AppSettings): Promise<void> {
  try {
    const path = await getSettingsFilePath();
    const json = JSON.stringify(settings, null, 2);
    await writeTextFile(path, json);
    console.log("[SettingsPersistence] saveSettings ok");
  } catch (error) {
    console.error("[SettingsPersistence] saveSettings error:", error);
  }
}

/**
 * 以“补丁”的形式更新设置。
 * 会读取当前 settings，与 patch 合并后再写入。
 */
export async function updateSettings(
  patch: Partial<AppSettings>,
): Promise<AppSettings> {
  const current = await loadSettings();
  const next: AppSettings = {
    ...current,
    ...patch,
    version: 1,
  };
  await saveSettings(next);
  return next;
}

/**
 * 有效的封面缓存目录：
 * - 如果用户设置了 coverCacheDir，则使用该路径
 * - 否则使用默认的 AppData/covers
 *
 * 当前版本只是一个工具函数，CoverCache 还没有真正切换到这里，
 * 但后续可以直接在 CoverCache 中调用。
 */
export async function getEffectiveCoverCacheDir(): Promise<string> {
  const settings = await loadSettings();
  if (settings.coverCacheDir && settings.coverCacheDir.trim().length > 0) {
    return settings.coverCacheDir.trim();
  }
  const root = await appDataDir();
  return await join(root, "covers");
}
