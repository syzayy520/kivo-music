// src/navigation/navigationModel.ts

export type TabKey = "library" | "playlist" | "nowPlaying" | "settings";

export interface NavItem {
  key: TabKey;
  label: string;
  description?: string;
}

export const MAIN_NAV_ITEMS: NavItem[] = [
  {
    key: "library",
    label: "资料库",
    description: "浏览本地音乐、按歌曲/专辑/艺人查看",
  },
  {
    key: "playlist",
    label: "播放列表",
    description: "当前队列与智能列表",
  },
  {
    key: "nowPlaying",
    label: "正在播放",
    description: "大封面视图与详细信息",
  },
  {
    key: "settings",
    label: "设置",
    description: "封面缓存、日志、实验功能",
  },
];
