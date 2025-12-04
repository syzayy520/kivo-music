// src/styles/ThemeContext.tsx
import React, {
  createContext,
  useContext,
  useMemo,
  useState,
  PropsWithChildren,
} from "react";
import { kivoTheme, type KivoTheme } from "./theme";

/**
 * 预留多主题名称。
 *
 * 目前只有一个默认主题，后面可以扩展：
 * - "kivoDefault"：当前这套蓝色 Apple-ish 风格
 * - "kivoClassic"：偏 macOS 10.15 的浅色
 * - "kivoDark"：类 Spotify 深灰
 * - "kivoWarm"：类网易云偏暖色
 */
export type ThemeName = "kivoDefault";

export interface KivoThemeContextValue {
  theme: KivoTheme;
  themeName: ThemeName;
  /**
   * 切换主题名称。后续会在设置页里调用，用于多皮肤切换。
   */
  setThemeName: (name: ThemeName) => void;
}

const KivoThemeContext = createContext<KivoThemeContextValue | null>(null);

/**
 * KivoThemeProvider
 *
 * 全局主题提供者：
 * - 内部维护当前主题名（ThemeName）
 * - 根据主题名选择对应的 theme token（目前只有 kivoTheme）
 * - 提供 useKivoTheme() hook 让各个组件访问
 */
export const KivoThemeProvider: React.FC<PropsWithChildren> = ({ children }) => {
  const [themeName, setThemeName] = useState<ThemeName>("kivoDefault");

  const theme = useMemo<KivoTheme>(() => {
    // 预留多主题扩展位
    switch (themeName) {
      case "kivoDefault":
      default:
        return kivoTheme;
    }
  }, [themeName]);

  const value: KivoThemeContextValue = useMemo(
    () => ({
      theme,
      themeName,
      setThemeName,
    }),
    [theme, themeName],
  );

  return (
    <KivoThemeContext.Provider value={value}>
      {children}
    </KivoThemeContext.Provider>
  );
};

/**
 * useKivoTheme
 *
 * 在组件中获取当前 theme & themeName & setThemeName。
 * 使用方式：
 *   const { theme, themeName, setThemeName } = useKivoTheme();
 */
export const useKivoTheme = (): KivoThemeContextValue => {
  const ctx = useContext(KivoThemeContext);
  if (!ctx) {
    throw new Error(
      "useKivoTheme must be used within a KivoThemeProvider (主题相关组件必须被包裹在 KivoThemeProvider 内部)。",
    );
  }
  return ctx;
};
