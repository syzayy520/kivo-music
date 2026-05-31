import type { CSSProperties } from 'react'
import { libraryStatCardTokens } from './libraryStatCardTokens'

type LibraryStatCardStyle = CSSProperties & Record<`--${string}`, string>

export const libraryStatCardStyle: LibraryStatCardStyle = {
  '--km-library-stat-card-padding': `${libraryStatCardTokens.padding}px`,
  '--km-library-stat-card-border-color': libraryStatCardTokens.borderColor,
  '--km-library-stat-card-radius': `${libraryStatCardTokens.radius}px`,
  '--km-library-stat-card-bg': libraryStatCardTokens.background,
  '--km-library-stat-card-value-size': libraryStatCardTokens.valueFontSize,
  '--km-library-stat-card-value-letter-spacing': libraryStatCardTokens.valueTracking,
}
