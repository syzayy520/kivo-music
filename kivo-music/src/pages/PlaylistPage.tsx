// src/pages/PlaylistPage.tsx
import React from "react";
import { usePlayerStore } from "../store/player";

function formatTime(value: number | undefined): string {
  if (!value || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}

/**
 * 当前播放列表页面
 * - 显示当前队列中的所有歌曲
 * - 高亮当前正在播放的那一条
 * - 双击任意一行从该位置开始播放
 * - 支持一键「清空播放列表」
 */
const PlaylistPage: React.FC = () => {
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);
  const playTrack = usePlayerStore(
    (s: any) => s.playTrack ?? (() => {}),
  ) as (index: number) => void;
  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  ) as (tracks: any[]) => void;

  const total = Array.isArray(playlist) ? playlist.length : 0;
  const hasTracks = total > 0;

  const handleRowDoubleClick = (index: number) => {
    if (!hasTracks) return;
    playTrack(index);
  };

  const handleClearPlaylist = () => {
    if (!hasTracks) return;
    // 直接把播放列表设为空，player store 会把 currentIndex 重置为 -1
    setPlaylist([]);
  };

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
      }}
    >
      {/* 顶部标题区 */}
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
            共 {total} 首歌曲
            {currentIndex >= 0 &&
              currentIndex < total &&
              ` · 正在播放第 ${currentIndex + 1} 首`}
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
            onClick={handleClearPlaylist}
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

      {/* 列表区域 */}
      <div
        style={{
          flex: 1,
          borderRadius: 8,
          border: "1px solid #e5e7eb",
          overflow: "hidden",
          backgroundColor: "#ffffff",
        }}
      >
        {!hasTracks ? (
          <div
            style={{
              height: "100%",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              fontSize: 13,
              color: "#9ca3af",
            }}
          >
            当前播放列表为空，请在资料库中双击歌曲开始播放。
          </div>
        ) : (
          <div
            style={{
              height: "100%",
              overflow: "auto",
            }}
          >
            <table
              style={{
                width: "100%",
                borderCollapse: "collapse",
                fontSize: 13,
              }}
            >
              <thead>
                <tr
                  style={{
                    backgroundColor: "#f9fafb",
                    borderBottom: "1px solid #e5e7eb",
                  }}
                >
                  <th
                    style={{
                      width: 40,
                      padding: "6px 8px",
                      textAlign: "right",
                      fontWeight: 500,
                      color: "#6b7280",
                    }}
                  >
                    #
                  </th>
                  <th
                    style={{
                      padding: "6px 8px",
                      textAlign: "left",
                      fontWeight: 500,
                      color: "#6b7280",
                    }}
                  >
                    标题
                  </th>
                  <th
                    style={{
                      padding: "6px 8px",
                      textAlign: "left",
                      fontWeight: 500,
                      color: "#6b7280",
                    }}
                  >
                    艺人
                  </th>
                  <th
                    style={{
                      padding: "6px 8px",
                      textAlign: "left",
                      fontWeight: 500,
                      color: "#6b7280",
                    }}
                  >
                    专辑
                  </th>
                  <th
                    style={{
                      padding: "6px 8px",
                      textAlign: "right",
                      fontWeight: 500,
                      color: "#6b7280",
                      width: 80,
                    }}
                  >
                    时长
                  </th>
                </tr>
              </thead>
              <tbody>
                {playlist.map((raw: any, index: number) => {
                  const isActive = index === currentIndex;

                  const title =
                    (raw && raw.title) || "未命名歌曲";
                  const artist =
                    (raw && raw.artist) || "未知艺人";
                  const album =
                    (raw && raw.album) || "";
                  const duration =
                    (raw && raw.duration) || 0;

                  const baseBg =
                    index % 2 === 0 ? "#ffffff" : "#f9fafb";
                  const rowBg = isActive ? "#eff6ff" : baseBg;
                  const titleColor = isActive
                    ? "#1d4ed8"
                    : "#111827";

                  return (
                    <tr
                      key={(raw && (raw.id || raw.filePath)) ?? index}
                      onDoubleClick={() =>
                        handleRowDoubleClick(index)
                      }
                      style={{
                        backgroundColor: rowBg,
                        cursor: "default",
                      }}
                    >
                      <td
                        style={{
                          padding: "4px 8px",
                          textAlign: "right",
                          fontSize: 12,
                          color: isActive
                            ? "#2563eb"
                            : "#9ca3af",
                        }}
                      >
                        {index + 1}
                      </td>
                      <td
                        style={{
                          padding: "4px 8px",
                          maxWidth: 260,
                          whiteSpace: "nowrap",
                          overflow: "hidden",
                          textOverflow: "ellipsis",
                          color: titleColor,
                          fontWeight: isActive ? 600 : 400,
                        }}
                      >
                        {title}
                      </td>
                      <td
                        style={{
                          padding: "4px 8px",
                          maxWidth: 180,
                          whiteSpace: "nowrap",
                          overflow: "hidden",
                          textOverflow: "ellipsis",
                          color: "#4b5563",
                        }}
                      >
                        {artist}
                      </td>
                      <td
                        style={{
                          padding: "4px 8px",
                          maxWidth: 220,
                          whiteSpace: "nowrap",
                          overflow: "hidden",
                          textOverflow: "ellipsis",
                          color: "#9ca3af",
                        }}
                      >
                        {album}
                      </td>
                      <td
                        style={{
                          padding: "4px 8px",
                          textAlign: "right",
                          fontVariantNumeric: "tabular-nums",
                          color: "#6b7280",
                          fontSize: 12,
                        }}
                      >
                        {formatTime(duration)}
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>

      <div
        style={{
          marginTop: 8,
          fontSize: 11,
          color: "#9ca3af",
        }}
      >
        提示：双击任意一行可以从该位置开始播放。后续版本会在此基础上增加「拖动排序 /
        从队列中移除 / 保存为歌单」等高级功能。
      </div>
    </div>
  );
};

export default PlaylistPage;
