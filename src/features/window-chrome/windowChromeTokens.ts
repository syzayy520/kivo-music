export const windowChromeTokens = {
  height: 28,
  controlWidth: 42,
  controlFontSize: 13,
  controlColor: 'rgba(255, 255, 255, 0.48)',
  controlHoverColor: 'var(--km-text)',
  controlHoverBackground: 'rgba(255, 255, 255, 0.08)',
  dangerHoverBackground: '#e81123',
} as const

export type WindowChromeTokens = typeof windowChromeTokens
