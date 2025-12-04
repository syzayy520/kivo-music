// src/components/settings/GeneralSettingsPanel.tsx
import React from "react";
import { useKivoTheme } from "../../styles/ThemeContext";

export const GeneralSettingsPanel: React.FC = () => {
  const { theme, themeName } = useKivoTheme();

  const readableThemeName =
    themeName === "kivoDefault" ? "Kivo 默认 · Apple 风格底色" : themeName;

  const containerStyle: React.CSSProperties = {
    padding: 16,
    display: "flex",
    flexDirection: "column",
    gap: 16,
  };

  const sectionStyle: React.CSSProperties = {
    padding: 16,
    borderRadius: theme.radius.lg,
    backgroundColor: "#ffffff",
    boxShadow: theme.shadow.card,
    borderWidth: 1,
    borderStyle: "solid",
    borderColor: theme.colors.borderSubtle,
  };

  const sectionTitleStyle: React.CSSProperties = {
    margin: 0,
    fontSize: 18,
    color: theme.colors.textOnLight,
  };

  const sectionSubTitleStyle: React.CSSProperties = {
    margin: "4px 0 0",
    fontSize: 13,
    color: theme.colors.textMutedOnLight,
  };

  return (
    <div style={containerStyle}>
      {/* 常规说明区域 */}
      <section style={sectionStyle}>
        <h2 style={sectionTitleStyle}>常规设置</h2>
        <p style={sectionSubTitleStyle}>
          这里会集中放一些全局通用的选项，比如启动行为、语言、主题偏好等。
          目前功能还比较少，后续会逐步补齐。
        </p>
      </section>

      {/* 外观 & 主题区域（多皮肤入口） */}
      <section style={sectionStyle}>
        <h2 style={sectionTitleStyle}>外观与主题</h2>
        <p style={sectionSubTitleStyle}>
          当前版本只内置了一套 Kivo 默认主题，后续会在这里切换
          Apple 风格 / 网易云风格 / QQ 音乐风格等多套皮肤。
        </p>

        <div
          style={{
            marginTop: 12,
            display: "flex",
            flexDirection: "column",
            gap: 8,
            fontSize: 13,
            color: theme.colors.textOnLight,
          }}
        >
          <div>
            <span style={{ fontWeight: 500 }}>当前主题：</span>
            <span>{readableThemeName}</span>
          </div>

          <label
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 4,
              maxWidth: 320,
            }}
          >
            <span style={{ fontSize: 12, color: theme.colors.textMutedOnLight }}>
              主题预设（预留多皮肤入口）
            </span>
            <select
              value={themeName}
              disabled
              style={{
                padding: "6px 8px",
                borderRadius: theme.radius.md,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: theme.colors.borderSubtle,
                backgroundColor: "#f9fafb",
                fontSize: 13,
                color: theme.colors.textOnLight,
                cursor: "not-allowed",
              }}
            >
              <option value={themeName}>
                Kivo 默认主题（当前） — 后续将支持切换多套皮肤
              </option>
            </select>
          </label>

          <p
            style={{
              margin: "4px 0 0",
              fontSize: 12,
              color: theme.colors.textMutedOnLight,
              lineHeight: 1.5,
            }}
          >
            说明：为了保证架构清晰，主题切换统一经过
            <code style={{ margin: "0 4px" }}>KivoThemeProvider</code>
            。当我们增加 “Apple Music 复刻皮肤”、“网易云暖色皮肤”
            等预设时，只需要在这里选择即可，无需改动业务代码。
          </p>
        </div>
      </section>
    </div>
  );
};
