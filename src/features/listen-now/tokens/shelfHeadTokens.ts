export const shelfHeadTokens = {
  gap: 10,
  bottomSpace: 10,
  eyebrowFontSize: 10,
  eyebrowWeight: 780,
  eyebrowTracking: '0.08em',
  titleTopSpace: 3,
  titleFontSize: 18,
  titleTracking: '-0.035em',
  buttonPaddingY: 8,
  buttonPaddingX: 14,
  darkButtonBackground: 'rgba(255, 255, 255, 0.07)',
} as const

export type ShelfHeadTokens = typeof shelfHeadTokens
