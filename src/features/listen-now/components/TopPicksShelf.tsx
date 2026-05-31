import type { Album } from '../../../shared/types/music'
import { resolveArtworkStyle } from '../../../shared/artwork/resolveArtworkClass'
import { t } from '../../../shared/i18n'
import { listenNowCopy } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

function topPickCopy(index: number) {
  return listenNowCopy.topPicks.items[index] ?? listenNowCopy.topPicks.items[0]
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
          <p>{t(listenNowCopy.topPicks.eyebrowKey)}</p>
          <h3>{t(listenNowCopy.topPicks.titleKey)}</h3>
        </div>
      </div>
      <div className="km-top-picks-grid" aria-label={t(listenNowCopy.topPicks.titleKey)}>
        {listenNowData.topPicks.map((album, index) => {
          const copy = topPickCopy(index)

          return (
            <article className="km-top-pick-card" data-featured={index === 0} key={`top-pick-${album.id}`}>
              {renderTopPickArtwork(album, index)}
              <div className="km-top-pick-copy">
                <p>{t(copy.eyebrowKey)}</p>
                <h2>{album.title}</h2>
                <span>{t(copy.descriptionKey)}</span>
              </div>
              <button
                className="km-top-pick-play"
                type="button"
                aria-label={t('home.topPick.playAriaLabel', { album: album.title })}
              >
                ▶
              </button>
            </article>
          )
        })}
      </div>
    </section>
  )
}
