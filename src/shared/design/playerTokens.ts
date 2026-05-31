export const playerTokens = {
  barHeight: 92,
  minBarHeight: 74,
  floatingBottom: 16,
  minTrackColumn: 260,
  maxTrackColumn: 320,
  minCoreColumn: 360,
  minMetaColumn: 210,
  maxMetaColumn: 280,
} as const

export type PlayerTokens = typeof playerTokens
