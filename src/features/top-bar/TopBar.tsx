import { ACTIVE_HOME_EXPERIENCE_SHELL_MODE } from '../home/homeExperienceRegistry'
import { t } from '../../shared/i18n'
import { topBarCopy } from './topBarCopy'
import { topBarStyle } from './topBarStyle'
import './topBar.css'
import './topBarPremiumLocal.css'

function TopBarMoreIcon() {
  return (
    <svg className="km-topbar-more-icon" viewBox="0 0 24 24" aria-hidden="true">
      <circle cx="5" cy="12" r="1.7" />
      <circle cx="12" cy="12" r="1.7" />
      <circle cx="19" cy="12" r="1.7" />
    </svg>
  )
}

export function TopBar() {
  const showPremiumLocalControls = ACTIVE_HOME_EXPERIENCE_SHELL_MODE === 'premiumLocal'

  return (
    <header className="km-topbar" style={topBarStyle}>
      <div className="km-topbar-title">
        <h1>{t(topBarCopy.titleKey)}</h1>
      </div>
      <label className="km-search">
        <span>{t(topBarCopy.searchLabelKey)}</span>
        <input
          aria-label={t(topBarCopy.searchAriaLabelKey)}
          placeholder={t(topBarCopy.searchPlaceholderKey)}
        />
      </label>
      <div className="km-library-status">
        <small>{t(topBarCopy.statusLabelKey)}</small>
        <strong>{t(topBarCopy.statusValueKey)}</strong>
      </div>
      {showPremiumLocalControls ? (
        <button className="km-topbar-more" type="button" aria-label="More options">
          <TopBarMoreIcon />
        </button>
      ) : null}
    </header>
  )
}
