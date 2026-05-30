import type { Album } from '../../../shared/types/music'
import { listenNowCopy } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

type ShelfKey = keyof typeof listenNowData.shelves

type AlbumShelfProps = {
  shelfKey: ShelfKey
}

function artistName(album: Album) {
  return listenNowData.artists.find((artist) => artist.id === album.artistId)?.name ?? 'Unknown Artist'
}

export function AlbumShelf({ shelfKey }: AlbumShelfProps) {
  const shelfCopy = listenNowCopy.shelves[shelfKey]
  const shelfAlbums = listenNowData.shelves[shelfKey]

  return (
    <section className="km-album-shelf" id={shelfKey}>
      <div className="km-shelf-head">
        <div>
          <p>{shelfCopy.eyebrow}</p>
          <h3>{shelfCopy.title}</h3>
        </div>
        <button type="button">{listenNowCopy.showAll}</button>
      </div>
      <div className="km-album-grid">
        {shelfAlbums.map((album) => (
          <article className="km-album-card" key={`${shelfKey}-${album.id}`}>
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
