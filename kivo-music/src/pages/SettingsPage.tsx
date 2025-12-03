// src/pages/SettingsPage.tsx
import React, { useEffect, useState } from "react";
import { appDataDir, join } from "@tauri-apps/api/path";
import type { Theme } from "../persistence/SettingsPersistence";
import {
  loadSettings,
  updateSettings,
  getEffectiveCoverCacheDir,
} from "../persistence/SettingsPersistence";

interface SettingsPageProps {
  theme: Theme;
  onThemeChange: (next: Theme) => void;
}

const SettingsPage: React.FC<SettingsPageProps> = ({
  theme,
  onThemeChange,
}) => {
  const [coversDir, setCoversDir] = useState<string>("加载中…");
  const [settingsLoaded, setSettingsLoaded] = useState(false);
  const [defaultCoversDir, setDefaultCoversDir] = useState<string>("");

  useEffect(() => {
    let cancelled = false;

    const load = async () => {
      try {
        // 有效的封面缓存目录（后面可以变更）
        const effective = await getEffectiveCoverCacheDir();
        if (!cancelled) setCoversDir(effective);

        // 从 settings.json 里再读一遍，防止被手动改过
        const s = await loadSettings();
        if (!cancelled && s.theme && s.theme !== theme) {
          onThemeChange(s.theme);
        }

        if (!cancelled) setSettingsLoaded(true);
      } catch (e) {
        console.error("[SettingsPage] load error:", e);
        if (!cancelled) {
          setCoversDir("无法获取路径（请查看控制台日志）");
          setSettingsLoaded(true);
        }
      }
    };

    load();

    return () => {
      cancelled = true;
    };
  }, [theme, onThemeChange]);

  useEffect(() => {
    // 默认封面目录：AppData/covers
    (async () => {
      try {
        const root = await appDataDir();
        const dir = await join(root, "covers");
        setDefaultCoversDir(dir);
      } catch (e) {
        console.error("[SettingsPage] load default covers dir error:", e);
      }
    })();
  }, []);

  const handleThemeChange = async (next: Theme) => {
    onThemeChange(next);
    await updateSettings({ theme: next });
  };

  const themeLabel = (t: Theme) => {
    if (t === "system") return "跟随系统";
    if (t === "light") return "浅色";
    if (t === "dark") return "深色";
    return t;
  };

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        gap: 16,
        fontSize: 13,
        color: "#111827",
      }}
    >
      <div>
        <h1
          style={{
            fontSize: 18,
            fontWeight: 600,
            marginBottom: 4,
          }}
        >
          设置
        </h1>
        <p
          style={{
            fontSize: 12,
            color: "#6b7280",
          }}
        >
          这里会集中放置主题、缓存目录等通用设置。当前版本已经可以记住你选择的主题，
          封面缓存目录暂时还固定在 AppData 下，后续版本会支持切换到其他磁盘。
        </p>
      </div>

      {/* 主题 */}
      <section
        style={{
          padding: 12,
          borderRadius: 8,
          border: "1px solid #e5e7eb",
          backgroundColor: "#ffffff",
        }}
      >
        <div
          style={{
            fontSize: 14,
            fontWeight: 600,
            marginBottom: 4,
          }}
        >
          主题
        </div>
        <p
          style={{
            fontSize: 12,
            color: "#6b7280",
            marginBottom: 8,
          }}
        >
          当前只会影响整体背景色等基础样式，后面 UI 定稿后会进一步细化到各个控件。
        </p>
        <div
          style={{
            display: "flex",
            gap: 12,
            fontSize: 12,
            color: "#4b5563",
          }}
        >
          {(["system", "light", "dark"] as Theme[]).map((value) => (
            <label key={value} style={{ cursor: "pointer" }}>
              <input
                type="radio"
                name="theme"
                value={value}
                checked={theme === value}
                onChange={() => handleThemeChange(value)}
                style={{ marginRight: 4 }}
              />
              {themeLabel(value)}
            </label>
          ))}
        </div>
        {!settingsLoaded && (
          <p
            style={{
              fontSize: 11,
              color: "#9ca3af",
              marginTop: 6,
            }}
          >
            正在读取设置…
          </p>
        )}
      </section>

      {/* 封面缓存位置（只读展示） */}
      <section
        style={{
          padding: 12,
          borderRadius: 8,
          border: "1px solid #e5e7eb",
          backgroundColor: "#f9fafb",
        }}
      >
        <div
          style={{
            fontSize: 14,
            fontWeight: 600,
            marginBottom: 4,
          }}
        >
          封面缓存目录（当前版本）
        </div>
        <p
          style={{
            fontSize: 12,
            color: "#4b5563",
            marginBottom: 6,
          }}
        >
          目前所有封面图片都会缓存到以下目录：
        </p>
        <code
          style={{
            display: "block",
            padding: "6px 8px",
            borderRadius: 6,
            backgroundColor: "#111827",
            color: "#e5e7eb",
            fontSize: 12,
            wordBreak: "break-all",
          }}
        >
          {coversDir}
        </code>
        {defaultCoversDir && (
          <p
            style={{
              fontSize: 11,
              color: "#6b7280",
              marginTop: 6,
            }}
          >
            默认目录为：{defaultCoversDir}
            。后续我们会在这里增加一个「选择目录…」按钮，让你可以把封面统一缓存到
            D 盘 / E 盘等其他硬盘。
          </p>
        )}
      </section>
    </div>
  );
};

export default SettingsPage;
