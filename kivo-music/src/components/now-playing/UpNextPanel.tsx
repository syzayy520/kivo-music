// src/components/now-playing/UpNextPanel.tsx
import React from "react";
import { usePlayerStore } from "../../store/player";
import { kivoTheme } from "../../styles/theme";
import {
  clearQueue,
  playFromQueue,
  removeFromQueue,
} from "../../playlists/playQueueModel";

/**
 * 接下来播放（Up Next）面板。
 * - 读取 playerStore 中的 playlist / currentIndex。
 * - 双击某行可以从该曲目开始播放。
 * - 支持移除后续歌曲、清空队列。
 */
export const UpNextPanel: React.FC = () => {
  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);

  // 队列为空时的占位
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
        <div>当前播放队列为空。可以在资料库或播放列表中选择歌曲开始播放。</div>
      </div>
    );
  }

  const headerTitle = "接下来播放";
  const statusText =
    currentIndex >= 0 && currentIndex < playlist.length
      ? `当前第 ${currentIndex + 1} / ${playlist.length} 首 · ${
          isPlaying ? "播放中" : "已暂停"
        }`
      : `共 ${playlist.length} 首曲目`;

  const handleClear = () => clearQueue();
  const handleRemove = (idx: number) => removeFromQueue(idx);
  const handlePlayFrom = (idx: number) => playFromQueue(idx);

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

      {/* 列表 */}
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
        <div
          style={{
            flex: 1,
            minHeight: 0,
            overflowY: "auto",
          }}
        >
          {playlist.map((track: any, index: number) => {
            const isCurrent = index === currentIndex;
            const isNext = index === currentIndex + 1;
            const canRemove = !isCurrent;

            // 关键修改：即使 filePath 一样，也用 index 拼接，保证 key 唯一
            const filePath =
              track?.filePath ?? track?.path ?? track?.location ?? "";
            const idPart =
              track?.id ??
              track?.trackId ??
              (filePath ? filePath : track?.title ?? "track");
            const key = `${idPart}::${index}`;

            return (
              <div
                key={key}
                onDoubleClick={() => handlePlayFrom(index)}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 8,
                  padding: "6px 10px",
                  fontSize: 12,
                  cursor: "pointer",
                  backgroundColor: isCurrent
                    ? "rgba(37,99,235,0.10)"
                    : "transparent",
                  borderBottom: "1px solid rgba(148,163,184,0.25)",
                }}
              >
                {/* 序号 / 状态 */}
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
                    {track?.title || "未知标题"}
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
                    {track?.artist || "未知艺人"}
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

                {/* 删除按钮 */}
                <div
                  style={{
                    width: 40,
                    textAlign: "right",
                  }}
                >
                  {canRemove && (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleRemove(index);
                      }}
                      style={{
                        border: "none",
                        background: "transparent",
                        color: "#9ca3af",
                        cursor: "pointer",
                        fontSize: 14,
                      }}
                      title="从当前队列中移除这首歌"
                    >
                      ✕
                    </button>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default UpNextPanel;
