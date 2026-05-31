import type { CSSProperties } from 'react'
import { listenNowTokens } from './listenNowTokens'

type ListenNowStyle = CSSProperties & Record<`--${string}`, string>

export const listenNowStyle: ListenNowStyle = {
  '--km-listen-section-bottom': `${listenNowTokens.sectionBottomSpace}px`,
  '--km-listen-head-bottom': `${listenNowTokens.headBottomSpace}px`,
  '--km-listen-top-picks-gap': `${listenNowTokens.topPicksGap}px`,
  '--km-listen-top-pick-radius': `${listenNowTokens.topPickRadius}px`,
  '--km-listen-top-pick-copy-top': `${listenNowTokens.topPickCopyTop}px`,
  '--km-listen-top-pick-copy-inline': `${listenNowTokens.topPickCopyInline}px`,
  '--km-listen-top-pick-copy-bottom': `${listenNowTokens.topPickCopyBottom}px`,
  '--km-listen-top-pick-play-right': `${listenNowTokens.topPickPlayRight}px`,
  '--km-listen-top-pick-play-bottom': `${listenNowTokens.topPickPlayBottom}px`,
  '--km-listen-top-pick-play-size': `${listenNowTokens.topPickPlaySize}px`,
  '--km-listen-shelf-head-gap': `${listenNowTokens.shelfHeadGap}px`,
  '--km-listen-shelf-head-bottom': `${listenNowTokens.shelfHeadBottomSpace}px`,
  '--km-listen-shelf-button-y': `${listenNowTokens.shelfButtonPaddingY}px`,
  '--km-listen-shelf-button-x': `${listenNowTokens.shelfButtonPaddingX}px`,
  '--km-listen-album-grid-gap': `${listenNowTokens.albumGridGap}px`,
  '--km-listen-album-shelf-last-bottom': `${listenNowTokens.albumShelfLastBottomSpace}px`,
  '--km-listen-album-card-radius': `${listenNowTokens.albumCardRadius}px`,
  '--km-listen-album-art-radius': `${listenNowTokens.albumArtRadius}px`,
  '--km-listen-album-art-bottom': `${listenNowTokens.albumArtBottomSpace}px`,
}
