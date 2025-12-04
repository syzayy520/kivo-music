// src/styles/theme.ts
export const kivoTheme = {
  colors: {
    // 背景
    appBackground: "#020617", // 整体最底层（接近现在的深色）
    sidebarBackground: "linear-gradient(180deg, #020617 0%, #020617 30%, #020617 100%)",
    contentBackground: "#f3f4f6",

    // 顶部大头部卡片
    headerBackground: "linear-gradient(135deg, #1d4ed8 0%, #2563eb 40%, #0ea5e9 100%)",
    headerBorder: "rgba(15, 23, 42, 0.16)",

    // 文字
    textOnDark: "#e5e7eb",
    textMutedOnDark: "#9ca3af",
    textOnLight: "#111827",
    textMutedOnLight: "#6b7280",
    textOnPrimary: "#ffffff",

    // 主色 & 强调色
    primary: "#2563eb",
    primarySoft: "rgba(37, 99, 235, 0.12)",
    danger: "#ef4444",

    // 边框 & 分割线
    borderSubtle: "rgba(148, 163, 184, 0.35)",
    borderStrong: "rgba(15, 23, 42, 0.24)",

    // 列表 Hover / 选中
    rowHover: "rgba(37, 99, 235, 0.06)",
    rowActive: "rgba(37, 99, 235, 0.12)",
  },

  radius: {
    sm: 6,
    md: 10,
    lg: 16,
    xl: 24,
    pill: 999,
  },

  spacing: {
    xs: 4,
    sm: 8,
    md: 12,
    lg: 16,
    xl: 24,
    "2xl": 32,
  },

  shadow: {
    card: "0 18px 45px rgba(15, 23, 42, 0.45)",
    subtle: "0 4px 12px rgba(15, 23, 42, 0.35)",
  },
};

export type KivoTheme = typeof kivoTheme;
