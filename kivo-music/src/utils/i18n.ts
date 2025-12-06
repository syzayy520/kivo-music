// src/utils/i18n.ts

/**
 * 简单的多语言工具：
 * - 当前只区分中文（zh）和英文（en）
 * - 通过浏览器 / 系统的 navigator.language 自动检测
 * - 将来可以在这里接入“手动设置语言”的配置
 */

export type AppLanguage = "zh" | "en";

let cachedLanguage: AppLanguage | null = null;

function detectLanguageFromNavigator(): AppLanguage {
  if (typeof navigator === "undefined" || !navigator.language) {
    return "en";
  }

  const lang = navigator.language.toLowerCase();

  // 所有以 zh 开头的都认为是中文（简体 / 繁体）
  if (lang.startsWith("zh")) {
    return "zh";
  }

  return "en";
}

/**
 * 获取当前应用语言。
 * 目前只做一次简单缓存，不依赖 React。
 */
export function getAppLanguage(): AppLanguage {
  if (cachedLanguage) return cachedLanguage;
  cachedLanguage = detectLanguageFromNavigator();
  return cachedLanguage;
}

/**
 * 文本翻译 helper。
 * 使用方式：t("中文", "English")
 */
export function t(zh: string, en: string): string {
  const lang = getAppLanguage();
  return lang === "zh" ? zh : en;
}
