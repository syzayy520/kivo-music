import React, { useEffect, useRef } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { usePlayerStore } from "../store/player";

/**
 * 不渲染 UI，只负责管理 <audio> 元素，
 * 把 Zustand 里的播放状态同步到真正的音频播放。
 */
export const AudioEngine: React.FC = () => {
  const audioRef = useRef<HTMLAudioElement | null>(null);

  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);
  const volume = usePlayerStore((s) => s.volume);
  const pendingSeek = usePlayerStore((s) => s.pendingSeek);

  const setPosition = usePlayerStore((s) => s.setPosition);
  const setDuration = usePlayerStore((s) => s.setDuration);
  const clearPendingSeek = usePlayerStore((s) => s.clearPendingSeek);
  const next = usePlayerStore((s) => s.next);

  // 同步播放源 / 播放状态 / 音量
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    audio.volume = volume;

    const track =
      currentIndex != null && currentIndex >= 0
        ? playlist[currentIndex]
        : undefined;

    if (!track) {
      audio.pause();
      audio.removeAttribute("src");
      audio.load();
      return;
    }

    const newSrc = convertFileSrc(track.filePath);

    if (audio.src !== newSrc) {
      audio.src = newSrc;
      audio.load();
    }

    if (isPlaying) {
      audio
        .play()
        .catch((err) => console.warn("[AudioEngine] play() error:", err));
    } else {
      audio.pause();
    }
  }, [playlist, currentIndex, isPlaying, volume]);

  // 处理 seek
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    if (pendingSeek == null) return;

    audio.currentTime = pendingSeek;
    clearPendingSeek();
  }, [pendingSeek, clearPendingSeek]);

  // 绑定事件
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    const handleTimeUpdate = () => {
      setPosition(audio.currentTime || 0);
    };

    const handleLoaded = () => {
      setDuration(audio.duration || 0);
    };

    const handleEnded = () => {
      next();
    };

    audio.addEventListener("timeupdate", handleTimeUpdate);
    audio.addEventListener("loadedmetadata", handleLoaded);
    audio.addEventListener("ended", handleEnded);

    return () => {
      audio.removeEventListener("timeupdate", handleTimeUpdate);
      audio.removeEventListener("loadedmetadata", handleLoaded);
      audio.removeEventListener("ended", handleEnded);
    };
  }, [setPosition, setDuration, next]);

  return <audio ref={audioRef} style={{ display: "none" }} />;
};

export default AudioEngine;
