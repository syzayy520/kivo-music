export const shellTokens = {
  appMinWidth: 1040,
  appMinHeight: 720,
  topRowHeight: 28,
  sidebarWidth: 250,
  sidebarCompactWidth: 224,
  stageBottomSpace: 182,
  stageScrollbarWidth: 7,
  stageScrollbarThumbBorder: 3,
} as const

export type ShellTokens = typeof shellTokens
