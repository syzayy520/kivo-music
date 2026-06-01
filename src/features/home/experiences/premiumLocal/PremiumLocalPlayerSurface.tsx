import { premiumLocalHomeCopy } from './premiumLocalHomeCopy'

export function PremiumLocalPlayerSurface() {
  return (
    <section className="premium-local-home__player-surface" aria-label="Premium local player surface">
      <h2>{premiumLocalHomeCopy.nowPlayingTitle}</h2>
      <p>{premiumLocalHomeCopy.nowPlayingSubtitle}</p>
      <dl>
        <div>
          <dt>Requested</dt>
          <dd>256f</dd>
        </div>
        <div>
          <dt>Submitted</dt>
          <dd>256f</dd>
        </div>
        <div>
          <dt>Deferred</dt>
          <dd>0f</dd>
        </div>
      </dl>
    </section>
  )
}

