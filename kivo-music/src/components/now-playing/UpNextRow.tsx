// src/components/now-playing/UpNextRow.tsx
import React from "react";
import type { PlayerTrack } from "../../types/track";
import {
  KivoContextMenu,
  KivoContextMenuItem,
} from "../common/KivoContextMenu";

export interface UpNextRowProps {
  track: PlayerTrack;
  index: number;
  isCurrent: boolean;
  isNext: boolean;
  canMoveUp: boolean;
  canMoveDown: boolean;
  isRemovable: boolean;
  onPlay: () => void;
  onMoveUp: () => void;
  onMoveDown: () => void;
  onRemove?: () => void;
  onOpenFolder: () => void;
}

/**
 * Up Next 队列中的单行。
 *
 * 仅负责展示 & 行内交互：
 * - 双击播放；
 * - 上移 / 下移；
 * - 删除（可选）；
 * - 右键菜单（复用上述能力 + 打开文件所在文件夹）。
 *
 * 真正的队列修改逻辑由调用方（UpNextPanel）通过回调提供。
 */
export const UpNextRow: React.FC<UpNextRowProps> = ({
  track,
  index,
  isCurrent,
  isNext,
  canMoveUp,
  canMoveDown,
  isRemovable,
  onPlay,
  onMoveUp,
  onMoveDown,
  onRemove,
  onOpenFolder,
}) => {
  const title = track?.title || "未知标题";
  const artist = (track as any)?.artist || "未知艺人";

  // 右键菜单状态
  const [menuVisible, setMenuVisible] = React.useState(false);
  const [menuPosition, setMenuPosition] = React.useState({ x: 0, y: 0 });

  const openContextMenu: React.MouseEventHandler<HTMLDivElement> = (event) => {
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

  const handleRowDoubleClick: React.MouseEventHandler<HTMLDivElement> = (
    event,
  ) => {
    event.stopPropagation();
    onPlay();
  };

  const handleMoveUpClick: React.MouseEventHandler<HTMLButtonElement> = (
    event,
  ) => {
    event.stopPropagation();
    onMoveUp();
  };

  const handleMoveDownClick: React.MouseEventHandler<HTMLButtonElement> = (
    event,
  ) => {
    event.stopPropagation();
    onMoveDown();
  };

  const handleRemoveClick: React.MouseEventHandler<HTMLButtonElement> = (
    event,
  ) => {
    event.stopPropagation();
    onRemove?.();
  };

  const contextMenuItems: KivoContextMenuItem[] = [
    {
      key: "play",
      label: "播放此曲目",
      onClick: () => {
        onPlay();
      },
    },
    {
      key: "moveUp",
      label: "上移一位",
      onClick: () => {
        onMoveUp();
      },
      disabled: !canMoveUp,
    },
    {
      key: "moveDown",
      label: "下移一位",
      onClick: () => {
        onMoveDown();
      },
      disabled: !canMoveDown,
    },
    {
      key: "openFolder",
      label: "打开文件所在文件夹",
      onClick: () => {
        onOpenFolder();
      },
    },
    {
      key: "remove",
      label: "从当前队列中移除",
      onClick: () => {
        onRemove && onRemove();
      },
      disabled: !isRemovable || !onRemove,
      danger: true,
    },
  ];

  return (
    <>
      <div
        onDoubleClick={handleRowDoubleClick}
        onContextMenu={openContextMenu}
        style={{
          display: "flex",
          alignItems: "center",
          gap: 8,
          padding: "6px 10px",
          fontSize: 12,
          cursor: "pointer",
          backgroundColor: isCurrent ? "rgba(37,99,235,0.1)" : "transparent",
          borderBottom: "1px solid rgba(148,163,184,0.25)",
        }}
      >
        {/* 序号 / 播放状态 */}
        <div
          style={{
            width: 28,
            textAlign: "right",
            paddingRight: 4,
            fontVariantNumeric: "tabular-nums",
            color: isCurrent ? "#2563eb" : "#6b7280",
          }}
        >
          {isCurrent ? "▶" : index + 1}
        </div>

        {/* 标题 + 艺人 */}
        <div
          style={{
            flex: 1,
            minWidth: 0,
            display: "flex",
            flexDirection: "column",
          }}
        >
          <div
            style={{
              fontSize: 13,
              fontWeight: isCurrent ? 600 : 500,
              color: "#111827",
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {title}
          </div>
          <div
            style={{
              fontSize: 11,
              color: "#6b7280",
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {artist}
          </div>
        </div>

        {/* 当前 / 下一首 标记 */}
        <div
          style={{
            minWidth: 64,
            textAlign: "right",
            fontSize: 11,
            color: isCurrent ? "#2563eb" : "#6b7280",
          }}
        >
          {isCurrent ? "正在播放" : isNext ? "下一首" : ""}
        </div>

        {/* 队列操作按钮：上移 / 下移 / 删除 */}
        <div
          style={{
            minWidth: 88,
            display: "flex",
            alignItems: "center",
            justifyContent: "flex-end",
            gap: 4,
          }}
        >
          {canMoveUp && (
            <button
              onClick={handleMoveUpClick}
              style={iconButtonStyle}
              title="上移一位"
            >
              ↑
            </button>
          )}
          {canMoveDown && (
            <button
              onClick={handleMoveDownClick}
              style={iconButtonStyle}
              title="下移一位"
            >
              ↓
            </button>
          )}
          {isRemovable && onRemove && (
            <button
              onClick={handleRemoveClick}
              style={iconButtonStyle}
              title="从当前队列中移除这首歌"
            >
              ✕
            </button>
          )}
        </div>
      </div>

      {/* 右键菜单 */}
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

const iconButtonStyle: React.CSSProperties = {
  border: "none",
  background: "transparent",
  color: "#9ca3af",
  cursor: "pointer",
  fontSize: 13,
};

export default UpNextRow;
