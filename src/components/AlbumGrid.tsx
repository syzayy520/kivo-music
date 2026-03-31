import { memo, type CSSProperties } from "react";
import type { AlbumCard } from "../data/library";

export interface AlbumGridProps {
  albums: AlbumCard[];
}

export const AlbumGrid = memo(function AlbumGrid({ albums }: AlbumGridProps) {
  return (
    <div className="albums">
      {albums.map((album, index) => (
        <article className={index === 0 ? "album featured" : "album"} key={album.title}>
          <div className="art" style={album.tone ? ({ "--art-tone": album.tone } as CSSProperties) : undefined} />
          <span className="title-ellipsis">{album.title}</span>
          <span className="sub-ellipsis">{album.artist}</span>
        </article>
      ))}
    </div>
  );
});
