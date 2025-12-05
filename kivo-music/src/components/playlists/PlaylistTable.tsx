// src/components/playlists/PlaylistTable.tsx
import React from "react";
import { PlayerTrack } from "../../store/player";
import {
  getTrackTitle,
  getTrackArtist,
  getTrackAlbum,
} from "../../library/libraryModel";
import {
  playNext,
  appendToQueue,
  playFromQueue,
  getQueueSnapshot,
} from "../../playlists/playQueueModel";

interface PlaylistTableProps {
  tracks: PlayerTrack[];
  /** 当前正在播放的索引（从 store 传进来，作为兜底） */
  currentIndex: number;
  /** 生成稳定的 identity key，用于高亮、去重等 */
  makeIdentityKey: (track: any) => string;
}

/**
 * 播放列表页表格视图。
 *
 * 职责：
 * - 只负责 UI + 行为（双击播放 / 下一首 / 加入队列）。
 * - 所有“真实队列”操作统一走 playQueueModel：
 *   - playFromQueue / appendToQueue / playNext / getQueueSnapshot
 */
const PlaylistTable: React.FC<PlaylistTableProps> = ({
  tracks,
  currentIndex,
  makeIdentityKey,
}) => {
  const safeTracks: PlayerTrack[] = Array.isArray(tracks)
    ? (tracks as PlayerTrack[])
    : [];

  // 当前队列快照，用于判断“是否在队列中”“是否为当前播放曲目”
  const queueSnapshot = getQueueSnapshot();
  const queuePlaylist = Array.isArray(queueSnapshot.playlist)
    ? queueSnapshot.playlist
    : [];
  const queueCurrentIndex =
    typeof queueSnapshot.currentIndex === "number" &&
    queueSnapshot.currentIndex >= 0
      ? queueSnapshot.currentIndex
      : currentIndex;

  // 用 identity key 建一个“队列索引映射”
  const queueIndexByKey = new Map<string, number>();
  queuePlaylist.forEach((t, i) => {
    const key = makeIdentityKey(t);
    if (key && !queueIndexByKey.has(key)) {
      queueIndexByKey.set(key, i);
    }
  });

  /** 双击行：如果在队列中 → playFromQueue；否则先 append 再从队列播放 */
  const handleRowDoubleClick = (track: PlayerTrack): void => {
    const identity = makeIdentityKey(track);
    if (!identity) return;

    let queueIndex =
      queueIndexByKey.has(identity) && queueIndexByKey.get(identity) !== undefined
        ? (queueIndexByKey.get(identity) as number)
        : -1;

    if (queueIndex < 0) {
      // 不在当前队列：先追加，再从新队列中找到它并播
      appendToQueue([track]);
      const refreshed = getQueueSnapshot();
      const list = Array.isArray(refreshed.playlist)
        ? refreshed.playlist
        : [];
      queueIndex = list.findIndex(
        (t) => makeIdentityKey(t) === identity,
      );
    }

    if (queueIndex >= 0) {
      playFromQueue(queueIndex);
    }
  };

  /** 设为下一首播放（插入队列 currentIndex+1） */
  const handlePlayNext = (track: PlayerTrack): void => {
    playNext([track]);
  };

  /** 追加到当前队列末尾 */
  const handleAppendToQueue = (track: PlayerTrack): void => {
    appendToQueue([track]);
  };

  const formatLastPlayed = (
    iso: string | null | undefined,
  ): string => {
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

  return (
    <div
      style={{
        width: "100%",
        height: "100%",
        overflow: "auto",
        borderRadius: 8,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: "rgba(148,163,184,0.4)",
        backgroundColor: "rgba(15,23,42,0.02)",
      }}
    >
      <table
        style={{
          width: "100%",
          borderCollapse: "collapse",
          tableLayout: "fixed",
          fontSize: 13,
        }}
      >
        <thead>
          <tr
            style={{
              textAlign: "left",
              userSelect: "none",
              color: "#6b7280",
            }}
          >
            <th style={{ width: 40, padding: "8px 12px" }}>#</th>
            <th style={{ width: "28%", padding: "8px 12px" }}>标题</th>
            <th style={{ width: "20%", padding: "8px 12px" }}>艺人</th>
            <th style={{ width: "22%", padding: "8px 12px" }}>专辑</th>
            <th style={{ width: 80, padding: "8px 12px" }}>播放次数</th>
            <th style={{ width: 180, padding: "8px 12px" }}>最近播放</th>
            <th style={{ width: 60, padding: "8px 12px" }}>喜欢</th>
            <th style={{ width: 140, padding: "8px 12px" }}>操作</th>
          </tr>
        </thead>
        <tbody>
          {safeTracks.length === 0 ? (
            <tr>
              <td
                colSpan={8}
                style={{
                  padding: "16px 12px",
                  textAlign: "center",
                  color: "#9ca3af",
                }}
              >
                当前列表暂无歌曲
              </td>
            </tr>
          ) : (
            safeTracks.map((track, index) => {
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
                  onDoubleClick={() => handleRowDoubleClick(track)}
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
                  <td style={{ padding: "6px 12px", textAlign: "right" }}>
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
                        handlePlayNext(track);
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
                      设为下一首
                    </button>
                    <button
                      type="button"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleAppendToQueue(track);
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

export default PlaylistTable;
