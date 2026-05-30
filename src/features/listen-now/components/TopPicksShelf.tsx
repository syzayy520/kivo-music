import type { Album } from '../../../shared/types/music'
import { resolveArtworkStyle } from '../../../shared/artwork/resolveArtworkClass'
import { listenNowCopy } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

function topPickEyebrow(index: number) {
  return listenNowCopy.topPickEyebrows[index] ?? listenNowCopy.topPickEyebrows[0]
}

function topPickDescription(index: number) {
  return listenNowCopy.topPickDescriptions[index] ?? listenNowCopy.topPickDescriptions[0]
}

function renderTopPickArtwork(album: Album, index: number) {
  const artworkSource = album as Album & {
    artworkUrl?: string
    coverUrl?: string
    coverPath?: string
    imageUrl?: string
  }
  return <div className="km-top-pick-art" style={resolveArtworkStyle(artworkSource, index)} />
}

export function TopPicksShelf() {
  return (
    <section className="km-top-picks" id="listen-now">
      <div className="km-shelf-head km-top-picks-head">
        <div>
          <p>{listenNowCopy.heroEyebrow}</p>
          <h3>{listenNowCopy.topPicksTitle}</h3>
        </div>
      </div>
      <div className="km-top-picks-grid" aria-label={listenNowCopy.topPicksTitle}>
        {listenNowData.topPicks.map((album, index) => (
          <article className="km-top-pick-card" data-featured={index === 0} key={`top-pick-${album.id}`}>
            {renderTopPickArtwork(album, index)}
            <div className="km-top-pick-copy">
              <p>{topPickEyebrow(index)}</p>
              <h2>{album.title}</h2>
              <span>{topPickDescription(index)}</span>
            </div>
            <button className="km-top-pick-play" type="button" aria-label={`Play ${album.title}`}>
              ▶
            </button>
          </article>
        ))}
      </div>
    </section>
  )
}
