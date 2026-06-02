type SidebarIconProps = {
  id: string
}

export function SidebarIcon({ id }: SidebarIconProps) {
  switch (id) {
    case 'albums':
      return <Icon><rect x="5" y="4" width="14" height="16" rx="3" /><path d="M9 8h6" /><path d="M9 12h4" /></Icon>
    case 'artists':
      return <Icon><circle cx="12" cy="8" r="3" /><path d="M6.5 19c.8-3.3 2.7-5 5.5-5s4.7 1.7 5.5 5" /></Icon>
    case 'songs':
      return <Icon><path d="M9 18V6l9-2v11" /><circle cx="7" cy="18" r="2" /><circle cx="16" cy="15" r="2" /></Icon>
    case 'playlists':
      return <Icon><path d="M5 7h10" /><path d="M5 12h9" /><path d="M5 17h6" /><path d="m16 15 3 2-3 2v-4Z" /></Icon>
    case 'now-playing':
      return <Icon><path d="M6 17V9" /><path d="M10 17V6" /><path d="M14 17v-5" /><path d="M18 17V8" /></Icon>
    case 'settings':
      return <Icon><circle cx="12" cy="12" r="3" /><path d="M19 12a7 7 0 0 0-.1-1.2l2-1.5-2-3.4-2.4 1a7 7 0 0 0-2-1.1L14 3h-4l-.5 2.8a7 7 0 0 0-2 1.1l-2.4-1-2 3.4 2 1.5A7 7 0 0 0 5 12c0 .4 0 .8.1 1.2l-2 1.5 2 3.4 2.4-1a7 7 0 0 0 2 1.1L10 21h4l.5-2.8a7 7 0 0 0 2-1.1l2.4 1 2-3.4-2-1.5c.1-.4.1-.8.1-1.2Z" /></Icon>
    case 'listen-now':
    default:
      return <Icon><path d="M5 11.5 12 5l7 6.5" /><path d="M7 10.5V19h10v-8.5" /><path d="M10 19v-5h4v5" /></Icon>
  }
}

function Icon({ children }: { children: React.ReactNode }) {
  return (
    <svg className="km-sidebar-premium-icon" viewBox="0 0 24 24" aria-hidden="true">
      {children}
    </svg>
  )
}
