import { PremiumLocalHomeIcon } from './PremiumLocalHomeIcon'

type PremiumLocalFeatureCardProps = {
  title: string
  description: string
}

export function PremiumLocalFeatureCard({
  title,
  description,
}: PremiumLocalFeatureCardProps) {
  return (
    <article className="premium-local-home__feature-card">
      <PremiumLocalHomeIcon label="◎" />
      <h3>{title}</h3>
      <p>{description}</p>
    </article>
  )
}

