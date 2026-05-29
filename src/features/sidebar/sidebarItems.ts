export type SidebarGroup = {
  title: string
  items: SidebarItem[]
}

export type SidebarItem = {
  id: string
  label: string
  badge?: string
}

export const sidebarGroups: SidebarGroup[] = [
  {
    title: 'For You',
    items: [{ id: 'listen-now', label: 'Listen Now' }],
  },
  {
    title: 'Library',
    items: [
      { id: 'albums', label: 'Albums' },
      { id: 'artists', label: 'Artists' },
      { id: 'songs', label: 'Songs' },
      { id: 'playlists', label: 'Playlists' },
      { id: 'recently-added', label: 'Recently Added' },
    ],
  },
  {
    title: 'Playlists',
    items: [
      { id: 'liked', label: 'Liked Songs' },
      { id: 'midnight-drive', label: 'Midnight Drive' },
      { id: 'focus-room', label: 'Focus Room' },
    ],
  },
  {
    title: 'System',
    items: [
      { id: 'now-playing', label: 'Now Playing' },
      { id: 'settings', label: 'Settings' },
    ],
  },
]
