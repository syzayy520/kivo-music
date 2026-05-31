import type { Album } from '../../../shared/types/music'
import { resolveArtworkStyle } from '../../../shared/artwork/resolveArtworkClass'
import { t } from '../../../shared/i18n'
import { listenNowCopy, type ListenNowShelfKey } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

type AlbumShelfProps = {
  shelfKey: ListenNowShelfKey
}

function artistName(album: Album) {
  return listenNowData.artists.find((artist) => artist.id === album.artistId)?.name ?? t('common.unknownArtist')
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
  const shelfTitle = t(shelfCopy.titleKey)

  return (
    <section className="km-album-shelf" id={shelfKey}>
      <div className="km-shelf-head">
        <div>
          <p>{t(shelfCopy.eyebrowKey)}</p>
          <h3>{shelfTitle}</h3>
        </div>
        <button type="button">{t(listenNowCopy.showAllKey)}</button>
      </div>
      <div className="km-album-grid" aria-label={shelfTitle}>
        {shelfAlbums.map((album, index) => {
          const artist = artistName(album)

          return (
            <article
              aria-label={t('home.albumCard.ariaLabel', { album: album.title, artist })}
              className="km-album-card"
              draggable
              key={`${shelfKey}-${album.id}`}
            >
              {renderAlbumArtwork(album, index)}
              <strong>{album.title}</strong>
              <span>{artist}</span>
              <em>{album.qualityLabel}</em>
            </article>
          )
        })}
      </div>
    </section>
  )
}
