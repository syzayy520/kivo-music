export const albumShelfTokens = {
  gridGap: 16,
  shelfLastBottomSpace: 182,
  cardMinWidth: 116,
  cardMaxWidth: 132,
  cardRadius: 13,
  cardTransitionMs: 160,
  cardHoverLift: '-2px',
  cardActiveLift: '-1px',
  cardActiveScale: 0.985,
  cardActiveOpacity: 0.72,
  artRadius: 11,
  artBottomSpace: 8,
  titleFontSize: 12,
  titleWeight: 760,
  titleTracking: '-0.01em',
  artistTopSpace: 2,
  artistFontSize: 12,
  qualityTopSpace: 3,
  qualityFontSize: 11,
} as const

export type AlbumShelfTokens = typeof albumShelfTokens
