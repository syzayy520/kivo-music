// src/audio-backend/html/HtmlAudioBackend.ts
import { convertFileSrc } from "@tauri-apps/api/core";
import { log } from "../../utils/log";
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
  private readonly context: AudioBackendContext;
  private snapshot: PlayerSnapshot | null = null;

  // 事件绑定引用，保持 this 指向稳定
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

    const { playlist, currentIndex, isPlaying } = snapshot;
    const hasPlaylist = Array.isArray(playlist) && playlist.length > 0;
    const track = hasPlaylist ? playlist[currentIndex] : null;

    // 1. 队列为空 → 正常状态（例如应用刚启动、用户清空队列）
    if (!hasPlaylist) {
      log.debug(
        "HtmlAudioBackend",
        "当前没有播放队列，停止播放并清空 src",
        { currentIndex },
      );
      try {
        audio.pause();
        audio.removeAttribute("src");
        audio.load();
      } catch (err) {
        log.warn("HtmlAudioBackend", "清空队列时重置 audio 失败", { err });
      }
      this.context.onTimeUpdate?.(0, null);
      return;
    }

    // 2. 队列存在但当前索引找不到曲目 → 通常是越界 / 状态不同步
    if (!track) {
      log.warn(
        "HtmlAudioBackend",
        "playlist 存在但 currentIndex 无效，视为当前曲目结束，交给上层处理",
        { currentIndex, playlistLength: playlist.length },
      );
      try {
        audio.pause();
      } catch {
        // ignore
      }
      this.context.onEnded?.();
      return;
    }

    // 3. 无法解析出文件路径 → 认为是坏数据，跳过
    const filePath = this.getTrackFilePath(track);
    if (!filePath) {
      const error = new Error(
        "当前曲目缺少可用的文件路径 (filePath/path/location/src)",
      );
      log.warn(
        "HtmlAudioBackend",
        "无法从 track 中解析出文件路径，自动跳过该曲目",
        { currentIndex, track },
      );
      this.context.onError?.(error);
      this.context.onEnded?.();
      return;
    }

    const srcUrl = convertFileSrc(filePath);

    // 4. 如果切换了曲目 / 路径，与当前 src 不同，则重新加载
    if (audio.src !== srcUrl) {
      log.debug("HtmlAudioBackend", "切换底层 audio.src", {
        currentIndex,
        filePath,
        srcUrl,
      });

      try {
        audio.src = srcUrl;
        audio.load();
      } catch (err) {
        log.error("HtmlAudioBackend", "设置 audio.src 或 load() 失败，将跳过当前曲目", {
          err,
          filePath,
          srcUrl,
        });
        this.context.onError?.(err);
        this.context.onEnded?.();
        return;
      }
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
   * 应用一次 pendingSeek 请求。
   * - 由 AudioEngine 控制 pendingSeek 生命周期，这里只负责“执行”。
   */
  applyPendingSeek(pendingSeek: number | null): void {
    const audio = this.audio;
    if (!audio || pendingSeek == null || Number.isNaN(pendingSeek)) return;

    try {
      audio.currentTime = pendingSeek;
      // 立即同步一次进度给上层，避免 UI 跳动不及时
      const duration =
        Number.isFinite(audio.duration) && audio.duration > 0
          ? audio.duration
          : null;
      this.context.onTimeUpdate?.(audio.currentTime || 0, duration);
    } catch (err) {
      log.warn("HtmlAudioBackend", "applyPendingSeek 失败", {
        pendingSeek,
        err,
      });
      this.context.onError?.(err);
    }
  }

  /**
   * 设置播放音量（0 ~ 1）。
   */
  setVolume(volume: number): void {
    const audio = this.audio;
    if (!audio) return;

    // 做一层兜底，避免组件传入越界值
    const safeVolume = Math.min(1, Math.max(0, volume));
    audio.volume = safeVolume;
  }

  // === 事件绑定相关 ===

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

  // === 事件处理 ===

  /**
   * 统一处理进度更新：
   * - timeupdate
   * - loadedmetadata
   * - durationchange
   */
  private handleTimeUpdate() {
    const audio = this.audio;
    if (!audio) return;

    try {
      const currentTime = audio.currentTime || 0;
      const duration =
        Number.isFinite(audio.duration) && audio.duration > 0
          ? audio.duration
          : null;
      this.context.onTimeUpdate?.(currentTime, duration);
    } catch (err) {
      log.warn("HtmlAudioBackend", "读取 audio 进度失败", { err });
    }
  }

  private handleEnded() {
    log.debug("HtmlAudioBackend", "音频播放结束 (ended)");
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
   * - code === 1 (MEDIA_ERR_ABORTED)：通常是用户切歌 / 手动中断 → 记 debug 即可；
   * - code === 3/4：解码失败 / 格式不支持 → 视为当前曲目坏了，自动当作 ended；
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

    // 1 = MEDIA_ERR_ABORTED：一般是用户操作中断播放
    if (code === 1) {
      log.debug(
        "HtmlAudioBackend",
        "收到 MEDIA_ERR_ABORTED（通常是用户中断播放），忽略",
        { code, message },
      );
      return;
    }

    // 3 = MEDIA_ERR_DECODE，4 = MEDIA_ERR_SRC_NOT_SUPPORTED
    if (code === 3 || code === 4) {
      log.warn(
        "HtmlAudioBackend",
        "媒体解码失败或格式不受支持，视为坏文件，自动跳过当前曲目",
        { code, message },
      );
      this.context.onError?.(mediaError);

      try {
        audio.pause();
      } catch {
        // ignore
      }

      this.context.onEnded?.();
      return;
    }

    // 其他错误：上抛 + 记 error 日志
    log.error("HtmlAudioBackend", "audio 触发未知错误", { code, message });
    this.context.onError?.(mediaError);
  }

  // === 工具方法 ===

  /**
   * 尝试从 track 对象中解析出本地文件路径。
   * - 兼容多种字段命名，避免前后版本不一致导致播放失败。
   */
  private getTrackFilePath(track: any): string | null {
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
