// src/pages/SettingsPage.tsx
import React, { useState } from "react";
import { GeneralSettingsPanel } from "../components/settings/GeneralSettingsPanel";
import { CoverCachePanel } from "../components/settings/CoverCachePanel";
import { DeveloperSettingsPanel } from "../components/settings/DeveloperSettingsPanel";

type SettingsTab = "general" | "coverCache" | "developer";

const tabLabels: Record<SettingsTab, string> = {
  general: "常规",
  coverCache: "封面缓存",
  developer: "开发者",
};

export default function SettingsPage() {
  // 为了和现在行为一致，这里默认选中“封面缓存”tab
  const [activeTab, setActiveTab] = useState<SettingsTab>("coverCache");

  function renderActivePanel() {
    switch (activeTab) {
      case "general":
        return <GeneralSettingsPanel />;
      case "coverCache":
        return <CoverCachePanel />;
      case "developer":
        return <DeveloperSettingsPanel />;
      default:
        return null;
    }
  }

  return (
    <div
      style={{
        padding: 16,
        display: "flex",
        flexDirection: "column",
        gap: 12,
        color: "#fff",
        height: "100%",
        boxSizing: "border-box",
      }}
    >
      <h1
        style={{
          fontSize: 20,
          fontWeight: 600,
          marginBottom: 4,
        }}
      >
        设置
      </h1>

      <div
        style={{
          display: "flex",
          flex: 1,
          minHeight: 0,
          borderRadius: 12,
          border: "1px solid rgba(255,255,255,0.06)",
          overflow: "hidden",
        }}
      >
        {/* 左侧 Tab 列表 */}
        <div
          style={{
            width: 160,
            borderRight: "1px solid rgba(255,255,255,0.06)",
            padding: 8,
            boxSizing: "border-box",
            display: "flex",
            flexDirection: "column",
            gap: 4,
            background:
              "linear-gradient(180deg, rgba(0,0,0,0.4), rgba(0,0,0,0.7))",
          }}
        >
          <div
            style={{
              padding: "4px 8px 8px",
              fontSize: 14,
              fontWeight: 600,
              opacity: 0.9,
            }}
          >
            设置分类
          </div>

          {(["general", "coverCache", "developer"] as SettingsTab[]).map(
            (tab) => {
              const isActive = activeTab === tab;
              return (
                <button
                  key={tab}
                  type="button"
                  onClick={() => setActiveTab(tab)}
                  style={{
                    textAlign: "left",
                    width: "100%",
                    padding: "6px 10px",
                    fontSize: 13,
                    borderRadius: 8,
                    border: "none",
                    cursor: "pointer",
                    backgroundColor: isActive
                      ? "rgba(255,255,255,0.12)"
                      : "transparent",
                    color: isActive ? "#fff" : "#ccc",
                    transition:
                      "background-color 0.12s ease-out, color 0.12s ease-out",
                  }}
                >
                  {tabLabels[tab]}
                </button>
              );
            },
          )}
        </div>

        {/* 右侧内容区域 */}
        <div
          style={{
            flex: 1,
            minWidth: 0,
            minHeight: 0,
            overflow: "auto",
            background:
              "linear-gradient(135deg, rgba(255,255,255,0.04), rgba(0,0,0,0.4))",
          }}
        >
          {renderActivePanel()}
        </div>
      </div>
    </div>
  );
}
