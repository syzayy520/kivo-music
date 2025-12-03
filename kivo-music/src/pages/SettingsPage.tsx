// src/pages/SettingsPage.tsx
import React, { useEffect, useState } from "react";
import { getEffectiveCoverCacheDir } from "../persistence/SettingsPersistence";

const SettingsPage: React.FC = () => {
  const [coversDir, setCoversDir] = useState<string>("正在读取…");

  useEffect(() => {
    (async () => {
      try {
        const dir = await getEffectiveCoverCacheDir();
        setCoversDir(dir);
      } catch (err) {
        console.error("[SettingsPage] getEffectiveCoverCacheDir error:", err);
        setCoversDir("获取失败（详见控制台日志）");
      }
    })();
  }, []);

  return (
    <div
      style={{
        padding: "16px 20px",
        height: "100%",
        overflow: "auto",
        fontSize: 14,
        color: "#111827",
      }}
    >
      <h1 style={{ fontSize: 18, fontWeight: 600, marginBottom: 12 }}>
        设置 / 调试
      </h1>

      <section
        style={{
          marginBottom: 24,
          padding: 16,
          borderRadius: 8,
          border: "1px solid #e5e7eb",
          background: "#ffffff",
        }}
      >
        <h2
          style={{
            fontSize: 15,
            fontWeight: 600,
            marginBottom: 8,
          }}
        >
          封面缓存目录（只读）
        </h2>
        <p style={{ marginBottom: 8, lineHeight: 1.6 }}>
          当前有效的封面缓存目录（后续会在这里支持修改目录 / 迁移缓存）：
        </p>
        <code
          style={{
            display: "block",
            padding: "8px 10px",
            borderRadius: 6,
            background: "#f9fafb",
            border: "1px solid #e5e7eb",
            wordBreak: "break-all",
            fontFamily:
              '"SF Mono", ui-monospace, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
          }}
        >
          {coversDir}
        </code>
      </section>

      <section
        style={{
          padding: 16,
          borderRadius: 8,
          border: "1px solid #e5e7eb",
          background: "#ffffff",
        }}
      >
        <h2
          style={{
            fontSize: 15,
            fontWeight: 600,
            marginBottom: 8,
          }}
        >
          规划中的功能（摘要）
        </h2>
        <ul style={{ paddingLeft: 18, lineHeight: 1.7 }}>
          <li>在这里选择自定义封面缓存目录（例如 D 盘某个文件夹）。</li>
          <li>更换缓存目录时自动迁移已有封面文件，避免“复制一堆垃圾”。</li>
          <li>显示封面缓存占用大小，一键清理缓存。</li>
          <li>展示调试信息：曲目总数、封面命中率、虚拟列表调优开关等。</li>
        </ul>
      </section>
    </div>
  );
};

export default SettingsPage;
