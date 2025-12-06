// src/components/settings/CoverCacheDirectorySection.tsx
import React from "react";
import { useI18n } from "../../i18n";

interface Props {
  busy: boolean;
  effectiveDir: string;
  onChooseDir: () => void | Promise<void>;
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

export const CoverCacheDirectorySection: React.FC<Props> = ({
  busy,
  effectiveDir,
  onChooseDir,
}) => {
  const { t } = useI18n();
  const defaultPath = "/com.administrator.kivo-music/covers";

  return (
    <Section title={t("settings.cache.directory.title")}>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 10,
        }}
      >
        <div>
          <div style={{ opacity: 0.8, fontSize: 13, marginBottom: 4 }}>
            {t("settings.cache.directory.currentLabel")}
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
            {effectiveDir || t("settings.cache.directory.loading")}
          </code>
        </div>

        <button
          type="button"
          disabled={busy}
          onClick={() => {
            void onChooseDir();
          }}
          style={{
            ...buttonBase,
            marginTop: 8,
            alignSelf: "flex-start",
            cursor: busy ? "default" : "pointer",
            backgroundColor: busy ? "#4b4f58" : "#4a4f5c",
            color: "#fff",
          }}
        >
          {t("settings.cache.directory.chooseButton")}
        </button>

        <div style={{ opacity: 0.7, fontSize: 12 }}>
          <span>{t("settings.cache.directory.help")}</span>
          <code style={{ marginLeft: 4 }}>{defaultPath}</code>
        </div>
      </div>
    </Section>
  );
};
