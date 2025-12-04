// src/components/playlists/PlaylistTable.tsx
import React from "react";

interface PlaylistTableProps {
  playlist: any[];
  currentIndex: number;
  onRowDoubleClick: (index: number) => void;
}

function formatTime(value: number | undefined): string {
  if (!value || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}

/**
 * 播放列表主体表格：
 * - 只负责渲染 table，不参与业务逻辑
 * - 当前播放行高亮
 * - 双击行时通过回调告诉上层
 */
export const PlaylistTable: React.FC<PlaylistTableProps> = ({
  playlist,
  currentIndex,
  onRowDoubleClick,
}) => {
  const hasTracks = Array.isArray(playlist) && playlist.length > 0;

  if (!hasTracks) {
    return (
      <div
        style={{
          flex: 1,
          borderRadius: 8,
          border: "1px solid #e5e7eb",
          backgroundColor: "#ffffff",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          fontSize: 13,
          color: "#9ca3af",
        }}
      >
        当前播放列表为空，请在资料库中双击歌曲开始播放。
      </div>
    );
  }

  return (
    <div
      style={{
        flex: 1,
        borderRadius: 8,
        border: "1px solid #e5e7eb",
        overflow: "hidden",
        backgroundColor: "#ffffff",
      }}
    >
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

              const title = (raw && raw.title) || "未命名歌曲";
              const artist = (raw && raw.artist) || "未知艺人";
              const album = (raw && raw.album) || "";
              const duration = (raw && raw.duration) || 0;

              const baseBg = index % 2 === 0 ? "#ffffff" : "#f9fafb";
              const rowBg = isActive ? "#eff6ff" : baseBg;
              const titleColor = isActive ? "#1d4ed8" : "#111827";

              return (
                <tr
                  key={(raw && (raw.id || raw.filePath)) ?? index}
                  onDoubleClick={() => onRowDoubleClick(index)}
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
                      color: isActive ? "#2563eb" : "#9ca3af",
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
    </div>
  );
};
