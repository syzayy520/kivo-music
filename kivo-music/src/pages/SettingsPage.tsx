// src/pages/SettingsPage.tsx
import React, { useEffect, useState } from "react";
import { appDataDir, join } from "@tauri-apps/api/path";

const SettingsPage: React.FC = () => {
  const [coversDir, setCoversDir] = useState<string>("加载中…");

  useEffect(() => {
    let cancelled = false;

    const loadPath = async () => {
      try {
        const root = await appDataDir();
        const dir = await join(root, "covers");
        if (!cancelled) {
          setCoversDir(dir);
        }
      } catch (e) {
        console.error("[SettingsPage] load covers dir failed:", e);
        if (!cancelled) {
          setCoversDir("无法获取路径（请查看控制台日志）");
        }
      }
    };

    loadPath();

    return () => {
      cancelled = true;
    };
  }, []);

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
          这里会集中放置主题、缓存目录等通用设置。当前先把封面缓存的位置展示出来，
          后面我们会在这里加上“更改缓存目录”的功能。
        </p>
      </div>

      {/* 封面缓存位置 */}
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
          目前所有封面图片都会缓存到以下目录（在 Windows 上一般在
          用户的 AppData Roaming 区域）：
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
        <p
          style={{
            fontSize: 12,
            color: "#6b7280",
            marginTop: 6,
          }}
        >
          未来我们会在这里提供一个按钮，让你可以选择其他磁盘（例如 D 盘 /
          E 盘）作为封面缓存目录，并自动迁移已有封面。
        </p>
      </section>

      {/* 主题设置占位 */}
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
          主题（预留）
        </div>
        <p
          style={{
            fontSize: 12,
            color: "#6b7280",
            marginBottom: 8,
          }}
        >
          这里以后会支持「浅色 / 深色 / 跟随系统」，目前只是预留位置，还没真正改变
          UI 颜色。
        </p>
        <div
          style={{
            display: "flex",
            gap: 12,
            fontSize: 12,
            color: "#4b5563",
          }}
        >
          <label>
            <input type="radio" name="theme" defaultChecked /> 跟随系统
          </label>
          <label>
            <input type="radio" name="theme" /> 浅色
          </label>
          <label>
            <input type="radio" name="theme" /> 深色
          </label>
        </div>
      </section>
    </div>
  );
};

export default SettingsPage;
