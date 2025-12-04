// src/components/library/LibraryTrackRow.tsx
import React from "react";
import {
  LibraryTrack,
  getTrackTitle,
  getTrackArtist,
  getTrackAlbum,
  toggleFavorite,
} from "../../library/libraryModel";

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
 * 单行曲目展示行：
 * - 双击播放
 * - ♥ 喜欢
 * - “下一首 / 加入队列”按钮
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
  const title = getTrackTitle(track);
  const artist = getTrackArtist(track);
  const album = getTrackAlbum(track);
  const playCount = (track as any).playCount ?? 0;
  const lastPlayedLabel = formatLastPlayed((track as any).lastPlayedAt);
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
      <td
        style={{
          padding: "6px 10px",
          maxWidth: 260,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
          borderBottom: "1px solid #f3f4f6",
        }}
      >
        {title}
      </td>
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
        {album || "-"}
      </td>
      <td
        style={{
          padding: "6px 10px",
          textAlign: "right",
          borderBottom: "1px solid #f3f4f6",
          fontVariantNumeric: "tabular-nums",
          color: "#4b5563",
        }}
      >
        {playCount}
      </td>
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
          title={favorite ? "取消喜欢" : "标记为喜欢"}
        >
          {favorite ? "♥" : "♡"}
        </button>
      </td>
      <td
        style={{
          padding: "6px 10px",
          textAlign: "center",
          borderBottom: "1px solid #f3f4f6",
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
          title="设为下一首播放"
        >
          下一首
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
          title="添加到当前队列末尾"
        >
          加入队列
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
