// src/components/playlists/PlaylistTableBody.tsx
import React from "react";
import type { PlayerTrack } from "../../types/track";
import { useI18n } from "../../i18n";
import {
  getTrackTitle,
  getTrackArtist,
  getTrackAlbum,
} from "../../library/libraryModel";

interface PlaylistTableBodyProps {
  tracks: PlayerTrack[];
  queueCurrentIndex: number;
  queueIndexByKey: Map<string, number>;
  makeIdentityKey: (track: any) => string;
  formatLastPlayed: (iso: string | null | undefined) => string;
  onRowDoubleClick: (track: PlayerTrack) => void;
  onRowContextMenu: (
    event: React.MouseEvent<HTMLTableRowElement>,
    track: PlayerTrack,
  ) => void;
  onPlayNext: (track: PlayerTrack) => void;
  onAppendToQueue: (track: PlayerTrack) => void;
}

const PlaylistTableBody: React.FC<PlaylistTableBodyProps> = ({
  tracks,
  queueCurrentIndex,
  queueIndexByKey,
  makeIdentityKey,
  formatLastPlayed,
  onRowDoubleClick,
  onRowContextMenu,
  onPlayNext,
  onAppendToQueue,
}) => {
  const { t } = useI18n();

  const safeTracks: PlayerTrack[] = Array.isArray(tracks)
    ? (tracks as PlayerTrack[])
    : [];

  // 空列表态
  if (safeTracks.length === 0) {
    return (
      <tbody>
        <tr>
          <td
            colSpan={8}
            style={{
              padding: "16px 12px",
              textAlign: "center",
              color: "#9ca3af",
            }}
          >
            {t("playlists.table.empty.currentTab")}
          </td>
        </tr>
      </tbody>
    );
  }

  return (
    <tbody>
      {safeTracks.map((track, index) => {
        const identity = makeIdentityKey(track);
        const filePath =
          (track as any).filePath ?? (track as any).path ?? "";
        const baseKey = identity || filePath || "track";
        // 确保 key 在列表内绝对唯一
        const key = `${baseKey}::${index}`;

        const queueIndex =
          identity && queueIndexByKey.has(identity)
            ? (queueIndexByKey.get(identity) as number)
            : -1;

        const isCurrent =
          queueIndex >= 0 && queueIndex === queueCurrentIndex;

        const title = getTrackTitle(track as any);
        const artist = getTrackArtist(track as any);
        const album = getTrackAlbum(track as any);
        const playCount = (track as any).playCount ?? 0;
        const lastPlayed = formatLastPlayed(
          (track as any).lastPlayedAt ?? null,
        );
        const isFavorite = !!(track as any).favorite;

        return (
          <tr
            key={key}
            onDoubleClick={() => onRowDoubleClick(track)}
            onContextMenu={(e) => onRowContextMenu(e, track)}
            style={{
              cursor: "pointer",
              backgroundColor: isCurrent
                ? "rgba(59,130,246,0.08)" // 当前播放高亮
                : index % 2 === 0
                ? "transparent"
                : "rgba(15,23,42,0.02)", // 轻微斑马纹
              transition: "background-color 120ms ease-out",
            }}
          >
            <td style={{ padding: "6px 12px", color: "#6b7280" }}>
              {index + 1}
            </td>
            <td
              style={{
                padding: "6px 12px",
                whiteSpace: "nowrap",
                textOverflow: "ellipsis",
                overflow: "hidden",
              }}
            >
              {title}
            </td>
            <td
              style={{
                padding: "6px 12px",
                whiteSpace: "nowrap",
                textOverflow: "ellipsis",
                overflow: "hidden",
              }}
            >
              {artist}
            </td>
            <td
              style={{
                padding: "6px 12px",
                whiteSpace: "nowrap",
                textOverflow: "ellipsis",
                overflow: "hidden",
              }}
            >
              {album}
            </td>
            <td
              style={{
                padding: "6px 12px",
                textAlign: "right",
              }}
            >
              {playCount}
            </td>
            <td
              style={{
                padding: "6px 12px",
                whiteSpace: "nowrap",
                textOverflow: "ellipsis",
                overflow: "hidden",
                color: "#6b7280",
              }}
            >
              {lastPlayed}
            </td>
            <td
              style={{
                padding: "6px 12px",
                textAlign: "center",
              }}
            >
              {isFavorite ? "★" : "☆"}
            </td>
            <td
              style={{
                padding: "6px 12px",
                whiteSpace: "nowrap",
              }}
            >
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  onPlayNext(track);
                }}
                style={{
                  borderWidth: 0,
                  padding: "2px 6px",
                  marginRight: 8,
                  borderRadius: 999,
                  fontSize: 12,
                  cursor: "pointer",
                  backgroundColor: "rgba(37,99,235,0.08)",
                }}
              >
                {t("playlists.table.row.action.playNext")}
              </button>
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  onAppendToQueue(track);
                }}
                style={{
                  borderWidth: 0,
                  padding: "2px 6px",
                  borderRadius: 999,
                  fontSize: 12,
                  cursor: "pointer",
                  backgroundColor: "rgba(15,23,42,0.04)",
                }}
              >
                {t(
                  "playlist.table.actions.appendToQueue",
                  "加入队列",
                )}
              </button>
            </td>
          </tr>
        );
      })}
    </tbody>
  );
};

export default PlaylistTableBody;
