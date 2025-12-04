// src/components/playlists/PlaylistHeader.tsx
import React from "react";

interface PlaylistHeaderProps {
  total: number;
  currentIndex: number;
  hasTracks: boolean;
  onClearPlaylist: () => void;
}

/**
 * 播放列表顶部区域：
 * - 标题 + 简要统计
 * - 「清空播放列表」按钮
 *
 * 只关心文案和按钮状态，不直接操作 player store。
 */
export const PlaylistHeader: React.FC<PlaylistHeaderProps> = ({
  total,
  currentIndex,
  hasTracks,
  onClearPlaylist,
}) => {
  const subtitleParts: string[] = [`共 ${total} 首歌曲`];
  if (hasTracks && currentIndex >= 0 && currentIndex < total) {
    subtitleParts.push(`正在播放第 ${currentIndex + 1} 首`);
  }

  return (
    <div
      style={{
        marginBottom: 12,
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        gap: 8,
      }}
    >
      <div>
        <div
          style={{
            fontSize: 16,
            fontWeight: 600,
            marginBottom: 2,
          }}
        >
          当前播放列表
        </div>
        <div
          style={{
            fontSize: 12,
            color: "#6b7280",
          }}
        >
          {subtitleParts.join(" · ")}
        </div>
      </div>

      <div
        style={{
          display: "flex",
          gap: 8,
        }}
      >
        <button
          type="button"
          onClick={onClearPlaylist}
          disabled={!hasTracks}
          style={{
            padding: "4px 10px",
            borderRadius: 6,
            border: "1px solid #e5e7eb",
            backgroundColor: hasTracks ? "#ffffff" : "#f3f4f6",
            color: hasTracks ? "#b91c1c" : "#9ca3af",
            fontSize: 12,
            cursor: hasTracks ? "pointer" : "default",
          }}
        >
          清空播放列表
        </button>
      </div>
    </div>
  );
};
