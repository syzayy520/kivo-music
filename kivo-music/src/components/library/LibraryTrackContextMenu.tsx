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
}

/**
 * 资料库单曲右键菜单（仅负责 UI）
 */
export const LibraryTrackContextMenu: React.FC<
  LibraryTrackContextMenuProps
> = ({ visible, x, y, onClose, onPlay, onPlayNext, onAppendToQueue }) => {
  if (!visible) return null;

  return (
    <div
      style={{
        position: "fixed",
        inset: 0,
        zIndex: 40,
      }}
      onClick={onClose}
    >
      <div
        style={{
          position: "absolute",
          top: y,
          left: x,
          minWidth: 180,
          padding: 4,
          backgroundColor: "#111827",
          borderRadius: 8,
          boxShadow: "0 12px 30px rgba(15,23,42,0.45)",
          border: "1px solid rgba(148,163,184,0.35)",
          color: "#e5e7eb",
          fontSize: 13,
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <MenuItem onClick={onPlay}>播放此歌曲</MenuItem>
        <MenuItem onClick={onPlayNext}>设为下一首播放</MenuItem>
        <MenuItem onClick={onAppendToQueue}>添加到当前队列末尾</MenuItem>
      </div>
    </div>
  );
};

interface MenuItemProps {
  children: React.ReactNode;
  onClick: () => void;
}

const MenuItem: React.FC<MenuItemProps> = ({ children, onClick }) => (
  <button
    type="button"
    onClick={onClick}
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
