// src/pages/NowPlayingPage.tsx
import React, { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";

function formatTime(value: number | undefined): string {
  if (!value || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}

// 和 TrackList 一样的渐变颜色逻辑
const pickGradientForKey = (key: string | undefined): [string, string] => {
  const palettes: [string, string][] = [
    ["#6366f1", "#ec4899"],
    ["#0ea5e9", "#22c55e"],
    ["#f97316", "#facc15"],
    ["#14b8a6", "#3b82f6"],
    ["#e11d48", "#f97316"],
    ["#8b5cf6", "#ec4899"],
  ];

  const k = key && key.length > 0 ? key : "kivo-default";
  let hash = 0;
  for (let i = 0; i < k.length; i++) {
    hash = (hash * 31 + k.charCodeAt(i)) | 0;
  }
  const index = Math.abs(hash) % palettes.length;
  return palettes[index];
};

function guessCoverPathFromFilePath(filePath: string | undefined): string | null {
  if (!filePath) return null;

  const str = String(filePath);
  const parts = str.split(/[/\\]/);
  if (parts.length === 0) return null;

  const fileName = parts[parts.length - 1];
  const dir = parts.slice(0, parts.length - 1).join(
    str.includes("\\") ? "\\" : "/",
  );

  const baseName = fileName.replace(/\.[^.]+$/, "");

  const candidates = [
        "cover.jpg",
        "cover.png",
        "folder.jpg",
        "folder.png",
        "front.jpg",
        "front.png",
        `${baseName}.jpg`,
        `${baseName}.png`,
      ];

  const sep =
    dir.endsWith("\\") || dir.endsWith("/")
      ? ""
      : str.includes("\\")
      ? "\\"
      : "/";

  return dir ? `${dir}${sep}${candidates[0]}` : candidates[0];
}

const NowPlayingPage: React.FC = () => {
  const playlist = usePlayerStore(
    (s: any) => s.playlist ?? s.tracks ?? [],
  );
  const currentIndex = usePlayerStore(
    (s: any) => s.currentIndex ?? -1,
  );
  const currentTime = usePlayerStore(
    (s: any) => s.currentTime ?? 0,
  );
  const duration = usePlayerStore(
    (s: any) => s.duration ?? 0,
  );
  const isPlaying = usePlayerStore(
    (s: any) => s.isPlaying ?? false,
  );
  const togglePlay = usePlayerStore(
    (s: any) => s.togglePlay ?? (() => {}),
  );
  const next = usePlayerStore((s: any) => s.next ?? (() => {}));
  const prev = usePlayerStore((s: any) => s.prev ?? (() => {}));
  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  );

  const track = playlist[currentIndex] ?? null;

  // 当前封面是否加载失败（比如我们猜的 cover.jpg 不存在）
  const [coverError, setCoverError] = useState(false);
  useEffect(() => {
    // 切歌时重置错误状态
    setCoverError(false);
  }, [currentIndex]);

  const coverSrc = useMemo(() => {
    if (!track) return null;

    // 1. 手动选过封面：优先用
    if (track.coverPath) {
      return convertFileSrc(String(track.coverPath));
    }

    // 2. 自动猜当前文件夹下的 cover.jpg / folder.jpg
    if (track.filePath) {
      const guessed = guessCoverPathFromFilePath(String(track.filePath));
      if (guessed) {
        return convertFileSrc(guessed);
      }
    }

    return null;
  }, [track]);

  const [g1, g2] = pickGradientForKey(
    track?.id ?? track?.title ?? track?.filePath,
  );

  const handlePickCover = async () => {
    if (!track) return;

    try {
      const result = await open({
        multiple: false,
        filters: [
          {
            name: "Image",
            extensions: ["jpg", "jpeg", "png", "webp", "bmp"],
          },
        ],
      });

      if (!result) return;

      const path = Array.isArray(result) ? result[0] : result;
      const fullPath = String(path);

      // 更新内存中的 playlist
      const updated = playlist.map((t: any, idx: number) =>
        idx === currentIndex ? { ...t, coverPath: fullPath } : t,
      );

      setPlaylist(updated);

      // 同步写回 JSON 库文件
      try {
        await saveLibrary(updated as any[]);
        console.info("[NowPlaying] saveLibrary with cover ok");
      } catch (error) {
        console.error("[NowPlaying] saveLibrary failed:", error);
      }
    } catch (error) {
      console.error("[NowPlaying] pick cover failed:", error);
    }
  };

  if (!track) {
    return (
      <div
        style={{
          padding: "32px 24px",
          fontSize: 14,
          color: "#6b7280",
        }}
      >
        当前没有正在播放的歌曲。你可以先到「资料库」中导入并播放一首歌，然后再回到这里查看正在播放视图。
      </div>
    );
  }

  const title = track.title || "未命名歌曲";
  const artist = track.artist || "未知艺人";
  const album = track.album || "";
  const timeText = `${formatTime(currentTime)} / ${formatTime(duration)}`;

  return (
    <div
      style={{
        padding: "32px 24px",
        display: "flex",
        flexDirection: "column",
        gap: 24,
      }}
    >
      <h2
        style={{
          fontSize: 20,
          fontWeight: 600,
          marginBottom: 4,
        }}
      >
        正在播放
      </h2>

      <div
        style={{
          display: "flex",
          gap: 32,
          alignItems: "flex-start",
          flexWrap: "wrap",
        }}
      >
        {/* 左侧：大封面卡片 */}
        <div
          style={{
            width: 260,
            height: 260,
            borderRadius: 24,
            background: coverSrc && !coverError
              ? "#000000"
              : `linear-gradient(135deg, ${g1}, ${g2})`,
            boxShadow: "0 18px 40px rgba(15,23,42,0.28)",
            overflow: "hidden",
            position: "relative",
          }}
        >
          {coverSrc && !coverError ? (
            <img
              src={coverSrc}
              alt=""
              style={{
                width: "100%",
                height: "100%",
                objectFit: "cover",
              }}
              onError={() => setCoverError(true)}
            />
          ) : (
            <div
              style={{
                position: "absolute",
                inset: 0,
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center",
                color: "white",
                textShadow: "0 2px 6px rgba(0,0,0,0.4)",
                padding: "0 32px",
                textAlign: "center",
              }}
            >
              <div
                style={{
                  fontSize: 16,
                  fontWeight: 600,
                  marginBottom: 6,
                }}
              >
                封面图
              </div>
              <div
                style={{
                  fontSize: 12,
                  opacity: 0.9,
                  lineHeight: 1.4,
                }}
              >
                如果同目录下存在 cover.jpg / folder.jpg 等文件，会尝试自动作为封面。
                你也可以在右侧手动选择一张图片。
              </div>
            </div>
          )}
        </div>

        {/* 右侧：信息 + 操作 */}
        <div
          style={{
            flex: 1,
            minWidth: 260,
          }}
        >
          <div
            style={{
              marginBottom: 12,
            }}
          >
            <div
              style={{
                fontSize: 13,
                color: "#9ca3af",
                marginBottom: 4,
              }}
            >
              正在播放：
            </div>
            <div
              style={{
                fontSize: 22,
                fontWeight: 600,
                marginBottom: 4,
              }}
            >
              {title}
            </div>
            <div
              style={{
                fontSize: 14,
                color: "#6b7280",
              }}
            >
              {artist}
              {album ? ` · ${album}` : ""}
            </div>
          </div>

          <div
            style={{
              marginBottom: 12,
              fontSize: 13,
              color: "#6b7280",
            }}
          >
            {timeText}
          </div>

          <div
            style={{
              display: "flex",
              gap: 8,
              marginBottom: 16,
              flexWrap: "wrap",
            }}
          >
            <button
              onClick={prev}
              style={{
                padding: "6px 10px",
                borderRadius: 6,
                border: "1px solid #e5e7eb",
                background: "#ffffff",
                fontSize: 13,
                cursor: "pointer",
              }}
            >
              ⏮ 上一首
            </button>
            <button
              onClick={togglePlay}
              style={{
                padding: "6px 10px",
                borderRadius: 6,
                border: "1px solid #2563eb",
                background: "#2563eb",
                color: "#ffffff",
                fontSize: 13,
                cursor: "pointer",
              }}
            >
              {isPlaying ? "⏸ 暂停" : "▶ 播放"}
            </button>
            <button
              onClick={next}
              style={{
                padding: "6px 10px",
                borderRadius: 6,
                border: "1px solid #e5e7eb",
                background: "#ffffff",
                fontSize: 13,
                cursor: "pointer",
              }}
            >
              ⏭ 下一首
            </button>
          </div>

          <div
            style={{
              marginBottom: 16,
              fontSize: 13,
              color: "#6b7280",
            }}
          >
            正在播放中，你可以在底部进度条拖动 / 切歌、调整音量。
          </div>

          <div
            style={{
              marginTop: 12,
            }}
          >
            <button
              onClick={handlePickCover}
              style={{
                padding: "6px 12px",
                borderRadius: 6,
                border: "1px solid #e5e7eb",
                background: "#ffffff",
                fontSize: 13,
                cursor: "pointer",
              }}
            >
              选择封面图片…
            </button>
            <span
              style={{
                marginLeft: 8,
                fontSize: 12,
                color: "#9ca3af",
              }}
            >
              （当前版本会直接记住你选择的图片路径；
              后面我们会把封面复制到 AppData 的封面缓存目录里。）
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NowPlayingPage;
