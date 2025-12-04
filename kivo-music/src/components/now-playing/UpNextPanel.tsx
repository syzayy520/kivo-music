// src/components/now-playing/UpNextPanel.tsx
import React from "react";
import { usePlayerStore } from "../../store/player";
import { kivoTheme } from "../../styles/theme";
import {
  clearQueue,
  playFromQueue,
  removeFromQueue,
  moveInQueue,
} from "../../playlists/playQueueModel";
import { UpNextRow } from "./UpNextRow";

/**
 * 接下来播放（Up Next）面板。
 *
 * 设计原则：
 * - 这里只关心“拿数据 + 组织布局 + 调用队列模型”；
 * - 单行的展示和交互细节拆到 UpNextRow，避免文件过长；
 * - 所有对队列的修改统一走 playQueueModel。
 */
export const UpNextPanel: React.FC = () => {
  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);

  // 队列为空时的占位视图
  if (!playlist || playlist.length === 0) {
    return (
      <div
        style={{
          flex: 1,
          minHeight: 0,
          borderRadius: kivoTheme.radius.lg,
          border: `1px dashed ${kivoTheme.colors.borderSubtle}`,
          background: "#ffffff",
          padding: kivoTheme.spacing.lg,
          fontSize: 13,
          color: "#9ca3af",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
          textAlign: "center",
          gap: 8,
        }}
      >
        <div style={{ fontSize: 14, fontWeight: 600, color: "#111827" }}>
          接下来播放
        </div>
        <div>
          当前播放队列为空。可以在资料库或播放列表中选择歌曲开始播放。
        </div>
      </div>
    );
  }

  const total = playlist.length;
  const headerTitle = "接下来播放";
  const statusText =
    currentIndex >= 0 && currentIndex < total
      ? `当前第 ${currentIndex + 1} / ${total} 首 · ${
          isPlaying ? "播放中" : "已暂停"
        }`
      : `共 ${total} 首曲目`;

  const handleClear = () => clearQueue();

  return (
    <div
      style={{
        flex: 1,
        minHeight: 0,
        borderRadius: kivoTheme.radius.lg,
        border: `1px solid ${kivoTheme.colors.borderSubtle}`,
        background: "#ffffff",
        padding: kivoTheme.spacing.lg,
        fontSize: 13,
        color: "#111827",
        display: "flex",
        flexDirection: "column",
        gap: kivoTheme.spacing.sm,
      }}
    >
      {/* 头部 */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: kivoTheme.spacing.sm,
        }}
      >
        <div>
          <div
            style={{
              fontSize: 14,
              fontWeight: 600,
              marginBottom: 2,
            }}
          >
            {headerTitle}
          </div>
          <div
            style={{
              fontSize: 12,
              color: "#6b7280",
            }}
          >
            {statusText}
          </div>
        </div>

        <button
          onClick={handleClear}
          style={{
            padding: "4px 10px",
            fontSize: 12,
            borderRadius: 9999,
            border: "1px solid #ef4444",
            background: "rgba(248,113,113,0.08)",
            color: "#b91c1c",
            cursor: "pointer",
            whiteSpace: "nowrap",
          }}
        >
          清空队列
        </button>
      </div>

      {/* 列表区域 */}
      <div
        style={{
          flex: 1,
          minHeight: 0,
          marginTop: kivoTheme.spacing.sm,
          borderRadius: kivoTheme.radius.md,
          background: "#f9fafb",
          overflow: "hidden",
          display: "flex",
          flexDirection: "column",
        }}
      >
        {/* 列头 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            padding: "6px 10px",
            fontSize: 11,
            color: "#6b7280",
            borderBottom: "1px solid rgba(148,163,184,0.4)",
            background: "rgba(15,23,42,0.02)",
          }}
        >
          <div
            style={{
              width: 28,
              textAlign: "right",
              paddingRight: 4,
            }}
          >
            #
          </div>
          <div
            style={{
              flex: 1,
              minWidth: 0,
            }}
          >
            曲目 / 艺人
          </div>
          <div
            style={{
              minWidth: 64,
              textAlign: "right",
            }}
          >
            状态
          </div>
          <div
            style={{
              minWidth: 88,
              textAlign: "right",
            }}
          >
            操作
          </div>
        </div>

        {/* 列表内容 */}
        <div
          style={{
            flex: 1,
            minHeight: 0,
            overflowY: "auto",
          }}
        >
          {playlist.map((track, index) => {
            const isCurrent = index === currentIndex;
            const isNext = index === currentIndex + 1;
            const canMoveUp = index > 0;
            const canMoveDown = index < total - 1;
            const canRemove = !isCurrent;

            const handlePlayFrom = () => playFromQueue(index);
            const handleMoveUp = () => moveInQueue(index, index - 1);
            const handleMoveDown = () => moveInQueue(index, index + 1);
            const handleRemove = () => removeFromQueue(index);

            return (
              <UpNextRow
                // 即使 filePath 一样，也用 index 拼接，保证 key 唯一
                key={buildTrackRowKey(track, index)}
                track={track}
                index={index}
                isCurrent={isCurrent}
                isNext={isNext}
                canMoveUp={canMoveUp}
                canMoveDown={canMoveDown}
                isRemovable={canRemove}
                onPlay={handlePlayFrom}
                onMoveUp={handleMoveUp}
                onMoveDown={handleMoveDown}
                onRemove={canRemove ? handleRemove : undefined}
              />
            );
          })}
        </div>
      </div>
    </div>
  );
};

function buildTrackRowKey(track: any, index: number): string {
  const filePath = track?.filePath ?? track?.path ?? track?.location ?? "";
  const idPart =
    track?.id ??
    track?.trackId ??
    (filePath ? filePath : track?.title ?? "track");
  return `${idPart}::${index}`;
}

export default UpNextPanel;
