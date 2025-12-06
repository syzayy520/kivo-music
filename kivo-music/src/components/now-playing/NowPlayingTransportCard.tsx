// src/components/now-playing/NowPlayingTransportCard.tsx
import React, { useState } from "react";
import { useI18n } from "../../i18n";

interface NowPlayingTransportCardProps {
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  volume: number;
  onTogglePlay: () => void;
  onPrev: () => void;
  onNext: () => void;
  onSeek: (position: number) => void;
  onVolumeChange: (volume: number) => void;
}

function formatTime(sec: number): string {
  if (!Number.isFinite(sec) || sec < 0) return "0:00";
  const s = Math.floor(sec);
  const m = Math.floor(s / 60);
  const r = s % 60;
  return `${m}:${String(r).padStart(2, "0")}`;
}

/** 负责进度条 + 上/下一首 + 播放/暂停 + 音量控制 */
const NowPlayingTransportCard: React.FC<NowPlayingTransportCardProps> = ({
  isPlaying,
  currentTime,
  duration,
  volume,
  onTogglePlay,
  onPrev,
  onNext,
  onSeek,
  onVolumeChange,
}) => {
  const { t } = useI18n();

  const [seeking, setSeeking] = useState(false);
  const [seekValue, setSeekValue] = useState(0);

  const safeDuration = duration > 0 ? duration : 0;
  const safeCurrent = Math.min(
    Math.max(currentTime, 0),
    safeDuration || Number.MAX_VALUE,
  );
  const sliderValue = seeking ? seekValue : safeCurrent;
  const progress = safeDuration > 0 ? sliderValue / safeDuration : 0;

  const handleSeekStart = () => {
    setSeeking(true);
    setSeekValue(safeCurrent);
  };

  const handleSeekChange = (value: number) => {
    const clamped = Math.min(Math.max(value, 0), safeDuration);
    setSeekValue(clamped);
  };

  const handleSeekCommit = (value: number) => {
    const clamped = Math.min(Math.max(value, 0), safeDuration);
    setSeeking(false);
    setSeekValue(clamped);
    onSeek(clamped);
  };

  const handleVolumeChange = (value: number) => {
    const clamped = Math.min(Math.max(value, 0), 1);
    onVolumeChange(clamped);
  };

  const cardStyle: React.CSSProperties = {
    borderRadius: 24,
    padding: 16,
    background:
      "linear-gradient(135deg, rgba(15,23,42,0.98), rgba(15,23,42,0.9))",
    boxShadow:
      "0 18px 40px rgba(15,23,42,0.30), 0 0 0 1px rgba(148,163,184,0.25)",
    display: "flex",
    flexDirection: "column",
    gap: 10,
    color: "#e5e7eb",
  };

  const timeRowStyle: React.CSSProperties = {
    display: "flex",
    justifyContent: "space-between",
    fontSize: 12,
    color: "#9ca3af",
    marginBottom: 6,
  };

  const progressRailStyle: React.CSSProperties = {
    position: "relative",
    width: "100%",
    height: 4,
    borderRadius: 999,
    background:
      "linear-gradient(90deg, rgba(30,64,175,0.4), rgba(148,163,184,0.25))",
    overflow: "hidden",
  };

  const progressThumbBaseStyle: React.CSSProperties = {
    position: "absolute",
    left: 0,
    top: 0,
    height: "100%",
    borderRadius: 999,
    background:
      "linear-gradient(90deg, #60a5fa, #a855f7, #ec4899, #f97316)",
    transformOrigin: "left center",
    transform: "scaleX(0)",
    transition: seeking ? "none" : "transform 120ms ease-out",
  };

  const sliderInputStyle: React.CSSProperties = {
    position: "absolute",
    left: 0,
    top: -4,
    width: "100%",
    height: 12,
    opacity: 0,
    cursor: "pointer",
  };

  const controlsRowStyle: React.CSSProperties = {
    marginTop: 12,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    gap: 12,
  };

  const playButtonsRowStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    gap: 8,
  };

  const volumeRowStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    gap: 6,
    minWidth: 160,
  };

  return (
    <div style={cardStyle}>
      {/* 时间 + 进度条 */}
      <div style={timeRowStyle}>
        <span>{formatTime(sliderValue)}</span>
        <span>{formatTime(safeDuration)}</span>
      </div>

      <div
        style={{
          position: "relative",
          width: "100%",
          borderRadius: 999,
          overflow: "hidden",
          background: "#020617",
          boxShadow:
            "0 12px 30px rgba(15,23,42,0.10), 0 0 0 1px rgba(226,232,240,0.9)",
        }}
      >
        <div style={progressRailStyle}>
          <div
            style={{
              ...progressThumbBaseStyle,
              transformOrigin: "left center",
              transform: `scaleX(${progress})`,
            }}
          />
        </div>
        <input
          type="range"
          min={0}
          max={safeDuration || 0}
          step={0.1}
          value={sliderValue}
          onMouseDown={handleSeekStart}
          onChange={(e) =>
            handleSeekChange(
              Number((e.target as HTMLInputElement).value) || 0,
            )
          }
          onMouseUp={(e) =>
            handleSeekCommit(
              Number((e.target as HTMLInputElement).value) || 0,
            )
          }
          style={sliderInputStyle}
        />
      </div>

      {/* 播放控制 + 音量 */}
      <div style={controlsRowStyle}>
        {/* 播放控制按钮 */}
        <div style={playButtonsRowStyle}>
          <button
            onClick={onPrev}
            style={{
              padding: "4px 10px",
              fontSize: 13,
              borderRadius: 9999,
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              cursor: "pointer",
            }}
          >
            {t("player.bar.tooltip.prev")}
          </button>
          <button
            onClick={onTogglePlay}
            style={{
              padding: "4px 16px",
              fontSize: 13,
              borderRadius: 9999,
              border: "1px solid #2563eb",
              background: isPlaying ? "#2563eb" : "#ffffff",
              color: isPlaying ? "#ffffff" : "#2563eb",
              cursor: "pointer",
            }}
          >
            {isPlaying
              ? t("player.bar.tooltip.pause")
              : t("player.bar.tooltip.play")}
          </button>
          <button
            onClick={onNext}
            style={{
              padding: "4px 10px",
              fontSize: 13,
              borderRadius: 9999,
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              cursor: "pointer",
            }}
          >
            {t("player.bar.tooltip.next")}
          </button>
        </div>

        {/* 音量 */}
        <div style={volumeRowStyle}>
          <span style={{ fontSize: 14 }}>🔊</span>
          <input
            type="range"
            min={0}
            max={1}
            step={0.01}
            value={volume}
            onChange={(e) =>
              handleVolumeChange(Number(e.target.value) || 0)
            }
            style={{ flex: 1 }}
          />
        </div>
      </div>
    </div>
  );
};

export default NowPlayingTransportCard;
