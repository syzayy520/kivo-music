// src/components/layout/Sidebar.tsx
import React from "react";
import type { TabKey } from "../../navigation/navigationModel";
import { MAIN_NAV_ITEMS } from "../../navigation/navigationModel";
import { useKivoTheme } from "../../styles/ThemeContext";

interface SidebarProps {
  currentTab: TabKey;
  onChangeTab: (tab: TabKey) => void;
  onEnterMiniMode: () => void;
}

/**
 * 主侧边栏（左侧导航）
 *
 * 注意：
 * - 不再混用 background 和 backgroundColor，全部用 backgroundColor，避免 React 警告。
 */
export const Sidebar: React.FC<SidebarProps> = ({
  currentTab,
  onChangeTab,
  onEnterMiniMode,
}) => {
  const { theme } = useKivoTheme();

  const containerStyle: React.CSSProperties = {
    width: 232,
    padding: "14px 12px",
    boxSizing: "border-box",
    display: "flex",
    flexDirection: "column",
    gap: 16,
    backgroundColor: theme.colors.sidebarBackground,
    color: theme.colors.textOnDark,
    borderRight: `1px solid ${theme.colors.borderSubtle}`,
  };

  const appTitleStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    gap: 10,
    padding: "6px 6px 10px 6px",
  };

  const logoCircleStyle: React.CSSProperties = {
    width: 28,
    height: 28,
    borderRadius: theme.radius.pill,
    background:
      "radial-gradient(circle at 30% 30%, #38bdf8 0%, #2563eb 45%, #0f172a 100%)",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    boxShadow: theme.shadow.subtle,
    fontSize: 16,
    fontWeight: 700,
    color: "#e5e7eb",
  };

  const appTextStyle: React.CSSProperties = {
    display: "flex",
    flexDirection: "column",
    justifyContent: "center",
  };

  const appNameStyle: React.CSSProperties = {
    fontSize: 14,
    fontWeight: 600,
    letterSpacing: 0.2,
  };

  const appSubtitleStyle: React.CSSProperties = {
    fontSize: 11,
    color: theme.colors.textMutedOnDark,
  };

  const navSectionStyle: React.CSSProperties = {
    marginTop: 4,
    display: "flex",
    flexDirection: "column",
    gap: 4,
  };

  const navHeaderStyle: React.CSSProperties = {
    padding: "0 6px",
    fontSize: 11,
    textTransform: "uppercase",
    letterSpacing: 0.12,
    color: theme.colors.textMutedOnDark,
  };

  const bottomSectionStyle: React.CSSProperties = {
    marginTop: "auto",
    paddingTop: 12,
    borderTop: `1px solid rgba(15,23,42,0.65)`,
    display: "flex",
    flexDirection: "column",
    gap: 8,
  };

  const miniButtonStyle: React.CSSProperties = {
    width: "100%",
    padding: "6px 10px",
    borderRadius: theme.radius.md,
    border: `1px solid ${theme.colors.borderSubtle}`,
    backgroundColor: "rgba(15,23,42,0.35)",
    color: theme.colors.textOnDark,
    fontSize: 12,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    cursor: "pointer",
  };

  const miniButtonLeftStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    gap: 6,
  };

  const miniBadgeStyle: React.CSSProperties = {
    padding: "1px 6px",
    borderRadius: theme.radius.pill,
    border: `1px solid ${theme.colors.borderSubtle}`,
    fontSize: 10,
    color: theme.colors.textMutedOnDark,
  };

  const footerTextStyle: React.CSSProperties = {
    padding: "0 2px",
    fontSize: 10,
    color: theme.colors.textMutedOnDark,
    lineHeight: 1.4,
  };

  const renderNavButton = (item: (typeof MAIN_NAV_ITEMS)[number]) => {
    const active = item.key === currentTab;

    const baseStyle: React.CSSProperties = {
      width: "100%",
      border: "none",
      outline: "none",
      borderRadius: theme.radius.md,
      padding: "6px 8px",
      backgroundColor: "transparent", // ✅ 只用 backgroundColor
      color: theme.colors.textOnDark,
      fontSize: 13,
      cursor: "pointer",
      display: "flex",
      flexDirection: "column",
      alignItems: "flex-start",
      gap: 2,
      textAlign: "left",
    };

    const activeStyle: React.CSSProperties = active
      ? {
          backgroundColor: theme.colors.rowActive,
        }
      : {};

    const handleClick: React.MouseEventHandler<HTMLButtonElement> = (e) => {
      e.preventDefault();
      onChangeTab(item.key);
    };

    return (
      <button
        key={item.key}
        onClick={handleClick}
        style={{
          ...baseStyle,
          ...activeStyle,
        }}
        onMouseEnter={(e) => {
          if (active) return;
          e.currentTarget.style.backgroundColor = theme.colors.rowHover;
        }}
        onMouseLeave={(e) => {
          if (active) return;
          e.currentTarget.style.backgroundColor = "transparent";
        }}
      >
        <span
          style={{
            fontWeight: active ? 600 : 500,
            fontSize: 13,
          }}
        >
          {item.label}
        </span>
        {item.description && (
          <span
            style={{
              fontSize: 11,
              color: theme.colors.textMutedOnDark,
              whiteSpace: "nowrap",
              textOverflow: "ellipsis",
              overflow: "hidden",
              maxWidth: "100%",
            }}
          >
            {item.description}
          </span>
        )}
      </button>
    );
  };

  return (
    <aside style={containerStyle}>
      {/* 顶部 Logo / 标题 */}
      <div style={appTitleStyle}>
        <div style={logoCircleStyle}>♪</div>
        <div style={appTextStyle}>
          <div style={appNameStyle}>Kivo Music</div>
          <div style={appSubtitleStyle}>本地 · 高性能 · Apple 风格</div>
        </div>
      </div>

      {/* 主导航 */}
      <section style={navSectionStyle}>
        <div style={navHeaderStyle}>资料库 & 播放</div>
        {MAIN_NAV_ITEMS.map((item) => renderNavButton(item))}
      </section>

      {/* 底部：Mini 模式入口 + 文案 */}
      <div style={bottomSectionStyle}>
        <button style={miniButtonStyle} onClick={onEnterMiniMode}>
          <div style={miniButtonLeftStyle}>
            <span style={{ fontSize: 13 }}>Mini 播放器</span>
            <span style={miniBadgeStyle}>实验中</span>
          </div>
          <span style={{ fontSize: 14 }}>↗</span>
        </button>

        <div style={footerTextStyle}>
          Kivo · 本地音乐播放器
          <br />
          目标：比 Apple Music 更好用的 Windows 客户端。
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
