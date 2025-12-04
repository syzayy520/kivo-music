// src/components/AudioEngine.tsx
import React, { useEffect, useRef } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { usePlayerStore } from "../store/player";

/**
 * 全局隐形播放器：
 * - 只渲染一个 <audio> 元素
 * - 根据 player store 里的状态来加载音频 / 播放 / 暂停 / 跳转
 * - 把播放进度、时长等事件写回 store
 *
 * 这里特意保证：
 *   👉 仅在「当前曲目变化」时才会重新设置 audio.src
 *   👉 单纯切换 isPlaying（暂停 / 继续）不会重置进度
 */
export const AudioEngine: React.FC = () => {
  const audioRef = useRef<HTMLAudioElement | null>(null);

  // 播放列表 & 当前曲目
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);
  const isPlaying = usePlayerStore((s: any) => s.isPlaying ?? false);
  const volume = usePlayerStore((s: any) => s.volume ?? 1);
  const pendingSeek = usePlayerStore((s: any) => s.pendingSeek ?? null);

  // 事件回写
  const setPosition = usePlayerStore((s: any) => s.setPosition ?? (() => {}));
  const setDuration = usePlayerStore((s: any) => s.setDuration ?? (() => {}));
  const clearPendingSeek = usePlayerStore(
    (s: any) => s.clearPendingSeek ?? (() => {}),
  );
  const next = usePlayerStore((s: any) => s.next ?? (() => {}));

  const currentTrack =
    currentIndex >= 0 && currentIndex < playlist.length
      ? playlist[currentIndex]
      : null;

  // 绑定 <audio> 事件：timeupdate / loadedmetadata / ended
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    const handleTimeUpdate = () => {
      setPosition(audio.currentTime || 0);
    };

    const handleLoadedMetadata = () => {
      if (!Number.isFinite(audio.duration)) return;
      setDuration(audio.duration);
    };

    const handleEnded = () => {
      next();
    };

    audio.addEventListener("timeupdate", handleTimeUpdate);
    audio.addEventListener("loadedmetadata", handleLoadedMetadata);
    audio.addEventListener("ended", handleEnded);

    return () => {
      audio.removeEventListener("timeupdate", handleTimeUpdate);
      audio.removeEventListener("loadedmetadata", handleLoadedMetadata);
      audio.removeEventListener("ended", handleEnded);
    };
  }, [setPosition, setDuration, next]);

  // 当「当前曲目」变化时，才重新设置 src / load
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    if (!currentTrack) {
      audio.removeAttribute("src");
      audio.load();
      return;
    }

    const src = convertFileSrc(String(currentTrack.filePath));

    // 用 dataset 记一下当前 src，避免重复赋值导致重置播放进度
    const htmlAudio = audio as HTMLAudioElement & { dataset: DOMStringMap };
    if (htmlAudio.dataset.src !== src) {
      htmlAudio.dataset.src = src;
      audio.src = src;
      audio.load();
    }

    // 如果当前应该是播放状态，就自动开播
    if (isPlaying) {
      audio.play().catch((err: any) => {
  // 这种情况是 play() 被立即 pause() 打断，属于正常现象，直接忽略
  if (err && err.name === "AbortError") {
    return;
  }
  console.error("[AudioEngine] play error after track change", err);
});

    }
  }, [currentTrack && currentTrack.filePath]); // 只关心曲目变化

  // 仅根据 isPlaying 来控制 播放/暂停，不改 src
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio || !currentTrack) return;

    if (isPlaying) {
      audio.play().catch((err: any) => {
  if (err && err.name === "AbortError") {
    // 快速点击播放/暂停时也会出现 AbortError，同样是正常现象
    return;
  }
  console.error("[AudioEngine] play error on toggle", err);
});

    } else {
      audio.pause();
    }
  }, [isPlaying, currentTrack && currentTrack.filePath]);

  // 音量变化
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    const clamped = Math.max(0, Math.min(1, Number(volume) || 0));
    audio.volume = clamped;
  }, [volume]);

  // 处理 seek：只在 pendingSeek 有值时改 currentTime
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    if (pendingSeek == null || !Number.isFinite(pendingSeek)) return;

    try {
      audio.currentTime = pendingSeek;
    } catch (err) {
      console.error("[AudioEngine] failed to seek", err);
    } finally {
      clearPendingSeek();
    }
  }, [pendingSeek, clearPendingSeek]);

  return <audio ref={audioRef} style={{ display: "none" }} preload="metadata" />;
};

export default AudioEngine;
