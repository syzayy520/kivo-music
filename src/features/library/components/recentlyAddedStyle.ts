import type { CSSProperties } from 'react'
import { recentlyAddedTokens } from './recentlyAddedTokens'

type RecentlyAddedStyle = CSSProperties & Record<`--${string}`, string>

export const recentlyAddedStyle: RecentlyAddedStyle = {
  '--km-recently-added-card-min': `${recentlyAddedTokens.cardMinWidth}px`,
  '--km-recently-added-card-preferred': recentlyAddedTokens.cardPreferredWidth,
  '--km-recently-added-card-max': `${recentlyAddedTokens.cardMaxWidth}px`,
  '--km-recently-added-gap': `${recentlyAddedTokens.gap}px`,
  '--km-recently-added-bottom-padding': `${recentlyAddedTokens.bottomPadding}px`,
  '--km-recently-added-art-radius': `${recentlyAddedTokens.artRadius}px`,
  '--km-recently-added-art-bottom': `${recentlyAddedTokens.artBottomSpace}px`,
  '--km-recently-added-title-size': `${recentlyAddedTokens.titleFontSize}px`,
  '--km-recently-added-quality-top': `${recentlyAddedTokens.qualityTopSpace}px`,
  '--km-recently-added-quality-size': `${recentlyAddedTokens.qualityFontSize}px`,
  '--km-recently-added-quality-line-height': String(recentlyAddedTokens.qualityLineHeight),
}
