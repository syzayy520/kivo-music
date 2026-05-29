import { albums } from '../../../data/mock/albums'
import { artists } from '../../../data/mock/artists'
import { listenNowCopy } from '../listenNowCopy'

function artistName(artistId: string) {
  return artists.find((artist) => artist.id === artistId)?.name ?? 'Unknown Artist'
}

export function AlbumShelf() {
  return (
    <section className="km-shelf" id="albums">
      <div className="km-section-head">
        <div>
          <p>{listenNowCopy.heavyRotation}</p>
          <h3>Weighted by ratings, plays, recency, and recommendation controls.</h3>
        </div>
        <button type="button">Show All</button>
      </div>
      <div className="km-album-grid">
        {albums.map((album) => (
          <article className="km-card" key={album.id}>
            <div className={album.artworkClass} />
            <strong>{album.title}</strong>
            <span>{artistName(album.artistId)}</span>
            <em>{album.qualityLabel}</em>
          </article>
        ))}
      </div>
    </section>
  )
}
