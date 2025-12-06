// src/navigation/navigationModel.ts

export type TabKey = "library" | "playlist" | "nowPlaying" | "settings";

/**
 * 单个导航项的定义。
 *
 * label / description：
 *   - 提供默认文案（主要是中文），用于 i18n 找不到 key 时的兜底。
 *
 * labelKey / descriptionKey：
 *   - i18n 的 key，用于根据当前语言返回正确文案。
 */
export interface NavItem {
  key: TabKey;
  label: string;
  description?: string;
  labelKey?: string;
  descriptionKey?: string;
}

export const MAIN_NAV_ITEMS: NavItem[] = [
  {
    key: "library",
    label: "资料库",
    description: "浏览本地音乐、按歌曲/专辑/艺人查看",
    labelKey: "nav.library.label",
    descriptionKey: "nav.library.description",
  },
  {
    key: "playlist",
    label: "播放列表",
    description: "当前队列与智能列表",
    labelKey: "nav.playlist.label",
    descriptionKey: "nav.playlist.description",
  },
  {
    key: "nowPlaying",
    label: "正在播放",
    description: "大封面视图与详细信息",
    labelKey: "nav.nowPlaying.label",
    descriptionKey: "nav.nowPlaying.description",
  },
  {
    key: "settings",
    label: "设置",
    description: "封面缓存、日志、实验功能",
    labelKey: "nav.settings.label",
    descriptionKey: "nav.settings.description",
  },
];
