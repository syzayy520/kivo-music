export const recentlyAddedTokens = {
  cardMinWidth: 132,
  cardPreferredWidth: '16vw',
  cardMaxWidth: 176,
  gap: 14,
  bottomPadding: 8,
  artRadius: 14,
  artBottomSpace: 10,
  titleFontSize: 13,
  qualityTopSpace: 4,
  qualityFontSize: 12,
  qualityLineHeight: 1.35,
} as const

export type RecentlyAddedTokens = typeof recentlyAddedTokens
