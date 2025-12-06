// src/audio-backend/html/HtmlAudioBackend.ts
import { convertFileSrc } from "@tauri-apps/api/core";
import { log } from "../../utils/log";
import { bumpPlayStatsForTrack } from "../../library/libraryModel";
import type {
  AudioBackend,
  AudioBackendContext,
  PlayerSnapshot,
} from "../AudioBackend";

/**
 * 基于 <audio> 的播放后端实现（HTML 音频内核）。
 *
 * 职责：
 * - 持有一个隐藏的 <audio> 元素；
 * - 根据 PlayerSnapshot 驱动底层播放状态；
 * - 把进度 / 结束 / 错误通过 AudioBackendContext 回调给上层；
 * - 对异常场景（坏文件 / 路径失效 / 解码失败）做“可恢复”的处理：
 *   - 不崩溃、不白屏；
 *   - 坏文件自动跳到下一首；
 *   - 统一打日志，方便后续在 Developer 面板里展示。
 */
export class HtmlAudioBackend implements AudioBackend {
  private audio: HTMLAudioElement | null = null;
  private context: AudioBackendContext;
  private snapshot: PlayerSnapshot | null = null;

  // 统一绑定到 audio 事件的回调，避免频繁创建新函数
  private boundOnTimeUpdate = () => this.handleTimeUpdate();
  private boundOnLoadedMetadata = () => this.handleTimeUpdate();
  private boundOnDurationChange = () => this.handleTimeUpdate();
  private boundOnEnded = () => this.handleEnded();
  private boundOnError = () => this.handleError();
  private boundOnStalled = () => this.handleStalled();
  private boundOnSuspend = () => this.handleStalled();

  constructor(context: AudioBackendContext) {
    this.context = context;
  }

  /**
   * 由 AudioEngine 注入隐藏的 <audio> 元素。
   * - 允许重复调用：会自动解绑旧元素、绑定到新元素。
   */
  attachAudioElement(audio: HTMLAudioElement): void {
    if (this.audio === audio) return;

    if (this.audio) {
      this.detachEvents(this.audio);
    }

    this.audio = audio;
    this.attachEvents(audio);

    // 初始状态同步（如果之前已经收到过 snapshot）
    if (this.snapshot) {
      void this.updateFromState(this.snapshot);
    }
  }

  /**
   * 释放资源并解绑事件。
   */
  destroy(): void {
    if (!this.audio) return;

    try {
      this.detachEvents(this.audio);
    } catch (err) {
      log.warn("HtmlAudioBackend", "destroy() 时解绑事件失败", { err });
    }

    try {
      this.audio.pause();
      this.audio.removeAttribute("src");
      this.audio.load();
    } catch (err) {
      log.warn("HtmlAudioBackend", "destroy() 时重置 audio 元素失败", { err });
    }

    this.audio = null;
    this.snapshot = null;
  }

  /**
   * 根据 player 最小状态快照更新底层播放状态。
   *
   * 约定：
   * - 当队列为空时：这是正常状态，不视为错误，不触发 onEnded；
   * - 当有队列但 currentIndex 无效 / track 无法解析到有效路径时：
   *   - 视为“当前曲目无法播放 / 已结束”，调用 onEnded，让上层决定下一步（下一首 / 停止）。
   */
  async updateFromState(snapshot: PlayerSnapshot): Promise<void> {
    this.snapshot = snapshot;

    const audio = this.audio;
    if (!audio) {
      // audio 元素尚未挂载，等 attachAudioElement 之后会自动用最新快照同步
      log.debug(
        "HtmlAudioBackend",
        "updateFromState 调用时 audio 元素尚未就绪，稍后会自动同步",
        { snapshot },
      );
      return;
    }

    const playlist = Array.isArray(snapshot.playlist) ? snapshot.playlist : [];
    const currentIndex =
      typeof snapshot.currentIndex === "number" ? snapshot.currentIndex : -1;
    const isPlaying = !!snapshot.isPlaying;

    // 1. 队列为空：重置 audio 状态，视为正常 idle，不回调 onEnded
    if (playlist.length === 0 || currentIndex < 0) {
      log.debug(
        "HtmlAudioBackend",
        "队列为空或索引无效，重置 audio 至空闲状态",
        { currentIndex, playlistLength: playlist.length },
      );
      try {
        audio.pause();
        audio.removeAttribute("src");
        audio.load();
      } catch (err) {
        log.warn(
          "HtmlAudioBackend",
          "重置空闲状态时操作 audio 失败（忽略）",
          { err },
        );
      }
      return;
    }

    // 2. 解析当前 track 对应的文件路径
    const track = playlist[currentIndex];
    const filePath = this.resolveTrackFilePath(track);

    if (!filePath) {
      log.warn(
        "HtmlAudioBackend",
        "当前曲目无法解析到有效文件路径，将视为已结束并交给上层处理",
        { currentIndex, track },
      );
      this.context.onEnded?.();
      return;
    }

    // 3. 如果 src 发生变化，则切歌
    const nextSrc = convertFileSrc(filePath);

    if (audio.src !== nextSrc) {
      log.debug("HtmlAudioBackend", "检测到切歌，更新 audio.src", {
        currentIndex,
        filePath,
      });
      try {
        audio.pause();
      } catch (err) {
        log.warn("HtmlAudioBackend", "切歌前 pause() 失败（忽略）", { err });
      }

      try {
        audio.src = nextSrc;
        // 切歌后重置进度，让 loadedmetadata/durationchange 事件重新上报
        audio.currentTime = 0;
      } catch (err) {
        log.error("HtmlAudioBackend", "设置 audio.src 失败，将尝试跳过当前曲目", {
          err,
          filePath,
        });
        this.context.onError?.(err);
        this.context.onEnded?.();
        return;
      }
    }

    // 4. 根据 isPlaying 决定播放 / 暂停
    if (!isPlaying) {
      try {
        audio.pause();
      } catch (err) {
        log.warn("HtmlAudioBackend", "调用 pause() 失败", { err });
      }
      return;
    }

    // 5. 播放 / 暂停
    if (isPlaying) {
      try {
        await audio.play();
      } catch (err: any) {
        // play() 被立即 pause 打断会抛 AbortError，属正常情况
        if (err && (err.name === "AbortError" || err.code === 20)) {
          log.debug(
            "HtmlAudioBackend",
            "play() 被 AbortError 中断（通常是切歌或暂停触发），视为正常",
            { err },
          );
          return;
        }

        // 其余情况属于真正的播放失败 → 记录错误并自动跳过
        log.error("HtmlAudioBackend", "play() 调用失败，将尝试跳过当前曲目", {
          err,
          currentIndex,
          filePath,
        });
        this.context.onError?.(err);
        try {
          audio.pause();
        } catch {
          // ignore
        }
        this.context.onEnded?.();
      }
    } else {
      try {
        audio.pause();
      } catch (err) {
        log.warn("HtmlAudioBackend", "调用 pause() 失败", { err });
      }
    }
  }

  /**
   * 应用 pendingSeek：这里直接修改 audio.currentTime。
   */
  applyPendingSeek(pendingSeek: number | null): void {
    const audio = this.audio;
    if (!audio || pendingSeek == null || Number.isNaN(pendingSeek)) return;

    try {
      audio.currentTime = pendingSeek;
    } catch (err) {
      log.warn("HtmlAudioBackend", "应用 pendingSeek 失败（忽略）", {
        pendingSeek,
        err,
      });
    }
  }

  /**
   * 设置音量（0~1）
   */
  setVolume(volume: number): void {
    const audio = this.audio;
    if (!audio) return;

    const clamped = Math.max(0, Math.min(1, volume));
    try {
      audio.volume = clamped;
    } catch (err) {
      log.warn("HtmlAudioBackend", "设置音量失败（忽略）", { volume, err });
    }
  }

  // === 内部事件处理 & 工具方法 ===

  private attachEvents(audio: HTMLAudioElement) {
    audio.addEventListener("timeupdate", this.boundOnTimeUpdate);
    audio.addEventListener("loadedmetadata", this.boundOnLoadedMetadata);
    audio.addEventListener("durationchange", this.boundOnDurationChange);
    audio.addEventListener("ended", this.boundOnEnded);
    audio.addEventListener("error", this.boundOnError);
    audio.addEventListener("stalled", this.boundOnStalled);
    audio.addEventListener("suspend", this.boundOnSuspend);
  }

  private detachEvents(audio: HTMLAudioElement) {
    audio.removeEventListener("timeupdate", this.boundOnTimeUpdate);
    audio.removeEventListener("loadedmetadata", this.boundOnLoadedMetadata);
    audio.removeEventListener("durationchange", this.boundOnDurationChange);
    audio.removeEventListener("ended", this.boundOnEnded);
    audio.removeEventListener("error", this.boundOnError);
    audio.removeEventListener("stalled", this.boundOnStalled);
    audio.removeEventListener("suspend", this.boundOnSuspend);
  }

  /**
   * 同步当前时间 & 时长到上层。
   * - timeupdate / loadedmetadata / durationchange 都会触发这里。
   */
  private handleTimeUpdate() {
    const audio = this.audio;
    if (!audio) return;

    const currentTime = audio.currentTime ?? 0;
    const duration =
      audio.duration && Number.isFinite(audio.duration)
        ? audio.duration
        : null;

    this.context.onTimeUpdate?.(currentTime, duration);
  }

  private handleEnded() {
    log.debug("HtmlAudioBackend", "音频播放结束 (ended)");

    // 在“自然播放结束”时更新一次播放统计：
    // - 只在正常 ended 事件中统计，不在错误 / 路径无效时统计；
    // - 利用当前快照中的 playlist/currentIndex 找到对应曲目，
    //   交给 libraryModel 负责持久化和归一化。
    try {
      const snapshot = this.snapshot;
      const list = Array.isArray(snapshot?.playlist) ? snapshot!.playlist : [];
      const index =
        typeof snapshot?.currentIndex === "number" ? snapshot!.currentIndex : -1;

      if (index >= 0 && index < list.length) {
        const finishedTrack = list[index];
        // 这里不关心 track 的具体结构，由 bumpPlayStatsForTrack 负责做 identity 匹配。
        // 即使库里没有这首歌，也只是静默忽略。
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        bumpPlayStatsForTrack(finishedTrack as any);
      }
    } catch (err) {
      log.warn("HtmlAudioBackend", "handleEnded 统计播放次数失败（忽略此错误）", {
        err,
      });
    }

    this.context.onEnded?.();
  }

  /**
   * 网络 / 读取中断（针对本地文件通常是暂时性），只在 debug 层面记录：
   * - 避免正常播放过程中控制台被 warn 淹没；
   * - 真正排查问题时再看这些日志即可。
   */
  private handleStalled() {
    const audio = this.audio;
    if (!audio) return;

    log.debug(
      "HtmlAudioBackend",
      "音频数据出现短暂的加载停滞或中断 (stalled/suspend)",
      {
        readyState: audio.readyState,
        // 某些 runtime 下可能没有 networkState，这里做个兜底
        networkState: (audio as any).networkState,
      },
    );
  }

  /**
   * audio 元素报错：
   * - code === 1 (MEDIA_ERR_ABORTED)：通常是被用户中断，视为正常；
   * - 其他：通过 onError 抛给上层，并写 error 日志。
   */
  private handleError() {
    const audio = this.audio;
    if (!audio) return;

    const mediaError = (audio as any).error as MediaError | null;
    if (!mediaError) {
      log.error("HtmlAudioBackend", "audio 触发 error 事件，但 error 对象为空");
      return;
    }

    const { code, message } = mediaError as any;

    if (code === 1) {
      // 用户中断（例如切歌），不视为错误
      log.debug(
        "HtmlAudioBackend",
        "audio 报错 code=1 (MEDIA_ERR_ABORTED)，视为用户中断",
        { code, message },
      );
      return;
    }

    log.error("HtmlAudioBackend", "audio 报错", { code, message });

    this.context.onError?.(mediaError);
  }

  /**
   * 尝试从 playlist 的 track 对象中解析出文件路径：
   * - 兼容多种字段命名；
   * - 如果都拿不到，返回 null。
   */
  private resolveTrackFilePath(track: any): string | null {
    if (!track || typeof track !== "object") return null;

    const candidate =
      track.filePath ??
      track.path ??
      track.location ??
      track.src ??
      null;

    if (typeof candidate !== "string" || candidate.trim().length === 0) {
      return null;
    }

    return candidate;
  }
}
