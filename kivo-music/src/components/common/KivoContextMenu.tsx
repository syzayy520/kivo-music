// src/components/common/KivoContextMenu.tsx
import React, { useEffect } from "react";
import { createPortal } from "react-dom";
import { kivoTheme } from "../../styles/theme";

export interface KivoContextMenuItem {
  key: string;
  label: string;
  onClick: () => void;
  disabled?: boolean;
  danger?: boolean;
}

export interface KivoContextMenuProps {
  visible: boolean;
  x: number;
  y: number;
  items: KivoContextMenuItem[];
  onClose: () => void;
}

/**
 * 通用右键菜单组件。
 *
 * 特性：
 * - 使用 fixed + portal，定位到屏幕坐标 (x, y)；
 * - 点击菜单外部或按 Esc 关闭；
 * - 点击菜单内部不会被全局 mousedown 提前关闭；
 * - 简单避免菜单超出屏幕边界。
 */
export const KivoContextMenu: React.FC<KivoContextMenuProps> = ({
  visible,
  x,
  y,
  items,
  onClose,
}) => {
  const menuRef = React.useRef<HTMLDivElement | null>(null);

  // 监听全局点击 / Esc 关闭菜单
  useEffect(() => {
    if (!visible) return;

    const handleMouseDown = (event: MouseEvent) => {
      if (!visible) return;
      const target = event.target as Node | null;

      // 如果点在菜单内部，不关闭，让按钮自己的 onClick 处理
      if (menuRef.current && target && menuRef.current.contains(target)) {
        return;
      }

      onClose();
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        onClose();
      }
    };

    document.addEventListener("mousedown", handleMouseDown);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("mousedown", handleMouseDown);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [visible, onClose]);

  if (!visible) return null;

  // 简单估算菜单大小，避免跑出屏幕外
  const estimatedWidth = 220;
  const estimatedHeight = items.length * 32 + 16;

  let left = x;
  let top = y;

  if (typeof window !== "undefined") {
    const margin = 8;
    const maxLeft = window.innerWidth - estimatedWidth - margin;
    const maxTop = window.innerHeight - estimatedHeight - margin;

    left = Math.max(margin, Math.min(left, maxLeft));
    top = Math.max(margin, Math.min(top, maxTop));
  }

  const menu = (
    <div
      ref={menuRef}
      style={{
        position: "fixed",
        left,
        top,
        zIndex: 9999,
        minWidth: 200,
        backgroundColor: "#ffffff",
        borderRadius: 10,
        border: `1px solid ${kivoTheme.colors.borderSubtle}`,
        boxShadow: "0 18px 45px rgba(15, 23, 42, 0.35)",
        padding: "4px 0",
        fontSize: 13,
        color: "#111827",
        userSelect: "none",
      }}
    >
      {items.map((item) => {
        const handleClick: React.MouseEventHandler<HTMLButtonElement> = (
          event,
        ) => {
          event.stopPropagation();
          if (item.disabled) return;
          // 先执行动作，再关闭菜单
          item.onClick();
          onClose();
        };

        return (
          <button
            key={item.key}
            onClick={handleClick}
            disabled={item.disabled}
            style={{
              width: "100%",
              textAlign: "left",
              padding: "6px 12px",
              border: "none",
              background: "transparent",
              cursor: item.disabled ? "default" : "pointer",
              fontSize: 13,
              color: item.disabled
                ? "#9ca3af"
                : item.danger
                ? "#b91c1c"
                : "#111827",
              display: "flex",
              alignItems: "center",
            }}
            onMouseEnter={(e) => {
              if (item.disabled) return;
              e.currentTarget.style.backgroundColor = "rgba(15,23,42,0.06)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = "transparent";
            }}
          >
            <span
              style={{
                flex: 1,
                whiteSpace: "nowrap",
                overflow: "hidden",
                textOverflow: "ellipsis",
              }}
            >
              {item.label}
            </span>
          </button>
        );
      })}
    </div>
  );

  return createPortal(menu, document.body);
};

export default KivoContextMenu;
