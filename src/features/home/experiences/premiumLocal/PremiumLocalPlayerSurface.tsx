import { premiumLocalHomeCopy } from './premiumLocalHomeCopy'
import { premiumLocalHomeData } from './premiumLocalHomeData'
import { PremiumLocalHomeIcon } from './PremiumLocalHomeIcon'

export function PremiumLocalPlayerSurface() {
  const player = premiumLocalHomeData.player

  return (
    <section className="km-home-premium-player-surface" aria-label={premiumLocalHomeCopy.player.ariaLabel}>
      <section className="km-home-premium-player-track">
        <div
          className="km-home-premium-player-art"
          style={{ backgroundImage: `url(${player.imageUrl})` }}
          aria-hidden="true"
        />
        <div className="km-home-premium-player-copy">
          <strong>{player.title}</strong>
          <span>{player.artist}</span>
        </div>
        <button type="button" aria-label={premiumLocalHomeCopy.player.favoriteAriaLabel}>
          <PremiumLocalHomeIcon name="sparkle" />
        </button>
      </section>

      <section className="km-home-premium-player-core" aria-label={premiumLocalHomeCopy.player.controlsAriaLabel}>
        <div className="km-home-premium-player-buttons">
          <button type="button" aria-label={premiumLocalHomeCopy.player.previousAriaLabel}>
            <PremiumLocalHomeIcon name="previous" />
          </button>
          <button className="km-home-premium-player-play" type="button" aria-label={premiumLocalHomeCopy.player.pauseAriaLabel}>
            <PremiumLocalHomeIcon name="pause" />
          </button>
          <button type="button" aria-label={premiumLocalHomeCopy.player.nextAriaLabel}>
            <PremiumLocalHomeIcon name="next" />
          </button>
        </div>
        <div className="km-home-premium-player-timeline" aria-label={premiumLocalHomeCopy.player.progressAriaLabel}>
          <span>{player.currentTime}</span>
          <div><i style={{ width: `${player.progressPercent}%` }} /></div>
          <span>{player.duration}</span>
        </div>
      </section>

      <section className="km-home-premium-player-meta" aria-label={premiumLocalHomeCopy.player.metaAriaLabel}>
        <span>{player.quality}</span>
        <div className="km-home-premium-player-volume" aria-label={premiumLocalHomeCopy.player.volumeAriaLabel}>
          <PremiumLocalHomeIcon name="volume" />
          <div><i style={{ width: `${player.volumePercent}%` }} /></div>
        </div>
        <button type="button" aria-label={premiumLocalHomeCopy.player.queueAriaLabel}>
          <PremiumLocalHomeIcon name="queue" />
        </button>
      </section>
    </section>
  )
}
