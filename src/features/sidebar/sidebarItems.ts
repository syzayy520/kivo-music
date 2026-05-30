export type SidebarItem = {
  id: string
  label: string
}

export type SidebarGroup = {
  title: string
  items: SidebarItem[]
}

export const sidebarGroups: SidebarGroup[] = [
  {
    title: 'For You',
    items: [{ id: 'listen-now', label: 'Home' }],
  },
  {
    title: 'Library',
    items: [
      { id: 'albums', label: 'Albums' },
      { id: 'artists', label: 'Artists' },
      { id: 'songs', label: 'Songs' },
      { id: 'playlists', label: 'Playlists' },
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
