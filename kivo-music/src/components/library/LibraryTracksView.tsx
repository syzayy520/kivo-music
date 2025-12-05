import React from "react";
import { invoke } from "@tauri-apps/api/core";
import { LibraryTrack, getTrackTitle } from "../../library/libraryModel";
import { usePlayerStore, PlayerTrack } from "../../store/player";
import { appendToQueue, playNext } from "../../playlists/playQueueModel";
import { LibraryTrackRow } from "./LibraryTrackRow";
import { LibraryTrackContextMenu } from "./LibraryTrackContextMenu";
import { log } from "../../utils/log";

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
 * 为调用 tauri-plugin-opener 规整 Windows 扩展路径：
 * - \\?\C:\Music\xxx      => C:\Music\xxx
 * - \\?\UNC\Nas\12t\xxx   => \\Nas\12t\xxx
 */
const normalizePathForReveal = (rawPath: string): string => {
  let p = rawPath;

  const extendedUncPrefix = "\\\\?\\UNC\\";
  if (p.startsWith(extendedUncPrefix)) {
    const rest = p.slice(extendedUncPrefix.length);
    // 还原成标准 UNC：\\Nas\12t\...
    return "\\\\" + rest;
  }

  const extendedLocalPrefix = "\\\\?\\";
  if (p.startsWith(extendedLocalPrefix)) {
    // 本地盘：去掉 \\?\ 前缀
    return p.slice(extendedLocalPrefix.length);
  }

  return p;
};

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

  const handleOpenInFolder = (track: LibraryTrack) => {
    const rawPath =
      (track as any).filePath ??
      (track as any).path ??
      (track as any).location ??
      "";

    const filePath = typeof rawPath === "string" ? rawPath : "";

    if (!filePath) {
      log.warn(
        "LibraryTracksView",
        "无法打开文件所在文件夹：当前曲目缺少 filePath/path/location 字段",
        { track },
      );
      return;
    }

    const normalizedPath = normalizePathForReveal(filePath);

    // UNC 路径（NAS / 共享盘）：reveal_item_in_dir 对这种路径兼容性不好，
    // 这里退一步，直接打开所在目录。
    const isUncPath =
      normalizedPath.startsWith("\\\\") && !/^[a-zA-Z]:\\/.test(normalizedPath);

    if (isUncPath) {
      const lastSlashIndex = Math.max(
        normalizedPath.lastIndexOf("\\"),
        normalizedPath.lastIndexOf("/"),
      );

      const dirPath =
        lastSlashIndex > 1
          ? normalizedPath.slice(0, lastSlashIndex)
          : normalizedPath;

      invoke("plugin:opener|open_path", { path: dirPath }).catch(
        (err: unknown) => {
          log.warn(
            "LibraryTracksView",
            "调用 opener 打开 UNC 目录失败（插件未配置、路径不兼容或权限不足）",
            { filePath, normalizedPath, dirPath, err },
          );
        },
      );
      return;
    }

    // 本地盘路径：使用 reveal_item_in_dir 在资源管理器中高亮文件。
    invoke("plugin:opener|reveal_item_in_dir", {
      path: normalizedPath,
    }).catch((err: unknown) => {
      log.warn(
        "LibraryTracksView",
        "调用 opener 插件打开文件所在文件夹失败（插件未配置、路径不兼容或权限不足）",
        { filePath, normalizedPath, err },
      );
    });
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
        onOpenInFolder={() => {
          if (!contextMenu.track) return;
          handleOpenInFolder(contextMenu.track);
          closeContextMenu();
        }}
      />
    </div>
  );
};
