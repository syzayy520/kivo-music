// src/components/AudioEngine.tsx
import React, { useEffect, useRef } from "react";
import { usePlayerStore } from "../store/player";
import type { AudioBackend } from "../audio-backend/AudioBackend";
import { HtmlAudioBackend } from "../audio-backend/html/HtmlAudioBackend";
import { log } from "../utils/log";

/**
 * AudioEngine
 *
 * - 只负责创建隐藏的 <audio> 元素和管理一个 AudioBackend 实例
 * - 从 player store 读取最小状态快照交给后端
 * - 接收后端回调，把进度 / 结束写回 store
 *
 * 所有具体播放逻辑都在 HtmlAudioBackend 里实现。
 */
export const AudioEngine: React.FC = () => {
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const backendRef = useRef<AudioBackend | null>(null);

  // 用 any 避免和 store 类型过度耦合，方便以后重构
  const playlist = usePlayerStore((s: any) => s.playlist || []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? 0);
  const isPlaying = usePlayerStore((s: any) => s.isPlaying ?? false);
  const pendingSeek = usePlayerStore((s: any) => s.pendingSeek ?? null);
  const volume = usePlayerStore((s: any) => s.volume ?? 1);

  // 初始化后端实例 + 绑定 audio 元素
  useEffect(() => {
    const backend = new HtmlAudioBackend({
      onTimeUpdate(currentTime: number, duration: number | null) {
        // 直接通过 setState 写回 currentTime/duration
        usePlayerStore.setState((prev: any) => ({
          ...prev,
          currentTime,
          duration: duration ?? prev?.duration ?? 0,
        }));
      },
      onEnded() {
        const state: any = usePlayerStore.getState();
        if (typeof state.next === "function") {
          state.next();
        } else {
          log.info(
            "AudioEngine",
            "onEnded 触发，但 playerStore 未提供 next()，已忽略",
          );
        }
      },
      onError(error: unknown) {
        log.error("AudioEngine", "播放错误", { error });
      },
    });

    backendRef.current = backend;

    if (audioRef.current) {
      backend.attachAudioElement(audioRef.current);
    }

    return () => {
      backend.destroy();
      backendRef.current = null;
    };
  }, []);

  // 播放状态相关（playlist / currentIndex / isPlaying）变化时同步给后端
  useEffect(() => {
    const backend = backendRef.current;
    if (!backend) return;

    backend.updateFromState({
      playlist,
      currentIndex,
      isPlaying,
    });
  }, [playlist, currentIndex, isPlaying]);

  // pendingSeek → 后端
  useEffect(() => {
    const backend = backendRef.current;
    if (!backend) return;
    if (pendingSeek == null) return;

    backend.applyPendingSeek(pendingSeek);

    // 清空 pendingSeek，避免重复 seek
    usePlayerStore.setState((prev: any) => ({
      ...prev,
      pendingSeek: null,
    }));
  }, [pendingSeek]);

  // 音量变化 → 后端
  useEffect(() => {
    const backend = backendRef.current;
    if (!backend) return;
    backend.setVolume(volume);
  }, [volume]);

  // 只渲染一个隐藏的 <audio>，给后端用
  return <audio ref={audioRef} style={{ display: "none" }} />;
};
