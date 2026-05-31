export const topBarTokens = {
  minHeight: 72,
  titleMinColumn: 480,
  searchMinColumn: 240,
  searchMaxColumn: 360,
  compactTitleMinColumn: 360,
  compactSearchMinColumn: 220,
  compactSearchMaxColumn: 300,
  gap: 16,
  paddingTop: 20,
  paddingBottom: 12,
  searchGap: 9,
  searchPaddingBlock: 9,
  searchPaddingInline: 13,
  statusPaddingBlock: 9,
  statusPaddingInline: 12,
  statusRadius: 14,
  statusDotSize: 6,
  statusDotGap: 7,
} as const

export type TopBarTokens = typeof topBarTokens
