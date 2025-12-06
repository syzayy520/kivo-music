// src/components/now-playing/NowPlayingCoverCard.tsx
import React from "react";
import { useNowPlayingCover } from "../../hooks/useNowPlayingCover";
import { kivoTheme } from "../../styles/theme";
import { KivoButton } from "../common/KivoButton";
import { useI18n } from "../../i18n";

interface NowPlayingCoverCardProps {
  // 这里 track 类型在全局还比较散，就先用 any，后面可以统一成 DomainTrack 类型
  track: any | null;
  playlist: any[];
  currentIndex: number;
}

// 这里只解构真正会用到的 theme 字段，避免 lint 报未使用
const { radius, shadow, spacing, colors } = kivoTheme;

const cardStyle: React.CSSProperties = {
  borderRadius: radius["2xl"],
  padding: spacing.lg,
  background: "#ffffff",
  boxShadow: shadow.card,
  display: "flex",
  flexDirection: "column",
  gap: spacing.md,
  height: "100%",
  boxSizing: "border-box",
};

const headerRowStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: spacing.sm,
};

const headerTextBlockStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: 4,
  minWidth: 0,
};

const headerTitleStyle: React.CSSProperties = {
  fontSize: 16,
  fontWeight: 600,
  color: colors.textOnLight,
};

const headerSubtitleStyle: React.CSSProperties = {
  fontSize: 12,
  color: colors.textMutedOnLight,
  lineHeight: 1.6,
};

const artworkWrapperStyle: React.CSSProperties = {
  marginTop: spacing.sm,
  borderRadius: radius.xl,
  overflow: "hidden",
  background:
    "radial-gradient(circle at top, #0f172a 0%, #020617 50%, #000000 100%)",
  flex: 1,
  minHeight: 0,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
};

const artworkImageStyle: React.CSSProperties = {
  width: "100%",
  height: "100%",
  objectFit: "cover",
};

const artworkPlaceholderStyle: React.CSSProperties = {
  padding: spacing.lg,
  textAlign: "center",
  color: colors.textMutedOnDark,
};

const artworkPlaceholderTitleStyle: React.CSSProperties = {
  fontSize: 14,
  fontWeight: 500,
  marginBottom: 4,
};

const artworkPlaceholderSubtitleStyle: React.CSSProperties = {
  fontSize: 12,
  lineHeight: 1.6,
};

const footerStatusRowStyle: React.CSSProperties = {
  marginTop: spacing.sm,
  fontSize: 11,
  color: "#9ca3af",
  display: "flex",
  justifyContent: "space-between",
};

const NowPlayingCoverCard: React.FC<NowPlayingCoverCardProps> = ({
  track,
  playlist,
  currentIndex,
}) => {
  const { t } = useI18n();

  const { coverSrc, isLoading, hasError, pickCover } = useNowPlayingCover(
    track,
    playlist,
    currentIndex,
  );

  const anyTrack = track as any;
  const hasTrack = !!anyTrack;
  const canPickCover = hasTrack && !isLoading;

  const handlePickCoverClick = async () => {
    if (!canPickCover) return;
    try {
      await pickCover();
    } catch (err) {
      console.error("[NowPlayingCoverCard] pickCover failed:", err);
    }
  };

  return (
    <div style={cardStyle}>
      {/* 标题 + 按钮行 */}
      <div style={headerRowStyle}>
        <div style={headerTextBlockStyle}>
          <div style={headerTitleStyle}>{t("nowPlaying.cover.title")}</div>
          <div style={headerSubtitleStyle}>
            {t("nowPlaying.cover.subtitle")}
          </div>
        </div>
        <KivoButton
          variant="secondary"
          size="sm"
          onClick={handlePickCoverClick}
          disabled={!canPickCover}
        >
          {t("nowPlaying.cover.button.change")}
        </KivoButton>
      </div>

      {/* 封面展示区 */}
      <div style={artworkWrapperStyle}>
        {coverSrc ? (
          <img
            src={coverSrc}
            alt={t("nowPlaying.cover.alt")}
            style={artworkImageStyle}
          />
        ) : (
          <div style={artworkPlaceholderStyle}>
            <div style={artworkPlaceholderTitleStyle}>
              {t("nowPlaying.cover.placeholder.title")}
            </div>
            <div style={artworkPlaceholderSubtitleStyle}>
              {hasTrack
                ? t("nowPlaying.cover.placeholder.subtitle.withTrack")
                : t("nowPlaying.cover.placeholder.subtitle.noTrack")}
            </div>
          </div>
        )}
      </div>

      {/* 状态提示 */}
      <div style={footerStatusRowStyle}>
        <span>
          {isLoading
            ? t("nowPlaying.cover.status.loading")
            : t("nowPlaying.cover.status.idle")}
        </span>
        {hasError && (
          <span style={{ color: "#b91c1c" }}>
            {t("nowPlaying.cover.status.error")}
          </span>
        )}
      </div>
    </div>
  );
};

export default NowPlayingCoverCard;
