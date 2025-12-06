// src/components/PlayerBar.tsx
import React, { useCallback, useMemo } from "react";
import { usePlayerStore } from "../store/player";
import { useKivoTheme } from "../styles/ThemeContext";
import { useI18n } from "../i18n";

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
 * - 展示当前播放进度 / 总时长
 * - 展示当前曲目标题 & 艺人
 * - 提供上一首 / 播放暂停 / 下一首
 * - 提供音量调节
 *
 * 注意：所有实际播放行为仍通过 player store（AudioEngine）统一处理，
 * 这里不直接接触 audio 元素。
 */
export const PlayerBar: React.FC = () => {
  const { t } = useI18n();
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

  const currentTrack = useMemo(() => {
    if (!Array.isArray(playlist) || playlist.length === 0) return null;
    if (
      currentIndex == null ||
      currentIndex < 0 ||
      currentIndex >= playlist.length
    ) {
      return playlist[0];
    }
    return playlist[currentIndex];
  }, [playlist, currentIndex]);

  const handleTogglePlay = useCallback(() => {
    if (!Array.isArray(playlist) || playlist.length === 0) return;

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

  const displayTitle =
    (currentTrack && (currentTrack as any).title) ||
    t("player.bar.fallbackTitle");
  const displayArtist = (() => {
    if (!currentTrack) {
      return t("player.bar.fallbackArtist");
    }
    const rawArtist =
      typeof (currentTrack as any).artist === "string"
        ? ((currentTrack as any).artist as string)
        : "";
    const trimmed = rawArtist.trim();
    if (!trimmed || trimmed === "未知艺人") {
      return t("player.bar.fallbackArtist");
    }
    return trimmed;
  })();

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
            {displayTitle}
          </div>
          <div
            style={{
              ...metaTextStyle,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {displayArtist}
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
            title={t("player.bar.tooltip.prev")}
            aria-label={t("player.bar.tooltip.prev")}
            type="button"
          >
            ⏮
          </button>
          <button
            onClick={handleTogglePlay}
            style={{ ...controlButtonStyle, fontSize: 18 }}
            title={
              isPlaying
                ? t("player.bar.tooltip.pause")
                : t("player.bar.tooltip.play")
            }
            aria-label={
              isPlaying
                ? t("player.bar.tooltip.pause")
                : t("player.bar.tooltip.play")
            }
            type="button"
          >
            {isPlaying ? "⏸" : "▶️"}
          </button>
          <button
            onClick={next}
            style={controlButtonStyle}
            title={t("player.bar.tooltip.next")}
            aria-label={t("player.bar.tooltip.next")}
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
