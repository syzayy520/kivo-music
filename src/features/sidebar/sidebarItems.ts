import type { TranslationKey } from '../../shared/i18n'

export type SidebarItem = {
  id: string
  labelKey: TranslationKey
}

export type SidebarGroup = {
  titleKey: TranslationKey
  items: SidebarItem[]
}

export const sidebarGroups: SidebarGroup[] = [
  {
    titleKey: 'sidebar.group.forYou',
    items: [{ id: 'listen-now', labelKey: 'sidebar.home' }],
  },
  {
    titleKey: 'sidebar.group.library',
    items: [
      { id: 'albums', labelKey: 'sidebar.albums' },
      { id: 'artists', labelKey: 'sidebar.artists' },
      { id: 'songs', labelKey: 'sidebar.songs' },
      { id: 'playlists', labelKey: 'sidebar.playlists' },
    ],
  },
  {
    titleKey: 'sidebar.group.system',
    items: [
      { id: 'now-playing', labelKey: 'sidebar.nowPlaying' },
      { id: 'settings', labelKey: 'sidebar.settings' },
    ],
  },
]
