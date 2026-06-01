import type { PremiumLocalAlbumCard } from './premiumLocalHomeData'
import { premiumLocalHomeCopy } from './premiumLocalHomeCopy'
import { PremiumLocalHomeIcon } from './PremiumLocalHomeIcon'

type PremiumLocalHomeShelfProps = {
  eyebrow: string
  title: string
  items: PremiumLocalAlbumCard[]
}

export function PremiumLocalHomeShelf({ eyebrow, title, items }: PremiumLocalHomeShelfProps) {
  return (
    <section className="km-home-premium-shelf" aria-label={title}>
      <header className="km-home-premium-shelf-head">
        <div>
          <span>{eyebrow}</span>
          <h3>{title}</h3>
        </div>
        <button type="button">{premiumLocalHomeCopy.showAllLabel}</button>
      </header>
      <div className="km-home-premium-album-grid">
        {items.map((item) => (
          <article className="km-home-premium-album-card" key={item.id}>
            <div
              className="km-home-premium-album-art"
              style={{ backgroundImage: `url(${item.imageUrl})` }}
              aria-hidden="true"
            >
              <button type="button" aria-label={premiumLocalHomeCopy.playAlbumAriaLabel.replace('{album}', item.title)}>
                <PremiumLocalHomeIcon name="play" />
              </button>
            </div>
            <strong>{item.title}</strong>
            <span>{item.artist}</span>
            <p>{item.meta}</p>
          </article>
        ))}
      </div>
    </section>
  )
}
