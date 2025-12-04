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

export const CoverCachePanel: React.FC = () => {
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
      setMessage("封面缓存统计已刷新。");
    }
  }

  useEffect(() => {
    void refreshAll(false);
  }, []);

  async function handleRefreshStats() {
    if (busy) return;
    setBusy(true);
    setMessage("");
    try {
      await refreshAll(true);
    } catch (error) {
      console.error("[CoverCachePanel] 刷新统计失败:", error);
      setMessage("刷新封面缓存统计失败，请检查控制台日志。");
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
        title: "选择封面缓存目录",
      });

      if (!selected) {
        // 用户取消
        return;
      }

      const picked = String(selected);
      const oldDir = await getEffectiveCoverCacheDir();

      if (picked === oldDir) {
        setMessage("封面缓存目录没有变化。");
        return;
      }

      const shouldMigrate = window.confirm(
        [
          "检测到你修改了封面缓存目录。",
          "",
          "是否将当前封面缓存一起迁移到新目录？",
          "",
          "推荐选择【是】，这样已经设置好的封面不会丢失；",
          "选择【否】则从空缓存开始，新封面会写到新目录。",
        ].join("\n"),
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

      const newStats = await getCoverCacheStats();
      setStats(newStats);
      setMessage(
        "封面缓存目录已更新。" +
          (shouldMigrate ? " 已尝试迁移旧缓存。" : " 未迁移旧缓存。"),
      );
    } catch (error) {
      console.error("[CoverCachePanel] 选择封面目录失败:", error);
      setMessage("选择封面缓存目录时发生错误，请检查控制台日志。");
    } finally {
      setBusy(false);
    }
  }

  async function handleClearCache() {
    if (busy) return;

    const ok = window.confirm(
      [
        "确定要清空所有封面缓存吗？",
        "",
        "这不会删除你的音乐或 kivo-library.json，",
        "只是删掉封面缓存，下次会重新按规则加载 / 选择封面。",
      ].join("\n"),
    );
    if (!ok) return;

    setBusy(true);
    setMessage("");

    try {
      await clearCoverCache();
      const newStats = await getCoverCacheStats();
      setStats(newStats);
      setMessage("封面缓存已完全清空。");
    } catch (error) {
      console.error("[CoverCachePanel] 清空封面缓存失败:", error);
      setMessage("清空封面缓存失败，请检查控制台日志。");
    } finally {
      setBusy(false);
    }
  }

  async function handleRepairIndex() {
    if (busy) return;

    const ok = window.confirm(
      [
        "此操作会扫描封面索引（covers.json / folder-covers.json），",
        "并自动移除那些指向“已经不存在的文件”的记录。",
        "",
        "不会删除任何新的封面文件，也不会影响音乐库，只是做索引自检 / 清理。",
        "",
        "确定要继续吗？",
      ].join("\n"),
    );
    if (!ok) return;

    setBusy(true);
    setMessage("");

    try {
      const result: BrokenCoverCleanupResult =
        await cleanupBrokenCoverEntries();
      const newStats = await getCoverCacheStats();
      setStats(newStats);

      setMessage(
        [
          "封面索引自检完成：",
          `Track 封面索引检查 ${result.coverChecked} 条，移除 ${result.coverRemoved} 条失效记录；`,
          `文件夹封面索引检查 ${result.folderChecked} 条，移除 ${result.folderRemoved} 条失效记录。`,
        ].join("\n"),
      );
    } catch (error) {
      console.error("[CoverCachePanel] 修复封面索引失败:", error);
      setMessage("修复封面索引失败，请检查控制台日志。");
    } finally {
      setBusy(false);
    }
  }

  const buttonBase: React.CSSProperties = {
    padding: "7px 14px",
    borderRadius: 999,
    border: "none",
    fontSize: 13,
    display: "inline-flex",
    alignItems: "center",
    justifyContent: "center",
    gap: 6,
    cursor: busy ? "default" : "pointer",
    transition:
      "transform 0.1s ease-out, box-shadow 0.15s ease-out, background-color 0.15s ease-out",
    boxShadow: "0 4px 10px rgba(0,0,0,0.18)",
    minWidth: 96,
    height: 30,
  };

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
      <Section title="封面缓存目录">
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: 10,
          }}
        >
          <div>
            <div style={{ opacity: 0.8, fontSize: 13, marginBottom: 4 }}>
              当前有效目录（实际正在使用的目录）：
            </div>
            <code
              style={{
                fontSize: 12,
                padding: 6,
                borderRadius: 6,
                backgroundColor: "rgba(0,0,0,0.35)",
                wordBreak: "break-all",
              }}
            >
              {effectiveDir || "加载中…"}
            </code>
          </div>

          <button
            type="button"
            disabled={busy}
            onClick={handleChooseCoverDir}
            style={{
              ...buttonBase,
              alignSelf: "flex-start",
              backgroundColor: busy ? "#5b6b8a" : "#2b7cff",
              color: "#fff",
            }}
          >
            选择封面缓存目录…
          </button>

          <div style={{ opacity: 0.7, fontSize: 12 }}>
            默认会使用系统 AppData 下的
            <code style={{ marginLeft: 4, marginRight: 4 }}>
              /com.administrator.kivo-music/covers
            </code>
            ，你可以把缓存迁移到 D/E 盘等更大的磁盘。
          </div>
        </div>
      </Section>

      <Section title="封面缓存统计 & 管理">
        {stats ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 8,
              fontSize: 13,
            }}
          >
            <div>缓存文件数：{stats.fileCount}</div>
            <div>缓存总大小：{stats.humanReadableSize}</div>
            <div>Track 封面索引条数：{stats.trackEntries}</div>
            <div>文件夹封面索引条数：{stats.folderEntries}</div>

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
                onClick={handleRefreshStats}
                style={{
                  ...buttonBase,
                  backgroundColor: busy ? "#4b4f58" : "#4a4f5c",
                  color: "#fff",
                }}
              >
                {busy ? "正在处理…" : "刷新统计"}
              </button>

              <button
                type="button"
                disabled={busy}
                onClick={handleRepairIndex}
                style={{
                  ...buttonBase,
                  backgroundColor: busy ? "#4f7d5a" : "#27ae60",
                  color: "#fff",
                }}
              >
                修复封面索引
              </button>

              <button
                type="button"
                disabled={busy}
                onClick={handleClearCache}
                style={{
                  ...buttonBase,
                  backgroundColor: busy ? "#874040" : "#c0392b",
                  color: "#fff",
                }}
              >
                清空封面缓存
              </button>
            </div>
          </div>
        ) : (
          <div style={{ fontSize: 13 }}>正在加载缓存统计信息…</div>
        )}
      </Section>

      <Section title="调试说明（给后来接手的人看的小注释）">
        <ul style={{ margin: 0, paddingLeft: 18, fontSize: 12, opacity: 0.8 }}>
          <li>
            封面缓存目录和设置保存在{" "}
            <code>AppData/com.administrator.kivo-music</code> 下面；
          </li>
          <li>
            <code>covers.json</code> 记录“track ➜ 封面缓存文件路径”；
          </li>
          <li>
            <code>folder-covers.json</code>{" "}
            记录“文件夹 ➜ 封面扫描结果（包括没有封面）”，
            用来避免对不存在的 <code>cover.jpg</code> 重复发请求；
          </li>
          <li>
            播放页 / 列表页只要改用{" "}
            <code>resolveCoverPathForTrack(track)</code>{" "}
            拿封面，就可以统一走这套缓存逻辑。
          </li>
        </ul>
      </Section>

      <div style={messageBox}>{message || " "}</div>
    </div>
  );
};
