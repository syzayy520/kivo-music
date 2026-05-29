import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'
import { listenNowCopy } from './listenNowCopy'
import './listenNow.css'

function artistName(artistId: string) {
  return artists.find((artist) => artist.id === artistId)?.name ?? 'Unknown Artist'
}

export function ListenNowPage() {
  const heroAlbum = albums[0]
  const shelfAlbums = albums.slice(1)

  return (
    <main className="km-listen-now">
      <section className="km-hero-card">
        <div className="km-hero-art" style={{ background: heroAlbum.artwork }} />
        <div className="km-hero-copy">
          <p>{listenNowCopy.heroEyebrow}</p>
          <h2>{listenNowCopy.heroTitle}</h2>
          <span>{artistName(heroAlbum.artistId)} · {heroAlbum.qualityLabel}</span>
          <strong>{listenNowCopy.heroDescription}</strong>
          <div className="km-hero-actions">
            <button type="button">Play</button>
            <button type="button" className="secondary">View Album</button>
          </div>
        </div>
      </section>

      <section className="km-shelf">
        <div className="km-section-heading">
          <h3>{listenNowCopy.heavyRotation}</h3>
          <p>Weighted by plays, ratings, recency, and recommendation controls.</p>
        </div>
        <div className="km-album-row">
          {albums.map((album) => (
            <article className="km-album-card" key={album.id}>
              <div className="km-album-art" style={{ background: album.artwork }} />
              <strong>{album.title}</strong>
              <span>{artistName(album.artistId)}</span>
              <em>{album.qualityLabel}</em>
            </article>
          ))}
        </div>
      </section>

      <section className="km-shelf two-column">
        <div>
          <h3>{listenNowCopy.rediscover}</h3>
          <p>Albums you collected but have not heard recently.</p>
        </div>
        <div className="km-rediscover-list">
          {shelfAlbums.map((album) => (
            <article key={album.id}>
              <div style={{ background: album.artwork }} />
              <span>{album.title}</span>
              <small>{album.year} · {album.genre}</small>
            </article>
          ))}
        </div>
      </section>
    </main>
  )
}
