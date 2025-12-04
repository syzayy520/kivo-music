// src/audio-backend/AudioBackend.ts

/**
 * 播放后端接收的最小播放器状态快照。
 * 只包含与“底层播放”强相关的字段。
 */
export interface PlayerSnapshot {
  playlist: any[];
  currentIndex: number;
  isPlaying: boolean;
}

/**
 * 播放后端通过回调把进度 / 结束 / 错误抛给上层（AudioEngine）。
 */
export interface AudioBackendContext {
  onTimeUpdate?(currentTime: number, duration: number | null): void;
  onEnded?(): void;
  onError?(error: unknown): void;
}

/**
 * 统一的播放后端接口。
 */
export interface AudioBackend {
  /**
   * 绑定一个隐藏的 <audio> 元素。
   */
  attachAudioElement(audio: HTMLAudioElement): void;

  /**
   * 根据当前播放器状态快照更新底层播放行为：
   * - 切歌
   * - 播放 / 暂停
   */
  updateFromState(snapshot: PlayerSnapshot): Promise<void> | void;

  /**
   * 应用一次 pendingSeek 请求。
   */
  applyPendingSeek(pendingSeek: number | null): void;

  /**
   * 设置播放音量（0~1）。
   */
  setVolume(volume: number): void;

  /**
   * 释放资源并解绑事件。
   */
  destroy(): void;
}
