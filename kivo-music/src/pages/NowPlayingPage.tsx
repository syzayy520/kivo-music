import React, { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";

type AnyTrack = any;

// 简单时间格式化：秒 → mm:ss
const formatTime = (value: number | undefined | null) => {
  if (value == null || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
};

// 根据 track 的 id / 标题 生成一组渐变颜色（封面缺失时的占位图）
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

// 从文件路径推断封面图片的本地路径（不访问硬盘，只做字符串处理）
function guessCoverPathFromFilePath(filePath: string | undefined): string | null {
  if (!filePath) return null;

  const str = String(filePath);
  const parts = str.split(/[/\\]/);
  if (parts.length === 0) return null;

  const fileName = parts[parts.length - 1];
  const dir = parts.slice(0, parts.length - 1).join(
    str.includes("\\") ? "\\" : "/",
  );

  const baseName = fileName.replace(/\.[^.]+$/, ""); // 去掉扩展名

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

  const sep = dir.endsWith("\\") || dir.endsWith("/") ? "" : str.includes("\\") ? "\\" : "/";
  // 只取第一个候选；是否真实存在交给 <img> 自己试，如果失败我们会回退占位图
  return dir ? `${dir}${sep}${candidates[0]}` : candidates[0];
}

const NowPlayingPage: React.FC = () => {
  // 播放相关状态（为了兼容之前写法，这里都加了兜底）
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? 0);
  const isPlaying = usePlayerStore((s: any) => s.isPlaying ?? s.playing ?? false);
  const currentTime = usePlayerStore((s: any) => s.currentTime ?? s.time ?? 0);
  const duration = usePlayerStore((s: any) => s.duration ?? 0);

  const next = usePlayerStore((s: any) => s.next ?? (() => {}));
  const prev = usePlayerStore((s: any) => s.prev ?? (() => {}));
  const togglePlay = usePlayerStore((s: any) => s.togglePlay ?? s.toggle ?? (() => {}));
  const setPlaylist = usePlayerStore((s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}));

  const track = playlist[currentIndex] as AnyTrack | undefined;

  const [c1, c2] = useMemo(
    () => pickGradientForKey(track?.id ?? track?.title),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [track?.id, track?.title],
  );

  const title = track?.title ?? "未选择歌曲";
  const artist = track?.artist ?? "未知艺人";

  // 封面相关状态
  const [coverSrc, setCoverSrc] = useState<string | null>(null);
  const [coverFailed, setCoverFailed] = useState(false);

  // 每次当前歌曲或 coverPath 变化时，重新推断封面路径
  useEffect(() => {
    if (!track) {
      setCoverSrc(null);
      setCoverFailed(false);
      return;
    }

    const explicitCoverPath: string | undefined = track.coverPath;
    const pathToUse = explicitCoverPath || guessCoverPathFromFilePath(track.filePath);

    if (!pathToUse) {
      setCoverSrc(null);
      setCoverFailed(false);
      return;
    }

    const src = convertFileSrc(pathToUse);
    setCoverSrc(src);
    setCoverFailed(false);
  }, [track?.filePath, track?.coverPath]);

  // 让用户手动选择一张封面图片，并写入 playlist + 库文件
  const handleChooseCover = async () => {
    if (!track) return;

    try {
      const result = await open({
        multiple: false,
        filters: [
          {
            name: "图片",
            extensions: ["jpg", "jpeg", "png", "webp"],
          },
        ],
      });

      if (!result) return;

      const path = Array.isArray(result) ? String(result[0]) : String(result);

      const updated: AnyTrack[] = (playlist as AnyTrack[]).map((t, idx) =>
        idx === currentIndex ? { ...t, coverPath: path } : t,
      );

      setPlaylist(updated);

      try {
        await saveLibrary(updated);
        console.info(
          "[NowPlaying] saveLibrary after cover change, tracks:",
          updated.length,
        );
      } catch (error) {
        console.error("[NowPlaying] saveLibrary failed:", error);
      }

      const src = convertFileSrc(path);
      setCoverSrc(src);
      setCoverFailed(false);
    } catch (error) {
      console.error("[NowPlaying] choose cover failed:", error);
    }
  };

  return (
    <div style={{ padding: "24px" }}>
      <h2
        style={{
          fontSize: 20,
          fontWeight: 600,
          marginBottom: 16,
        }}
      >
        正在播放
      </h2>

      {!track ? (
        <div style={{ fontSize: 14, color: "#6b7280" }}>
          当前没有正在播放的歌曲，请先在「资料库」中选择一首歌曲播放。
        </div>
      ) : (
        <div
          style={{
            display: "flex",
            gap: 24,
            alignItems: "flex-start",
          }}
        >
          {/* 封面区域：优先用手动封面，其次猜 cover.jpg，最后渐变占位 */}
          <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
            <div
              style={{
                width: 220,
                height: 220,
                borderRadius: 16,
                overflow: "hidden",
                position: "relative",
                boxShadow: "0 10px 30px rgba(15,23,42,0.35)",
                backgroundImage: `linear-gradient(135deg, ${c1}, ${c2})`,
                userSelect: "none",
              }}
            >
              {/* 背景：渐变占位图上的文案 */}
              {(coverFailed || !coverSrc) && (
                <div
                  style={{
                    position: "absolute",
                    inset: 0,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    color: "white",
                    fontSize: 14,
                    textAlign: "center",
                    padding: 16,
                  }}
                >
                  <div>
                    <div style={{ fontWeight: 600, marginBottom: 4 }}>
                      封面图（预留位）
                    </div>
                    <div style={{ fontSize: 12, opacity: 0.9 }}>
                      如果同目录下存在 cover.jpg / folder.jpg 等文件，
                      将自动作为专辑封面显示；你也可以手动选择一张图片。
                    </div>
                  </div>
                </div>
              )}

              {/* 真实封面：加载失败时隐藏并回退到上面的占位文案 */}
              {coverSrc && !coverFailed && (
                <img
                  src={coverSrc}
                  alt={title}
                  style={{
                    width: "100%",
                    height: "100%",
                    objectFit: "cover",
                    display: "block",
                  }}
                  onError={() => setCoverFailed(true)}
                />
              )}
            </div>

            <button
              type="button"
              onClick={handleChooseCover}
              style={{
                width: 220,
                padding: "6px 10px",
                borderRadius: 999,
                border: "1px solid #d1d5db",
                background: "#ffffff",
                fontSize: 12,
                color: "#374151",
                cursor: "pointer",
              }}
            >
              选择封面图片…
            </button>
          </div>

          {/* 右侧：歌曲信息 + 播放状态 */}
          <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ marginBottom: 8, fontSize: 16, color: "#6b7280" }}>
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
                marginBottom: 24,
              }}
            >
              {artist}
            </div>

            {/* 时间 / 进度展示（读底部 PlayerBar 的状态） */}
            <div
              style={{
                fontSize: 14,
                color: "#4b5563",
                marginBottom: 12,
              }}
            >
              {formatTime(currentTime)} / {formatTime(duration)}
            </div>

            {/* 控制按钮：上一首 / 暂停 / 下一首 */}
            <div
              style={{
                display: "flex",
                gap: 12,
                alignItems: "center",
                marginBottom: 16,
              }}
            >
              <button
                type="button"
                onClick={prev}
                style={{
                  padding: "6px 12px",
                  borderRadius: 6,
                  border: "1px solid #d1d5db",
                  background: "white",
                  cursor: "pointer",
                }}
              >
                ⏮ 上一首
              </button>

              <button
                type="button"
                onClick={togglePlay}
                style={{
                  padding: "8px 18px",
                  borderRadius: 999,
                  border: "none",
                  background: "#4f46e5",
                  color: "white",
                  fontWeight: 600,
                  cursor: "pointer",
                }}
              >
                {isPlaying ? "⏸ 暂停" : "▶ 播放"}
              </button>

              <button
                type="button"
                onClick={next}
                style={{
                  padding: "6px 12px",
                  borderRadius: 6,
                  border: "1px solid #d1d5db",
                  background: "white",
                  cursor: "pointer",
                }}
              >
                ⏭ 下一首
              </button>
            </div>

            <div
              style={{
                fontSize: 12,
                color: "#9ca3af",
                lineHeight: 1.6,
              }}
            >
              播放进度、音量、拖动等仍然由底部的播放条负责，
              这里主要是更直观地展示当前播放的歌曲信息。
              <br />
              如果你手动为某首歌选择了封面图片，我们会把路径写入资料库，
              下次打开软件会直接用这张封面；如果没有手动封面，会尝试
              <code>cover.jpg</code> 等文件；都找不到时就显示渐变占位图。
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default NowPlayingPage;
