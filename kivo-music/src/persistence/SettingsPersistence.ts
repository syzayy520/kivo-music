// src/persistence/SettingsPersistence.ts
import { appDataDir, join } from '@tauri-apps/api/path';
import { exists, mkdir, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

export interface KivoSettings {
  /**
   * 封面缓存目录
   * 如果为空，则使用默认：
   *   appDataDir()/covers
   */
  coverCacheDir?: string | null;
}

const SETTINGS_FILE_NAME = 'kivo-settings.json';

async function ensureDirExists(dir: string): Promise<void> {
  try {
    if (!(await exists(dir))) {
      await mkdir(dir, { recursive: true });
    }
  } catch (error) {
    console.error('[SettingsPersistence] ensureDirExists失败:', dir, error);
  }
}

async function getSettingsFilePath(): Promise<string> {
  const appData = await appDataDir();
  await ensureDirExists(appData);
  return await join(appData, SETTINGS_FILE_NAME);
}

export async function loadSettings(): Promise<KivoSettings> {
  const path = await getSettingsFilePath();

  try {
    if (!(await exists(path))) {
      return {};
    }

    const raw = await readTextFile(path);
    if (!raw.trim()) return {};

    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === 'object') {
      return parsed as KivoSettings;
    }
  } catch (error) {
    console.error('[SettingsPersistence] 读取设置失败，使用空设置:', error);
  }

  return {};
}

export async function saveSettings(settings: KivoSettings): Promise<void> {
  const path = await getSettingsFilePath();

  try {
    const json = JSON.stringify(settings ?? {}, null, 2);
    await writeTextFile(path, json);
  } catch (error) {
    console.error('[SettingsPersistence] 保存设置失败:', error);
  }
}

/**
 * 获取“有效的封面缓存目录”
 * 优先使用用户设置的 coverCacheDir，其次是默认：
 *   appDataDir()/covers
 */
export async function getEffectiveCoverCacheDir(): Promise<string> {
  const settings = await loadSettings();

  if (settings.coverCacheDir && settings.coverCacheDir.trim().length > 0) {
    return settings.coverCacheDir;
  }

  const appData = await appDataDir();
  return await join(appData, 'covers');
}
