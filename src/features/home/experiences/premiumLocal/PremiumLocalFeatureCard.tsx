import { PremiumLocalHomeIcon, type PremiumLocalHomeIconName } from './PremiumLocalHomeIcon'

type PremiumLocalFeatureCardProps = {
  eyebrow: string
  title: string
  description: string
  imageUrl: string
  iconName: PremiumLocalHomeIconName
  tone: 'dark' | 'soft'
}

export function PremiumLocalFeatureCard({
  eyebrow,
  title,
  description,
  imageUrl,
  iconName,
  tone,
}: PremiumLocalFeatureCardProps) {
  return (
    <article
      className={`km-home-premium-feature-card km-home-premium-feature-card-${tone}`}
      style={{ backgroundImage: `url(${imageUrl})` }}
    >
      <div className="km-home-premium-feature-shade" aria-hidden="true" />
      <section className="km-home-premium-feature-copy">
        <span>{eyebrow}</span>
        <h3>{title}</h3>
        <p>{description}</p>
      </section>
      <button className="km-home-premium-feature-action" type="button" aria-label={title}>
        <PremiumLocalHomeIcon name={iconName} />
      </button>
    </article>
  )
}
