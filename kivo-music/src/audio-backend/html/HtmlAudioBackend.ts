// src/audio-backend/html/HtmlAudioBackend.ts
import { convertFileSrc } from "@tauri-apps/api/core";
import { log } from "../../utils/log";
import type {
  AudioBackend,
  AudioBackendContext,
  PlayerSnapshot,
} from "../AudioBackend";

/**
 * 基于 <audio> 的默认播放后端实现。
 *
 * - 负责处理切歌 / 播放 / 暂停 / seek / 音量
 * - 通过回调把进度 / 结束 / 错误抛给上层（AudioEngine）
 * - 对外只依赖 AudioBackend 接口，不直接碰 store
 */
export class HtmlAudioBackend implements AudioBackend {
  private audio: HTMLAudioElement | null = null;
  private context: AudioBackendContext;
  private snapshot: PlayerSnapshot | null = null;

  private boundOnTimeUpdate = () => this.handleTimeUpdate();
  private boundOnLoadedMetadata = () => this.handleTimeUpdate();
  private boundOnDurationChange = () => this.handleTimeUpdate();
  private boundOnEnded = () => this.handleEnded();
  private boundOnError = () => this.handleError();

  constructor(context: AudioBackendContext) {
    this.context = context;
  }

  attachAudioElement(audio: HTMLAudioElement): void {
    if (this.audio === audio) return;

    // 先解绑旧的
    if (this.audio) {
      this.detachEvents(this.audio);
    }

    this.audio = audio;
    this.attachEvents(audio);

    // 如果之前已经有状态，重新套一次
    if (this.snapshot) {
      void this.updateFromState(this.snapshot);
    }
  }

  destroy(): void {
    if (this.audio) {
      this.detachEvents(this.audio);
      this.audio = null;
    }
    this.snapshot = null;
  }

  async updateFromState(snapshot: PlayerSnapshot): Promise<void> {
    this.snapshot = snapshot;

    const audio = this.audio;
    if (!audio) return;

    const { playlist, currentIndex, isPlaying } = snapshot;
    const track = playlist?.[currentIndex];
    if (!track) {
      audio.pause();
      return;
    }

    const srcPath = this.getTrackFilePath(track);
    if (!srcPath) {
      log.warn("HtmlAudioBackend", "无法从曲目中解析出可用的文件路径", {
        track,
      });
      audio.pause();
      return;
    }

    // Tauri 下需要 convertFileSrc
    const srcUrl = convertFileSrc(srcPath);

    if (audio.src !== srcUrl) {
      audio.src = srcUrl;
      audio.load();
    }

    if (isPlaying) {
      try {
        await audio.play();
      } catch (err: any) {
        if (err && err.name === "AbortError") {
          // play() 刚发出去就被 pause() 打断是正常情况
          log.debug(
            "HtmlAudioBackend",
            "play() 被 AbortError 中断，视为正常中断",
            { err },
          );
          return;
        }
        this.context.onError?.(err);
        log.error("HtmlAudioBackend", "play() 失败", { err });
      }
    } else {
      audio.pause();
    }
  }

  applyPendingSeek(pendingSeek: number | null): void {
    const audio = this.audio;
    if (!audio) return;
    if (pendingSeek == null || Number.isNaN(pendingSeek)) return;

    try {
      audio.currentTime = pendingSeek;
      this.context.onTimeUpdate?.(audio.currentTime, audio.duration || null);
    } catch (err) {
      this.context.onError?.(err);
      log.error("HtmlAudioBackend", "applyPendingSeek 失败", {
        pendingSeek,
        err,
      });
    }
  }

  setVolume(volume: number): void {
    const audio = this.audio;
    if (!audio) return;

    const clamped = Math.min(
      1,
      Math.max(0, Number.isFinite(volume) ? volume : 0),
    );
    audio.volume = clamped;
  }

  // === 事件绑定 / 解绑 ===

  private attachEvents(audio: HTMLAudioElement) {
    audio.addEventListener("timeupdate", this.boundOnTimeUpdate);
    audio.addEventListener("loadedmetadata", this.boundOnLoadedMetadata);
    audio.addEventListener("durationchange", this.boundOnDurationChange);
    audio.addEventListener("ended", this.boundOnEnded);
    audio.addEventListener("error", this.boundOnError);
  }

  private detachEvents(audio: HTMLAudioElement) {
    audio.removeEventListener("timeupdate", this.boundOnTimeUpdate);
    audio.removeEventListener("loadedmetadata", this.boundOnLoadedMetadata);
    audio.removeEventListener("durationchange", this.boundOnDurationChange);
    audio.removeEventListener("ended", this.boundOnEnded);
    audio.removeEventListener("error", this.boundOnError);
  }

  // === 事件响应 ===

  private handleTimeUpdate() {
    const audio = this.audio;
    if (!audio) return;
    const duration =
      Number.isFinite(audio.duration) && audio.duration > 0
        ? audio.duration
        : null;
    this.context.onTimeUpdate?.(audio.currentTime || 0, duration);
  }

  private handleEnded() {
    this.context.onEnded?.();
  }

  /**
   * audio 元素触发 error 事件时的处理：
   *
   * - code 1（ABORTED）：用户中止 / 切歌，视为正常情况，写 debug 日志
   * - code 3 / 4（DECODE / SRC_NOT_SUPPORTED）：文件解码失败或格式不支持，
   *   打 warn 日志并当作“自然播放结束”，让上层走 next()
   * - 其它错误：当成严重错误，调用 onError + error 日志
   */
  private handleError() {
    const audio = this.audio;
    const mediaError = (audio && (audio.error as any)) || null;
    if (!mediaError) return;

    const code = Number(mediaError.code ?? NaN);
    const info = {
      code,
      message: mediaError.message ?? undefined,
      mediaError,
    };

    // 1: MEDIA_ERR_ABORTED（用户中止 / 切歌）
    if (code === 1) {
      log.debug("HtmlAudioBackend", "音频播放被中止 (code 1)", info);
      return;
    }

    // 3: MEDIA_ERR_DECODE
    // 4: MEDIA_ERR_SRC_NOT_SUPPORTED
    if (code === 3 || code === 4) {
      log.warn(
        "HtmlAudioBackend",
        "音频解码失败或格式不支持，自动跳过当前曲目",
        info,
      );
      // 当作正常结束，让上层按 ended 逻辑走 next()
      this.context.onEnded?.();
      return;
    }

    // 其它错误，当成严重错误
    this.context.onError?.(mediaError);
    log.error("HtmlAudioBackend", "audio 元素触发错误事件", info);
  }

  // === 工具方法 ===

  private getTrackFilePath(track: any): string | null {
    return (
      track?.filePath ??
      track?.path ??
      track?.location ??
      track?.src ??
      null
    );
  }
}
