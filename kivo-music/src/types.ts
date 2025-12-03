// src/types.ts

/**
 * 全局 Track ID 类型。
 * 目前直接用字符串（通常是文件绝对路径），后续可以换成 hash 也不影响外部代码。
 */
export type TrackId = string;

/**
 * Kivo Music 内部使用的统一曲目信息结构。
 *
 * 说明：
 * - 为了兼容老代码，保留了 `filePath` + `path` 两个字段（都指向同一个文件）。
 * - 额外允许透传一些扩展字段（index signature），方便以后加字段不影响老版本。
 */
export interface MusicTrack {
  /** 稳定的唯一 ID，目前等于 filePath（未来可以改成 hash） */
  id: TrackId;

  /** 音频文件绝对路径（推荐新代码都用这个字段） */
  filePath: string;

  /** 兼容老代码的别名字段，表示同样的含义（文件绝对路径） */
  path?: string;

  /** 展示用标题（默认从文件名推断） */
  title: string;

  /** 艺人名称 */
  artist: string;

  /** 专辑名称 */
  album?: string;

  /** 时长（单位：秒），未知时为 0 */
  duration?: number;

  /** 封面图片路径（可以是原图路径或缓存后的路径） */
  coverPath?: string | null;

  /** 加入资料库的时间（ISO 字符串），历史数据可能没有 */
  addedAt?: string;

  /** 播放次数（预留） */
  playCount?: number;

  /** 最近一次播放时间（预留） */
  lastPlayedAt?: string | null;

  /** 预留：允许扩展自定义字段，避免类型过于死板 */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  [key: string]: any;
}

/**
 * 资料库 JSON 文件 schema v1。
 *
 * 保存格式：
 * {
 *   "schemaVersion": 1,
 *   "tracks": [ ... MusicTrack ... ]
 * }
 */
export interface LibraryFileV1 {
  schemaVersion: 1;
  tracks: MusicTrack[];
}

/** 当前使用的资料库 schema 版本号 */
export const CURRENT_LIBRARY_SCHEMA_VERSION = 1;

/** 库文件类型（如果后续有 v2/v3，可以做联合类型） */
export type LibraryFile = LibraryFileV1;
