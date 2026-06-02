import { ACTIVE_HOME_EXPERIENCE_SHELL_MODE } from '../home/homeExperienceRegistry'
import { t } from '../../shared/i18n'
import { SidebarIcon } from './SidebarIcon'
import { sidebarGroups } from './sidebarItems'
import { sidebarStyle } from './sidebarStyle'
import './sidebar.css'
import './sidebarPremiumLocal.css'

export function Sidebar() {
  const isPremiumLocal = ACTIVE_HOME_EXPERIENCE_SHELL_MODE === 'premiumLocal'

  return (
    <aside className="km-sidebar" aria-label={t('sidebar.ariaLabel')} style={sidebarStyle}>
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
                {isPremiumLocal ? <SidebarIcon id={item.id} /> : null}
                <span>{t(item.labelKey)}</span>
              </a>
            ))}
          </section>
        ))}
      </nav>
      {isPremiumLocal ? (
        <section className="km-sidebar-premium-card" aria-label="本地用户入口">
          <div>
            <strong>Danny Rico</strong>
          </div>
          <span className="km-sidebar-premium-chevron" aria-hidden="true">⌄</span>
        </section>
      ) : null}
    </aside>
  )
}
