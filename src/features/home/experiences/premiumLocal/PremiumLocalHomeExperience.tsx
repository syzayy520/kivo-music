import { premiumLocalHomeCopy } from './premiumLocalHomeCopy'
import { premiumLocalHomeData } from './premiumLocalHomeData'
import { PremiumLocalFeatureCard } from './PremiumLocalFeatureCard'
import { PremiumLocalHomeHero } from './PremiumLocalHomeHero'
import { PremiumLocalHomeIcon } from './PremiumLocalHomeIcon'
import { PremiumLocalHomeShelf } from './PremiumLocalHomeShelf'
import { PremiumLocalPlayerSurface } from './PremiumLocalPlayerSurface'
import './premiumLocalHome.css'

export function PremiumLocalHomeExperience() {
  return (
    <section className="km-home-premium-shell" aria-label={premiumLocalHomeCopy.ariaLabel}>
      <header className="km-home-premium-header">
        <section className="km-home-premium-heading">
          <span>{premiumLocalHomeCopy.header.eyebrow}</span>
          <h2>{premiumLocalHomeCopy.header.title}</h2>
          <p>{premiumLocalHomeCopy.header.description}</p>
        </section>

        <section
          className="km-home-premium-header-actions"
          aria-label={premiumLocalHomeCopy.header.actionsAriaLabel}
        >
          <label className="km-home-premium-search">
            <PremiumLocalHomeIcon name="search" />
            <input
              aria-label={premiumLocalHomeCopy.header.searchAriaLabel}
              placeholder={premiumLocalHomeCopy.header.searchPlaceholder}
              readOnly
            />
          </label>
          <div
            className="km-home-premium-library-pill"
            aria-label={premiumLocalHomeCopy.header.statusAriaLabel}
          >
            <PremiumLocalHomeIcon name="library" />
            <span>{premiumLocalHomeCopy.header.statusLabel}</span>
            <strong>{premiumLocalHomeCopy.header.statusValue}</strong>
          </div>
        </section>
      </header>

      <section className="km-home-premium-spotlight" aria-label={premiumLocalHomeCopy.spotlightAriaLabel}>
        <PremiumLocalHomeHero />
        <section className="km-home-premium-feature-stack" aria-label={premiumLocalHomeCopy.featureStackAriaLabel}>
          {premiumLocalHomeData.features.map((feature) => (
            <PremiumLocalFeatureCard
              key={feature.id}
              eyebrow={feature.eyebrow}
              title={feature.title}
              description={feature.description}
              imageUrl={feature.imageUrl}
              iconName={feature.iconName}
              tone={feature.tone}
            />
          ))}
        </section>
      </section>

      <PremiumLocalHomeShelf
        items={premiumLocalHomeData.recentlyPlayed}
        title={premiumLocalHomeCopy.shelves.recentlyPlayed.title}
        eyebrow={premiumLocalHomeCopy.shelves.recentlyPlayed.eyebrow}
      />
      <PremiumLocalHomeShelf
        items={premiumLocalHomeData.recentlyAdded}
        title={premiumLocalHomeCopy.shelves.recentlyAdded.title}
        eyebrow={premiumLocalHomeCopy.shelves.recentlyAdded.eyebrow}
      />
      <PremiumLocalPlayerSurface />
    </section>
  )
}
