export const shellTokens = {
  appMinWidth: 1040,
  appMinHeight: 720,
  topRowHeight: 28,
  sidebarWidth: 250,
  sidebarCompactWidth: 224,
  stageBottomSpace: 182,
  stageScrollbarWidth: 7,
  stageScrollbarThumbBorder: 3,
  stageScrollbarColor: 'rgba(28, 24, 30, 0.03)',
  stageScrollbarHoverColor: 'rgba(28, 24, 30, 0.11)',
  darkStageScrollbarColor: 'rgba(255, 255, 255, 0.05)',
  darkStageScrollbarHoverColor: 'rgba(255, 255, 255, 0.16)',
} as const

export type ShellTokens = typeof shellTokens
