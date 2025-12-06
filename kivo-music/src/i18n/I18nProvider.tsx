// src/i18n/I18nProvider.tsx
import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { zhCNMessages } from "./locales/zh-CN";
import { enUSMessages } from "./locales/en-US";

export type SupportedLocale = "zh-CN" | "en-US";

const LOCALE_STORAGE_KEY = "kivo.locale";

/**
 * 所有文案字典。
 * - 若 key 在当前语言缺失，会自动回退到 en-US 版本；
 * - 仍然缺失时返回 defaultValue 或 key 本身，用于 debug。
 */
const allMessages: Record<SupportedLocale, Record<string, string>> = {
  "zh-CN": zhCNMessages,
  "en-US": enUSMessages,
};

interface I18nContextValue {
  locale: SupportedLocale;
  t: (key: string, defaultValue?: string) => string;
  setLocale: (locale: SupportedLocale) => void;
}

const I18nContext = createContext<I18nContextValue | undefined>(undefined);

function resolveLocale(raw: string | null | undefined): SupportedLocale {
  if (!raw) return "en-US";
  const lower = raw.toLowerCase();

  if (lower.startsWith("zh")) return "zh-CN";
  return "en-US";
}

/**
 * 检测初始语言：
 * 1. localStorage 中手动选择的语言；
 * 2. navigator.language（通常等于系统语言）；
 * 3. 默认 en-US。
 */
function detectInitialLocale(): SupportedLocale {
  if (typeof window !== "undefined") {
    try {
      const stored = window.localStorage.getItem(LOCALE_STORAGE_KEY);
      if (stored === "zh-CN" || stored === "en-US") {
        return stored;
      }
    } catch {
      // ignore
    }
  }

  if (typeof navigator !== "undefined" && navigator.language) {
    return resolveLocale(navigator.language);
  }

  return "en-US";
}

export const I18nProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [locale, setLocaleState] = useState<SupportedLocale>(() =>
    detectInitialLocale(),
  );

  const setLocale = useCallback((next: SupportedLocale) => {
    setLocaleState(next);
    if (typeof window !== "undefined") {
      try {
        window.localStorage.setItem(LOCALE_STORAGE_KEY, next);
      } catch {
        // ignore
      }
    }
  }, []);

  const t = useCallback(
    (key: string, defaultValue?: string) => {
      const dict = allMessages[locale] ?? allMessages["en-US"];
      const fallback = allMessages["en-US"];

      if (Object.prototype.hasOwnProperty.call(dict, key)) {
        return dict[key];
      }
      if (Object.prototype.hasOwnProperty.call(fallback, key)) {
        return fallback[key];
      }
      return defaultValue ?? key;
    },
    [locale],
  );

  useEffect(() => {
    if (typeof document !== "undefined") {
      document.documentElement.lang = locale;
    }
  }, [locale]);

  const value = useMemo<I18nContextValue>(
    () => ({
      locale,
      t,
      setLocale,
    }),
    [locale, t, setLocale],
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
};

export function useI18n(): I18nContextValue {
  const ctx = useContext(I18nContext);
  if (!ctx) {
    throw new Error("useI18n must be used within an I18nProvider");
  }
  return ctx;
}
