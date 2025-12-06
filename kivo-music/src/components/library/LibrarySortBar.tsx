// src/components/library/LibrarySortBar.tsx
import React from "react";
import { useI18n } from "../../i18n";

type SortKey = "none" | "title" | "artist";

interface LibrarySortBarProps {
  sortKey: SortKey;
  sortAsc: boolean;
  onToggleSort: (key: SortKey) => void;
}

/**
 * 排序区域：默认顺序 / 按标题 / 按艺人
 */
export const LibrarySortBar: React.FC<LibrarySortBarProps> = ({
  sortKey,
  sortAsc,
  onToggleSort,
}) => {
  const { t } = useI18n();

  const sortLabel = (key: SortKey) => {
    if (sortKey !== key) return "";
    return sortAsc ? "▲" : "▼";
  };

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        fontSize: 12,
        color: "#6b7280",
      }}
    >
      <div style={{ display: "flex", gap: 12 }}>
        <button
          type="button"
          onClick={() => onToggleSort("none")}
          style={{
            border: "none",
            background: "transparent",
            cursor: "pointer",
            color: sortKey === "none" ? "#111827" : "#6b7280",
          }}
        >
          {t("library.sortBar.option.none")}
        </button>

        <button
          type="button"
          onClick={() => onToggleSort("title")}
          style={{
            border: "none",
            background: "transparent",
            cursor: "pointer",
            color: sortKey === "title" ? "#111827" : "#6b7280",
          }}
        >
          {t("library.sortBar.option.title")} {sortLabel("title")}
        </button>

        <button
          type="button"
          onClick={() => onToggleSort("artist")}
          style={{
            border: "none",
            background: "transparent",
            cursor: "pointer",
            color: sortKey === "artist" ? "#111827" : "#6b7280",
          }}
        >
          {t("library.sortBar.option.artist")} {sortLabel("artist")}
        </button>
      </div>

      <div>{t("library.sortBar.hint.doubleClickToPlay")}</div>
    </div>
  );
};
