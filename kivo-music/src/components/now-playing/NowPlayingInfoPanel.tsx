// src/components/now-playing/NowPlayingInfoPanel.tsx
import React from "react";

interface NowPlayingInfoPanelProps {
  track: any | null;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
}

const boxStyle: React.CSSProperties = {
  borderRadius: 24,
  padding: 16,
  background: "#ffffff",
  boxShadow: "0 10px 30px rgba(15,23,42,0.06)",
  display: "flex",
  flexDirection: "column",
  gap: 12,
  height: "100%",
};

const NowPlayingInfoPanel: React.FC<NowPlayingInfoPanelProps> = ({
  track,
  isPlaying,
  currentTime,
  duration,
}) => {
  const anyTrack = track as any;
  const title =
    anyTrack?.title ?? anyTrack?.name ?? anyTrack?.fileName ?? "未选择曲目";
  const artist =
    anyTrack?.artist ?? anyTrack?.albumArtist ?? "未知艺人";
  const album = anyTrack?.album ?? "";
  const filePath =
    anyTrack?.filePath ?? anyTrack?.path ?? anyTrack?.location ?? "";

  const safeDuration = duration > 0 ? duration : 0;
  const safeCurrent = Math.min(
    Math.max(currentTime, 0),
    safeDuration || Number.MAX_VALUE,
  );
  const progress = safeDuration > 0 ? safeCurrent / safeDuration : 0;

  const formatTime = (sec: number) => {
    if (!Number.isFinite(sec) || sec < 0) return "00:00";
    const s = Math.floor(sec);
    const m = Math.floor(s / 60);
    const r = s % 60;
    const mm = String(m).padStart(2, "0");
    const ss = String(r).padStart(2, "0");
    return `${mm}:${ss}`;
  };

  return (
    <div style={boxStyle}>
      <div>
        <div style={{ fontSize: 16, fontWeight: 600, marginBottom: 4 }}>
          {title}
        </div>
        <div style={{ fontSize: 13, color: "#6b7280" }}>
          {artist}
          {album ? ` · ${album}` : ""}
        </div>
      </div>

      {filePath && (
        <div
          style={{
            fontSize: 11,
            color: "#9ca3af",
            wordBreak: "break-all",
            marginTop: 4,
          }}
        >
          {filePath}
        </div>
      )}

      {/* 进度条 */}
      <div style={{ marginTop: 12 }}>
        <div
          style={{
            width: "100%",
            height: 4,
            borderRadius: 9999,
            background: "#e5e7eb",
            overflow: "hidden",
          }}
        >
          <div
            style={{
              width: `${progress * 100}%`,
              height: "100%",
              background: "#3b82f6",
              transition: "width 0.1s linear",
            }}
          />
        </div>
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            marginTop: 4,
            fontSize: 11,
            color: "#6b7280",
          }}
        >
          <span>{formatTime(safeCurrent)}</span>
          <span>{formatTime(safeDuration)}</span>
        </div>
      </div>

      {/* 底部状态说明卡片 */}
      <div
        style={{
          marginTop: 12,
          padding: 10,
          borderRadius: 12,
          background: "#f9fafb",
          fontSize: 11,
          color: "#6b7280",
          lineHeight: 1.6,
        }}
      >
        当前页面与底部播放器、Mini 模式共享同一播放器状态。
        在「资料库 / 播放列表」中操作播放/切歌，都会实时反映到这里。
      </div>

      <div
        style={{
          marginTop: "auto",
          fontSize: 11,
          color: "#9ca3af",
          display: "flex",
          justifyContent: "space-between",
        }}
      >
        <span>
          状态：{track ? (isPlaying ? "正在播放" : "已暂停") : "未选择曲目"}
        </span>
        <span>Now Playing · v3 布局</span>
      </div>
    </div>
  );
};

export default NowPlayingInfoPanel;
