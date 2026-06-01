import { PremiumLocalFeatureCard } from './PremiumLocalFeatureCard'
import { PremiumLocalHomeHero } from './PremiumLocalHomeHero'
import { PremiumLocalHomeShelf } from './PremiumLocalHomeShelf'
import { PremiumLocalPlayerSurface } from './PremiumLocalPlayerSurface'
import {
  premiumLocalFeatures,
  premiumLocalShelves,
} from './premiumLocalHomeData'
import './premiumLocalHome.css'

export function PremiumLocalHomeExperience() {
  return (
    <div className="premium-local-home">
      <PremiumLocalHomeHero />
      <PremiumLocalPlayerSurface />
      <section className="premium-local-home__features" aria-label="Premium local features">
        {premiumLocalFeatures.map((feature) => (
          <PremiumLocalFeatureCard
            key={feature.title}
            title={feature.title}
            description={feature.description}
          />
        ))}
      </section>
      <PremiumLocalHomeShelf title="本地精选" items={premiumLocalShelves} />
    </div>
  )
}

