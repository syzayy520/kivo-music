// 路径：src/playerStateMachine.ts
//
// 播放器状态机相关的纯函数集合。
// 设计目标：
// - 把 "下一首 / 上一首 / 循环 / 随机" 的规则集中管理；
// - 不依赖 React / Zustand / PlayerTrack，仅依赖简化的队列游标；
// - 方便单元测试与未来扩展播放模式。
//
// 当前阶段：
// - 仅提供计算下一首/上一首索引的基础实现；
// - 尚未在 store / 组件中接入，行为保持不变；
// - 后续 S1 / S2 中可以逐步让 playerStore 调用这些函数。

/**
 * 播放模式。
 *
 * - "order"      ：顺序播放，到末尾停住；
 * - "repeat-all" ：列表循环，末尾回到首曲；
 * - "repeat-one" ：单曲循环，始终停在当前索引；
 * - "shuffle"    ：随机播放（当前实现为简单随机）。
 */
export type PlayMode = "order" | "repeat-all" | "repeat-one" | "shuffle";

/**
 * 队列游标的抽象。
 *
 * 为了保持与现有实现解耦，只保留最小必要信息：
 * - length      ：当前队列长度；
 * - currentIndex：当前播放索引（-1 表示没有选中任何曲目）。
 */
export interface QueueCursor {
  length: number;
  currentIndex: number;
}

/**
 * 计算“下一首”的索引。
 *
 * 约定：
 * - 返回 -1 表示：没有下一首（例如：空队列，或顺序模式下已经是最后一首）；
 * - 调用方在真正切歌前，应先判断返回值是否合法。
 */
export function computeNextIndex(
  cursor: QueueCursor,
  mode: PlayMode,
): number {
  const { length, currentIndex } = cursor;

  if (length <= 0) return -1;
  if (currentIndex < 0 || currentIndex >= length) {
    // 当前索引非法时，默认从第 0 首开始
    return 0;
  }

  switch (mode) {
    case "repeat-one": {
      // 单曲循环：保持在当前索引
      return currentIndex;
    }
    case "order": {
      // 顺序播放：最后一首之后就没有下一首了
      if (currentIndex >= length - 1) return -1;
      return currentIndex + 1;
    }
    case "repeat-all": {
      // 列表循环：末尾回到开头
      return (currentIndex + 1) % length;
    }
    case "shuffle": {
      // 简单随机：从所有曲目中随机一个索引
      // （后续可以改成：避免连续两次随机到同一首、或使用预先打乱的列表等）
      if (length === 1) return currentIndex;
      const next = randomIntExcluding(length, currentIndex);
      return next;
    }
    default: {
      // 兜底：保持当前索引不变
      return currentIndex;
    }
  }
}

/**
 * 计算“上一首”的索引。
 *
 * 约定：
 * - 返回 -1 表示：没有上一首（例如：顺序模式下已经是第一首）；
 */
export function computePrevIndex(
  cursor: QueueCursor,
  mode: PlayMode,
): number {
  const { length, currentIndex } = cursor;

  if (length <= 0) return -1;
  if (currentIndex < 0 || currentIndex >= length) {
    // 当前索引非法时，默认从第 0 首开始
    return 0;
  }

  switch (mode) {
    case "repeat-one": {
      return currentIndex;
    }
    case "order": {
      // 顺序播放：已经是第一首就没有上一首
      if (currentIndex <= 0) return -1;
      return currentIndex - 1;
    }
    case "repeat-all": {
      // 列表循环：第一首的上一首是最后一首
      if (currentIndex <= 0) return length - 1;
      return currentIndex - 1;
    }
    case "shuffle": {
      if (length === 1) return currentIndex;
      const prev = randomIntExcluding(length, currentIndex);
      return prev;
    }
    default: {
      return currentIndex;
    }
  }
}

/**
 * 判断队列是否“实质上结束”。
 *
 * 目前只在顺序播放（order）下有意义：
 * - order 模式下，当前已是最后一首且 computeNextIndex 返回 -1；
 * - 其他模式一律认为不会“结束”（因为可以根据模式继续切歌）。
 *
 * 暂时未在外部使用，后续可以配合“播放完自动停止”之类的逻辑。
 */
export function isQueueExhausted(
  cursor: QueueCursor,
  mode: PlayMode,
): boolean {
  if (mode !== "order") return false;
  if (cursor.length <= 0) return true;
  if (cursor.currentIndex < 0 || cursor.currentIndex >= cursor.length) {
    return true;
  }
  return cursor.currentIndex === cursor.length - 1;
}

/**
 * 从 [0, length) 范围内随机一个整数，且尽量避免 equalTo。
 *
 * 假设 length >= 2。
 */
function randomIntExcluding(length: number, equalTo: number): number {
  const raw = Math.floor(Math.random() * length);
  if (raw === equalTo) {
    // 简单处理：往前或往后挪一位
    if (raw === 0) return 1;
    return raw - 1;
  }
  return raw;
}
