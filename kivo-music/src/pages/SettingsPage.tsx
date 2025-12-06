// src/pages/SettingsPage.tsx
import { useState } from "react";
import { GeneralSettingsPanel } from "../components/settings/GeneralSettingsPanel";
import { CoverCachePanel } from "../components/settings/CoverCachePanel";
import { DeveloperSettingsPanel } from "../components/settings/DeveloperSettingsPanel";
import { useI18n } from "../i18n";

type SettingsTab = "general" | "coverCache" | "developer";

const tabLabelKeys: Record<SettingsTab, string> = {
  general: "settings.tabs.general",
  coverCache: "settings.tabs.coverCache",
  developer: "settings.tabs.developer",
};

export default function SettingsPage() {
  // 为了和现在行为一致，这里默认选中“封面缓存”tab
  const [activeTab, setActiveTab] = useState<SettingsTab>("coverCache");
  const { t } = useI18n();

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
          letterSpacing: 0.2,
          margin: 0,
        }}
      >
        {t("settings.page.title")}
      </h1>

      <div
        style={{
          display: "flex",
          flex: 1,
          minHeight: 0,
          gap: 12,
        }}
      >
        {/* 左侧 Tab 列表 */}
        <div
          style={{
            width: 200,
            borderRadius: 12,
            padding: 8,
            backgroundColor: "rgba(15,23,42,0.9)",
            border: "1px solid rgba(148,163,184,0.6)",
            boxSizing: "border-box",
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
            {t("settings.page.sidebar.sectionLabel")}
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
                    display: "flex",
                    width: "100%",
                    padding: "6px 10px",
                    borderRadius: 8,
                    borderWidth: 0,
                    fontSize: 14,
                    textAlign: "left",
                    cursor: "pointer",
                    backgroundColor: isActive
                      ? "rgba(255,255,255,0.12)"
                      : "transparent",
                    color: isActive ? "#f9fafb" : "#cbd5f5",
                    marginBottom: 2,
                  }}
                >
                  {t(tabLabelKeys[tab])}
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
