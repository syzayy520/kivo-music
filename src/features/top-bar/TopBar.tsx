import './topBar.css'

export function TopBar() {
  return (
    <header className="km-topbar">
      <div>
        <p>Listen Now</p>
        <h1>Your music, beautifully alive.</h1>
      </div>
      <label className="km-search" aria-label="Search library">
        <span>Search</span>
        <input placeholder="Albums, artists, lyrics, quality" />
      </label>
      <div className="km-topbar-status">
        <span>Local Library</span>
        <strong>Lossless Ready</strong>
      </div>
    </header>
  )
}
