// src/components/settings/CoverCacheDebugSection.tsx
import React from "react";
import { useI18n } from "../../i18n";

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

export const CoverCacheDebugSection: React.FC = () => {
  const { t } = useI18n();

  return (
    <Section title={t("settings.cache.debug.title")}>
      <ul style={{ margin: 0, paddingLeft: 18, fontSize: 12, opacity: 0.8 }}>
        <li>
          {t("settings.cache.debug.item1.prefix")}
          <code>AppData/com.administrator.kivo-music</code>
          {t("settings.cache.debug.item1.suffix")}
        </li>
        <li>
          <code>covers.json</code>
          {t("settings.cache.debug.item2")}
        </li>
        <li>
          <code>folder-covers.json</code>
          {t("settings.cache.debug.item3.prefix")}
          <code>cover.jpg</code>
          {t("settings.cache.debug.item3.suffix")}
        </li>
        <li>
          {t("settings.cache.debug.item4.prefix")}
          <code>resolveCoverPathForTrack(track)</code>
          {t("settings.cache.debug.item4.suffix")}
        </li>
      </ul>
    </Section>
  );
};
