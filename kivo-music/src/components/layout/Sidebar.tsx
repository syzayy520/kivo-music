// src/components/layout/Sidebar.tsx
import React from "react";
import type { TabKey } from "../../navigation/navigationModel";
import { MAIN_NAV_ITEMS } from "../../navigation/navigationModel";

interface SidebarProps {
  currentTab: TabKey;
  onChangeTab: (tab: TabKey) => void;
  onEnterMiniMode: () => void;
}

const sidebarStyle: React.CSSProperties = {
  width: 220,
  padding: "14px 12px",
  boxSizing: "border-box",
  display: "flex",
  flexDirection: "column",
  gap: 12,
  borderRight: "1px solid #e5e7eb",
  background:
    "linear-gradient(180deg, #020617 0%, #020617 40%, #020617 55%, #0f172a 100%)",
  color: "#e5e7eb",
};

const appTitleStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: 8,
};

const navListStyle: React.CSSProperties = {
  marginTop: 8,
  display: "flex",
  flexDirection: "column",
  gap: 4,
};

const navButtonBase: React.CSSProperties = {
  width: "100%",
  borderRadius: 9999,
  padding: "6px 10px",
  border: "none",
  background: "transparent",
  color: "#cbd5f5",
  fontSize: 13,
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  cursor: "pointer",
};

const miniSectionStyle: React.CSSProperties = {
  marginTop: "auto",
  paddingTop: 10,
  borderTop: "1px solid rgba(148,163,184,0.3)",
};

const miniButtonStyle: React.CSSProperties = {
  width: "100%",
  borderRadius: 9999,
  padding: "6px 10px",
  border: "1px solid rgba(148,163,184,0.8)",
  background: "rgba(15,23,42,0.9)",
  color: "#e5e7eb",
  fontSize: 12,
  cursor: "pointer",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  gap: 6,
  whiteSpace: "nowrap",
};

export const Sidebar: React.FC<SidebarProps> = ({
  currentTab,
  onChangeTab,
  onEnterMiniMode,
}) => {
  return (
    <aside style={sidebarStyle}>
      {/* 顶部 Logo / 标题 */}
      <div style={appTitleStyle}>
        <div
          style={{
            width: 18,
            height: 18,
            borderRadius: "9999px",
            background:
              "radial-gradient(circle at 30% 30%, #22c55e, #16a34a)",
          }}
        />
        <div>
          <div style={{ fontSize: 14, fontWeight: 600 }}>Kivo Music</div>
          <div
            style={{
              fontSize: 11,
              color: "rgba(148,163,184,0.9)",
            }}
          >
            本地播放器 · v3
          </div>
        </div>
      </div>

      {/* 导航 */}
      <nav style={navListStyle}>
        {MAIN_NAV_ITEMS.map((item) => {
          const active = item.key === currentTab;

          return (
            <button
              key={item.key}
              type="button"
              onClick={() => onChangeTab(item.key)}
              style={{
                ...navButtonBase,
                background: active
                  ? "linear-gradient(90deg,#1d4ed8,#2563eb)"
                  : "transparent",
                color: active ? "#f9fafb" : "#cbd5f5",
              }}
            >
              <span
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 6,
                }}
              >
                <span
                  style={{
                    width: 4,
                    height: 4,
                    borderRadius: 9999,
                    backgroundColor: active ? "#f9fafb" : "transparent",
                  }}
                />
                <span>{item.label}</span>
              </span>
              {active && (
                <span
                  style={{
                    fontSize: 10,
                    opacity: 0.9,
                  }}
                >
                  ●
                </span>
              )}
            </button>
          );
        })}
      </nav>

      {/* Mini 模式入口 */}
      <div style={miniSectionStyle}>
        <button
          type="button"
          style={miniButtonStyle}
          onClick={onEnterMiniMode}
          title="切换到极简 Mini 模式"
        >
          ◻ 极简 Mini 模式
        </button>
        <div
          style={{
            marginTop: 4,
            fontSize: 10,
            color: "rgba(148,163,184,0.9)",
          }}
        >
          Mini 模式共享同一播放队列，可随时用 Esc 或快捷键返回。
        </div>
      </div>
    </aside>
  );
};
