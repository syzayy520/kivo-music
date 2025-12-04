// src/components/library/LibraryHeader.tsx
import React from "react";

interface LibraryHeaderProps {
  total: number;
  filteredCount: number;
  keyword: string;
  onKeywordChange: (value: string) => void;
  hasDisplayedTracks: boolean;
  onPlayAll: () => void;
  onShufflePlay: () => void;
  onImport: () => void;
  onClearLibrary: () => void;
}

/**
 * 资料库顶部标题 + 搜索 + 操作按钮
 * 只负责展示和触发回调，不直接碰 store。
 */
export const LibraryHeader: React.FC<LibraryHeaderProps> = ({
  total,
  filteredCount,
  keyword,
  onKeywordChange,
  hasDisplayedTracks,
  onPlayAll,
  onShufflePlay,
  onImport,
  onClearLibrary,
}) => {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        gap: 12,
      }}
    >
      {/* 左侧：标题 + 统计 */}
      <div>
        <h1
          style={{
            fontSize: 18,
            fontWeight: 600,
            marginBottom: 4,
          }}
        >
          本地音乐资料库
        </h1>
        <p
          style={{
            fontSize: 12,
            color: "#6b7280",
          }}
        >
          共 {total} 首歌曲
          {keyword && ` · 匹配到 ${filteredCount} 首`}
        </p>
      </div>

      {/* 右侧：搜索框 + 按钮 */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 8,
        }}
      >
        <input
          type="text"
          id="kivo-library-search"
          value={keyword}
          onChange={(e) => onKeywordChange(e.target.value)}
          placeholder="搜索标题 / 艺人 / 专辑"
          autoComplete="off"
          spellCheck={false}
          style={{
            minWidth: 220,
            borderRadius: 6,
            border: "1px solid #d1d5db",
            padding: "4px 8px",
            fontSize: 13,
            outline: "none",
          }}
        />

        <button
          type="button"
          onClick={onPlayAll}
          disabled={!hasDisplayedTracks}
          style={{
            padding: "4px 10px",
            borderRadius: 6,
            border: "1px solid #22c55e",
            backgroundColor: hasDisplayedTracks ? "#22c55e" : "#e5e7eb",
            color: hasDisplayedTracks ? "#ffffff" : "#9ca3af",
            fontSize: 12,
            cursor: hasDisplayedTracks ? "pointer" : "default",
          }}
        >
          ▶ 播放全部
        </button>

        <button
          type="button"
          onClick={onShufflePlay}
          disabled={!hasDisplayedTracks}
          style={{
            padding: "4px 10px",
            borderRadius: 6,
            border: "1px solid #3b82f6",
            backgroundColor: hasDisplayedTracks ? "#ffffff" : "#e5e7eb",
            color: hasDisplayedTracks ? "#1d4ed8" : "#9ca3af",
            fontSize: 12,
            cursor: hasDisplayedTracks ? "pointer" : "default",
          }}
        >
          🔀 随机播放
        </button>

        <button
          type="button"
          onClick={onImport}
          style={{
            padding: "4px 10px",
            borderRadius: 6,
            border: "none",
            background: "#8b5cf6",
            color: "#ffffff",
            fontSize: 13,
            cursor: "pointer",
          }}
        >
          + 导入本地音乐文件
        </button>

        <button
          type="button"
          onClick={onClearLibrary}
          style={{
            padding: "4px 10px",
            borderRadius: 6,
            border: "none",
            background: "#f3f4f6",
            color: "#4b5563",
            fontSize: 13,
            cursor: "pointer",
          }}
        >
          清空资料库
        </button>
      </div>
    </div>
  );
};
