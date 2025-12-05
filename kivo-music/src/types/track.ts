// 路径：src/types/track.ts
// 说明：
//   统一管理“曲目”相关的核心类型。
//   当前阶段主要是：
//   - 对现有 LibraryTrack / PlayerTrack 的集中出口；
//   - 约定 TrackId / TrackIdentity 的语义；
//   - 定义与库中统计字段对齐的 TrackRuntimeMeta，方便后续统一模型。
//   这一文件本身不包含任何业务逻辑，只做类型与命名的集中管理。

import type { LibraryTrack as LibraryTrackModel } from "../library/libraryModel";
import type { PlayerTrack as PlayerTrackStore } from "../store/player";

/**
 * 曲目主键。
 *
 * 约定：
 * - 在库、队列、统计等场景里，用同一套 TrackId 做关联；
 * - 当前阶段不强制要求和现有 id 字段完全一致，
 *   但所有新代码应优先围绕 TrackId 这一概念来设计。
 */
export type TrackId = string;

/**
 * 曲目运行时元数据。
 *
 * 与 `library/libraryModel.ts` 中 LibraryTrack 上的统计字段保持语义一致：
 * - playCount     ：播放次数
 * - lastPlayedAt  ：最近播放时间
 * - addedAt       ：加入曲库时间
 * - favorite      ：是否标记为“喜欢”
 * - rating        ：预留评分字段
 *
 * 说明：
 * - 字段暂时都设为可选，避免和现有旧数据格式强耦合；
 * - 后续可以在库加载 / 持久化层逐步收紧（例如默认补 0）。
 */
export interface TrackRuntimeMeta {
  /** 总播放次数（对应 LibraryTrack.playCount） */
  playCount?: number;
  /** 最近一次播放时间（ISO 字符串，如 new Date().toISOString()） */
  lastPlayedAt?: string | null;
  /** 添加到曲库的时间（ISO 字符串） */
  addedAt?: string | null;
  /** 是否标记为“喜欢”（对应 LibraryTrack.favorite） */
  favorite?: boolean;
  /** 可选评分（与 LibraryTrack.rating 对齐，预留给后续功能） */
  rating?: number;
}

/**
 * 应用内统一使用的“曲目”基础形态。
 *
 * 当前等价于 libraryModel 中的 LibraryTrack：
 * - 来自本地资料库
 * - 可能包含文件路径、基础元信息、统计数据等
 *
 * 后续如果需要真正拆分 CoreTrack / LibraryTrack，
 * 可以在不改调用方 import 路径的前提下调整这里的定义。
 */
export type CoreTrack = LibraryTrackModel;

/**
 * 资料库中的曲目类型。
 *
 * 实际定义在 `library/libraryModel.ts` 中，这里做一层出口，
 * 方便其它模块统一从 `types/track` 引用。
 */
export type LibraryTrack = LibraryTrackModel;

/**
 * 播放队列中的曲目类型。
 *
 * 实际定义在 `store/player.ts` 中，这里做一层出口，
 * 方便其它模块统一从 `types/track` 引用。
 */
export type PlayerTrack = PlayerTrackStore;

/**
 * 一首曲目的 identity 形式。
 *
 * 典型来源：
 * - track.id / track.trackId
 * - track.filePath / track.path / track.location
 *
 * 说明：
 * - 具体计算逻辑仍由 `libraryModel.getTrackIdentity` 负责；
 * - 这里仅作为语义别名，便于在其它模块表达“这是用于去重/比对的身份串”。
 */
export type TrackIdentity = string;
