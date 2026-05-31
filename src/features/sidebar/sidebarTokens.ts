export const sidebarTokens = {
  paddingBlock: 22,
  paddingInline: 14,
  brandGap: 10,
  brandBottomSpace: 26,
  brandMarkSize: 32,
  brandMarkRadius: 10,
  brandNameFontSize: 13,
  brandTaglineFontSize: 11,
  navGap: 18,
  navLabelBottomSpace: 7,
  navLabelFontSize: 10,
  navItemVerticalMargin: 1,
  navItemBlockPadding: 8,
  navItemInlinePadding: 10,
  navItemRadius: 10,
  navItemFontSize: 13,
} as const

export type SidebarTokens = typeof sidebarTokens
