// src/components/settings/CoverCacheStatsSection.tsx
import React from "react";
import type { CoverCacheStats } from "../../persistence/CoverCache";
import { useI18n } from "../../i18n";

interface Props {
  busy: boolean;
  stats: CoverCacheStats | null;
  onRefreshStats: () => void | Promise<void>;
  onRepairIndex: () => void | Promise<void>;
  onClearCache: () => void | Promise<void>;
}

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({
  title,
  children,
}) => (
  <section style={{ marginBottom: 24 }}>
    <h2
      style={{
        fontSize: 16,
        fontWeight: 600,
        marginBottom: 8,
      }}
    >
      {title}
    </h2>
    <div
      style={{
        padding: 12,
        borderRadius: 10,
        border: "1px solid rgba(255,255,255,0.08)",
        background:
          "linear-gradient(135deg, rgba(255,255,255,0.04), rgba(0,0,0,0.15))",
        boxShadow: "0 8px 20px rgba(0,0,0,0.15)",
      }}
    >
      {children}
    </div>
  </section>
);

const buttonBase: React.CSSProperties = {
  padding: "7px 14px",
  borderRadius: 999,
  border: "none",
  fontSize: 13,
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  gap: 6,
  transition:
    "transform 0.1s ease-out, box-shadow 0.15s ease-out, background-color 0.15s ease-out",
  boxShadow: "0 4px 10px rgba(0,0,0,0.18)",
  minWidth: 96,
  height: 30,
};

export const CoverCacheStatsSection: React.FC<Props> = ({
  busy,
  stats,
  onRefreshStats,
  onRepairIndex,
  onClearCache,
}) => {
  const { t } = useI18n();

  if (!stats) {
    return (
      <Section title={t("settings.cache.stats.title")}>
        <div style={{ fontSize: 13 }}>
          {t("settings.cache.stats.loading")}
        </div>
      </Section>
    );
  }

  return (
    <Section title={t("settings.cache.stats.title")}>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 8,
          fontSize: 13,
        }}
      >
        <div>
          {t("settings.cache.stats.fileCountLabel")}
          {stats.fileCount}
        </div>
        <div>
          {t("settings.cache.stats.sizeLabel")}
          {stats.humanReadableSize}
        </div>
        <div>
          {t("settings.cache.stats.trackEntriesLabel")}
          {stats.trackEntries}
        </div>
        <div>
          {t("settings.cache.stats.folderEntriesLabel")}
          {stats.folderEntries}
        </div>

        <div
          style={{
            display: "flex",
            flexWrap: "wrap",
            gap: 8,
            marginTop: 10,
          }}
        >
          <button
            type="button"
            disabled={busy}
            onClick={() => {
              void onRefreshStats();
            }}
            style={{
              ...buttonBase,
              cursor: busy ? "default" : "pointer",
              backgroundColor: busy ? "#4b4f58" : "#4a4f5c",
              color: "#fff",
            }}
          >
            {busy
              ? t("settings.cache.stats.action.processing")
              : t("settings.cache.stats.action.refresh")}
          </button>

          <button
            type="button"
            disabled={busy}
            onClick={() => {
              void onRepairIndex();
            }}
            style={{
              ...buttonBase,
              cursor: busy ? "default" : "pointer",
              backgroundColor: busy ? "#4f7d5a" : "#27ae60",
              color: "#fff",
            }}
          >
            {t("settings.cache.stats.action.repairIndex")}
          </button>

          <button
            type="button"
            disabled={busy}
            onClick={() => {
              void onClearCache();
            }}
            style={{
              ...buttonBase,
              cursor: busy ? "default" : "pointer",
              backgroundColor: busy ? "#874040" : "#c0392b",
              color: "#fff",
            }}
          >
            {t("settings.cache.stats.action.clearCache")}
          </button>
        </div>
      </div>
    </Section>
  );
};
