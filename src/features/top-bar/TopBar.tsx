import { t } from '../../shared/i18n'
import { topBarCopy } from './topBarCopy'
import './topBar.css'

export function TopBar() {
  return (
    <header className="km-topbar">
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
    </header>
  )
}
