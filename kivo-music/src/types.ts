// 全局通用类型

export interface MusicTrack {
  /** 轨道唯一 ID，可以直接用 path */
  id: string;
  /** 歌曲名（先用文件名，后面再用标签覆写） */
  title: string;
  /** 艺人 */
  artist: string;
  /** 专辑 */
  album: string;
  /** 时长（秒），现在还没有真实值可以先为 0 */
  duration: number;
  /** 本地绝对路径 */
  path: string;
}
