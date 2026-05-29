import { sidebarGroups } from './sidebarItems'
import './sidebar.css'

export function Sidebar() {
  return (
    <aside className="km-sidebar" aria-label="Kivo Music navigation">
      <div className="km-brand">
        <span className="km-brand-mark">K</span>
        <div>
          <strong>Kivo Music</strong>
          <small>Flagship local player</small>
        </div>
      </div>

      <nav className="km-sidebar-nav">
        {sidebarGroups.map((group) => (
          <section className="km-sidebar-group" key={group.title}>
            <p>{group.title}</p>
            {group.items.map((item) => (
              <button
                className={item.id === 'listen-now' ? 'active' : ''}
                key={item.id}
                type="button"
              >
                <span>{item.label}</span>
                {item.badge ? <em>{item.badge}</em> : null}
              </button>
            ))}
          </section>
        ))}
      </nav>
    </aside>
  )
}
