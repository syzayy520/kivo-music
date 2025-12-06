// src/components/playlists/PlaylistTable.tsx
import React from "react";
import { invoke } from "@tauri-apps/api/core";
import type { PlayerTrack } from "../../types/track";
import {
  playNext,
  appendToQueue,
  playFromQueue,
  getQueueSnapshot,
} from "../../playlists/playQueueModel";
import { LibraryTrackContextMenu } from "../library/LibraryTrackContextMenu";
import PlaylistTableHeader from "./PlaylistTableHeader";
import PlaylistTableBody from "./PlaylistTableBody";

interface PlaylistTableProps {
  /** 当前 tab 下的曲目列表（由上层传入） */
  tracks: PlayerTrack[];
  /** 当前正在播放的索引（从 store 传进来，作为兜底） */
  currentIndex: number;
  /** 生成稳定的 identity key，用于高亮、去重等 */
  makeIdentityKey: (track: any) => string;
}

interface ContextMenuState {
  visible: boolean;
  x: number;
  y: number;
  track: PlayerTrack | null;
}

/** 从 track 中解析出所在目录路径 */
function getDirectoryPathFromTrack(track: PlayerTrack): string | null {
  const anyTrack = track as any;
  const fullPath =
    anyTrack.filePath ?? anyTrack.path ?? anyTrack.location ?? null;
  if (!fullPath || typeof fullPath !== "string") return null;

  const lastSep = Math.max(
    fullPath.lastIndexOf("\\"),
    fullPath.lastIndexOf("/"),
  );
  if (lastSep <= 0) return null;

  return fullPath.slice(0, lastSep);
}

/** 处理 Windows UNC / \\?\ 前缀，兼容 NAS / 网络盘 */
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
 * 调用 tauri-plugin-opener 打开文件所在目录（统一用 open_path）
 */
async function openFolderForTrack(track: PlayerTrack): Promise<void> {
  const rawDir = getDirectoryPathFromTrack(track);
  if (!rawDir) {
    console.warn("[Kivo][PlaylistTable]", "无法解析曲目的所在目录", { track });
    return;
  }

  const dirPath = normalizeWindowsDirPath(rawDir);

  try {
    await invoke("plugin:opener|open_path", { path: dirPath });
  } catch (err) {
    console.warn(
      "[Kivo][PlaylistTable]",
      "调用 opener 打开文件所在目录失败（插件未配置、路径不兼容或权限不足）",
      {
        dirPath,
        err,
        track,
      },
    );
  }
}

/** 格式化最近播放时间 */
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

/**
 * 播放列表页表格视图。
 *
 * 职责：
 * - 只负责 UI + 行为（双击播放 / 下一首 / 加入队列 / 右键菜单）。
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

  const [contextMenu, setContextMenu] = React.useState<ContextMenuState>({
    visible: false,
    x: 0,
    y: 0,
    track: null,
  });

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
  queuePlaylist.forEach((tItem, i) => {
    const key = makeIdentityKey(tItem);
    if (key && !queueIndexByKey.has(key)) {
      queueIndexByKey.set(key, i);
    }
  });

  /** 双击行：在队列中 → playFromQueue；否则先 append 再从队列播放 */
  const handleRowDoubleClick = (track: PlayerTrack): void => {
    const identity = makeIdentityKey(track);
    if (!identity) return;

    let queueIndex =
      queueIndexByKey.has(identity) &&
      queueIndexByKey.get(identity) !== undefined
        ? (queueIndexByKey.get(identity) as number)
        : -1;

    if (queueIndex < 0) {
      // 不在当前队列：先追加，再从新队列中找到它并播放
      appendToQueue([track]);
      const refreshed = getQueueSnapshot();
      const list = Array.isArray(refreshed.playlist)
        ? refreshed.playlist
        : [];
      queueIndex = list.findIndex(
        (tItem) => makeIdentityKey(tItem) === identity,
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

  /** 打开右键菜单 */
  const handleOpenContextMenu = (
    event: React.MouseEvent<HTMLTableRowElement>,
    track: PlayerTrack,
  ) => {
    event.preventDefault();
    setContextMenu({
      visible: true,
      x: event.clientX,
      y: event.clientY,
      track,
    });
  };

  /** 关闭右键菜单 */
  const handleCloseContextMenu = () => {
    setContextMenu((prev) =>
      prev.visible ? { ...prev, visible: false, track: null } : prev,
    );
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
        <PlaylistTableHeader />
        <PlaylistTableBody
          tracks={safeTracks}
          queueCurrentIndex={queueCurrentIndex}
          queueIndexByKey={queueIndexByKey}
          makeIdentityKey={makeIdentityKey}
          formatLastPlayed={formatLastPlayed}
          onRowDoubleClick={handleRowDoubleClick}
          onRowContextMenu={handleOpenContextMenu}
          onPlayNext={handlePlayNext}
          onAppendToQueue={handleAppendToQueue}
        />
      </table>

      <LibraryTrackContextMenu
        visible={contextMenu.visible && !!contextMenu.track}
        x={contextMenu.x}
        y={contextMenu.y}
        onClose={handleCloseContextMenu}
        onPlay={() => {
          if (!contextMenu.track) return;
          handleRowDoubleClick(contextMenu.track);
          handleCloseContextMenu();
        }}
        onPlayNext={() => {
          if (!contextMenu.track) return;
          handlePlayNext(contextMenu.track);
          handleCloseContextMenu();
        }}
        onAppendToQueue={() => {
          if (!contextMenu.track) return;
          handleAppendToQueue(contextMenu.track);
          handleCloseContextMenu();
        }}
        onOpenFolder={() => {
          if (!contextMenu.track) return;
          void openFolderForTrack(contextMenu.track);
          handleCloseContextMenu();
        }}
      />
    </div>
  );
};

export default PlaylistTable;
