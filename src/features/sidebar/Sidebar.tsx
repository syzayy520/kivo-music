import { t } from '../../shared/i18n'
import { sidebarGroups } from './sidebarItems'
import { sidebarStyle } from './sidebarStyle'
import './sidebar.css'

export function Sidebar() {
  return (
    <aside className="km-sidebar" aria-label="Navigation" style={sidebarStyle}>
      <div className="km-brand">
        <span>K</span>
        <div>
          <strong>{t('sidebar.brandName')}</strong>
          <small>{t('sidebar.brandTagline')}</small>
        </div>
      </div>
      <nav className="km-nav">
        {sidebarGroups.map((group) => (
          <section key={group.titleKey}>
            <p>{t(group.titleKey)}</p>
            {group.items.map((item) => (
              <a className={item.id === 'listen-now' ? 'active' : ''} href={`#${item.id}`} key={item.id}>
                {t(item.labelKey)}
              </a>
            ))}
          </section>
        ))}
      </nav>
    </aside>
  )
}
