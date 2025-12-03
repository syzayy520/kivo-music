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
 * - 双击某一行可以立刻播放这首歌
 */
const PlaylistPage: React.FC = () => {
  // ❗ 关键修改：三个 selector 分开调用，避免返回一个新对象导致 React 报 infinite loop
  const playlist = usePlayerStore(
    (s: any) => s.playlist ?? s.tracks ?? [],
  );
  const currentIndex = usePlayerStore(
    (s: any) => s.currentIndex ?? -1,
  );
  const playTrack = usePlayerStore(
    (s: any) => s.playTrack ?? (() => {}),
  );

  const count = playlist?.length ?? 0;

  return (
    <div style={{ padding: "16px 24px" }}>
      <h2
        style={{
          fontSize: 20,
          fontWeight: 600,
          marginBottom: 4,
        }}
      >
        当前播放列表
      </h2>

      <p
        style={{
          fontSize: 13,
          color: "#6b7280",
          marginBottom: 12,
        }}
      >
        共 {count} 首歌曲。这里显示的是当前播放器实际使用的队列顺序，
        双击某一行可以立即切换到那一首进行播放。
      </p>

      {count === 0 ? (
        <div
          style={{
            padding: "16px 12px",
            fontSize: 13,
            color: "#9ca3af",
            borderRadius: 6,
            border: "1px dashed #e5e7eb",
            background: "#f9fafb",
          }}
        >
          当前播放列表为空。你可以先在「资料库」中导入歌曲并开始播放，
          播放器会自动把它们加入到队列中。
        </div>
      ) : (
        <div
          style={{
            border: "1px solid #e5e7eb",
            borderRadius: 6,
            overflow: "hidden",
            background: "#ffffff",
          }}
        >
          <table
            style={{
              width: "100%",
              borderCollapse: "collapse",
              tableLayout: "fixed",
              fontSize: 13,
            }}
          >
            <thead
              style={{
                background: "#f9fafb",
                borderBottom: "1px solid #e5e7eb",
              }}
            >
              <tr>
                <th
                  style={{
                    width: 48,
                    padding: "6px 8px",
                    textAlign: "right",
                    color: "#6b7280",
                    fontWeight: 500,
                  }}
                >
                  #
                </th>
                <th
                  style={{
                    padding: "6px 8px",
                    textAlign: "left",
                    color: "#6b7280",
                    fontWeight: 500,
                  }}
                >
                  标题
                </th>
                <th
                  style={{
                    width: "28%",
                    padding: "6px 8px",
                    textAlign: "left",
                    color: "#6b7280",
                    fontWeight: 500,
                  }}
                >
                  艺人
                </th>
                <th
                  style={{
                    width: "24%",
                    padding: "6px 8px",
                    textAlign: "left",
                    color: "#6b7280",
                    fontWeight: 500,
                  }}
                >
                  专辑
                </th>
                <th
                  style={{
                    width: 72,
                    padding: "6px 8px",
                    textAlign: "right",
                    color: "#6b7280",
                    fontWeight: 500,
                  }}
                >
                  时长
                </th>
              </tr>
            </thead>
            <tbody>
              {playlist.map((raw: any, index: number) => {
                const isActive = index === currentIndex;

                const title = raw?.title || "未命名歌曲";
                const artist = raw?.artist || "未知艺人";
                const album = raw?.album || "";
                const duration = formatTime(raw?.duration);

                const key = raw?.id || raw?.filePath || index;

                return (
                  <tr
                    key={key}
                    onDoubleClick={() => playTrack(index)}
                    style={{
                      cursor: "pointer",
                      backgroundColor: isActive
                        ? "#eff6ff"
                        : index % 2 === 0
                        ? "#ffffff"
                        : "#f9fafb",
                    }}
                  >
                    <td
                      style={{
                        padding: "6px 8px",
                        textAlign: "right",
                        whiteSpace: "nowrap",
                        color: isActive ? "#2563eb" : "#6b7280",
                        fontWeight: isActive ? 600 : 400,
                      }}
                    >
                      {index + 1}
                    </td>
                    <td
                      style={{
                        padding: "6px 8px",
                        overflow: "hidden",
                        textOverflow: "ellipsis",
                        whiteSpace: "nowrap",
                        color: "#111827",
                        fontWeight: isActive ? 600 : 400,
                      }}
                    >
                      {title}
                    </td>
                    <td
                      style={{
                        padding: "6px 8px",
                        overflow: "hidden",
                        textOverflow: "ellipsis",
                        whiteSpace: "nowrap",
                        color: "#4b5563",
                      }}
                    >
                      {artist}
                    </td>
                    <td
                      style={{
                        padding: "6px 8px",
                        overflow: "hidden",
                        textOverflow: "ellipsis",
                        whiteSpace: "nowrap",
                        color: "#6b7280",
                      }}
                    >
                      {album}
                    </td>
                    <td
                      style={{
                        padding: "6px 8px",
                        textAlign: "right",
                        whiteSpace: "nowrap",
                        color: "#6b7280",
                      }}
                    >
                      {duration}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>

          <div
            style={{
              padding: "6px 8px",
              fontSize: 12,
              color: "#9ca3af",
              borderTop: "1px solid #e5e7eb",
            }}
          >
            提示：这里的顺序就是当前播放顺序。后面我们会增加“拖动排序 /
            从队列中移除”等高级功能。
          </div>
        </div>
      )}
    </div>
  );
};

export default PlaylistPage;
