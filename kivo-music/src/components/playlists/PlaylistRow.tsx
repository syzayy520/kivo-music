// src/components/playlists/PlaylistRow.tsx
import React from "react";
import {
  KivoContextMenu,
  KivoContextMenuItem,
} from "../common/KivoContextMenu";

interface PlaylistRowProps {
  index: number;
  title: string;
  artist: string;
  album: string;
  playCount: number;
  lastPlayedLabel: string;
  isCurrent: boolean;
  onRowDoubleClick: () => void;
  onPlayNext: () => void;
  onAppendToQueue: () => void;
}

/**
 * PlaylistRow
 *
 * 播放列表页中的单行组件。
 * - 左键双击：播放本行（由外部回调控制）；
 * - 右键：弹出菜单（播放 / 下一首 / 加入队列）。
 * 只负责展示和行内交互，不直接操作全局状态。
 */
const PlaylistRow: React.FC<PlaylistRowProps> = ({
  index,
  title,
  artist,
  album,
  playCount,
  lastPlayedLabel,
  isCurrent,
  onRowDoubleClick,
  onPlayNext,
  onAppendToQueue,
}) => {
  const rowBackground = isCurrent
    ? "rgba(37,99,235,0.06)"
    : index % 2 === 0
    ? "#ffffff"
    : "#f9fafb";

  // 右键菜单状态
  const [menuVisible, setMenuVisible] = React.useState(false);
  const [menuPosition, setMenuPosition] = React.useState({ x: 0, y: 0 });

  const openContextMenu: React.MouseEventHandler<HTMLTableRowElement> = (
    event,
  ) => {
    event.preventDefault();
    event.stopPropagation();

    setMenuPosition({
      x: event.clientX,
      y: event.clientY,
    });
    setMenuVisible(true);
  };

  const closeContextMenu = () => {
    setMenuVisible(false);
  };

  const handleRowDoubleClick: React.MouseEventHandler<HTMLTableRowElement> = (
    event,
  ) => {
    event.preventDefault();
    onRowDoubleClick();
  };

  const handlePlayNextClick: React.MouseEventHandler<HTMLButtonElement> = (
    event,
  ) => {
    event.stopPropagation();
    onPlayNext();
  };

  const handleAppendToQueueClick: React.MouseEventHandler<
    HTMLButtonElement
  > = (event) => {
    event.stopPropagation();
    onAppendToQueue();
  };

  const contextMenuItems: KivoContextMenuItem[] = [
    {
      key: "play",
      label: "播放此曲目",
      onClick: () => {
        onRowDoubleClick();
      },
    },
    {
      key: "next",
      label: "设为下一首播放",
      onClick: () => {
        onPlayNext();
      },
    },
    {
      key: "append",
      label: "加入当前队列尾部",
      onClick: () => {
        onAppendToQueue();
      },
    },
  ];

  return (
    <>
      <tr
        onDoubleClick={handleRowDoubleClick}
        onContextMenu={openContextMenu}
        style={{
          backgroundColor: rowBackground,
          cursor: "pointer",
        }}
      >
        {/* 序号列 */}
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

        {/* 标题列 */}
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

        {/* 艺人列 */}
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

        {/* 专辑列 */}
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

        {/* 播放次数列 */}
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

        {/* 最近播放列 */}
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

        {/* 操作列：下一首 / 加入队列 */}
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
            onClick={handleAppendToQueueClick}
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

      {/* 右键菜单本体 */}
      <KivoContextMenu
        visible={menuVisible}
        x={menuPosition.x}
        y={menuPosition.y}
        items={contextMenuItems}
        onClose={closeContextMenu}
      />
    </>
  );
};

export default PlaylistRow;
