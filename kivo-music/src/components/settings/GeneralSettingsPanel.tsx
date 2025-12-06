// src/components/settings/GeneralSettingsPanel.tsx
import React from "react";
import { useKivoTheme } from "../../styles/ThemeContext";
import { useI18n, type SupportedLocale } from "../../i18n";

export const GeneralSettingsPanel: React.FC = () => {
  const { theme, themeName } = useKivoTheme();
  const { locale, setLocale, t } = useI18n();

  // 主题当前值展示：默认主题用 i18n key，其它主题直接展示 themeName
  const readableThemeName =
    themeName === "kivoDefault"
      ? t("settings.general.theme.currentValue.default")
      : themeName;

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
    fontSize: 16,
    fontWeight: 600,
    color: theme.colors.textOnLight,
  };

  const sectionSubtitleStyle: React.CSSProperties = {
    margin: "4px 0 0 0",
    fontSize: 13,
    color: theme.colors.textMutedOnLight,
  };

  const kvRowStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    gap: 16,
    marginTop: 16,
  };

  const badgeStyle: React.CSSProperties = {
  padding: "2px 8px",
  borderRadius: theme.radius.pill,
  border: `1px solid ${theme.colors.borderSubtle}`,
  fontSize: 11,
  color: theme.colors.textMutedOnLight,
  backgroundColor: theme.colors.tagBg,
  whiteSpace: "nowrap",
};


  const buttonRowStyle: React.CSSProperties = {
    display: "flex",
    flexWrap: "wrap",
    gap: 8,
    marginTop: 8,
  };

  const langButtonBase: React.CSSProperties = {
    padding: "6px 10px",
    borderRadius: theme.radius.pill,
    borderWidth: 1,
    borderStyle: "solid",
    fontSize: 13,
    cursor: "pointer",
    backgroundColor: "transparent",
  };

  const renderLocaleLabel = (value: SupportedLocale): string => {
    switch (value) {
      case "zh-CN":
        return t("settings.general.language.zhCN");
      case "en-US":
      default:
        return t("settings.general.language.enUS");
    }
  };

  const handleLocaleChange = (next: SupportedLocale) => {
    if (next !== locale) {
      setLocale(next);
    }
  };

  return (
    <div style={containerStyle}>
      {/* 主题信息 */}
      <section style={sectionStyle}>
        <h2 style={sectionTitleStyle}>
          {t("settings.general.theme.title")}
        </h2>
        <p style={sectionSubtitleStyle}>
          {t("settings.general.theme.subtitle")}
        </p>

        <div style={kvRowStyle}>
          <div>
            <div
              style={{
                fontSize: 13,
                color: theme.colors.textOnLight,
              }}
            >
              <span style={{ fontWeight: 500 }}>
                {t("settings.general.theme.currentLabel")}
              </span>
              <span>{readableThemeName}</span>
            </div>
            <p
              style={{
                margin: "8px 0 0 0",
                fontSize: 12,
                color: theme.colors.textMutedOnLight,
                lineHeight: 1.5,
              }}
            >
              {t("settings.general.theme.description")}
            </p>
          </div>
          <span style={badgeStyle}>
            {t("settings.general.theme.currentValue.default")}
          </span>
        </div>
      </section>

      {/* 语言设置 */}
      <section style={sectionStyle}>
        <h2 style={sectionTitleStyle}>
          {t("settings.general.language.title")}
        </h2>
        <p style={sectionSubtitleStyle}>
          {t("settings.general.language.subtitle")}
        </p>

        <div style={{ marginTop: 12 }}>
          <div
            style={{
              fontSize: 13,
              color: theme.colors.textOnLight,
            }}
          >
            <span style={{ fontWeight: 500 }}>
              {t("settings.general.language.currentLabel")}
            </span>
            <span>{renderLocaleLabel(locale)}</span>
          </div>

          <div style={buttonRowStyle}>
            {(["zh-CN", "en-US"] as SupportedLocale[]).map((value) => {
              const isActive = locale === value;
              return (
                <button
                  key={value}
                  type="button"
                  onClick={() => handleLocaleChange(value)}
                  style={{
                    ...langButtonBase,
                    borderColor: isActive
                      ? theme.colors.primary
                      : theme.colors.borderSubtle,
                    color: isActive
                      ? theme.colors.primary
                      : theme.colors.textOnLight,
                    backgroundColor: isActive
                      ? "rgba(37, 99, 235, 0.06)"
                      : "transparent",
                  }}
                >
                  {renderLocaleLabel(value)}
                </button>
              );
            })}
          </div>

          <p
            style={{
              margin: "10px 0 0 0",
              fontSize: 12,
              color: theme.colors.textMutedOnLight,
              lineHeight: 1.5,
            }}
          >
            {t("settings.general.language.note")}
          </p>
        </div>
      </section>
    </div>
  );
};
