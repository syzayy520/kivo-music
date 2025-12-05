// src/components/library/LibraryTracksView.tsx
import React, { useCallback, useMemo, useState } from "react";
import type { LibraryTrack, PlayerTrack } from "../../types/track";
import {
  getTrackAlbum,
  getTrackArtist,
  getTrackIdentity,
  getTrackTitle,
} from "../../library/libraryModel";
import { usePlayerStore } from "../../store/player";
import { appendToQueue, playNext } from "../../playlists/playQueueModel";
import { LibraryTrackRow } from "./LibraryTrackRow";
import { LibraryTrackContextMenu } from "./LibraryTrackContextMenu";
import { invoke } from "@tauri-apps/api/core";
import { log } from "../../utils/log";

interface Props {
  tracks: LibraryTrack[];
  onPlayTrack: (track: LibraryTrack, index: number) => void;
}

interface ContextMenuState {
  visible: boolean;
  clientX: number;
  clientY: number;
  rowIndex: number | null;
  track: LibraryTrack | null;
}

/**
 * LibraryTrack -> PlayerTrack（给播放队列用）
 */
function makePlayerTrackFromLibraryTrack(track: LibraryTrack): PlayerTrack {
  const anyTrack = track as any;

  const filePath: string =
    anyTrack.filePath || anyTrack.path || anyTrack.location || "";

  const title = getTrackTitle(track);
  const artist = getTrackArtist(track);
  const album = getTrackAlbum(track) ?? "";

  const idSource =
    anyTrack.trackId ?? anyTrack.id ?? filePath ?? `${artist}-${title}`;

  const durationValue: number | undefined =
    typeof anyTrack.duration === "number"
      ? anyTrack.duration
      : typeof anyTrack.length === "number"
      ? anyTrack.length
      : undefined;

  return {
    id: String(idSource),
    title,
    artist,
    album,
    filePath,
    duration: durationValue,
    coverId: anyTrack.coverId,
    coverPath: anyTrack.coverPath,
  };
}

/**
 * 提取文件所在目录
 */
function getDirectoryPathFromTrack(track: LibraryTrack): string | null {
  const anyTrack = track as any;
  const raw =
    anyTrack.filePath || anyTrack.path || anyTrack.location || null;
  if (!raw) return null;

  const normalized = String(raw);
  const lastSep = Math.max(
    normalized.lastIndexOf("\\"),
    normalized.lastIndexOf("/"),
  );
  if (lastSep <= 0) return null;
  return normalized.slice(0, lastSep);
}

/**
 * 处理 Windows 下 \\?\UNC\ 前缀，给 opener 用
 */
function normalizeWindowsDirPath(dirPath: string): string {
  if (dirPath.startsWith("\\\\?\\UNC\\")) {
    // \\?\UNC\server\share\path -> \\server\share\path
    return "\\" + dirPath.slice("\\\\?\\UNC\\".length);
  }
  if (dirPath.startsWith("\\\\?\\")) {
    // \\?\C:\Music -> C:\Music
    return dirPath.slice("\\\\?\\".length);
  }
  return dirPath;
}

/**
 * 调用 tauri-plugin-opener 打开文件所在目录（只用 open_path，不再用 reveal_item_in_dir）
 */
async function openFolderForTrack(track: LibraryTrack): Promise<void> {
  const dirPathRaw = getDirectoryPathFromTrack(track);
  if (!dirPathRaw) {
    log.warn(
      "[Kivo][LibraryTracksView]",
      "无法解析曲目的所在目录",
      { track },
    );
    return;
  }

  const dirPath = normalizeWindowsDirPath(dirPathRaw);

  try {
    await invoke("plugin:opener|open_path", { path: dirPath });
  } catch (error) {
    log.warn(
      "[Kivo][LibraryTracksView]",
      "调用 opener 打开文件所在目录失败（插件未配置、路径不兼容或权限不足）",
      {
        filePath:
          (track as any).filePath ||
          (track as any).path ||
          (track as any).location,
        dirPath,
        err: String(error),
      },
    );
  }
}

export const LibraryTracksView: React.FC<Props> = ({
  tracks,
  onPlayTrack,
}) => {
  const [contextMenu, setContextMenu] = useState<ContextMenuState>({
    visible: false,
    clientX: 0,
    clientY: 0,
    rowIndex: null,
    track: null,
  });

  const currentIdentity = usePlayerStore((state) => {
    const current = state.playlist[state.currentIndex] as
      | PlayerTrack
      | undefined;
    if (!current) return null;
    try {
      return getTrackIdentity(current as any);
    } catch {
      return (current as any).id ?? null;
    }
  });

  const headerCellBase: React.CSSProperties = {
    padding: "6px 10px",
    fontSize: 12,
    color: "#6b7280",
    fontWeight: 500,
    borderBottom: "1px solid #1f2937",
    userSelect: "none",
    whiteSpace: "nowrap",
  };

  const handleRowPlay = useCallback(
    (track: LibraryTrack, index: number) => {
      onPlayTrack(track, index);
    },
    [onPlayTrack],
  );

  const handleRowPlayNext = useCallback((track: LibraryTrack) => {
    playNext([makePlayerTrackFromLibraryTrack(track)]);
  }, []);

  const handleRowAppendToQueue = useCallback((track: LibraryTrack) => {
    appendToQueue([makePlayerTrackFromLibraryTrack(track)]);
  }, []);

  const handleRowContextMenu = useCallback(
    (
      event: React.MouseEvent<HTMLTableRowElement>,
      track: LibraryTrack,
      index: number,
    ) => {
      event.preventDefault();
      setContextMenu({
        visible: true,
        clientX: event.clientX,
        clientY: event.clientY,
        rowIndex: index,
        track,
      });
    },
    [],
  );

  const closeContextMenu = useCallback(() => {
    setContextMenu((prev) =>
      prev.visible ? { ...prev, visible: false } : prev,
    );
  }, []);

  const handleContextPlay = () => {
    if (!contextMenu.track || contextMenu.rowIndex == null) return;
    onPlayTrack(contextMenu.track, contextMenu.rowIndex);
    closeContextMenu();
  };

  const handleContextPlayNext = () => {
    if (!contextMenu.track) return;
    playNext([makePlayerTrackFromLibraryTrack(contextMenu.track)]);
    closeContextMenu();
  };

  const handleContextAppendToQueue = () => {
    if (!contextMenu.track) return;
    appendToQueue([makePlayerTrackFromLibraryTrack(contextMenu.track)]);
    closeContextMenu();
  };

  const handleContextOpenFolder = () => {
    if (!contextMenu.track) return;
    void openFolderForTrack(contextMenu.track);
    closeContextMenu();
  };

  const rows = useMemo(
    () =>
      tracks.map((track, index) => {
        const identity = getTrackIdentity(track);
        const isCurrent =
          identity != null &&
          currentIdentity != null &&
          identity === currentIdentity;

        return (
          <LibraryTrackRow
            key={(identity ?? `${getTrackTitle(track)}-${index}`).toString()}
            track={track}
            index={index}
            isCurrent={!!isCurrent}
            onPlay={handleRowPlay}
            onPlayNext={handleRowPlayNext}
            onAppendToQueue={handleRowAppendToQueue}
            onContextMenu={handleRowContextMenu}
          />
        );
      }),
    [
      tracks,
      currentIdentity,
      handleRowPlay,
      handleRowPlayNext,
      handleRowAppendToQueue,
      handleRowContextMenu,
    ],
  );

  return (
    <div style={{ width: "100%", height: "100%", overflow: "auto" }}>
      <table
        style={{
          width: "100%",
          borderCollapse: "collapse",
          tableLayout: "fixed",
        }}
      >
        <thead>
          <tr>
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
            <th
              style={{
                ...headerCellBase,
                minWidth: 160,
              }}
            >
              标题
            </th>
            <th
              style={{
                ...headerCellBase,
                minWidth: 140,
              }}
            >
              艺人
            </th>
            <th
              style={{
                ...headerCellBase,
                minWidth: 140,
              }}
            >
              专辑
            </th>
            <th
              style={{
                ...headerCellBase,
                width: 80,
                textAlign: "right",
              }}
            >
              时长
            </th>
            <th
              style={{
                ...headerCellBase,
                width: 100,
                textAlign: "right",
              }}
            >
              播放次数
            </th>
            <th
              style={{
                ...headerCellBase,
                width: 140,
              }}
            >
              最近播放
            </th>
            <th
              style={{
                ...headerCellBase,
                width: 80,
                textAlign: "center",
              }}
            >
              操作
            </th>
          </tr>
        </thead>
        <tbody>{rows}</tbody>
      </table>

      <LibraryTrackContextMenu
        visible={contextMenu.visible}
        x={contextMenu.clientX}
        y={contextMenu.clientY}
        onClose={closeContextMenu}
        onPlay={handleContextPlay}
        onPlayNext={handleContextPlayNext}
        onAppendToQueue={handleContextAppendToQueue}
        onOpenFolder={handleContextOpenFolder}
      />
    </div>
  );
};
