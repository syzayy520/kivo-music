export const designTokenContract = {
  windowMinWidth: 1040,
  windowMinHeight: 720,
  sidebarWidth: 250,
  sidebarCompactWidth: 224,
  playerHeight: 92,
  playerMinHeight: 74,
  stageBottomSafeArea: 182,
} as const

export type DesignTokenContract = typeof designTokenContract
