// src/pages/NowPlayingPage.tsx
import React from "react";
import { usePlayerStore } from "../store/player";
import NowPlayingCoverCard from "../components/now-playing/NowPlayingCoverCard";
import NowPlayingInfoPanel from "../components/now-playing/NowPlayingInfoPanel";

const NowPlayingPage: React.FC = () => {
  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);
  const currentTime = usePlayerStore((s) => s.currentTime);
  const duration = usePlayerStore((s) => s.duration);

  const track = playlist[currentIndex] ?? null;

  return (
    <div
      style={{
        padding: 16,
        height: "100%",
        boxSizing: "border-box",
        display: "flex",
        flexDirection: "column",
        gap: 12,
      }}
    >
      {/* 标题 */}
      <div style={{ display: "flex", justifyContent: "space-between" }}>
        <div>
          <h1
            style={{
              margin: 0,
              fontSize: 18,
              fontWeight: 600,
            }}
          >
            正在播放
          </h1>
          <p
            style={{
              margin: 0,
              marginTop: 4,
              fontSize: 12,
              color: "#6b7280",
            }}
          >
            查看当前曲目的封面、信息与播放进度。
          </p>
        </div>
      </div>

      {/* 主体布局：左封面，右信息面板 */}
      <div
        style={{
          flex: 1,
          minHeight: 0,
          display: "grid",
          gridTemplateColumns: "minmax(0, 1.1fr) minmax(0, 1fr)",
          gap: 16,
        }}
      >
        <NowPlayingCoverCard
          track={track}
          playlist={playlist}
          currentIndex={currentIndex}
        />
        <NowPlayingInfoPanel
          track={track}
          isPlaying={isPlaying}
          currentTime={currentTime}
          duration={duration}
        />
      </div>
    </div>
  );
};

export default NowPlayingPage;
