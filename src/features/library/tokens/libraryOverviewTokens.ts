export const libraryOverviewTokens = {
  margin: '0 clamp(20px, 3vw, 32px) 24px',
  padding: 'clamp(18px, 2.2vw, 22px)',
  borderColor: 'rgba(255, 255, 255, 0.07)',
  radius: 28,
  background: 'rgba(255, 255, 255, 0.035)',
  eyebrowFontSize: 11,
  eyebrowWeight: 780,
  eyebrowTracking: '0.08em',
  titleTopSpace: 4,
  titleBottomSpace: 18,
  titleFontSize: 'clamp(22px, 2.2vw, 24px)',
  titleTracking: '-0.04em',
} as const

export type LibraryOverviewTokens = typeof libraryOverviewTokens
