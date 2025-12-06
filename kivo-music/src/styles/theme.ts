// src/styles/theme.ts

/**
 * Kivo UI v2 主题设计（深色 · 轻玻璃 · 信息优先）
 *
 * 设计目标：
 * - 深色背景 + 层级清晰的卡片式布局；
 * - Sidebar/PlayerBar/内容区共用一套 token，不各自硬编码颜色；
 * - 后续可以通过 ThemeContext 扩展多皮肤，但组件层只依赖这些 token。
 */
export const kivoTheme = {
  colors: {
    // === 全局背景 ===
    /**
     * 应用最底层背景（AppShell 用 background 渲染）。
     * 使用略带渐变的深色，避免纯黑太“死”。
     */
    appBackground:
      "radial-gradient(circle at top, #020617 0%, #020617 40%, #000112 100%)",

    /**
     * Sidebar 背景：比 appBackground 稍亮一点，形成“侧边柱”层级。
     * 只在 Sidebar 上用 backgroundColor。
     */
    sidebarBackground: "rgba(15, 23, 42, 0.96)",

    /**
     * 主内容卡片背景（MainLayout 中央那块）。
     * 注意：目前被当成 background 使用。
     */
    contentBackground: "rgba(15, 23, 42, 0.94)",

    // === 头部 / 信息栏 ===
    /**
     * PageHeader 等顶部信息栏的背景。
     * 不是 Apple Music 那种粉紫渐变，改为偏青蓝的自定义主色。
     */
    headerBackground:
      "linear-gradient(135deg, #0f172a 0%, #0f766e 40%, #0369a1 100%)",
    headerBorder: "rgba(15, 23, 42, 0.55)",

    // === 文本 ===
    textOnDark: "#e5e7eb", // 深色背景上的主文本
    textMutedOnDark: "#9ca3af", // 深色背景上的次要 / 说明文本

    textOnLight: "#111827", // 浅色背景上的主文本
    textMutedOnLight: "#6b7280", // 浅色背景上的次要文本

    textOnPrimary: "#f9fafb", // 主按钮 / 高亮块上的文本

    // === 品牌 & 强调色 ===
    /**
     * Kivo 主色：偏青蓝，不沿用 Apple Music 的粉紫。
     */
    primary: "#38bdf8",
    primarySoft: "rgba(56, 189, 248, 0.16)",

    success: "#4ade80",
    warning: "#facc15",
    danger: "#f97373",

    /**
     * Tag / 小标签背景色（如统计数字、状态 badge）。
     */
    tagBg: "rgba(148, 163, 184, 0.20)",

    // === 边框 & 分割线 ===
    borderSubtle: "rgba(148, 163, 184, 0.35)", // 列表 / 卡片的细边框
    borderStrong: "rgba(148, 163, 184, 0.65)", // 悬浮 / 强调态边框

    // === 列表行态（Library / Playlist / UpNext 共用） ===
    /**
     * Hover 行背景：轻微提亮，不要太抢眼。
     */
    rowHover: "rgba(148, 163, 184, 0.14)",

    /**
     * 当前选中 / 正在播放行的背景：
     * - 带一点主色，但依然偏暗；
     * - 用在 Sidebar 选中项、列表 active 行等。
     */
    rowActive: "rgba(56, 189, 248, 0.20)",
  },

  // === 圆角 ===
  radius: {
    /**
     * 小按钮 / Tag 等。
     */
    xs: 4,
    /**
     * 小型卡片、输入框。
     */
    sm: 8,
    /**
     * 一般卡片 / 列表行。
     */
    md: 12,
    /**
     * 大块区域（PlayerBar / Sidebar 内部容器等）。
     */
    lg: 16,
    /**
     * 主内容大卡片。
     */
    xl: 24,
    /**
     * 特大卡片 / 特殊模块预留。
     */
    "2xl": 32,
    /**
     * 胶囊形按钮 / Pill。
     * KivoButton、Sidebar 选中项等都会用。
     */
    pill: 9999,
  },

  // === 间距（统一用 4px 基础网格） ===
  spacing: {
    xs: 4,
    sm: 8,
    md: 12,
    lg: 16,
    xl: 24,
    "2xl": 32,
  },

  // === 阴影（尽量少用，多用在主内容卡片） ===
  shadow: {
    /**
     * 主要卡片阴影：
     * - 用在 MainLayout 主内容卡片、设置面板等；
     * - 深色背景下稍重一点，制造“浮起”的感觉。
     */
    card: "0 22px 60px rgba(15, 23, 42, 0.65)",

    /**
     * 次级阴影：
     * - Sidebar 顶部小块、浮层等；
     * - 比 card 更浅。
     */
    subtle: "0 10px 30px rgba(15, 23, 42, 0.45)",
  },
} as const;

export type KivoTheme = typeof kivoTheme;
