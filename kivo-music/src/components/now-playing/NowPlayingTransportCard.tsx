// src/components/now-playing/NowPlayingTransportCard.tsx
import React, { useState } from "react";

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
  const [seeking, setSeeking] = useState(false);
  const [seekValue, setSeekValue] = useState(0);

  const safeDuration = duration > 0 ? duration : 0;
  const safeCurrent = Math.min(
    Math.max(currentTime, 0),
    safeDuration || Number.MAX_VALUE,
  );
  const sliderValue = seeking ? seekValue : safeCurrent;

  const handleSeekChange = (value: number) => {
    setSeeking(true);
    setSeekValue(value);
  };

  const handleSeekCommit = (value: number) => {
    setSeeking(false);
    onSeek(value);
  };

  const handleVolumeChange = (value: number) => {
    onVolumeChange(value);
  };

  return (
    <div
      style={{
        borderRadius: 16,
        padding: 16,
        background: "#ffffff",
        boxShadow:
          "0 12px 30px rgba(15,23,42,0.10), 0 0 0 1px rgba(226,232,240,0.9)",
      }}
    >
      {/* 时间 + 进度条 */}
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          fontSize: 12,
          color: "#6b7280",
          marginBottom: 4,
        }}
      >
        <span>{formatTime(sliderValue)}</span>
        <span>{formatTime(safeDuration)}</span>
      </div>
      <input
        type="range"
        min={0}
        max={safeDuration || 0}
        step={0.1}
        value={sliderValue}
        onChange={(e) => handleSeekChange(Number(e.target.value) || 0)}
        onMouseUp={(e) =>
          handleSeekCommit(Number((e.target as HTMLInputElement).value) || 0)
        }
        onTouchEnd={(e) =>
          handleSeekCommit(Number((e.target as HTMLInputElement).value) || 0)
        }
        style={{
          width: "100%",
          cursor: "pointer",
        }}
      />

      {/* 播放控制 + 音量 */}
      <div
        style={{
          marginTop: 12,
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: 12,
        }}
      >
        {/* 播放控制按钮 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 8,
          }}
        >
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
            上一首
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
            {isPlaying ? "暂停" : "播放"}
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
            下一首
          </button>
        </div>

        {/* 音量 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 6,
            minWidth: 160,
          }}
        >
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
