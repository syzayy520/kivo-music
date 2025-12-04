// src/pages/NowPlayingPage.tsx
import React, { useMemo } from "react";
import { usePlayerStore } from "../store/player";
import NowPlayingCoverCard from "../components/now-playing/NowPlayingCoverCard";
import NowPlayingInfoPanel from "../components/now-playing/NowPlayingInfoPanel";
import UpNextPanel from "../components/now-playing/UpNextPanel";
import { VisualizerPanel } from "../components/now-playing/VisualizerPanel";

const pageContainerStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  display: "flex",
  flexDirection: "column",
  padding: "16px 24px",
  boxSizing: "border-box",
};

const headerStyle: React.CSSProperties = {
  marginBottom: 16,
  padding: "12px 4px",
};

const headerTitleStyle: React.CSSProperties = {
  fontSize: 18,
  fontWeight: 600,
  color: "#111827",
  marginBottom: 4,
};

const headerSubtitleStyle: React.CSSProperties = {
  fontSize: 13,
  color: "#6b7280",
  lineHeight: 1.6,
};

const bodyGridStyle: React.CSSProperties = {
  flex: 1,
  minHeight: 0,
  display: "grid",
  gridTemplateColumns: "minmax(0, 1.6fr) minmax(0, 1fr)",
  gap: 24,
};

const leftColumnStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: 16,
  minHeight: 0,
};

const rightColumnStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: 16,
  minHeight: 0,
};

const visualizerCardStyle: React.CSSProperties = {
  borderRadius: 24,
  padding: 16,
  background: "#ffffff",
  boxShadow: "0 10px 30px rgba(15,23,42,0.08)",
};

const NowPlayingPage: React.FC = () => {
  const state = usePlayerStore() as any;

  const playlist: any[] = Array.isArray(state.playlist)
    ? state.playlist
    : Array.isArray(state.tracks)
    ? state.tracks
    : [];

  const currentIndex: number =
    typeof state.currentIndex === "number" ? state.currentIndex : -1;

  const isPlaying: boolean = !!state.isPlaying;
  const currentTime: number =
    typeof state.currentTime === "number" ? state.currentTime : 0;
  const duration: number =
    typeof state.duration === "number" ? state.duration : 0;

  const currentTrack = useMemo(() => {
    if (!playlist || playlist.length === 0) return null;
    if (currentIndex >= 0 && currentIndex < playlist.length) {
      return playlist[currentIndex];
    }
    return playlist[0];
  }, [playlist, currentIndex]);

  const headerSubtitle = playlist.length
    ? `当前共有 ${playlist.length} 首歌曲在播放队列中。这里展示当前曲目的封面、详细信息，以及接下来要播放的队列。`
    : "当前播放队列为空。可以在“资料库”或“播放列表”中选择歌曲开始播放。";

  return (
    <div style={pageContainerStyle}>
      {/* 头部说明区域 */}
      <div style={headerStyle}>
        <div style={headerTitleStyle}>正在播放</div>
        <div style={headerSubtitleStyle}>
          {headerSubtitle}
          {"  "}
          本页面只负责展示，不再额外创建音频元素，所有播放行为仍由底部的播放器控制条统一管理。
        </div>
      </div>

      {/* 主体区域：左侧封面 + 信息 + 频谱，右侧“接下来播放”队列 */}
      <div style={bodyGridStyle}>
        {/* 左侧 */}
        <div style={leftColumnStyle}>
          <NowPlayingCoverCard
            track={currentTrack}
            playlist={playlist}
            currentIndex={currentIndex}
          />

          <NowPlayingInfoPanel
            track={currentTrack}
            isPlaying={isPlaying}
            currentTime={currentTime}
            duration={duration}
          />

          <div style={visualizerCardStyle}>
            <VisualizerPanel />
          </div>
        </div>

        {/* 右侧：Up Next 队列 */}
        <div style={rightColumnStyle}>
          <UpNextPanel />
        </div>
      </div>
    </div>
  );
};

export default NowPlayingPage;
