// src/components/PlayerBar.tsx
import React, { useCallback } from "react";
import { usePlayerStore } from "../store/player";
import { useKivoTheme } from "../styles/ThemeContext";

function formatTime(sec: number): string {
  if (!sec || !Number.isFinite(sec)) return "0:00";
  const s = Math.floor(sec);
  const m = Math.floor(s / 60);
  const rest = s % 60;
  return `${m}:${rest.toString().padStart(2, "0")}`;
}

/**
 * PlayerBar
 *
 * 全局底部播放器控制条：
 * - 显示当前播放进度 / 时长；
 * - 当前曲目信息；
 * - 上一首 / 播放 / 下一首；
 * - 音量调节。
 *
 * 样式完全走 theme，方便后续多皮肤切换。
 */
export const PlayerBar: React.FC = () => {
  const { theme } = useKivoTheme();

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
    currentIndex != null &&
    currentIndex >= 0 &&
    Array.isArray(playlist) &&
    currentIndex < playlist.length
      ? playlist[currentIndex]
      : undefined;

  const handleTogglePlay = useCallback(() => {
    if (!playlist || !Array.isArray(playlist) || playlist.length === 0) return;

    if (currentIndex == null || currentIndex < 0) {
      playTrack(0);
      return;
    }

    togglePlay();
  }, [playlist, currentIndex, playTrack, togglePlay]);

  const handleSeekChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const v = Number(e.target.value);
      if (!Number.isFinite(v)) return;
      seek(v);
    },
    [seek],
  );

  const handleVolumeChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const v = Number(e.target.value);
      if (!Number.isFinite(v)) return;
      setVolume(v);
    },
    [setVolume],
  );

  const safeDuration =
    Number.isFinite(duration) && duration > 0 ? duration : 0;
  const safePosition =
    Number.isFinite(currentTime) && currentTime > 0
      ? Math.min(currentTime, safeDuration || 0)
      : 0;

  const barStyle: React.CSSProperties = {
    display: "flex",
    flexDirection: "column",
    gap: theme.spacing.xs,
    fontSize: 12,
    padding: `${theme.spacing.sm}px ${theme.spacing.lg}px`,
    background: "rgba(15, 23, 42, 0.96)",
    color: theme.colors.textOnDark,
    backdropFilter: "blur(16px)",
  };

  const metaTextStyle: React.CSSProperties = {
    fontSize: 11,
    color: theme.colors.textMutedOnDark,
  };

  const controlButtonStyle: React.CSSProperties = {
    border: "none",
    background: "transparent",
    cursor: "pointer",
    fontSize: 16,
    padding: "0 4px",
    color: theme.colors.textOnDark,
  };

  return (
    <div style={barStyle}>
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
        <span style={{ minWidth: 40, textAlign: "left" }}>
          {formatTime(safeDuration)}
        </span>
      </div>

      {/* 歌曲信息 + 播放控制 + 音量 */}
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        {/* 歌曲信息 */}
        <div style={{ flex: 1, minWidth: 0 }}>
          <div
            style={{
              fontSize: 13,
              fontWeight: 500,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {currentTrack ? currentTrack.title : "未选择歌曲"}
          </div>
          <div
            style={{
              ...metaTextStyle,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
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
          <button
            onClick={prev}
            style={controlButtonStyle}
            title="上一首"
            type="button"
          >
            ⏮
          </button>
          <button
            onClick={handleTogglePlay}
            style={{ ...controlButtonStyle, fontSize: 18 }}
            title={isPlaying ? "暂停" : "播放"}
            type="button"
          >
            {isPlaying ? "⏸" : "▶️"}
          </button>
          <button
            onClick={next}
            style={controlButtonStyle}
            title="下一首"
            type="button"
          >
            ⏭
          </button>
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
