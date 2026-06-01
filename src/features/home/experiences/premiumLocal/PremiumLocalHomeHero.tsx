import { premiumLocalHomeCopy } from './premiumLocalHomeCopy'
import { premiumLocalHomeData } from './premiumLocalHomeData'
import { PremiumLocalHomeIcon } from './PremiumLocalHomeIcon'

export function PremiumLocalHomeHero() {
  const hero = premiumLocalHomeData.hero

  return (
    <article
      className="km-home-premium-hero"
      aria-label={premiumLocalHomeCopy.heroAriaLabel}
      style={{ backgroundImage: `url(${hero.imageUrl})` }}
    >
      <div className="km-home-premium-hero-overlay" aria-hidden="true" />
      <section className="km-home-premium-hero-copy">
        <span>{hero.eyebrow}</span>
        <h3>{hero.title}</h3>
        <p>{hero.artist}</p>
        <strong>{hero.description}</strong>
        <div className="km-home-premium-hero-progress" aria-label={hero.progressLabel}>
          <i style={{ width: `${hero.progressPercent}%` }} />
        </div>
        <em>{hero.progressLabel}</em>
        <div className="km-home-premium-hero-actions">
          <button className="km-home-premium-primary-button" type="button">
            <PremiumLocalHomeIcon name="play" />
            {hero.primaryActionLabel}
          </button>
          <button className="km-home-premium-secondary-button" type="button">
            <PremiumLocalHomeIcon name="queue" />
            {hero.secondaryActionLabel}
          </button>
          <button className="km-home-premium-icon-button" type="button" aria-label={premiumLocalHomeCopy.moreAriaLabel}>
            <PremiumLocalHomeIcon name="more" />
          </button>
        </div>
      </section>
    </article>
  )
}
