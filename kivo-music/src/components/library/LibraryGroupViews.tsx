import React, { useMemo } from "react";
import {
  LibraryTrack,
  getTrackAlbum,
  getTrackArtist,
} from "../../library/libraryModel";

type Mode = "albums" | "artists";

interface Props {
  mode: Mode;
  tracks: LibraryTrack[];
  onPlayGroup: (groupTracks: LibraryTrack[]) => void;
}

interface AlbumGroup {
  key: string;
  album: string;
  artist: string;
  tracks: LibraryTrack[];
}

interface ArtistGroup {
  key: string;
  artist: string;
  tracks: LibraryTrack[];
}

export const LibraryGroupViews: React.FC<Props> = ({
  mode,
  tracks,
  onPlayGroup,
}) => {
  const albumGroups = useMemo<AlbumGroup[]>(() => {
    if (mode !== "albums") return [];
    const map = new Map<string, AlbumGroup>();

    for (const track of tracks) {
      const album = getTrackAlbum(track);
      const artist = getTrackArtist(track);
      const key = `${album}__${artist}`;

      const existing = map.get(key);
      if (existing) {
        existing.tracks.push(track);
      } else {
        map.set(key, { key, album, artist, tracks: [track] });
      }
    }

    return Array.from(map.values()).sort((a, b) =>
      a.album.localeCompare(b.album, "zh-CN"),
    );
  }, [tracks, mode]);

  const artistGroups = useMemo<ArtistGroup[]>(() => {
    if (mode !== "artists") return [];
    const map = new Map<string, ArtistGroup>();

    for (const track of tracks) {
      const artist = getTrackArtist(track);
      const key = artist || "未知艺人";
      const existing = map.get(key);
      if (existing) {
        existing.tracks.push(track);
      } else {
        map.set(key, { key, artist, tracks: [track] });
      }
    }

    return Array.from(map.values()).sort((a, b) =>
      a.artist.localeCompare(b.artist, "zh-CN"),
    );
  }, [tracks, mode]);

  if (mode === "albums") {
    if (albumGroups.length === 0) {
      return (
        <div
          style={{
            flex: 1,
            minHeight: 0,
            borderRadius: 16,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: "#e5e7eb",
            background: "#ffffff",
            padding: 16,
            fontSize: 13,
            color: "#9ca3af",
          }}
        >
          当前筛选条件下没有专辑。
        </div>
      );
    }

    return (
      <div
        style={{
          flex: 1,
          minHeight: 0,
          borderRadius: 16,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: "#e5e7eb",
          background: "#ffffff",
          padding: 16,
          display: "grid",
          gridTemplateColumns: "repeat(auto-fill,minmax(180px,1fr))",
          gap: 16,
          overflow: "auto",
        }}
      >
        {albumGroups.map((group) => {
          const trackCount = group.tracks.length;
          const favoriteCount = group.tracks.filter((t) => t.favorite).length;

          return (
            <div
              key={group.key}
              onDoubleClick={() => onPlayGroup(group.tracks)}
              style={{
                borderRadius: 14,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: "#e5e7eb",
                padding: 12,
                cursor: "default",
                background:
                  "radial-gradient(circle at 0% 0%,rgba(37,99,235,0.06),#ffffff)",
              }}
            >
              <div
                style={{
                  borderRadius: 10,
                  background:
                    "linear-gradient(135deg,#0f172a,#1e293b,#020617)",
                  height: 96,
                  marginBottom: 8,
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  color: "#e5e7eb",
                  fontWeight: 600,
                  fontSize: 20,
                }}
              >
                {group.album.charAt(0)}
              </div>
              <div
                style={{
                  fontSize: 14,
                  fontWeight: 600,
                  marginBottom: 2,
                }}
              >
                {group.album}
              </div>
              <div
                style={{
                  fontSize: 12,
                  color: "#6b7280",
                  marginBottom: 4,
                }}
              >
                {group.artist}
              </div>
              <div
                style={{
                  fontSize: 11,
                  color: "#9ca3af",
                }}
              >
                {trackCount} 首歌曲
                {favoriteCount > 0 ? ` · ${favoriteCount} 首已标记喜欢` : ""}
              </div>
            </div>
          );
        })}
      </div>
    );
  }

  // mode === "artists"
  if (artistGroups.length === 0) {
    return (
      <div
        style={{
          flex: 1,
          minHeight: 0,
          borderRadius: 16,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: "#e5e7eb",
          background: "#ffffff",
          padding: 16,
          fontSize: 13,
          color: "#9ca3af",
        }}
      >
        当前筛选条件下没有艺人。
      </div>
    );
  }

  return (
    <div
      style={{
        flex: 1,
        minHeight: 0,
        borderRadius: 16,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: "#e5e7eb",
        background: "#ffffff",
        padding: 16,
        overflow: "auto",
      }}
    >
      <table
        style={{
          width: "100%",
          borderCollapse: "collapse",
          fontSize: 13,
        }}
      >
        <thead>
          <tr
            style={{
              background: "#f9fafb",
              borderBottom: "1px solid #e5e7eb",
            }}
          >
            <th
              style={{
                textAlign: "left",
                padding: "8px 12px",
                fontWeight: 500,
                color: "#6b7280",
              }}
            >
              艺人
            </th>
            <th
              style={{
                textAlign: "left",
                padding: "8px 12px",
                fontWeight: 500,
                color: "#6b7280",
                width: "20%",
              }}
            >
              歌曲数量
            </th>
            <th
              style={{
                textAlign: "left",
                padding: "8px 12px",
                fontWeight: 500,
                color: "#6b7280",
                width: "25%",
              }}
            >
              专辑数量
            </th>
          </tr>
        </thead>
        <tbody>
          {artistGroups.map((group) => {
            const trackCount = group.tracks.length;
            const albumSet = new Set(
              group.tracks.map((t) => getTrackAlbum(t)),
            );

            return (
              <tr
                key={group.key}
                onDoubleClick={() => onPlayGroup(group.tracks)}
                style={{
                  borderBottom: "1px solid #f3f4f6",
                  cursor: "default",
                }}
              >
                <td style={{ padding: "6px 12px" }}>{group.artist}</td>
                <td
                  style={{
                    padding: "6px 12px",
                    color: "#4b5563",
                  }}
                >
                  {trackCount}
                </td>
                <td
                  style={{
                    padding: "6px 12px",
                    color: "#6b7280",
                  }}
                >
                  {albumSet.size}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};
