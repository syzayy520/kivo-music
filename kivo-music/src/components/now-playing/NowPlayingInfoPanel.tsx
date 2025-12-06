// src/components/now-playing/NowPlayingInfoPanel.tsx
import React from "react";
import { useI18n } from "../../i18n";

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

const titleStyle: React.CSSProperties = {
  fontSize: 16,
  fontWeight: 600,
  marginBottom: 4,
};

const metaStyle: React.CSSProperties = {
  fontSize: 13,
  color: "#6b7280",
};

const filePathStyle: React.CSSProperties = {
  fontSize: 11,
  color: "#9ca3af",
  wordBreak: "break-all",
  marginTop: 4,
};

const progressRailStyle: React.CSSProperties = {
  position: "relative",
  height: 4,
  borderRadius: 9999,
  background: "#e5e7eb",
  overflow: "hidden",
};

const progressThumbBaseStyle: React.CSSProperties = {
  position: "absolute",
  inset: 0,
  borderRadius: 9999,
  background:
    "linear-gradient(90deg, rgba(59,130,246,1) 0%, rgba(96,165,250,1) 100%)",
};

const timeRowStyle: React.CSSProperties = {
  marginTop: 4,
  display: "flex",
  justifyContent: "space-between",
  fontSize: 11,
  color: "#6b7280",
};

const statusCardStyle: React.CSSProperties = {
  marginTop: 12,
  padding: 10,
  borderRadius: 12,
  background: "#f9fafb",
  fontSize: 11,
  color: "#6b7280",
  lineHeight: 1.6,
};

const footerStatusStyle: React.CSSProperties = {
  marginTop: "auto",
  fontSize: 11,
  color: "#9ca3af",
  display: "flex",
  justifyContent: "space-between",
};

const formatTime = (sec: number): string => {
  if (!Number.isFinite(sec) || sec <= 0) return "00:00";
  const s = Math.floor(sec);
  const m = Math.floor(s / 60);
  const r = s % 60;
  const mm = String(m).padStart(2, "0");
  const ss = String(r).padStart(2, "0");
  return `${mm}:${ss}`;
};

const NowPlayingInfoPanel: React.FC<NowPlayingInfoPanelProps> = ({
  track,
  isPlaying,
  currentTime,
  duration,
}) => {
  const { t } = useI18n();
  const anyTrack = track as any;

  const rawTitle =
    anyTrack?.title ??
    anyTrack?.name ??
    anyTrack?.fileName ??
    "";

  const rawArtist =
    anyTrack?.artist ??
    anyTrack?.albumArtist ??
    "";

  const rawAlbum = anyTrack?.album ?? "";

  const normalizedArtist = (() => {
    const trimmed =
      typeof rawArtist === "string" ? rawArtist.trim() : "";
    if (!trimmed) return "";
    if (trimmed === "未知艺人") return "";
    return trimmed;
  })();

  const normalizedAlbum = (() => {
    const trimmed =
      typeof rawAlbum === "string" ? rawAlbum.trim() : "";
    if (!trimmed) return "";
    if (trimmed === "未分专辑" || trimmed === "未知专辑") return "";
    return trimmed;
  })();

  const title =
    rawTitle && String(rawTitle).trim().length > 0
      ? String(rawTitle)
      : t("nowPlaying.info.fallbackTitle");

  const artist =
    normalizedArtist && normalizedArtist.length > 0
      ? normalizedArtist
      : t("nowPlaying.info.fallbackArtist");

  const album =
    normalizedAlbum && normalizedAlbum.length > 0
      ? normalizedAlbum
      : "";

  const filePath =
    anyTrack?.filePath ?? anyTrack?.path ?? anyTrack?.location ?? "";
  const safeDuration =
    typeof duration === "number" && Number.isFinite(duration) && duration > 0
      ? duration
      : 0;
  const safeCurrent =
    typeof currentTime === "number" &&
    Number.isFinite(currentTime) &&
    currentTime > 0 &&
    safeDuration > 0
      ? Math.min(currentTime, safeDuration)
      : 0;
  const progress =
    safeDuration > 0 ? Math.min(1, safeCurrent / safeDuration) : 0;

  const statusText = !track
    ? t("nowPlaying.info.status.noTrack")
    : isPlaying
    ? t("nowPlaying.info.status.playing")
    : t("nowPlaying.info.status.paused");

  return (
    <div style={boxStyle}>
      <div>
        <div style={titleStyle}>{title}</div>
        <div style={metaStyle}>
          {artist}
          {album ? ` · ${album}` : ""}
        </div>
      </div>

      {filePath && <div style={filePathStyle}>{filePath}</div>}

      {/* 进度条 */}
      <div style={{ marginTop: 12 }}>
        <div style={progressRailStyle}>
          <div
            style={{
              ...progressThumbBaseStyle,
              transformOrigin: "left center",
              transform: `scaleX(${progress})`,
            }}
          />
        </div>
        <div style={timeRowStyle}>
          <span>{formatTime(safeCurrent)}</span>
          <span>{formatTime(safeDuration)}</span>
        </div>
      </div>

      {/* 底部状态说明卡片 */}
      <div style={statusCardStyle}>
        {t("nowPlaying.info.description.line1")}
        <br />
        {t("nowPlaying.info.description.line2")}
      </div>

      <div style={footerStatusStyle}>
        <span>
          {t("nowPlaying.info.statusLabel")}
          {statusText}
        </span>
        <span>{t("nowPlaying.info.footerTag")}</span>
      </div>
    </div>
  );
};

export default NowPlayingInfoPanel;
