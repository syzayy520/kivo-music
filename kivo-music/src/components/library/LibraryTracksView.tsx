import React from "react";
import {
  LibraryTrack,
  getTrackTitle,
  getTrackArtist,
  getTrackAlbum,
  toggleFavorite,
} from "../../library/libraryModel";
import { usePlayerStore, PlayerTrack } from "../../store/player";
import { appendToQueue, playNext } from "../../playlists/playQueueModel";

interface Props {
  tracks: LibraryTrack[];
  onPlayTrack: (track: LibraryTrack, index: number) => void;
}

/**
 * 本地资料库 - “按歌曲”视图。
 *
 * 只做两件事：
 * 1. 把已经过滤 / 排好序的 tracks 画成表格；
 * 2. 行级操作：双击播放、喜欢、加入队列 / 设为下一首。
 */
export const LibraryTracksView: React.FC<Props> = ({ tracks, onPlayTrack }) => {
  const currentPlaylist = usePlayerStore(
    (s: any) => s.playlist ?? s.tracks ?? [],
  );
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);

  const currentTrack =
    Array.isArray(currentPlaylist) &&
    currentIndex >= 0 &&
    currentIndex < currentPlaylist.length
      ? (currentPlaylist[currentIndex] as any)
      : null;

  const currentFilePath: string | undefined =
    currentTrack?.filePath ?? currentTrack?.path ?? currentTrack?.location;

  const handleRowDoubleClick = (track: LibraryTrack, index: number) => {
    onPlayTrack(track, index);
  };

  const handleToggleFavorite = (track: LibraryTrack) => {
    toggleFavorite(track);
  };

  const handlePlayNext = (track: LibraryTrack) => {
    const asPlayerTrack = track as unknown as PlayerTrack;
    playNext([asPlayerTrack]);
  };

  const handleAppendToQueue = (track: LibraryTrack) => {
    const asPlayerTrack = track as unknown as PlayerTrack;
    appendToQueue([asPlayerTrack]);
  };

  const formatLastPlayed = (iso: string | null | undefined): string => {
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
  };

  const headerCellBase: React.CSSProperties = {
    padding: "8px 10px",
    textAlign: "left",
    fontWeight: 500,
    fontSize: 12,
    color: "#6b7280",
    borderBottom: "1px solid #e5e7eb",
    whiteSpace: "nowrap",
  };

  return (
    <div
      style={{
        flex: 1,
        minHeight: 0,
        overflow: "auto",
        background: "#ffffff",
        borderRadius: 16,
        boxShadow: "0 8px 24px rgba(15,23,42,0.08)",
      }}
    >
      <table
        style={{
          width: "100%",
          borderCollapse: "collapse",
          fontSize: 13,
        }}
      >
        <thead>
          <tr
            style={{
              background: "#f9fafb",
            }}
          >
            <th
              style={{
                ...headerCellBase,
                width: 48,
                textAlign: "right",
                paddingRight: 12,
              }}
            >
              #
            </th>
            <th style={headerCellBase}>标题</th>
            <th style={headerCellBase}>艺人</th>
            <th style={headerCellBase}>专辑</th>
            <th
              style={{
                ...headerCellBase,
                textAlign: "right",
                width: 80,
              }}
            >
              播放次数
            </th>
            <th
              style={{
                ...headerCellBase,
                width: 180,
              }}
            >
              最近播放
            </th>
            <th
              style={{
                ...headerCellBase,
                textAlign: "center",
                width: 60,
              }}
            >
              喜欢
            </th>
            <th
              style={{
                ...headerCellBase,
                textAlign: "center",
                width: 130,
              }}
            >
              操作
            </th>
          </tr>
        </thead>
        <tbody>
          {tracks.length === 0 ? (
            <tr>
              <td
                colSpan={8}
                style={{
                  padding: "40px 16px",
                  textAlign: "center",
                  fontSize: 13,
                  color: "#9ca3af",
                }}
              >
                当前没有任何本地音乐。点击上方「导入本地音乐」按钮添加一些歌曲吧。
              </td>
            </tr>
          ) : (
            tracks.map((track, index) => {
              const key =
                (track as any)?.id ??
                (track as any)?.trackId ??
                track.filePath ??
                track.path ??
                track.location ??
                index;

              const title = getTrackTitle(track);
              const artist = getTrackArtist(track);
              const album = getTrackAlbum(track);
              const playCount = track.playCount ?? 0;
              const lastPlayedLabel = formatLastPlayed(track.lastPlayedAt);
              const favorite = !!track.favorite;

              const trackFilePath =
                track.filePath ?? track.path ?? track.location;
              const isCurrent =
                !!trackFilePath && trackFilePath === currentFilePath;

              return (
                <tr
                  key={key}
                  onDoubleClick={() => handleRowDoubleClick(track, index)}
                  style={{
                    backgroundColor: isCurrent
                      ? "rgba(37,99,235,0.06)"
                      : index % 2 === 0
                      ? "#ffffff"
                      : "#f9fafb",
                    cursor: "pointer",
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
                    {album}
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
                      onClick={(e) => {
                        e.stopPropagation();
                        handleToggleFavorite(track);
                      }}
                      style={{
                        border: "none",
                        background: "transparent",
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
                      onClick={(e) => {
                        e.stopPropagation();
                        handlePlayNext(track);
                      }}
                      style={{
                        border: "none",
                        background: "transparent",
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
                      onClick={(e) => {
                        e.stopPropagation();
                        handleAppendToQueue(track);
                      }}
                      style={{
                        border: "none",
                        background: "transparent",
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
            })
          )}
        </tbody>
      </table>
    </div>
  );
};
