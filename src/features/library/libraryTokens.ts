import { libraryOverviewTokens } from './tokens/libraryOverviewTokens'
import { libraryStatTokens } from './tokens/libraryStatTokens'

export const libraryTokens = {
  overviewMargin: libraryOverviewTokens.margin,
  overviewPadding: libraryOverviewTokens.padding,
  overviewBorderColor: libraryOverviewTokens.borderColor,
  overviewRadius: libraryOverviewTokens.radius,
  overviewBackground: libraryOverviewTokens.background,
  eyebrowFontSize: libraryOverviewTokens.eyebrowFontSize,
  eyebrowWeight: libraryOverviewTokens.eyebrowWeight,
  eyebrowTracking: libraryOverviewTokens.eyebrowTracking,
  titleTopSpace: libraryOverviewTokens.titleTopSpace,
  titleBottomSpace: libraryOverviewTokens.titleBottomSpace,
  titleFontSize: libraryOverviewTokens.titleFontSize,
  titleTracking: libraryOverviewTokens.titleTracking,
  statsColumns: libraryStatTokens.columns,
  statsGap: libraryStatTokens.gap,
  statsBottomSpace: libraryStatTokens.bottomSpace,
  statPadding: libraryStatTokens.padding,
  statBorderColor: libraryStatTokens.borderColor,
  statRadius: libraryStatTokens.radius,
  statBackground: libraryStatTokens.background,
  statValueFontSize: libraryStatTokens.valueFontSize,
  statValueTracking: libraryStatTokens.valueTracking,
} as const

export type LibraryTokens = typeof libraryTokens
