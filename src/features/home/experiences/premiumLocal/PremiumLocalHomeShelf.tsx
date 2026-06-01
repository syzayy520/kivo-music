import type { PremiumLocalShelfItem } from './premiumLocalHomeData'

type PremiumLocalHomeShelfProps = {
  title: string
  items: PremiumLocalShelfItem[]
}

export function PremiumLocalHomeShelf({
  title,
  items,
}: PremiumLocalHomeShelfProps) {
  return (
    <section className="premium-local-home__shelf" aria-label={title}>
      <header>
        <h2>{title}</h2>
      </header>
      <ul>
        {items.map((item) => (
          <li key={item.id}>
            <strong>{item.title}</strong>
            <span>{item.subtitle}</span>
            <small>{item.meta}</small>
          </li>
        ))}
      </ul>
    </section>
  )
}

