import type { Album } from '../../../shared/types/music'
import { listenNowCopy } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

function artistName(album: Album) {
  return listenNowData.artists.find((artist) => artist.id === album.artistId)?.name ?? 'Unknown Artist'
}

export function AlbumShelf() {
  return (
    <section className="km-album-shelf" id="albums">
      <div className="km-shelf-head">
        <div>
          <p>{listenNowCopy.shelfEyebrow}</p>
          <h3>{listenNowCopy.shelfTitle}</h3>
        </div>
        <button type="button">Show All</button>
      </div>
      <div className="km-album-grid">
        {listenNowData.shelfAlbums.map((album) => (
          <article className="km-album-card" key={album.id}>
            <div className={`km-album-card-art artwork-${album.artworkKey}`} />
            <strong>{album.title}</strong>
            <span>{artistName(album)}</span>
            <em>{album.qualityLabel}</em>
          </article>
        ))}
      </div>
    </section>
  )
}
