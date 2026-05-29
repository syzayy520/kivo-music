import { sidebarGroups } from './sidebarItems'
import './sidebar.css'

export function Sidebar() {
  return (
    <aside className="km-sidebar" aria-label="Navigation">
      <div className="km-brand">
        <span>K</span>
        <div>
          <strong>Kivo Music</strong>
          <small>Local library player</small>
        </div>
      </div>
      <nav className="km-nav">
        {sidebarGroups.map((group) => (
          <section key={group.title}>
            <p>{group.title}</p>
            {group.items.map((item) => (
              <a className={item.id === 'listen-now' ? 'active' : ''} href={`#${item.id}`} key={item.id}>
                {item.label}
              </a>
            ))}
          </section>
        ))}
      </nav>
    </aside>
  )
}
