import type { Album } from '../../../shared/types/music'
import { resolveArtworkStyle } from '../../../shared/artwork/resolveArtworkClass'
import { listenNowCopy } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

type ShelfKey = keyof typeof listenNowData.shelves

type AlbumShelfProps = {
  shelfKey: ShelfKey
}

function artistName(album: Album) {
  return listenNowData.artists.find((artist) => artist.id === album.artistId)?.name ?? 'Unknown Artist'
}

function renderAlbumArtwork(album: Album, index: number) {
  const artworkSource = album as Album & {
    artworkUrl?: string
    coverUrl?: string
    coverPath?: string
    imageUrl?: string
  }
  return <div className="km-album-card-art" style={resolveArtworkStyle(artworkSource, index)} />
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
      <div className="km-album-grid" aria-label={shelfCopy.title}>
        {shelfAlbums.map((album, index) => (
          <article
            aria-label={`${album.title} by ${artistName(album)}`}
            className="km-album-card"
            draggable
            key={`${shelfKey}-${album.id}`}
          >
            {renderAlbumArtwork(album, index)}
            <strong>{album.title}</strong>
            <span>{artistName(album)}</span>
            <em>{album.qualityLabel}</em>
          </article>
        ))}
      </div>
    </section>
  )
}
