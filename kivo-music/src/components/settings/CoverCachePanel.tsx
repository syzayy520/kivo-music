// src/components/settings/CoverCachePanel.tsx
import React, { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import {
  clearCoverCache,
  getCoverCacheStats,
  migrateCoverCache,
  cleanupBrokenCoverEntries,
} from "../../persistence/CoverCache";
import type {
  CoverCacheStats,
  BrokenCoverCleanupResult,
} from "../../persistence/CoverCache";
import {
  getEffectiveCoverCacheDir,
  loadSettings,
  saveSettings,
} from "../../persistence/SettingsPersistence";
import type { KivoSettings } from "../../persistence/SettingsPersistence";
import { CoverCacheDirectorySection } from "./CoverCacheDirectorySection";
import { CoverCacheStatsSection } from "./CoverCacheStatsSection";
import { CoverCacheDebugSection } from "./CoverCacheDebugSection";
import { useI18n } from "../../i18n";

export const CoverCachePanel: React.FC = () => {
  const { t } = useI18n();

  const [settings, setSettings] = useState<KivoSettings>({});
  const [effectiveDir, setEffectiveDir] = useState<string>("");
  const [stats, setStats] = useState<CoverCacheStats | null>(null);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState("");

  async function refreshAll(showMessage = false) {
    const [loadedSettings, dir, s] = await Promise.all([
      loadSettings(),
      getEffectiveCoverCacheDir(),
      getCoverCacheStats(),
    ]);
    setSettings(loadedSettings);
    setEffectiveDir(dir);
    setStats(s);
    if (showMessage) {
      setMessage(t("settings.cache.message.refreshSuccess"));
    }
  }

  useEffect(() => {
    void refreshAll(false);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function handleRefreshStats() {
    if (busy) return;
    setBusy(true);
    setMessage("");
    try {
      await refreshAll(true);
    } catch (error) {
      console.error("[CoverCachePanel] 刷新统计失败:", error);
      setMessage(t("settings.cache.message.refreshFailure"));
    } finally {
      setBusy(false);
    }
  }

  async function handleChooseCoverDir() {
    if (busy) return;
    setBusy(true);
    setMessage("");

    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: t("settings.cache.dialog.chooseDirTitle"),
      });
      if (!selected) {
        // 用户取消
        return;
      }

      const picked = String(selected);
      const oldDir = await getEffectiveCoverCacheDir();
      if (picked === oldDir) {
        setMessage(t("settings.cache.message.dirUnchanged"));
        return;
      }

      const shouldMigrate = window.confirm(
        t("settings.cache.dialog.changeDirConfirm"),
      );

      const newSettings: KivoSettings = {
        ...settings,
        coverCacheDir: picked,
      };
      await saveSettings(newSettings);
      setSettings(newSettings);
      setEffectiveDir(picked);

      if (shouldMigrate) {
        await migrateCoverCache(oldDir, picked);
      }

      setMessage(
        shouldMigrate
          ? t("settings.cache.message.dirUpdatedWithMigrate")
          : t("settings.cache.message.dirUpdatedWithoutMigrate"),
      );

      const newStats = await getCoverCacheStats();
      setStats(newStats);
    } catch (error) {
      console.error("[CoverCachePanel] 选择封面目录失败:", error);
      setMessage(t("settings.cache.message.chooseDirError"));
    } finally {
      setBusy(false);
    }
  }

  async function handleClearCache() {
    if (busy) return;

    const ok = window.confirm(
      t("settings.cache.dialog.clearConfirm"),
    );
    if (!ok) return;

    setBusy(true);
    setMessage("");

    try {
      await clearCoverCache();
      const newStats = await getCoverCacheStats();
      setStats(newStats);
      setMessage(t("settings.cache.message.clearSuccess"));
    } catch (error) {
      console.error("[CoverCachePanel] 清空封面缓存失败:", error);
      setMessage(t("settings.cache.message.clearFailure"));
    } finally {
      setBusy(false);
    }
  }

  async function handleRepairIndex() {
    if (busy) return;

    const ok = window.confirm(
      t("settings.cache.dialog.repairConfirm"),
    );
    if (!ok) return;

    setBusy(true);
    setMessage("");

    try {
      const result: BrokenCoverCleanupResult =
        await cleanupBrokenCoverEntries();
      const newStats = await getCoverCacheStats();
      setStats(newStats);

      const msgLines = [
        t("settings.cache.message.repairCompleted"),
        t("settings.cache.message.repairTrackSummary")
          .replace("{coverChecked}", String(result.coverChecked))
          .replace("{coverRemoved}", String(result.coverRemoved)),
        t("settings.cache.message.repairFolderSummary")
          .replace("{folderChecked}", String(result.folderChecked))
          .replace("{folderRemoved}", String(result.folderRemoved)),
      ];
      setMessage(msgLines.join("\n"));
    } catch (error) {
      console.error("[CoverCachePanel] 修复封面索引失败:", error);
      setMessage(t("settings.cache.message.repairFailure"));
    } finally {
      setBusy(false);
    }
  }

  const messageBox: React.CSSProperties = {
    whiteSpace: "pre-line",
    fontSize: 12,
    padding: message ? 10 : 0,
    borderRadius: 8,
    backgroundColor: message ? "rgba(0,0,0,0.28)" : "transparent",
    border: message ? "1px solid rgba(255,255,255,0.08)" : "none",
    marginTop: 8,
    minHeight: 32,
    transition:
      "background-color 0.15s ease-out, border-color 0.15s ease-out",
  };

  return (
    <div
      style={{
        padding: 16,
        display: "flex",
        flexDirection: "column",
        gap: 16,
      }}
    >
      <CoverCacheDirectorySection
        busy={busy}
        effectiveDir={effectiveDir}
        onChooseDir={handleChooseCoverDir}
      />
      <CoverCacheStatsSection
        busy={busy}
        stats={stats}
        onRefreshStats={handleRefreshStats}
        onRepairIndex={handleRepairIndex}
        onClearCache={handleClearCache}
      />
      <CoverCacheDebugSection />
      <div style={messageBox}>{message || " "}</div>
    </div>
  );
};
