// src/components/playlists/PlaylistTableHeader.tsx
import React from "react";
import { useI18n } from "../../i18n";

const PlaylistTableHeader: React.FC = () => {
  const { t } = useI18n();

  return (
    <thead>
      <tr
        style={{
          textAlign: "left",
          userSelect: "none",
          color: "#6b7280",
        }}
      >
        <th style={{ width: 40, padding: "8px 12px" }}>#</th>
        <th style={{ width: "28%", padding: "8px 12px" }}>
          {t("playlists.table.header.title")}
        </th>
        <th style={{ width: "20%", padding: "8px 12px" }}>
          {t("playlists.table.header.artist")}
        </th>
        <th style={{ width: "22%", padding: "8px 12px" }}>
          {t("playlists.table.header.album")}
        </th>
        <th style={{ width: 80, padding: "8px 12px" }}>
          {t("playlists.table.header.playCount")}
        </th>
        <th style={{ width: 180, padding: "8px 12px" }}>
          {t("playlists.table.header.lastPlayed")}
        </th>
        <th style={{ width: 60, padding: "8px 12px" }}>
          {t("playlists.table.header.favorite")}
        </th>
        <th style={{ width: 140, padding: "8px 12px" }}>
          {t("playlists.table.header.actions")}
        </th>
      </tr>
    </thead>
  );
};

export default PlaylistTableHeader;
