// src/pages/NowPlayingPage.tsx
import React from "react";
import { usePlayerStore } from "../store/player";
import NowPlayingCoverCard from "../components/now-playing/NowPlayingCoverCard";
import NowPlayingInfoPanel from "../components/now-playing/NowPlayingInfoPanel";
import UpNextPanel from "../components/now-playing/UpNextPanel";
import { VisualizerPanel } from "../components/now-playing/VisualizerPanel";
import { useI18n } from "../i18n";

const pageContainerStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  display: "flex",
  flexDirection: "column",
  padding: "16px 24px",
  boxSizing: "border-box",
  gap: 16,
};

const headerStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "flex-start",
  justifyContent: "space-between",
  gap: 16,
};

const headerTitleBlockStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: 4,
};

const headerTitleStyle: React.CSSProperties = {
  fontSize: 22,
  fontWeight: 600,
};

const headerSubtitleStyle: React.CSSProperties = {
  fontSize: 13,
  lineHeight: 1.5,
  color: "#6b7280",
  maxWidth: 560,
  whiteSpace: "pre-line",
};

const headerNoteStyle: React.CSSProperties = {
  fontSize: 11,
  lineHeight: 1.5,
  color: "#9ca3af",
  maxWidth: 320,
  textAlign: "right",
};

const mainAreaStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  display: "flex",
  flexDirection: "row",
  gap: 16,
};

const leftColumnStyle: React.CSSProperties = {
  flex: 3,
  minWidth: 0,
  display: "flex",
  flexDirection: "column",
  gap: 16,
};

const leftTopRowStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "row",
  gap: 16,
  minHeight: 0,
};

const coverColumnStyle: React.CSSProperties = {
  flex: 3,
  minWidth: 0,
};

const infoColumnStyle: React.CSSProperties = {
  flex: 2,
  minWidth: 0,
};

const visualizerWrapperStyle: React.CSSProperties = {
  borderRadius: 24,
  overflow: "hidden",
  backgroundColor: "rgba(15,23,42,0.95)",
};

const rightColumnStyle: React.CSSProperties = {
  flex: 2,
  minWidth: 0,
};

const NowPlayingPage: React.FC = () => {
  const { t } = useI18n();
  // 为了兼容旧版本 state 结构，这里仍然用 any 做一层兜底
  const state = usePlayerStore() as any;

  const playlist: any[] = Array.isArray(state.playlist)
    ? state.playlist
    : Array.isArray(state.tracks)
    ? state.tracks
    : [];

  const currentIndex: number =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;

  const currentTrack =
    currentIndex >= 0 && currentIndex < playlist.length
      ? playlist[currentIndex]
      : null;

  const isPlaying: boolean = !!state.isPlaying;
  const currentTime: number =
    typeof state.currentTime === "number" ? state.currentTime : 0;
  const duration: number =
    typeof state.duration === "number" ? state.duration : 0;

  const totalCount = playlist.length;

  const subtitle =
    totalCount > 0
      ? t("nowPlaying.header.subtitle.nonEmpty").replace(
          "{count}",
          String(totalCount),
        )
      : t("nowPlaying.header.subtitle.empty");

  const note = t("nowPlaying.header.note");

  return (
    <div style={pageContainerStyle}>
      {/* 顶部标题区 */}
      <header style={headerStyle}>
        <div style={headerTitleBlockStyle}>
          <h1 style={headerTitleStyle}>{t("nowPlaying.header.title")}</h1>
          <p style={headerSubtitleStyle}>{subtitle}</p>
        </div>
        <p style={headerNoteStyle}>{note}</p>
      </header>

      {/* 主体区域：左侧封面+信息+可视化，右侧 Up Next 队列 */}
      <div style={mainAreaStyle}>
        <div style={leftColumnStyle}>
          <div style={leftTopRowStyle}>
            <div style={coverColumnStyle}>
              <NowPlayingCoverCard
                track={currentTrack}
                playlist={playlist}
                currentIndex={currentIndex}
              />
            </div>
            <div style={infoColumnStyle}>
              <NowPlayingInfoPanel
                track={currentTrack}
                isPlaying={isPlaying}
                currentTime={currentTime}
                duration={duration}
              />
            </div>
          </div>

          <div style={visualizerWrapperStyle}>
            <VisualizerPanel />
          </div>
        </div>

        <div style={rightColumnStyle}>
          <UpNextPanel />
        </div>
      </div>
    </div>
  );
};

export default NowPlayingPage;
