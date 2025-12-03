// src/components/TrackList.tsx
import React, { useMemo, useRef } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { MusicTrack } from "../types";

export interface TrackListProps {
  tracks: MusicTrack[];
  /** 双击播放 */
  onPlay?: (track: MusicTrack, index: number) => void;
  /** 当前正在播放的 track id，用来高亮行 */
  activeTrackId?: string | number | null;
}

export const TrackList: React.FC<TrackListProps> = ({
  tracks,
  onPlay,
  activeTrackId,
}) => {
  const parentRef = useRef<HTMLDivElement | null>(null);

  const rowVirtualizer = useVirtualizer({
    count: tracks.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 40,
    overscan: 10,
  });

  const rows = rowVirtualizer.getVirtualItems();

  const containerHeight = useMemo(() => {
    // 简单给个相对合适的高度，避免顶到底
    return "calc(100vh - 220px)";
  }, []);

  return (
    <div
      ref={parentRef}
      style={{
        position: "relative",
        width: "100%",
        height: containerHeight,
        overflow: "auto",
        borderRadius: 8,
        border: "1px solid #e5e7eb",
        backgroundColor: "#ffffff",
        fontSize: 13,
      }}
    >
      <div
        style={{
          height: rowVirtualizer.getTotalSize(),
          width: "100%",
          position: "relative",
        }}
      >
        {rows.map((virtualRow) => {
          const index = virtualRow.index;
          const track = tracks[index];
          if (!track) return null;

          const key =
            (track.id as any) ??
            track.filePath ??
            track.path ??
            index;

          const title =
            track.title || (track as any).name || "未知标题";
          const artist = track.artist || "未知艺人";
          const album = track.album || "";

          // 当前行是否是“正在播放”的那一首
          const isActive =
            activeTrackId != null &&
            (track.id === activeTrackId ||
              track.filePath === activeTrackId ||
              track.path === activeTrackId);

          // 行底色：正在播 > 斑马纹
          const baseBg =
            index % 2 === 0 ? "#ffffff" : "#f9fafb";
          const activeBg = "#eff6ff";

          // 小封面：只用 coverPath（来自手动设置 / 缓存），不再猜 cover.jpg
          let coverSrc: string | null = null;
          if (track.coverPath && typeof track.coverPath === "string") {
            try {
              coverSrc = convertFileSrc(track.coverPath);
            } catch {
              coverSrc = null;
            }
          }

          const handleDoubleClick = () => {
            onPlay?.(track, index);
          };

          return (
            <div
              key={key}
              data-index={index}
              onDoubleClick={handleDoubleClick}
              style={{
                position: "absolute",
                top: 0,
                left: 0,
                width: "100%",
                height: virtualRow.size,
                transform: `translateY(${virtualRow.start}px)`,
                display: "flex",
                alignItems: "center",
                padding: "4px 8px",
                boxSizing: "border-box",
                cursor: "default",
                backgroundColor: isActive ? activeBg : baseBg,
              }}
            >
              {/* 序号 */}
              <div
                style={{
                  width: 32,
                  textAlign: "right",
                  paddingRight: 8,
                  color: isActive ? "#2563eb" : "#9ca3af",
                  fontSize: 11,
                }}
              >
                {index + 1}
              </div>

              {/* 小封面 */}
              <div
                style={{
                  width: 32,
                  height: 32,
                  marginRight: 10,
                  borderRadius: 6,
                  overflow: "hidden",
                  backgroundColor: "#e5e7eb",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  fontSize: 10,
                  color: "#6b7280",
                  flexShrink: 0,
                }}
              >
                {coverSrc ? (
                  <img
                    src={coverSrc}
                    alt={title}
                    style={{
                      width: "100%",
                      height: "100%",
                      objectFit: "cover",
                    }}
                    onError={(e) => {
                      (e.currentTarget as HTMLImageElement).style.display =
                        "none";
                    }}
                  />
                ) : (
                  <span>封面</span>
                )}
              </div>

              {/* 标题 / 艺人 / 专辑 */}
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 12,
                  flex: 1,
                  minWidth: 0,
                }}
              >
                <div
                  style={{
                    flex: 2,
                    minWidth: 0,
                  }}
                >
                  <div
                    style={{
                      whiteSpace: "nowrap",
                      overflow: "hidden",
                      textOverflow: "ellipsis",
                      color: isActive ? "#1d4ed8" : "#111827",
                      fontWeight: isActive ? 600 : 400,
                    }}
                  >
                    {title}
                  </div>
                  <div
                    style={{
                      whiteSpace: "nowrap",
                      overflow: "hidden",
                      textOverflow: "ellipsis",
                      fontSize: 11,
                      color: "#6b7280",
                    }}
                  >
                    {artist}
                  </div>
                </div>

                <div
                  style={{
                    flex: 1,
                    minWidth: 0,
                    whiteSpace: "nowrap",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    fontSize: 11,
                    color: "#9ca3af",
                  }}
                >
                  {album}
                </div>
              </div>
            </div>
          );
        })}

        {tracks.length === 0 && (
          <div
            style={{
              position: "absolute",
              inset: 0,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              color: "#9ca3af",
              fontSize: 13,
            }}
          >
            当前资料库为空，请先导入本地音乐文件。
          </div>
        )}
      </div>
    </div>
  );
};
