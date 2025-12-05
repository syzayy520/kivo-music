// src/components/library/LibraryTrackContextMenu.tsx
import React from "react";

interface LibraryTrackContextMenuProps {
  visible: boolean;
  x: number;
  y: number;
  onClose: () => void;
  onPlay: () => void;
  onPlayNext: () => void;
  onAppendToQueue: () => void;
  onOpenFolder: () => void;
}

/**
 * 资料库单曲右键菜单（只负责 UI）
 */
export const LibraryTrackContextMenu: React.FC<
  LibraryTrackContextMenuProps
> = ({ visible, x, y, onClose, onPlay, onPlayNext, onAppendToQueue, onOpenFolder }) => {
  if (!visible) return null;

  const handleOverlayClick: React.MouseEventHandler<HTMLDivElement> = (event) => {
    event.preventDefault();
    onClose();
  };

  const handleMenuClick: React.MouseEventHandler<HTMLDivElement> = (event) => {
    event.stopPropagation();
  };

  return (
    <div
      onClick={handleOverlayClick}
      style={{
        position: "fixed",
        inset: 0,
        zIndex: 40,
        backgroundColor: "transparent",
      }}
    >
      <div
        onClick={handleMenuClick}
        style={{
          position: "fixed",
          top: y,
          left: x,
          minWidth: 200,
          padding: 4,
          borderRadius: 6,
          backgroundColor: "#111827",
          boxShadow: "0 10px 20px rgba(0,0,0,0.25)",
          border: "1px solid rgba(55,65,81,0.8)",
        }}
      >
        <ContextMenuItem onClick={onPlay}>播放此歌曲</ContextMenuItem>
        <ContextMenuItem onClick={onPlayNext}>设为下一首播放</ContextMenuItem>
        <ContextMenuItem onClick={onAppendToQueue}>
          添加到当前队列尾部
        </ContextMenuItem>

        <div
          style={{
            margin: "4px 0",
            borderTop: "1px solid rgba(55,65,81,0.9)",
          }}
        />

        <ContextMenuItem onClick={onOpenFolder}>
          打开文件所在文件夹
        </ContextMenuItem>
      </div>
    </div>
  );
};

interface ItemProps {
  onClick: () => void;
  children: React.ReactNode;
}

const ContextMenuItem: React.FC<ItemProps> = ({ onClick, children }) => {
  const handleClick: React.MouseEventHandler<HTMLButtonElement> = (event) => {
    event.stopPropagation();
    event.preventDefault();
    onClick();
  };

  return (
    <button
      type="button"
      onClick={handleClick}
      style={{
        width: "100%",
        padding: "6px 10px",
        border: "none",
        backgroundColor: "transparent",
        textAlign: "left",
        cursor: "pointer",
        fontSize: 13,
        color: "#e5e7eb",
      }}
    >
      {children}
    </button>
  );
};
