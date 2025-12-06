// src/components/layout/Sidebar.tsx
import React from "react";
import type { TabKey } from "../../navigation/navigationModel";
import { MAIN_NAV_ITEMS } from "../../navigation/navigationModel";
import { useKivoTheme } from "../../styles/ThemeContext";
import { useI18n } from "../../i18n";

interface SidebarProps {
  currentTab: TabKey;
  onChangeTab: (tab: TabKey) => void;
  onEnterMiniMode: () => void;
}

/**
 * 主侧边栏（左侧导航）
 *
 * 设计要求：
 * - 深色基调 + 轻玻璃风格；
 * - 所有可见文字走 i18n；
 * - 导航结构由 navigationModel 管理。
 */
export const Sidebar: React.FC<SidebarProps> = ({
  currentTab,
  onChangeTab,
  onEnterMiniMode,
}) => {
  const { theme } = useKivoTheme();
  const { t } = useI18n();

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
    borderRadius: theme.radius.lg,
    backgroundColor: theme.colors.primarySoft,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    fontSize: 16,
  };

  const appTextStyle: React.CSSProperties = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
  };

  const appNameStyle: React.CSSProperties = {
    fontSize: 15,
    fontWeight: 600,
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
    display: "flex",
    flexDirection: "column",
    gap: 8,
  };

  const miniButtonStyle: React.CSSProperties = {
    width: "100%",
    borderRadius: theme.radius.lg,
    border: `1px dashed ${theme.colors.borderSubtle}`,
    backgroundColor: "transparent",
    cursor: "pointer",
    padding: "6px 8px",
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    color: theme.colors.textOnDark,
  };

  const miniButtonLeftStyle: React.CSSProperties = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
  };

  const miniBadgeStyle: React.CSSProperties = {
    display: "inline-flex",
    alignItems: "center",
    justifyContent: "center",
    padding: "0 6px",
    borderRadius: theme.radius.pill,
    border: `1px solid ${theme.colors.borderSubtle}`,
    fontSize: 10,
  };

  const footerTextStyle: React.CSSProperties = {
    fontSize: 11,
    color: theme.colors.textMutedOnDark,
    lineHeight: 1.4,
  };

  const renderNavButton = (item: (typeof MAIN_NAV_ITEMS)[number]) => {
    const active = item.key === currentTab;

    // 文案优先走 i18n key，没配 key 才退回到默认 label/description
    const label = item.labelKey ? t(item.labelKey) : item.label;
    const description = item.descriptionKey
      ? t(item.descriptionKey)
      : item.description ?? "";

    const baseStyle: React.CSSProperties = {
      width: "100%",
      border: "none",
      outline: "none",
      borderRadius: theme.radius.md,
      padding: "6px 8px",
      backgroundColor: "transparent",
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
        type="button"
        onClick={handleClick}
        style={{ ...baseStyle, ...activeStyle }}
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
          {label}
        </span>
        {description && (
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
            {description}
          </span>
        )}
      </button>
    );
  };

  return (
    <aside style={containerStyle}>
      {/* 顶部：Logo + 副标题 */}
      <div style={appTitleStyle}>
        <div style={logoCircleStyle}>♪</div>
        <div style={appTextStyle}>
          <div style={appNameStyle}>Kivo Music</div>
          <div style={appSubtitleStyle}>
            {t("sidebar.brand.tagline")}
          </div>
        </div>
      </div>

      {/* 主导航 */}
      <section style={navSectionStyle}>
        <div style={navHeaderStyle}>
          {t("sidebar.section.libraryAndPlayback")}
        </div>
        {MAIN_NAV_ITEMS.map((item) => renderNavButton(item))}
      </section>

      {/* 底部：Mini 模式入口 + 文案 */}
      <div style={bottomSectionStyle}>
        <button
          type="button"
          style={miniButtonStyle}
          onClick={onEnterMiniMode}
        >
          <div style={miniButtonLeftStyle}>
            <span style={{ fontSize: 13 }}>
              {t("sidebar.miniPlayer.title")}
            </span>
            <span style={miniBadgeStyle}>
              {t("sidebar.miniPlayer.badge.experimental")}
            </span>
          </div>
          <span style={{ fontSize: 14 }}>↗</span>
        </button>

        <div style={footerTextStyle}>
          {t("sidebar.footer.line1")}
          <br />
          {t("sidebar.footer.line2")}
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
