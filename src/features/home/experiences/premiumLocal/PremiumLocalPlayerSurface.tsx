import { premiumLocalHomeCopy } from './premiumLocalHomeCopy'
import { premiumLocalHomeData } from './premiumLocalHomeData'
import { PremiumLocalHomeIcon } from './PremiumLocalHomeIcon'

export function PremiumLocalPlayerSurface() {
  const player = premiumLocalHomeData.player

  return (
    <section className="km-home-premium-player-surface" aria-label={premiumLocalHomeCopy.player.ariaLabel}>
      <div className="km-home-premium-player-kicker">
        <span>{premiumLocalHomeCopy.player.eyebrow}</span>
        <small>{player.statusLabel}</small>
      </div>

      <section className="km-home-premium-player-preview">
        <div
          className="km-home-premium-player-art"
          style={{ backgroundImage: `url(${player.imageUrl})` }}
          aria-hidden="true"
        />
        <div className="km-home-premium-player-copy">
          <strong>{player.title}</strong>
          <span>{player.artist}</span>
          <p>{player.resumeLabel}</p>
        </div>
        <button className="km-home-premium-player-play" type="button" aria-label={premiumLocalHomeCopy.player.playAriaLabel}>
          <PremiumLocalHomeIcon name="play" />
        </button>
      </section>

      <div className="km-home-premium-player-timeline" aria-label={premiumLocalHomeCopy.player.progressAriaLabel}>
        <span>{player.currentTime}</span>
        <div><i style={{ width: `${player.progressPercent}%` }} /></div>
        <span>{player.duration}</span>
      </div>

      <section className="km-home-premium-player-meta" aria-label={premiumLocalHomeCopy.player.metaAriaLabel}>
        <span>{player.quality}</span>
        <span>{player.queueSummary}</span>
        <button className="km-home-premium-player-queue-action" type="button" aria-label={premiumLocalHomeCopy.player.queueAriaLabel}>
          <PremiumLocalHomeIcon name="queue" />
        </button>
      </section>
    </section>
  )
}
