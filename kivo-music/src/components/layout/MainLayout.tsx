// src/components/layout/MainLayout.tsx
import React from "react";
import type { TabKey } from "../../navigation/navigationModel";
import { Sidebar } from "./Sidebar";
import { PlayerBar } from "../PlayerBar";
import LibraryPage from "../../pages/LibraryPage";
import PlaylistPage from "../../pages/PlaylistPage";
import NowPlayingPage from "../../pages/NowPlayingPage";
import SettingsPage from "../../pages/SettingsPage";
import { kivoTheme } from "../../styles/theme";

interface MainLayoutProps {
  currentTab: TabKey;
  onChangeTab: (tab: TabKey) => void;
  onEnterMiniMode: () => void;
}

const containerStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  display: "flex",
  flexDirection: "row",
  padding: kivoTheme.spacing.lg,
  boxSizing: "border-box",
  gap: kivoTheme.spacing.lg,
};

const contentShellStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  display: "flex",
  flexDirection: "column",
  background: kivoTheme.colors.contentBackground,
  borderRadius: kivoTheme.radius.xl,
  boxShadow: kivoTheme.shadow.card,
  overflow: "hidden",
};

const contentScrollWrapperStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  overflow: "hidden",
};

const contentInnerStyle: React.CSSProperties = {
  height: "100%",
  overflow: "auto",
};

const footerStyle: React.CSSProperties = {
  padding: `${kivoTheme.spacing.xs}px ${kivoTheme.spacing.lg}px`,
  background: "transparent",
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
      return null;
  }
};

export const MainLayout: React.FC<MainLayoutProps> = ({
  currentTab,
  onChangeTab,
  onEnterMiniMode,
}) => {
  return (
    <div style={containerStyle}>
      <Sidebar
        currentTab={currentTab}
        onChangeTab={onChangeTab}
        onEnterMiniMode={onEnterMiniMode}
      />

      <div style={contentShellStyle}>
        {/* 这里让各个 Page 自己负责内部头部 */}
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
