// src/pages/NowPlayingPage.tsx
import React, { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";
import { setCoverForTrack } from "../persistence/CoverCache";

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

  for (const name of candidates) {
    const guess =
      dir.length > 0 ? `${dir}/${name}` : name;

    // 这里先直接返回路径，交给 AudioEngine / 浏览器去加载.
    // 后续 B2 阶段会改成「按目录缓存 + 减少 500 spam」。
    return guess;
  }

  return null;
}

const NowPlayingPage: React.FC = () => {
  const playlist = usePlayerStore((s) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s) => s.currentIndex ?? 0);
  const isPlaying = usePlayerStore((s) => s.isPlaying ?? false);
  const currentTime = usePlayerStore((s) => s.currentTime ?? 0);
  const duration = usePlayerStore((s) => s.duration ?? 0);
  const seek = usePlayerStore((s) => s.seek ?? (() => {}));
  const togglePlay = usePlayerStore((s) => s.togglePlay ?? (() => {}));
  const next = usePlayerStore((s) => s.next ?? (() => {}));
  const prev = usePlayerStore((s) => s.prev ?? (() => {}));
  const setPlaylist = usePlayerStore(
    (s) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  );

  const track = playlist && playlist.length > 0
    ? playlist[currentIndex] ?? null
    : null;

  const [coverError, setCoverError] = useState(false);

  useEffect(() => {
    // 切歌时重置封面错误状态
    setCoverError(false);
  }, [track && track.id, currentIndex]);

  const coverSrc = useMemo(() => {
    if (!track || coverError) return null;

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
  }, [track, coverError]);

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

      // 通过 CoverCache 把图片复制到 AppData 封面仓库，
      // 返回一个 coverPath 已经指向缓存文件的新 track
      const updatedTrack = await setCoverForTrack(track as any, fullPath);

      // 更新内存中的 playlist，只改当前这条
      const updated = playlist.map((t: any, idx: number) =>
        idx === currentIndex ? { ...t, coverPath: updatedTrack.coverPath } : t,
      );

      setPlaylist(updated);
      setCoverError(false);

      // 同步写回 JSON 库文件
      try {
        await saveLibrary(updated as any[]);
        console.info("[NowPlaying] saveLibrary with cached cover ok");
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
        <div
          style={{
            padding: 16,
            borderRadius: 8,
            border: "1px dashed #d1d5db",
            background: "#f9fafb",
          }}
        >
          当前没有正在播放的歌曲。你可以先到「资料库」中导入并播放一首歌，然后再回到这里查看正在播放视图。
        </div>
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
          gap: 24,
          alignItems: "stretch",
        }}
      >
        {/* 左边：封面区 */}
        <div
          style={{
            width: 320,
            height: 320,
            borderRadius: 24,
            overflow: "hidden",
            background: `linear-gradient(135deg, ${g1}, ${g2})`,
            boxShadow:
              "0 18px 45px rgba(15,23,42,0.25)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            position: "relative",
          }}
        >
          {coverSrc && !coverError ? (
            <img
              src={coverSrc}
              alt={title}
              style={{
                width: "100%",
                height: "100%",
                objectFit: "cover",
              }}
              onError={() => {
                console.warn("[NowPlaying] cover load error");
                setCoverError(true);
              }}
            />
          ) : (
            <div
              style={{
                padding: 16,
                textAlign: "center",
                color: "#f9fafb",
              }}
            >
              <div
                style={{
                  fontSize: 18,
                  fontWeight: 500,
                  marginBottom: 8,
                }}
              >
                暂无封面图片
              </div>
              <div
                style={{
                  fontSize: 12,
                  opacity: 0.85,
                  lineHeight: 1.5,
                }}
              >
                你可以点击右侧的「选择封面图片…」按钮手动设置，
                <br />
                或者在歌曲所在文件夹中放置
                <code style={{ margin: "0 4px" }}>cover.jpg</code>、
                <code style={{ margin: "0 4px" }}>folder.jpg</code>
                等文件，系统会尝试自动识别。
              </div>
            </div>
          )}
        </div>

        {/* 右边：信息 + 控制区 */}
        <div
          style={{
            flex: 1,
            display: "flex",
            flexDirection: "column",
            gap: 16,
          }}
        >
          {/* 标题 / 艺人 / 专辑 */}
          <div>
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

          {/* 时间进度 */}
          <div>
            <div
              style={{
                fontSize: 12,
                color: "#6b7280",
                marginBottom: 4,
              }}
            >
              {timeText}
            </div>
            <input
              type="range"
              min={0}
              max={duration || 0}
              step={1}
              value={Math.min(currentTime || 0, duration || 0)}
              onChange={(e) => {
                const value = Number(e.target.value);
                if (Number.isFinite(value)) {
                  seek(value);
                }
              }}
              style={{ width: "100%" }}
            />
          </div>

          {/* 控制按钮 */}
          <div
            style={{
              display: "flex",
              alignItems: "center",
              gap: 12,
            }}
          >
            <button
              onClick={prev}
              style={{
                padding: "6px 10px",
                fontSize: 13,
                borderRadius: 999,
                border: "1px solid #e5e7eb",
                background: "#ffffff",
                cursor: "pointer",
              }}
            >
              ⏮ 上一首
            </button>
            <button
              onClick={togglePlay}
              style={{
                padding: "6px 16px",
                fontSize: 14,
                borderRadius: 999,
                border: "1px solid #1d4ed8",
                background: "#1d4ed8",
                color: "#ffffff",
                cursor: "pointer",
                minWidth: 90,
              }}
            >
              {isPlaying ? "⏸ 暂停" : "▶ 播放"}
            </button>
            <button
              onClick={next}
              style={{
                padding: "6px 10px",
                fontSize: 13,
                borderRadius: 999,
                border: "1px solid #e5e7eb",
                background: "#ffffff",
                cursor: "pointer",
              }}
            >
              ⏭ 下一首
            </button>
          </div>

          {/* 底部：封面操作 */}
          <div
            style={{
              marginTop: "auto",
              paddingTop: 16,
              borderTop: "1px solid #e5e7eb",
              display: "flex",
              alignItems: "center",
            }}
          >
            <button
              onClick={handlePickCover}
              style={{
                padding: "6px 12px",
                fontSize: 13,
                borderRadius: 999,
                border: "1px solid #e5e7eb",
                background: "#f9fafb",
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
              （现在会把你选择的封面复制到 AppData/com.administrator.kivo-music/covers 里，
              原图删掉或挪走也不影响显示。）
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NowPlayingPage;
