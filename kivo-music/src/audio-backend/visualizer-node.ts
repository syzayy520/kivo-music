// src/audio-backend/visualizer-node.ts
//
// Web Audio 频谱后端（预留实现）
//
// 说明：
// - 当前项目实际使用的是 VisualizerBus.ts 里的“伪频谱”实现，
//   这个文件暂时没有被引用，但为了未来可切换到真正 Web Audio，
//   我们保留一个结构清晰的实现。
// - 由于 tsconfig 开启了 noUnusedLocals，这里所有顶层变量必须
//   在导出的函数中被“读取”一次，否则会在 tauri build 时报 TS6133。

let audioContext: AudioContext | null = null;
let analyserNode: AnalyserNode | null = null;
let sourceNode: MediaElementAudioSourceNode | null = null;
let attachedElement: HTMLMediaElement | null = null;

/**
 * 懒加载 / 复用全局 AudioContext。
 */
function getOrCreateAudioContext(): AudioContext | null {
  if (typeof window === "undefined") return null;

  const AC =
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).AudioContext ||
    // Safari
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).webkitAudioContext;

  if (!AC) {
    console.warn("[VisualizerNode] 当前环境不支持 Web Audio API。");
    return null;
  }

  if (!audioContext) {
    audioContext = new AC();
  }

  return audioContext;
}

/**
 * 将频谱 Analyser 节点挂在到指定的 <audio> 元素上。
 * 如果浏览器因为 CORS 等原因拒绝 createMediaElementSource，会安全降级。
 */
export function attachVisualizerToAudioElement(
  element: HTMLAudioElement,
): void {
  const ctx = getOrCreateAudioContext();
  if (!ctx) return;

  attachedElement = element;

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
    // 同一个 <audio> 只能 createMediaElementSource 一次，
    // 如果这里失败且没有旧的 sourceNode，只能放弃频谱功能。
    if (!sourceNode) {
      console.warn(
        "[VisualizerNode] createMediaElementSource 失败，频谱不可用：",
        error,
      );
      analyserNode = null;
      return;
    }
    // 有旧 sourceNode 的情况下，继续复用旧的。
  }

  analyserNode = ctx.createAnalyser();
  analyserNode.fftSize = 2048;
  analyserNode.smoothingTimeConstant = 0.8;

  try {
    sourceNode.connect(analyserNode);
    analyserNode.connect(ctx.destination);
  } catch (error) {
    console.warn("[VisualizerNode] 连接 Analyser 节点失败：", error);
    analyserNode = null;
  }
}

/**
 * 解除与当前 <audio> 的绑定。
 */
export function detachVisualizerFromAudioElement(
  element?: HTMLAudioElement,
): void {
  if (element && attachedElement && element !== attachedElement) {
    // 传入的不是当前挂载的元素，直接忽略
    return;
  }

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

  analyserNode = null;
  sourceNode = null;
  attachedElement = null;
}

/**
 * 给前端可视化组件使用的 AnalyserNode。
 * 可能为 null（比如浏览器不支持，或者还没 attach）。
 */
export function getVisualizerAnalyser(): AnalyserNode | null {
  return analyserNode;
}

/**
 * 暴露当前绑定的 <audio> 元素。
 * 主要是为了“读取” attachedElement，避免 TS6133 报错，
 * 同时也方便未来如果需要做一些调试或外部查询。
 */
export function getVisualizerAttachedElement(): HTMLMediaElement | null {
  return attachedElement;
}

/**
 * 有些浏览器 / 系统会把 AudioContext 挂起（suspended），
 * 在需要时手动尝试恢复。
 */
export async function resumeVisualizerAudioContext(): Promise<void> {
  const ctx = audioContext;
  if (!ctx) return;

  if (ctx.state === "suspended") {
    try {
      await ctx.resume();
    } catch {
      // ignore
    }
  }
}
