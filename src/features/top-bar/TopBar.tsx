import { topBarCopy } from './topBarCopy'
import './topBar.css'

export function TopBar() {
  return (
    <header className="km-topbar">
      <div className="km-topbar-title">
        <h1>{topBarCopy.title}</h1>
      </div>
      <label className="km-search">
        <span>{topBarCopy.searchLabel}</span>
        <input aria-label="Search library" placeholder={topBarCopy.searchPlaceholder} />
      </label>
      <div className="km-library-status">
        <small>{topBarCopy.statusLabel}</small>
        <strong>{topBarCopy.statusValue}</strong>
      </div>
    </header>
  )
}
