// src/components/library/LibraryTracksView.tsx
import React from "react";
import { LibraryTrack, getTrackTitle } from "../../library/libraryModel";
import { usePlayerStore, PlayerTrack } from "../../store/player";
import { appendToQueue, playNext } from "../../playlists/playQueueModel";
import { LibraryTrackRow } from "./LibraryTrackRow";
import { LibraryTrackContextMenu } from "./LibraryTrackContextMenu";

interface Props {
  tracks: LibraryTrack[];
  onPlayTrack: (track: LibraryTrack, index: number) => void;
}

interface ContextMenuState {
  visible: boolean;
  clientX: number;
  clientY: number;
  track: LibraryTrack | null;
  rowIndex: number;
}

/**
 * 本地资料库 - “按歌曲”视图（壳组件）
 * - 负责：
 *   - 从 player store 里拿当前正在播放的曲目
 *   - 维护右键菜单的 state
 *   - 把单行渲染交给 LibraryTrackRow
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

  const [contextMenu, setContextMenu] = React.useState<ContextMenuState>({
    visible: false,
    clientX: 0,
    clientY: 0,
    track: null,
    rowIndex: -1,
  });

  const handleRowDoubleClick = (track: LibraryTrack, index: number) => {
    onPlayTrack(track, index);
  };

  const handlePlayNext = (track: LibraryTrack) => {
    const asPlayerTrack = track as unknown as PlayerTrack;
    playNext([asPlayerTrack]);
  };

  const handleAppendToQueue = (track: LibraryTrack) => {
    const asPlayerTrack = track as unknown as PlayerTrack;
    appendToQueue([asPlayerTrack]);
  };

  const openContextMenu = (
    event: React.MouseEvent<HTMLTableRowElement>,
    track: LibraryTrack,
    index: number,
  ) => {
    event.preventDefault();
    setContextMenu({
      visible: true,
      clientX: event.clientX,
      clientY: event.clientY,
      track,
      rowIndex: index,
    });
  };

  const closeContextMenu = () => {
    setContextMenu((prev) => ({
      ...prev,
      visible: false,
      track: null,
      rowIndex: -1,
    }));
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
        position: "relative",
        flex: 1,
        minHeight: 0,
        overflow: "auto",
        backgroundColor: "#ffffff",
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
              backgroundColor: "#f9fafb",
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
                  padding: 32,
                  textAlign: "center",
                  color: "#9ca3af",
                  fontSize: 13,
                }}
              >
                当前没有可显示的曲目，请先导入本地音乐。
              </td>
            </tr>
          ) : (
            tracks.map((track, index) => {
              const filePath =
                (track as any).filePath ??
                (track as any).path ??
                (track as any).location ??
                "";
              const isCurrent =
                !!currentFilePath && !!filePath && currentFilePath === filePath;

              const idCandidate =
                (track as any).id ??
                (track as any).trackId ??
                filePath ??
                getTrackTitle(track);

              const idPart =
                idCandidate && String(idCandidate).length > 0
                  ? String(idCandidate)
                  : "track";

              const rowKey = `${idPart}::${index}`;

              return (
                <LibraryTrackRow
                  key={rowKey}
                  track={track}
                  index={index}
                  isCurrent={isCurrent}
                  onPlay={handleRowDoubleClick}
                  onPlayNext={handlePlayNext}
                  onAppendToQueue={handleAppendToQueue}
                  onContextMenu={openContextMenu}
                />
              );
            })
          )}
        </tbody>
      </table>

      <LibraryTrackContextMenu
        visible={contextMenu.visible && !!contextMenu.track}
        x={contextMenu.clientX}
        y={contextMenu.clientY}
        onClose={closeContextMenu}
        onPlay={() => {
          if (!contextMenu.track || contextMenu.rowIndex < 0) return;
          handleRowDoubleClick(contextMenu.track, contextMenu.rowIndex);
          closeContextMenu();
        }}
        onPlayNext={() => {
          if (!contextMenu.track) return;
          handlePlayNext(contextMenu.track);
          closeContextMenu();
        }}
        onAppendToQueue={() => {
          if (!contextMenu.track) return;
          handleAppendToQueue(contextMenu.track);
          closeContextMenu();
        }}
      />
    </div>
  );
};
