// src/components/library/LibraryTrackContextMenu.tsx
import React, { useEffect } from "react";
import { useI18n } from "../../i18n";

export interface LibraryTrackContextMenuProps {
  visible: boolean;
  x: number;
  y: number;
  onClose: () => void;
  onPlay?: () => void;
  onPlayNext?: () => void;
  onAppendToQueue?: () => void;
  onOpenFolder?: () => void;
}

/**
 * 资料库 / 播放列表中的曲目右键菜单。
 *
 * 只负责展示和分发点击事件：
 * - 播放此歌曲
 * - 设为下一首播放
 * - 添加到当前队列尾部
 * - 打开文件所在文件夹
 *
 * 所有文案走 i18n，实际行为由父组件通过 props 传入。
 */
export const LibraryTrackContextMenu: React.FC<
  LibraryTrackContextMenuProps
> = ({
  visible,
  x,
  y,
  onClose,
  onPlay,
  onPlayNext,
  onAppendToQueue,
  onOpenFolder,
}) => {
  const { t } = useI18n();

  // ESC 关闭菜单
  useEffect(() => {
    if (!visible) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        onClose();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [visible, onClose]);

  if (!visible) {
    return null;
  }

  // 简单做一下防溢出，避免菜单超出窗口
  const menuWidth = 220;
  const menuItemHeight = 32;
  const itemCount = 4;
  const menuHeight = menuItemHeight * itemCount;

  const viewportWidth =
    typeof window !== "undefined" ? window.innerWidth : 1920;
  const viewportHeight =
    typeof window !== "undefined" ? window.innerHeight : 1080;

  let left = x;
  let top = y;

  if (left + menuWidth > viewportWidth - 8) {
    left = Math.max(8, viewportWidth - menuWidth - 8);
  }
  if (top + menuHeight > viewportHeight - 8) {
    top = Math.max(8, viewportHeight - menuHeight - 8);
  }

  const overlayStyle: React.CSSProperties = {
    position: "fixed",
    inset: 0,
    zIndex: 50,
  };

  const menuStyle: React.CSSProperties = {
    position: "fixed",
    left,
    top,
    width: menuWidth,
    borderRadius: 10,
    padding: 4,
    backgroundColor: "rgba(15,23,42,0.98)",
    border: "1px solid rgba(148,163,184,0.5)",
    boxShadow:
      "0 18px 40px rgba(15,23,42,0.55), 0 0 0 1px rgba(15,23,42,0.7)",
    color: "#e5e7eb",
    fontSize: 13,
    backdropFilter: "blur(16px)",
  };

  const itemStyle: React.CSSProperties = {
    width: "100%",
    borderRadius: 8,
    padding: "6px 10px",
    textAlign: "left",
    border: "none",
    outline: "none",
    backgroundColor: "transparent",
    color: "inherit",
    cursor: "pointer",
    fontSize: 13,
  };

  const handleItemClick =
    (cb?: () => void) => (e: React.MouseEvent<HTMLButtonElement>) => {
      e.stopPropagation();
      e.preventDefault();
      if (cb) cb();
      onClose();
    };

  return (
    <div
      style={overlayStyle}
      onClick={onClose}
      onContextMenu={(e) => {
        e.preventDefault();
        onClose();
      }}
    >
      <div
        style={menuStyle}
        onClick={(e) => e.stopPropagation()}
        onContextMenu={(e) => e.preventDefault()}
      >
        <button
          type="button"
          style={itemStyle}
          onClick={handleItemClick(onPlay)}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor =
              "rgba(37,99,235,0.30)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          {t("library.contextMenu.play")}
        </button>
        <button
          type="button"
          style={itemStyle}
          onClick={handleItemClick(onPlayNext)}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor =
              "rgba(37,99,235,0.30)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          {t("library.contextMenu.playNext")}
        </button>
        <button
          type="button"
          style={itemStyle}
          onClick={handleItemClick(onAppendToQueue)}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor =
              "rgba(37,99,235,0.30)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          {t("library.contextMenu.appendToQueue")}
        </button>
        <button
          type="button"
          style={itemStyle}
          onClick={handleItemClick(onOpenFolder)}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor =
              "rgba(37,99,235,0.30)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          {t("library.contextMenu.openInFolder")}
        </button>
      </div>
    </div>
  );
};

export default LibraryTrackContextMenu;
