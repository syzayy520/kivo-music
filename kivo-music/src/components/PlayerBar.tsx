import React, { useCallback } from "react";
import { usePlayerStore } from "../store/player";
import { kivoTheme } from "../styles/theme";

function formatTime(sec: number): string {
  if (!sec || !Number.isFinite(sec)) return "0:00";
  const s = Math.floor(sec);
  const m = Math.floor(s / 60);
  const rest = s % 60;
  return `${m}:${rest.toString().padStart(2, "0")}`;
}

export const PlayerBar: React.FC = () => {
  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);
  const currentTime = usePlayerStore((s) => s.currentTime);
  const duration = usePlayerStore((s) => s.duration);
  const volume = usePlayerStore((s) => s.volume);

  const prev = usePlayerStore((s) => s.prev);
  const next = usePlayerStore((s) => s.next);
  const playTrack = usePlayerStore((s) => s.playTrack);
  const togglePlay = usePlayerStore((s) => s.togglePlay);
  const seek = usePlayerStore((s) => s.seek);
  const setVolume = usePlayerStore((s) => s.setVolume);

  const currentTrack =
    currentIndex != null && currentIndex >= 0 && playlist
      ? playlist[currentIndex]
      : undefined;

  const handleTogglePlay = useCallback(() => {
    if (!playlist || playlist.length === 0) return;

    if (currentIndex == null || currentIndex < 0) {
      playTrack(0);
      return;
    }

    togglePlay();
  }, [playlist, currentIndex, playTrack, togglePlay]);

  const handleSeekChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const v = Number(e.target.value);
    if (!Number.isFinite(v)) return;
    seek(v);
  };

  const handleVolumeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const v = Number(e.target.value);
    if (!Number.isFinite(v)) return;
    setVolume(v);
  };

  const safeDuration = Number.isFinite(duration) && duration > 0 ? duration : 0;
  const safePosition =
    Number.isFinite(currentTime) && currentTime > 0
      ? Math.min(currentTime, safeDuration)
      : 0;

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: kivoTheme.spacing.xs,
        fontSize: 12,
        padding: `${kivoTheme.spacing.sm}px ${kivoTheme.spacing.lg}px`,
        borderTop: `1px solid ${kivoTheme.colors.borderSubtle}`,
        background: "rgba(15, 23, 42, 0.96)",
        color: kivoTheme.colors.textOnDark,
        backdropFilter: "blur(16px)",
      }}
    >
      {/* 进度条 */}
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <span style={{ minWidth: 40, textAlign: "right" }}>
          {formatTime(safePosition)}
        </span>
        <input
          type="range"
          min={0}
          max={safeDuration || 0}
          step={0.1}
          value={safePosition}
          onChange={handleSeekChange}
          style={{ flex: 1 }}
        />
        <span style={{ minWidth: 40 }}>{formatTime(safeDuration)}</span>
      </div>

      {/* 歌曲信息 + 播放控制 + 音量 */}
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        {/* 歌曲信息 */}
        <div style={{ flex: 1, minWidth: 0 }}>
          <div style={{ fontSize: 13 }}>
            {currentTrack ? currentTrack.title : "未选择歌曲"}
          </div>
          <div style={{ fontSize: 11, color: "#6b7280" }}>
            {currentTrack ? currentTrack.artist ?? "未知艺人" : "未知艺人"}
          </div>
        </div>

        {/* 播放按钮 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 4,
          }}
        >
          <button onClick={prev}>⏮</button>
          <button
            onClick={handleTogglePlay}
            style={{ fontSize: 14, padding: "0 6px" }}
          >
            {isPlaying ? "⏸" : "▶️"}
          </button>
          <button onClick={next}>⏭</button>
        </div>

        {/* 音量 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 4,
            minWidth: 140,
          }}
        >
          <span style={{ fontSize: 14 }}>🔊</span>
          <input
            type="range"
            min={0}
            max={1}
            step={0.01}
            value={volume}
            onChange={handleVolumeChange}
            style={{ flex: 1 }}
          />
        </div>
      </div>
    </div>
  );
};

export default PlayerBar;
