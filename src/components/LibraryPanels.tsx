import { memo } from "react";
import { frequentMixes, recentAlbums } from "../data/library";
import { AlbumGrid } from "./AlbumGrid";
import { MixGrid } from "./MixGrid";

export const LibraryPanels = memo(function LibraryPanels() {
  return (
    <div className="column">
      <section className="card recent-card">
        <div className="card-head">
          <div><h2>{"\u6700\u8fd1\u6dfb\u52a0"}</h2><p>{"\u6309\u4e13\u8f91\u7ee7\u7eed\u6d4f\u89c8\u3002"}</p></div>
          <button className="subtle-link" type="button">{"\u8fd1 7 \u5929"}</button>
        </div>
        <AlbumGrid albums={recentAlbums} />
      </section>
      <section className="card mix-card">
        <div className="card-head">
          <div><h2>{"\u5e38\u542c\u6df7\u97f3"}</h2><p>{"\u4fdd\u7559\u4e00\u70b9\u5185\u5bb9\u611f\uff0c\u4f46\u4e0d\u62a2\u9996\u9875\u4e3b\u89d2\u3002"}</p></div>
          <button className="subtle-link" type="button">{"\u67e5\u770b\u5168\u90e8"}</button>
        </div>
        <MixGrid mixes={frequentMixes} />
      </section>
    </div>
  );
});
