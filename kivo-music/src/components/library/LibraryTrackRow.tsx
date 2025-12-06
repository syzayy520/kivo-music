// src/components/library/LibraryTrackRow.tsx
import React from "react";
import {
  LibraryTrack,
  getTrackTitle,
  getTrackArtist,
  getTrackAlbum,
  toggleFavorite,
} from "../../library/libraryModel";
import { useI18n } from "../../i18n";

interface LibraryTrackRowProps {
  track: LibraryTrack;
  index: number;
  isCurrent: boolean;
  onPlay: (track: LibraryTrack, index: number) => void;
  onPlayNext: (track: LibraryTrack) => void;
  onAppendToQueue: (track: LibraryTrack) => void;
  onContextMenu: (
    event: React.MouseEvent<HTMLTableRowElement>,
    track: LibraryTrack,
    index: number,
  ) => void;
}

/**
 * 单行曲目：
 * - 双击播放
 * - 喜欢 ♥
 * - “下一首 / 加入队列”按钮（走 i18n 文案）
 * - 行右键触发外部的 context menu
 */
export const LibraryTrackRow: React.FC<LibraryTrackRowProps> = ({
  track,
  index,
  isCurrent,
  onPlay,
  onPlayNext,
  onAppendToQueue,
  onContextMenu,
}) => {
  const { t } = useI18n();

  const rawTitle = getTrackTitle(track);
  const rawArtist = getTrackArtist(track);
  const rawAlbum = getTrackAlbum(track);

  const title =
    rawTitle && rawTitle.trim().length > 0
      ? rawTitle
      : t("library.tracks.fallbackTitle");
  const artist =
    rawArtist && rawArtist.trim().length > 0
      ? rawArtist
      : t("library.tracks.fallbackArtist");
  const album =
    rawAlbum && rawAlbum.trim().length > 0
      ? rawAlbum
      : t("library.tracks.fallbackAlbum");

  const playCount = (track as any).playCount ?? 0;
  const lastPlayedLabel = formatLastPlayed(
    (track as any).lastPlayedAt ?? null,
  );
  const favorite = !!(track as any).favorite;

  const handleDoubleClick = () => {
    onPlay(track, index);
  };

  const handleToggleFavorite = (e: React.MouseEvent) => {
    e.stopPropagation();
    toggleFavorite(track);
  };

  const handlePlayNextClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    onPlayNext(track);
  };

  const handleAppendClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    onAppendToQueue(track);
  };

  return (
    <tr
      onDoubleClick={handleDoubleClick}
      onContextMenu={(e) => onContextMenu(e, track, index)}
      style={{
        backgroundColor: isCurrent
          ? "rgba(37,99,235,0.06)"
          : index % 2 === 0
          ? "#ffffff"
          : "#f9fafb",
        cursor: "pointer",
        transition: "background-color 0.12s ease-out",
      }}
    >
      {/* 序号 */}
      <td
        style={{
          padding: "6px 10px",
          textAlign: "right",
          fontVariantNumeric: "tabular-nums",
          color: isCurrent ? "#2563eb" : "#6b7280",
          borderBottom: "1px solid #f3f4f6",
        }}
      >
        {index + 1}
      </td>

      {/* 标题 */}
      <td
        style={{
          padding: "6px 10px",
          maxWidth: 240,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
          borderBottom: "1px solid #f3f4f6",
          color: "#111827",
        }}
      >
        {title}
      </td>

      {/* 艺人 */}
      <td
        style={{
          padding: "6px 10px",
          maxWidth: 160,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
          borderBottom: "1px solid #f3f4f6",
          color: "#4b5563",
        }}
      >
        {artist}
      </td>

      {/* 专辑 */}
      <td
        style={{
          padding: "6px 10px",
          maxWidth: 180,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
          borderBottom: "1px solid #f3f4f6",
          color: "#6b7280",
        }}
      >
        {album}
      </td>

      {/* 播放次数 */}
      <td
        style={{
          padding: "6px 10px",
          textAlign: "right",
          fontVariantNumeric: "tabular-nums",
          borderBottom: "1px solid #f3f4f6",
          color: "#4b5563",
        }}
      >
        {playCount}
      </td>

      {/* 最近播放时间 */}
      <td
        style={{
          padding: "6px 10px",
          borderBottom: "1px solid #f3f4f6",
          fontSize: 12,
          color: "#6b7280",
          whiteSpace: "nowrap",
        }}
      >
        {lastPlayedLabel}
      </td>

      {/* 喜欢 ♥ */}
      <td
        style={{
          padding: "6px 10px",
          textAlign: "center",
          borderBottom: "1px solid #f3f4f6",
        }}
      >
        <button
          onClick={handleToggleFavorite}
          style={{
            border: "none",
            backgroundColor: "transparent",
            cursor: "pointer",
            fontSize: 16,
            color: favorite ? "#f97316" : "#d1d5db",
          }}
          title={
            favorite
              ? t("library.tracks.favorite.tooltip.on")
              : t("library.tracks.favorite.tooltip.off")
          }
        >
          {favorite ? "♥" : "♡"}
        </button>
      </td>

      {/* 行内操作按钮：下一首 / 加入队列 */}
      <td
        style={{
          padding: "6px 10px",
          borderBottom: "1px solid #f3f4f6",
          fontSize: 12,
          color: "#2563eb",
          whiteSpace: "nowrap",
        }}
      >
        <button
          onClick={handlePlayNextClick}
          style={{
            border: "none",
            backgroundColor: "transparent",
            cursor: "pointer",
            fontSize: 12,
            color: "#2563eb",
            marginRight: 8,
          }}
          title={t("library.contextMenu.playNext")}
        >
          {t("library.contextMenu.playNext")}
        </button>

        <button
          onClick={handleAppendClick}
          style={{
            border: "none",
            backgroundColor: "transparent",
            cursor: "pointer",
            fontSize: 12,
            color: "#6b7280",
          }}
          title={t("library.contextMenu.appendToQueue")}
        >
          {t("library.contextMenu.appendToQueue")}
        </button>
      </td>
    </tr>
  );
};

function formatLastPlayed(iso: string | null | undefined): string {
  if (!iso) return "-";
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return "-";

  const y = date.getFullYear();
  const m = date.getMonth() + 1;
  const d = date.getDate();
  const hh = String(date.getHours()).padStart(2, "0");
  const mm = String(date.getMinutes()).padStart(2, "0");
  const ss = String(date.getSeconds()).padStart(2, "0");

  return `${y}/${m}/${d} ${hh}:${mm}:${ss}`;
}
