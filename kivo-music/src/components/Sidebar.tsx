import React from "react";

export type TabKey = "library" | "playlist" | "nowplaying";

interface SidebarProps {
  current: TabKey;
  onChange: (tab: TabKey) => void;
}

const tabs: { key: TabKey; label: string }[] = [
  { key: "library", label: "资料库" },
  { key: "playlist", label: "播放列表" },
  { key: "nowplaying", label: "正在播放" }
];

export const Sidebar: React.FC<SidebarProps> = ({
  current,
  onChange
}) => {
  return (
    <aside className="w-56 border-r border-gray-200 bg-white/60 h-full flex flex-col">
      <div className="px-4 pt-4 pb-2">
        <div className="flex items-center gap-2 text-lg font-semibold">
          <span className="text-purple-500 text-2xl">🎵</span>
          <span>Kivo Music</span>
        </div>
      </div>

      <nav className="mt-2 px-2 space-y-1">
        {tabs.map((tab) => {
          const active = tab.key === current;
          return (
            <button
              key={tab.key}
              onClick={() => onChange(tab.key)}
              className={
                "w-full text-left px-3 py-2 rounded-md text-sm " +
                (active
                  ? "bg-purple-100 text-purple-700 font-medium"
                  : "text-gray-700 hover:bg-gray-100")
              }
            >
              {tab.label}
            </button>
          );
        })}
      </nav>

      <div className="mt-auto p-3 text-xs text-gray-400">
        本地音乐播放器 · 未来 20+ 年 计划版
      </div>
    </aside>
  );
};
