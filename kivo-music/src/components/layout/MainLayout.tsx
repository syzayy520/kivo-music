// src/components/layout/MainLayout.tsx
import React from "react";
import type { TabKey } from "../../navigation/navigationModel";
import { Sidebar } from "./Sidebar";
import { PlayerBar } from "../PlayerBar";
import LibraryPage from "../../pages/LibraryPage";
import PlaylistPage from "../../pages/PlaylistPage";
import NowPlayingPage from "../../pages/NowPlayingPage";
import SettingsPage from "../../pages/SettingsPage";
import { useKivoTheme } from "../../styles/ThemeContext";

interface MainLayoutProps {
  currentTab: TabKey;
  onChangeTab: (tab: TabKey) => void;
  onEnterMiniMode: () => void;
}

/**
 * MainLayout
 *
 * 整个应用的主布局：
 * - 左侧 Sidebar（主导航）
 * - 右侧内容区：当前 Page + 底部 PlayerBar
 *
 * 注意：
 * - 所有背景 / 圆角 / 阴影从 theme 中取，方便后续多皮肤切换。
 */
export const MainLayout: React.FC<MainLayoutProps> = ({
  currentTab,
  onChangeTab,
  onEnterMiniMode,
}) => {
  const { theme } = useKivoTheme();

  const containerStyle: React.CSSProperties = {
    flex: 1,
    minHeight: 0,
    display: "flex",
    flexDirection: "row",
    padding: theme.spacing.lg,
    boxSizing: "border-box",
    gap: theme.spacing.lg,
  };

  const contentShellStyle: React.CSSProperties = {
    flex: 1,
    minHeight: 0,
    display: "flex",
    flexDirection: "column",
    background: theme.colors.contentBackground,
    borderRadius: theme.radius.xl,
    boxShadow: theme.shadow.card,
    overflow: "hidden",
  };

  const contentScrollWrapperStyle: React.CSSProperties = {
    flex: 1,
    minHeight: 0,
    overflow: "hidden",
  };

  const contentInnerStyle: React.CSSProperties = {
    height: "100%",
    boxSizing: "border-box",
    padding: theme.spacing.lg,
    overflowY: "auto",
  };

  const footerStyle: React.CSSProperties = {
    flexShrink: 0,
    borderTopWidth: 1,
    borderTopStyle: "solid",
    borderTopColor: theme.colors.borderSubtle,
    background:
      "linear-gradient(to top, rgba(15,23,42,0.08), rgba(15,23,42,0))",
  };

  const renderPage = (tab: TabKey): React.ReactNode => {
    switch (tab) {
      case "library":
        return <LibraryPage />;
      case "playlist":
        return <PlaylistPage />;
      case "nowPlaying":
        return <NowPlayingPage />;
      case "settings":
        return <SettingsPage />;
      default:
        return <LibraryPage />;
    }
  };

  return (
    <div style={containerStyle}>
      <Sidebar
        currentTab={currentTab}
        onChangeTab={onChangeTab}
        onEnterMiniMode={onEnterMiniMode}
      />

      <div style={contentShellStyle}>
        {/* 各 Page 自己负责内部头部 */}
        <main style={contentScrollWrapperStyle}>
          <div style={contentInnerStyle}>{renderPage(currentTab)}</div>
        </main>

        <footer style={footerStyle}>
          <PlayerBar />
        </footer>
      </div>
    </div>
  );
};

export default MainLayout;
