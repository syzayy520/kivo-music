// src/pages/NowPlayingPage.tsx
import React from "react";
import { usePlayer } from "../store/player";

function formatTime(sec: number): string {
  const s = Math.floor(sec || 0);
  const m = Math.floor(s / 60);
  const r = s % 60;
  return `${m}:${r.toString().padStart(2, "0")}`;
}

export const NowPlayingPage: React.FC = () => {
  const queue = usePlayer((s) => s.queue);
  const currentIndex = usePlayer((s) => s.currentIndex);
  const isPlaying = usePlayer((s) => s.isPlaying);
  const currentTime = usePlayer((s) => s.currentTime);
  const duration = usePlayer((s) => s.duration);
  const mode = usePlayer((s) => s.mode);

  const next = usePlayer((s) => s.next);
  const prev = usePlayer((s) => s.prev);
  const setPlaying = usePlayer((s) => s.setPlaying);

  const volume = usePlayer((s) => s.volume);
  const setVolume = usePlayer((s) => s.setVolume);

  const track =
    currentIndex >= 0 && currentIndex < queue.length
      ? queue[currentIndex]
      : null;

  const progressPercent =
    duration && duration > 0 ? Math.min(100, (currentTime / duration) * 100) : 0;

  if (!track) {
    return (
      <div className="h-full flex items-center justify-center text-gray-500 text-sm">
        暂无正在播放的歌曲，请先到「资料库」点一首歌来播放～
      </div>
    );
  }

  const title = track.title || "未知标题";
  const artist = track.artist || "未知艺人";
  const album = (track as any).album || "专辑未知";

  return (
    <div className="h-full flex items-center justify-center">
      <div className="w-full max-w-3xl mx-auto px-6 py-8">
        {/* 顶部信息 */}
        <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between mb-6">
          <div>
            <div className="text-xs text-gray-400 mb-1 tracking-wide">
              正在播放
            </div>
            <h1 className="text-2xl font-semibold text-gray-900">
              {title}
            </h1>
            <div className="mt-1 text-sm text-gray-500">
              {artist} · {album}
            </div>
          </div>

          <div className="text-right text-xs text-gray-400">
            <div>播放模式：{mode === "single" ? "单曲循环" : "顺序播放"}</div>
            <div>
              队列：{queue.length} 首歌，第 {currentIndex + 1} 首
            </div>
          </div>
        </div>

        {/* 歌曲封面占位（以后可以换成真正封面） */}
        <div className="mb-6 flex justify-center">
          <div className="w-48 h-48 rounded-2xl bg-gradient-to-br from-indigo-400 via-pink-400 to-rose-400 shadow-lg flex items-center justify-center text-white text-4xl font-bold select-none">
            ♪
          </div>
        </div>

        {/* 进度条 & 时间 */}
        <div className="mb-4">
          <input
            type="range"
            min={0}
            max={100}
            value={Number.isFinite(progressPercent) ? progressPercent : 0}
            readOnly
            className="w-full cursor-default accent-blue-500"
          />
          <div className="mt-1 flex items-center justify-between text-xs text-gray-500">
            <span>{formatTime(currentTime)}</span>
            <span>{formatTime(duration)}</span>
          </div>
        </div>

        {/* 控制按钮（调用同一个 player store，不会新建音频） */}
        <div className="flex items-center justify-center gap-6 mb-4">
          <button
            onClick={prev}
            className="px-3 py-2 rounded-full text-sm text-gray-600 hover:bg-gray-100 active:scale-95 transition"
          >
            上一首
          </button>

          <button
            onClick={() => setPlaying(!isPlaying)}
            className="px-6 py-2 rounded-full text-sm font-medium text-white bg-blue-500 hover:bg-blue-600 active:scale-95 transition shadow-sm"
          >
            {isPlaying ? "暂停" : "播放"}
          </button>

          <button
            onClick={next}
            className="px-3 py-2 rounded-full text-sm text-gray-600 hover:bg-gray-100 active:scale-95 transition"
          >
            下一首
          </button>
        </div>

        {/* 音量控制（和底部 PlayerBar 共享同一个 volume） */}
        <div className="flex items-center gap-3 justify-center text-xs text-gray-500">
          <span>音量</span>
          <input
            type="range"
            min={0}
            max={100}
            value={Math.round(volume * 100)}
            onChange={(e) => {
              const v = Number(e.target.value);
              if (!Number.isFinite(v)) return;
              setVolume(v / 100);
            }}
            className="w-40 accent-blue-500"
          />
        </div>
      </div>
    </div>
  );
};
