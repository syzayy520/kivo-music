import { libraryOverviewTokens } from './tokens/libraryOverviewTokens'
import { libraryStatsTokens } from './tokens/libraryStatsTokens'

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
  statsColumns: libraryStatsTokens.columns,
  statsGap: libraryStatsTokens.gap,
  statsBottomSpace: libraryStatsTokens.bottomSpace,
} as const

export type LibraryTokens = typeof libraryTokens
