import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'
import { tracks } from '../../data/mock/tracks'

export function AlbumDetail() {
  const album = albums[0]
  const artist = artists.find((item) => item.id === album.artistId)
  const albumTracks = tracks.filter((track) => track.albumId === album.id)

  return (
    <>
      <section className="km-album-detail">
        <div className={`km-album-art ${album.artworkClass}`} />
        <div className="km-album-copy">
          <p>Collected Album</p>
          <h2>{album.title}</h2>
          <span>{artist?.name} · {album.year} · {album.trackCount} songs · 42 min</span>
          <div className="km-badges">
            <em>Lossless</em>
            <em>{album.format}</em>
            <em>{album.qualityLabel}</em>
            <em>Album rating {album.rating}/5</em>
          </div>
          <div className="km-actions">
            <button type="button">Play Album</button>
            <button type="button" className="secondary">Shuffle</button>
            <button type="button" className="secondary">Favorite Album</button>
          </div>
        </div>
      </section>
      <section className="km-track-table" aria-label="Album tracks">
        {albumTracks.map((track, index) => (
          <article className={index === 0 ? 'playing' : ''} key={track.id}>
            <span>{track.trackNumber}</span>
            <strong>{track.title}</strong>
            <em>{track.liked ? 'Liked track' : 'Album-only track'}</em>
            <small>{track.qualityLabel}</small>
          </article>
        ))}
      </section>
    </>
  )
}
