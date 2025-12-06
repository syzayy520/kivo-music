// src/components/MiniPlayer.tsx
import React from "react";
import { usePlayerStore } from "../store/player";
import { useNowPlayingCover } from "../hooks/useNowPlayingCover";
import { useI18n } from "../i18n";

interface MiniPlayerProps {
  onExitMiniMode?: () => void;
}

const MiniPlayer: React.FC<MiniPlayerProps> = ({ onExitMiniMode }) => {
  const { t } = useI18n();

  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);
  const currentTime = usePlayerStore((s) => s.currentTime);
  const duration = usePlayerStore((s) => s.duration);
  const volume = usePlayerStore((s) => s.volume);

  const togglePlay = usePlayerStore((s) => s.togglePlay);
  const next = usePlayerStore((s) => s.next);
  const prev = usePlayerStore((s) => s.prev);
  const setVolume = usePlayerStore((s) => s.setVolume);
  const playTrack = usePlayerStore((s) => s.playTrack);

  const track = Array.isArray(playlist) ? playlist[currentIndex] ?? null : null;
  const { coverSrc } = useNowPlayingCover(track, playlist, currentIndex);

  const safeProgress =
    duration && duration > 0
      ? Math.min(1, Math.max(0, currentTime / duration))
      : 0;

  const anyTrack = track as any;
  const title =
    anyTrack?.title ??
    anyTrack?.name ??
    anyTrack?.fileName ??
    t("player.bar.fallbackTitle");
  const artist =
    anyTrack?.artist ??
    anyTrack?.albumArtist ??
    t("player.bar.fallbackArtist");
  const album = anyTrack?.album ?? "";

  const handleTogglePlay = () => {
    if (!Array.isArray(playlist) || playlist.length === 0) return;

    if (currentIndex == null || currentIndex < 0) {
      // If no track is selected yet, start from the first track
      playTrack(0);
      return;
    }

    togglePlay();
  };

  return (
    <div
      style={{
        flex: 1,
        display: "flex",
        flexDirection: "column",
        padding: 16,
        gap: 12,
      }}
    >
      {/* Header: title and exit button */}
      <header
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          marginBottom: 8,
        }}
      >
        <div>
          <div style={{ fontSize: 14, fontWeight: 600 }}>
            {t("miniPlayer.header.title")}
          </div>
          <div style={{ fontSize: 11, color: "#9ca3af" }}>
            {track
              ? t("miniPlayer.header.subtitle.hasTrack")
              : t("miniPlayer.header.subtitle.noTrack")}
          </div>
        </div>
        {onExitMiniMode && (
          <button
            onClick={onExitMiniMode}
            style={{
              padding: "4px 10px",
              fontSize: 12,
              borderRadius: 9999,
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              color: "#111827",
              cursor: "pointer",
              whiteSpace: "nowrap",
            }}
            type="button"
          >
            {t("miniPlayer.button.exit")}
          </button>
        )}
      </header>

      {/* Artwork and basic track info */}
      <div
        style={{
          flex: 1,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          gap: 12,
        }}
      >
        <div
          style={{
            width: 240,
            height: 240,
            borderRadius: 24,
            overflow: "hidden",
            background: "#e5e7eb",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          {coverSrc ? (
            <img
              src={coverSrc}
              alt={t("nowPlaying.cover.alt")}
              style={{
                width: "100%",
                height: "100%",
                objectFit: "cover",
                display: "block",
              }}
            />
          ) : (
            <span style={{ fontSize: 12, color: "#6b7280" }}>
              {t("miniPlayer.cover.placeholder")}
            </span>
          )}
        </div>

        <div style={{ textAlign: "center" }}>
          <div
            style={{
              fontSize: 14,
              fontWeight: 600,
              marginBottom: 2,
              maxWidth: 260,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {title}
          </div>
          <div
            style={{
              fontSize: 12,
              color: "#6b7280",
              maxWidth: 260,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {artist}
            {album ? ` · ${album}` : ""}
          </div>
        </div>
      </div>

      {/* Progress, transport controls, and volume */}
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 8,
        }}
      >
        {/* Progress bar (read-only display) */}
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
              width: `${safeProgress * 100}%`,
              height: "100%",
              background: "#3b82f6",
              transition: "width 0.1s linear",
            }}
          />
        </div>

        {/* Transport controls */}
        <div
          style={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            gap: 16,
            marginTop: 4,
          }}
        >
          <button
            onClick={prev}
            style={{
              width: 36,
              height: 36,
              borderRadius: "9999px",
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              cursor: "pointer",
              fontSize: 14,
            }}
            title={t("player.bar.tooltip.prev")}
            aria-label={t("player.bar.tooltip.prev")}
            type="button"
          >
            ⏮
          </button>
          <button
            onClick={handleTogglePlay}
            style={{
              width: 40,
              height: 40,
              borderRadius: "9999px",
              border: "1px solid #e5e7eb",
              background: "#111827",
              color: "#f9fafb",
              cursor: "pointer",
              fontSize: 16,
            }}
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
            {isPlaying ? "⏸" : "▶"}
          </button>
          <button
            onClick={next}
            style={{
              width: 36,
              height: 36,
              borderRadius: "9999px",
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              cursor: "pointer",
              fontSize: 14,
            }}
            title={t("player.bar.tooltip.next")}
            aria-label={t("player.bar.tooltip.next")}
            type="button"
          >
            ⏭
          </button>
        </div>

        {/* Volume slider: 0–1, shared with the main PlayerBar store */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 8,
            marginTop: 6,
          }}
        >
          <span style={{ fontSize: 11, color: "#6b7280" }}>
            {t("miniPlayer.volume.label")}
          </span>
          <input
            type="range"
            min={0}
            max={1}
            step={0.01}
            value={volume}
            onChange={(e) => {
              const v = parseFloat(e.target.value);
              if (!Number.isNaN(v)) setVolume(v);
            }}
            style={{ flex: 1 }}
          />
        </div>
      </div>
    </div>
  );
};

export default MiniPlayer;
