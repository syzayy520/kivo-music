export const libraryStatTokens = {
  columns: 4,
  gap: 10,
  bottomSpace: 22,
  padding: 14,
  borderColor: 'rgba(255, 255, 255, 0.07)',
  radius: 16,
  background: 'rgba(255, 255, 255, 0.035)',
  valueFontSize: 'clamp(20px, 2.2vw, 24px)',
  valueTracking: '-0.04em',
} as const

export type LibraryStatTokens = typeof libraryStatTokens
