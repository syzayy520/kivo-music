import type { CSSProperties } from 'react'
import { libraryTokens } from './libraryTokens'

type LibraryStyle = CSSProperties & Record<`--${string}`, string>

export const libraryStyle: LibraryStyle = {
  '--km-library-overview-margin': libraryTokens.overviewMargin,
  '--km-library-overview-padding': libraryTokens.overviewPadding,
  '--km-library-overview-border-color': libraryTokens.overviewBorderColor,
  '--km-library-overview-radius': `${libraryTokens.overviewRadius}px`,
  '--km-library-overview-bg': libraryTokens.overviewBackground,
  '--km-library-eyebrow-size': `${libraryTokens.eyebrowFontSize}px`,
  '--km-library-eyebrow-weight': String(libraryTokens.eyebrowWeight),
  '--km-library-eyebrow-letter-spacing': libraryTokens.eyebrowTracking,
  '--km-library-title-top': `${libraryTokens.titleTopSpace}px`,
  '--km-library-title-bottom': `${libraryTokens.titleBottomSpace}px`,
  '--km-library-title-size': libraryTokens.titleFontSize,
  '--km-library-title-letter-spacing': libraryTokens.titleTracking,
  '--km-library-stats-columns': String(libraryTokens.statsColumns),
  '--km-library-stats-gap': `${libraryTokens.statsGap}px`,
  '--km-library-stats-bottom': `${libraryTokens.statsBottomSpace}px`,
}
