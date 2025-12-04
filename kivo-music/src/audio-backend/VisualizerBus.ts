// src/audio-backend/VisualizerBus.ts
/**
 * VisualizerBus（安全回退版）
 *
 * 说明：
 * - 为了兼容 Tauri 的 asset.localhost 协议，暂时不再使用 Web Audio 的
 *   createMediaElementSource(audio)，否则会因为 CORS 限制导致输出全 0，
 *   甚至出现“有进度无声音”的情况。
 * - 当前实现只做一个“伪频谱”动画，完全不干扰 <audio> 的真实播放路径。
 *
 * 后续如果要接入真正的频谱：
 * - 可以在 Rust / Tauri 侧解码音频，再把数据喂给前端；
 * - 或者调整资源加载方式，使其对 Web Audio 来说是 origin-clean。
 */

let attachedAudio: HTMLAudioElement | null = null;

// 伪频谱使用的缓冲区
let fakeArray: Uint8Array | null = null;

/**
 * 初始化：现在只记录一下 audio 元素引用，不做任何 Web Audio 相关操作，
 * 确保不会影响原来的播放路径。
 */
export function initVisualizerForAudioElement(
  audioElement: HTMLAudioElement | null,
): void {
  attachedAudio = audioElement ?? null;
}

/**
 * 获取频谱数据：
 * - 当前返回的是一个“伪频谱”，用时间 + 简单的正弦波生成动画效果；
 * - 不读取真实音频数据，完全不会触发 CORS / Web Audio 限制。
 */
export function getFrequencyData(): Uint8Array | null {
  const BAR_COUNT = 96;

  if (!fakeArray || fakeArray.length !== BAR_COUNT) {
    fakeArray = new Uint8Array(BAR_COUNT);
  }

  const now = performance.now() / 1000; // 秒

  // 根据当前播放状态稍微调整动效节奏（如果拿不到 audio 就按播放状态未知处理）
  const playing =
    attachedAudio != null
      ? !attachedAudio.paused && !attachedAudio.ended
      : true;

  const speed = playing ? 2.4 : 0.8;

  for (let i = 0; i < BAR_COUNT; i++) {
    const phase = (i / BAR_COUNT) * Math.PI * 4;
    const wave = Math.sin(phase + now * speed);
    const envelope = Math.sin((i / BAR_COUNT) * Math.PI);
    const value = Math.abs(wave) * envelope;

    // 40 ~ 255 之间的值，看起来比较像音量起伏
    fakeArray[i] = Math.round(40 + value * 215);
  }

  return fakeArray;
}

/**
 * 标记当前是否有可用的“可视化源”。
 * 现在只要有挂过 audio 就算 true。
 */
export function hasVisualizer(): boolean {
  return !!attachedAudio;
}
