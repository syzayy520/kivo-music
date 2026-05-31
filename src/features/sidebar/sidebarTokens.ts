export const sidebarTokens = {
  paddingBlock: 22,
  paddingInline: 14,
  brandGap: 10,
  brandBottomSpace: 26,
  brandMarkSize: 32,
  brandMarkRadius: 10,
  navGap: 18,
  navLabelBottomSpace: 7,
  navItemBlockPadding: 8,
  navItemInlinePadding: 10,
  navItemRadius: 10,
} as const

export type SidebarTokens = typeof sidebarTokens
