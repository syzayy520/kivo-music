// src/audio-backend/visualizer-node.ts

let audioContext: AudioContext | null = null;
let analyserNode: AnalyserNode | null = null;
let sourceNode: MediaElementAudioSourceNode | null = null;
let attachedElement: HTMLMediaElement | null = null;

function getOrCreateAudioContext(): AudioContext | null {
  if (typeof window === "undefined") return null;

  const AC =
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).AudioContext ||
    // Safari
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).webkitAudioContext;

  if (!AC) {
    console.warn("[Visualizer] 当前环境不支持 Web Audio API");
    return null;
  }

  if (!audioContext) {
    audioContext = new AC();
  }
  return audioContext;
}

/**
 * 把隐藏的 <audio> 挂到 Web Audio Graph 上：
 * mediaElement → analyser → destination
 */
export function attachVisualizerToAudioElement(
  element: HTMLAudioElement,
): void {
  attachedElement = element;

  const ctx = getOrCreateAudioContext();
  if (!ctx) return;

  // 尝试断开旧连接，避免重复 connect 抛错
  try {
    sourceNode?.disconnect();
  } catch {
    // ignore
  }
  try {
    analyserNode?.disconnect();
  } catch {
    // ignore
  }

  try {
    sourceNode = ctx.createMediaElementSource(element);
  } catch (error) {
    // 同一个 <audio> 只能调用一次 createMediaElementSource
    // 如果已经创建过，这里直接复用旧的 sourceNode 即可
    if (!sourceNode) {
      console.warn(
        "[Visualizer] createMediaElementSource 失败，频谱不可用：",
        error,
      );
      return;
    }
  }

  if (!analyserNode) {
    analyserNode = ctx.createAnalyser();
    analyserNode.fftSize = 512;
    analyserNode.smoothingTimeConstant = 0.8;
  }

  try {
    if (sourceNode && analyserNode) {
      sourceNode.connect(analyserNode);
      analyserNode.connect(ctx.destination);
    }
  } catch (error) {
    console.warn("[Visualizer] 连接 AudioContext 失败：", error);
  }
}

/**
 * 解绑可视化（组件卸载时调用，防止内存泄漏）
 */
export function detachVisualizerFromAudioElement(): void {
  try {
    sourceNode?.disconnect();
  } catch {
    // ignore
  }
  try {
    analyserNode?.disconnect();
  } catch {
    // ignore
  }
  attachedElement = null;
}

/**
 * 给前端可视化组件使用的 AnalyserNode。
 * 可能为 null（比如浏览器不支持，或者还没 attach）。
 */
export function getVisualizerAnalyser(): AnalyserNode | null {
  return analyserNode;
}

export async function resumeVisualizerAudioContext(): Promise<void> {
  if (!audioContext) return;
  if (audioContext.state === "suspended") {
    try {
      await audioContext.resume();
    } catch {
      // ignore
    }
  }
}
