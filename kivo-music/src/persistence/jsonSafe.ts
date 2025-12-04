// src/persistence/jsonSafe.ts
import {
  exists,
  readTextFile,
  writeTextFile,
  copyFile,
} from "@tauri-apps/plugin-fs";
import { log } from "../utils/log";

/**
 * 安全读取 JSON：
 * - 文件不存在：返回 fallback
 * - 读取 / 解析失败：备份原文件为 *.corrupt-时间戳.bak，返回 fallback
 */
export async function safeReadJson<T>(
  path: string,
  fallback: T,
): Promise<T> {
  try {
    const has = await exists(path);
    if (!has) return fallback;

    const raw = await readTextFile(path);
    if (!raw.trim()) return fallback;

    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === "object") {
      return parsed as T;
    }

    // 解析出来不是对象，就当失败处理
    await backupCorruptFile(path);
  } catch (error) {
    log.error("jsonSafe", "读取或解析 JSON 失败", { path, error });
    await backupCorruptFile(path);
  }
  return fallback;
}

/**
 * 安全写入 JSON：
 * - 写之前如果有旧文件，会先拷贝一份 *.bak
 * - 写失败时保留旧文件 + 打日志
 */
export async function safeWriteJson<T>(
  path: string,
  data: T,
): Promise<void> {
  try {
    // 先备份旧文件
    if (await exists(path)) {
      const backup = path + ".bak";
      try {
        await copyFile(path, backup);
      } catch (error) {
        log.warn("jsonSafe", "备份旧 JSON 失败", { path, backup, error });
      }
    }

    const json = JSON.stringify(data ?? {}, null, 2);
    await writeTextFile(path, json);
  } catch (error) {
    log.error("jsonSafe", "写入 JSON 失败", { path, error });
    // 写失败这里先不删旧文件，保留给后续排查
  }
}

/**
 * 把疑似损坏的 JSON 备份成 *.corrupt-时间戳.bak
 */
async function backupCorruptFile(path: string): Promise<void> {
  try {
    if (!(await exists(path))) return;
    const stamp = new Date().toISOString().replace(/[:.]/g, "-");
    const backup = `${path}.corrupt-${stamp}.bak`;
    await copyFile(path, backup);
    log.warn("jsonSafe", "已备份疑似损坏的 JSON 文件", { path, backup });
  } catch (error) {
    log.warn("jsonSafe", "备份损坏 JSON 文件失败", { path, error });
  }
}
