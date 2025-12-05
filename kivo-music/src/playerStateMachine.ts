// src/playerStateMachine.ts
//
// 播放器“模式 + 队列游标”相关的纯函数集合。
// 目前主要职责：
//   - 统一 PlayMode 的类型定义；
//   - 封装“下一首 / 上一首 / 队列是否播放完”的索引计算逻辑；
//   - 不依赖 React / Zustand / DOM，方便单元测试与重用。

/**
 * 播放模式：
 * - "order"      ：顺序播放，到最后一首后停止；
 * - "repeat-all" ：列表循环，末尾回到第一首；
 * - "repeat-one" ：单曲循环，当前曲目播完后回到开头。
 */
export type PlayMode = "order" | "repeat-all" | "repeat-one";

/**
 * 队列游标：抽象出与具体 Track 无关的“播放位置”。
 */
export interface QueueCursor {
  /** 当前队列长度 */
  length: number;
  /** 当前播放索引；若为 -1 表示尚未选中任何曲目 */
  currentIndex: number;
}

/**
 * 计算“下一首”的索引。
 *
 * 约定：
 * - 返回 -1 表示：没有下一首（例如顺序播放到达末尾）。
 */
export function computeNextIndex(
  cursor: QueueCursor,
  mode: PlayMode,
): number {
  const { length, currentIndex } = cursor;

  if (length <= 0) return -1;

  // 未选中任何曲目时，默认从 0 开始
  if (currentIndex < 0 || currentIndex >= length) {
    return 0;
  }

  const lastIndex = length - 1;

  if (mode === "repeat-one") {
    // 单曲循环：保持在当前索引
    return currentIndex;
  }

  if (currentIndex >= lastIndex) {
    if (mode === "repeat-all") {
      // 列表循环：从末尾回到开头
      return 0;
    }
    // 顺序播放：已经是最后一首了，没有下一首
    return -1;
  }

  // 正常顺序 +1
  return currentIndex + 1;
}

/**
 * 计算“上一首”的索引。
 *
 * 约定：
 * - 返回 -1 表示：没有上一首（例如顺序播放、当前 index 非法等）。
 *
 * 注意：
 * - 这里只关注“索引变化”，
 *   “若已播放超过 N 秒则先回到本曲开头”这一类时间逻辑继续交给 player store。
 */
export function computePrevIndex(
  cursor: QueueCursor,
  mode: PlayMode,
): number {
  const { length, currentIndex } = cursor;

  if (length <= 0) return -1;

  if (currentIndex < 0 || currentIndex >= length) {
    return 0;
  }

  if (mode === "repeat-one") {
    return currentIndex;
  }

  if (currentIndex === 0) {
    // 对 repeat-all / order：都视为在第一首，由调用方决定只“回到开头”还是“停在第 0 首”
    return 0;
  }

  return currentIndex - 1;
}

/**
 * 判断队列在当前模式下是否“已经播放到最后一首”。
 *
 * 目前只对顺序播放（order）有意义：
 * - 列表为空或 currentIndex 非法：视为已结束；
 * - currentIndex === length - 1：视为已在末尾。
 */
export function isQueueExhausted(
  cursor: QueueCursor,
  mode: PlayMode,
): boolean {
  if (mode !== "order") return false;
  const { length, currentIndex } = cursor;
  if (length <= 0) return true;
  if (currentIndex < 0 || currentIndex >= length) return true;
  return currentIndex === length - 1;
}
