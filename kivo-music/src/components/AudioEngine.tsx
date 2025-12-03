// src/components/AudioEngine.tsx
import React, { useEffect, useRef } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import usePlayerStore from "../store/player";

export const AudioEngine: React.FC = () => {
  const audioRef = useRef<HTMLAudioElement | null>(null);

  const playlist = usePlayerStore((s) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s) => s.currentIndex ?? -1);
  const isPlaying = usePlayerStore((s) => s.isPlaying ?? false);
  const volume = usePlayerStore((s) => s.volume ?? 1);
  const pendingSeek = usePlayerStore((s) => s.pendingSeek ?? null);

  const setPosition = usePlayerStore((s) => s.setPosition ?? (() => {}));
  const setDuration = usePlayerStore((s) => s.setDuration ?? (() => {}));
  const clearPendingSeek = usePlayerStore((s) => s.clearPendingSeek ?? (() => {}));
  const setIsPlaying = usePlayerStore((s) => s.setIsPlaying ?? (() => {}));
  const next = usePlayerStore((s) => s.next ?? (() => {}));

  const currentTrack =
    playlist && playlist.length > 0 && currentIndex >= 0 && currentIndex < playlist.length
      ? playlist[currentIndex]
      : null;

  const currentSrc = currentTrack ? convertFileSrc(currentTrack.filePath) : "";

  // 当曲目或 src 改变时，加载新的音频
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    if (!currentTrack) {
      audio.removeAttribute("src");
      audio.load();
      setIsPlaying(false);
      return;
    }

    audio.src = currentSrc;
    audio.load();

    if (isPlaying) {
      audio
        .play()
        .catch((err) => {
          console.error("[AudioEngine] play error", err);
          setIsPlaying(false);
        });
    }
  }, [currentTrack, currentSrc, isPlaying, setIsPlaying]);

  // 音量变化
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    audio.volume = volume;
  }, [volume]);

  // 拖动进度：pendingSeek
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    if (pendingSeek == null) return;

    try {
      audio.currentTime = pendingSeek;
    } catch (e) {
      console.warn("[AudioEngine] failed to seek", e);
    } finally {
      clearPendingSeek();
    }
  }, [pendingSeek, clearPendingSeek]);

  // 事件监听：更新时间 / 时长 / 播放状态 / 自动下一首
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    const handleTimeUpdate = () => {
      setPosition(audio.currentTime || 0);
    };

    const handleLoadedMetadata = () => {
      if (!Number.isFinite(audio.duration)) return;
      setDuration(audio.duration || 0);
    };

    const handlePlay = () => setIsPlaying(true);
    const handlePause = () => setIsPlaying(false);

    const handleEnded = () => {
      next();
    };

    audio.addEventListener("timeupdate", handleTimeUpdate);
    audio.addEventListener("loadedmetadata", handleLoadedMetadata);
    audio.addEventListener("play", handlePlay);
    audio.addEventListener("pause", handlePause);
    audio.addEventListener("ended", handleEnded);

    return () => {
      audio.removeEventListener("timeupdate", handleTimeUpdate);
      audio.removeEventListener("loadedmetadata", handleLoadedMetadata);
      audio.removeEventListener("play", handlePlay);
      audio.removeEventListener("pause", handlePause);
      audio.removeEventListener("ended", handleEnded);
    };
  }, [setPosition, setDuration, setIsPlaying, next]);

  return <audio ref={audioRef} style={{ display: "none" }} />;
};

export default AudioEngine;
