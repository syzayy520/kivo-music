export const listenNowTokens = {
  sectionBottomSpace: 20,
  headBottomSpace: 12,
  topPicksGap: 16,
  topPickRadius: 16,
  topPickCopyTop: 16,
  topPickCopyInline: 18,
  topPickCopyBottom: 20,
  topPickPlayRight: 16,
  topPickPlayBottom: 18,
  topPickPlaySize: 36,
  shelfHeadGap: 10,
  shelfHeadBottomSpace: 10,
  shelfButtonPaddingY: 8,
  shelfButtonPaddingX: 14,
  albumGridGap: 16,
  albumShelfLastBottomSpace: 182,
  albumCardRadius: 13,
  albumArtRadius: 11,
  albumArtBottomSpace: 8,
} as const

export type ListenNowTokens = typeof listenNowTokens
