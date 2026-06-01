import { premiumLocalHomeCopy } from './premiumLocalHomeCopy'

export function PremiumLocalHomeHero() {
  return (
    <header className="premium-local-home__hero">
      <p>{premiumLocalHomeCopy.eyebrow}</p>
      <h1>{premiumLocalHomeCopy.title}</h1>
      <p>{premiumLocalHomeCopy.subtitle}</p>
      <div className="premium-local-home__actions">
        <button type="button">{premiumLocalHomeCopy.ctaPrimary}</button>
        <button type="button">{premiumLocalHomeCopy.ctaSecondary}</button>
      </div>
    </header>
  )
}

